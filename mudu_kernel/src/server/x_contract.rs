use async_trait::async_trait;
use futures::executor::block_on;
use mudu::common::buf::Buf;
use mudu::common::id::{AttrIndex, OID};
use mudu::common::result::RS;
use mudu::error::ec::EC;
use mudu::m_error;
use mudu_contract::tuple::build_tuple::build_tuple;
use mudu_contract::tuple::tuple_binary::TupleBinary as TupleRaw;
use mudu_contract::tuple::update_tuple::update_tuple;
use std::ops::Bound;
use std::sync::{Arc, Mutex};

use crate::contract::meta_mgr::MetaMgr;
use crate::contract::schema_table::SchemaTable;
use crate::contract::table_desc::TableDesc;
use crate::meta::meta_mgr_factory::MetaMgrFactory;
use crate::server::worker_snapshot::{KvItem, WorkerSnapshot, WorkerSnapshotMgr};
use crate::server::worker_storage::WorkerStorage;
use crate::server::worker_tx_manager::WorkerTxManager;
use crate::server::x_lock_mgr::XLockMgr;
use crate::wal::worker_log::ChunkedWorkerLogBackend;
use crate::wal::xl_batch::{new_xl_batch_writer, XLBatch};
use crate::x_engine::api::{
    AlterTable, Filter, OptDelete, OptInsert, OptRead, OptUpdate, Predicate, RSCursor, RangeData,
    TupleRow, VecDatum, VecSelTerm, XContract,
};
use crate::x_engine::tx_mgr::TxMgr;
type DatBin = Buf;

pub struct IoUringXContract {
    meta_mgr: Arc<dyn MetaMgr>,
    storage: Arc<WorkerStorage>,
    log: Option<ChunkedWorkerLogBackend>,
    snapshot_mgr: WorkerSnapshotMgr,
    tx_lock: XLockMgr,
    // commit_gate: AsyncMutex<()>,
}

struct VecCursor {
    inner: Mutex<VecCursorInner>,
}

struct VecCursorInner {
    rows: Vec<TupleRow>,
    index: usize,
}

impl IoUringXContract {
    pub fn new(meta_mgr: Arc<dyn MetaMgr>) -> Self {
        Self::with_log_and_data_dir(meta_mgr, None, 0, default_worker_storage_data_dir())
    }

    pub fn with_log(meta_mgr: Arc<dyn MetaMgr>, log: Option<ChunkedWorkerLogBackend>) -> Self {
        Self::with_log_and_data_dir(meta_mgr, log, 0, default_worker_storage_data_dir())
    }

    pub fn with_log_and_data_dir(
        meta_mgr: Arc<dyn MetaMgr>,
        log: Option<ChunkedWorkerLogBackend>,
        partition_id: OID,
        data_dir: String,
    ) -> Self {
        let storage = Arc::new(WorkerStorage::new(meta_mgr.clone(), partition_id, data_dir));
        storage.register_global();
        storage
            .bootstrap_existing_tables_sync()
            .unwrap_or_else(|e| panic!("bootstrap worker storage from meta failed: {e}"));
        Self {
            meta_mgr: meta_mgr.clone(),
            storage,
            log,
            snapshot_mgr: WorkerSnapshotMgr::default(),
            tx_lock: XLockMgr::new(),
        }
    }

    pub fn with_worker_log(log: ChunkedWorkerLogBackend) -> Self {
        Self::with_worker_log_and_data_dir(log, 0, default_worker_storage_data_dir())
    }

    pub fn with_worker_log_and_data_dir(
        log: ChunkedWorkerLogBackend,
        partition_id: OID,
        data_dir: String,
    ) -> Self {
        let meta_mgr = MetaMgrFactory::create(data_dir.clone())
            .unwrap_or_else(|e| panic!("create worker meta manager failed: {e}"));
        Self::with_log_and_data_dir(meta_mgr, Some(log.clone()), partition_id, data_dir)
    }

    pub fn worker_log(&self) -> Option<ChunkedWorkerLogBackend> {
        self.log.clone()
    }

    pub fn worker_begin_tx(&self) -> RS<Arc<dyn TxMgr>> {
        Ok(Arc::new(WorkerTxManager::new(self.snapshot_mgr.begin_tx())))
    }

    pub fn worker_rollback_tx(&self, tx_mgr: Arc<dyn TxMgr>) -> RS<()> {
        self.snapshot_mgr.end_tx(tx_mgr.xid())
    }

    pub fn worker_put(&self, key: Vec<u8>, value: Vec<u8>) -> RS<()> {
        let prepared = {
            let xid = self.snapshot_mgr.alloc_committed_ts();
            (
                self.storage.clone(),
                self.log.clone(),
                self.storage.prepare_worker_kv_autocommit(
                    xid,
                    key.clone(),
                    Some(value.clone()),
                    single_put_batch(xid, key, value),
                ),
            )
        };
        let (storage, log, prepared) = prepared;
        if let Some(log) = log {
            new_xl_batch_writer(log).append_sync(prepared.batch())?;
        }
        storage.apply_prepared_commit(prepared)
    }

    pub async fn worker_put_async(&self, key: Vec<u8>, value: Vec<u8>) -> RS<()> {
        let (storage, log, prepared) = {
            let xid = self.snapshot_mgr.alloc_committed_ts();
            (
                self.storage.clone(),
                self.log.clone(),
                self.storage.prepare_worker_kv_autocommit(
                    xid,
                    key.clone(),
                    Some(value.clone()),
                    single_put_batch(xid, key, value),
                ),
            )
        };
        if let Some(log) = log {
            new_xl_batch_writer(log).append(prepared.batch()).await?;
        }
        storage.apply_prepared_commit_async(prepared).await
    }

    pub fn worker_delete(&self, key: &[u8]) -> RS<()> {
        let key = key.to_vec();
        let prepared = {
            let xid = self.snapshot_mgr.alloc_committed_ts();
            (
                self.storage.clone(),
                self.log.clone(),
                self.storage.prepare_worker_kv_autocommit(
                    xid,
                    key.clone(),
                    None,
                    single_delete_batch(xid, key),
                ),
            )
        };
        let (storage, log, prepared) = prepared;
        if let Some(log) = log {
            new_xl_batch_writer(log).append_sync(prepared.batch())?;
        }
        storage.apply_prepared_commit(prepared)
    }

    pub async fn worker_delete_async(&self, key: &[u8]) -> RS<()> {
        let key = key.to_vec();
        let (storage, log, prepared) = {
            let xid = self.snapshot_mgr.alloc_committed_ts();
            (
                self.storage.clone(),
                self.log.clone(),
                self.storage.prepare_worker_kv_autocommit(
                    xid,
                    key.clone(),
                    None,
                    single_delete_batch(xid, key),
                ),
            )
        };
        if let Some(log) = log {
            new_xl_batch_writer(log).append(prepared.batch()).await?;
        }
        storage.apply_prepared_commit_async(prepared).await
    }

    pub async fn worker_get_async(&self, key: &[u8]) -> RS<Option<Vec<u8>>> {
        self.storage.kv_get(key, None).await
    }

    pub async fn worker_get_with_snapshot_async(
        &self,
        snapshot: &WorkerSnapshot,
        key: &[u8],
    ) -> RS<Option<Vec<u8>>> {
        self.storage.kv_get(key, Some(snapshot)).await
    }

    pub fn worker_get(&self, key: &[u8]) -> RS<Option<Vec<u8>>> {
        block_on(self.storage.kv_get(key, None))
    }

    pub fn worker_get_with_snapshot(
        &self,
        snapshot: &WorkerSnapshot,
        key: &[u8],
    ) -> RS<Option<Vec<u8>>> {
        block_on(self.storage.kv_get(key, Some(snapshot)))
    }

    pub fn worker_range_scan(&self, start_key: &[u8], end_key: &[u8]) -> RS<Vec<KvItem>> {
        block_on(self.storage.kv_range(start_key, end_key, None))
    }

    pub async fn worker_range_scan_async(
        &self,
        start_key: &[u8],
        end_key: &[u8],
    ) -> RS<Vec<KvItem>> {
        self.storage.kv_range(start_key, end_key, None).await
    }

    pub fn worker_range_scan_with_snapshot(
        &self,
        snapshot: &WorkerSnapshot,
        start_key: &[u8],
        end_key: &[u8],
    ) -> RS<Vec<KvItem>> {
        block_on(self.storage.kv_range(start_key, end_key, Some(snapshot)))
    }

    pub async fn worker_range_scan_with_snapshot_async(
        &self,
        snapshot: &WorkerSnapshot,
        start_key: &[u8],
        end_key: &[u8],
    ) -> RS<Vec<KvItem>> {
        self.storage
            .kv_range(start_key, end_key, Some(snapshot))
            .await
    }

    pub fn worker_commit_put_batch(
        &self,
        snapshot: &WorkerSnapshot,
        xid: u64,
        items: std::collections::BTreeMap<Vec<u8>, Option<Vec<u8>>>,
        batch: XLBatch,
    ) -> RS<()> {
        if items.is_empty() {
            return self.snapshot_mgr.end_tx(xid);
        }
        let (storage, log, prepared) = {
            let prepared = self
                .storage
                .prepare_worker_kv_commit(snapshot, xid, items, batch)?;
            (self.storage.clone(), self.log.clone(), prepared)
        };
        if let Some(log) = log {
            new_xl_batch_writer(log.clone()).append_sync(prepared.batch())?;
            log.flush()?;
        }
        storage.apply_prepared_commit(prepared)?;
        self.snapshot_mgr.end_tx(xid)
    }

    pub async fn worker_commit_put_batch_async(
        &self,
        snapshot: &WorkerSnapshot,
        xid: u64,
        items: std::collections::BTreeMap<Vec<u8>, Option<Vec<u8>>>,
        batch: XLBatch,
    ) -> RS<()> {
        if items.is_empty() {
            return self.snapshot_mgr.end_tx(xid);
        }
        let (storage, log, prepared) = {
            let prepared = self
                .storage
                .prepare_worker_kv_commit(snapshot, xid, items, batch)?;
            (self.storage.clone(), self.log.clone(), prepared)
        };
        if let Some(log) = log {
            new_xl_batch_writer(log.clone())
                .append(prepared.batch())
                .await?;
            log.flush_async().await?;
        }
        storage.apply_prepared_commit_async(prepared).await?;
        self.snapshot_mgr.end_tx(xid)
    }

    pub fn worker_commit_tx(&self, tx: Arc<dyn TxMgr>) -> RS<()> {
        let xid = tx.xid();
        if tx.is_empty() {
            return self.worker_rollback_tx(tx);
        }
        tx.build_write_ops();
        let (storage, log, prepared) = {
            let write_ops = tx.write_ops();
            let can_commit = self.tx_lock.try_lock_some(xid as OID, &write_ops);
            if !can_commit {
                return Err(m_error!(
                    EC::TxErr,
                    format!("transaction {} failed to acquire commit locks", xid)
                ));
            }
            let prepared = self.storage.prepare_commit(tx.as_ref())?;
            (self.storage.clone(), self.log.clone(), prepared)
        };
        let result = (|| -> RS<()> {
            if let Some(log) = log {
                new_xl_batch_writer(log.clone()).append_sync(prepared.batch())?;
                log.flush()?;
            }
            storage.apply_prepared_commit(prepared)?;
            Ok(())
        })();
        let write_ops = tx.write_ops();
        self.tx_lock.release(xid as OID, &write_ops);
        self.worker_rollback_tx(tx)?;
        result
    }

    pub async fn worker_commit_tx_async(&self, tx: Arc<dyn TxMgr>) -> RS<()> {
        let xid = tx.xid();
        if tx.is_empty() {
            return self.worker_rollback_tx(tx);
        }
        tx.build_write_ops();
        let (storage, log, prepared) = {
            let write_ops = tx.write_ops();
            let can_commit = self.tx_lock.try_lock_some(xid as OID, &write_ops);
            if !can_commit {
                return Err(m_error!(
                    EC::TxErr,
                    format!("transaction {} failed to acquire commit locks", xid)
                ));
            }
            let prepared = self.storage.prepare_commit_async(tx.as_ref()).await?;
            (self.storage.clone(), self.log.clone(), prepared)
        };
        let result = async {
            if let Some(log) = log {
                new_xl_batch_writer(log.clone()).append(prepared.batch()).await?;
                log.flush_async().await?;
            }
            storage.apply_prepared_commit_async(prepared).await?;
            Ok(())
        }
        .await;
        let write_ops = tx.write_ops();
        self.tx_lock.release(xid as OID, &write_ops);
        self.worker_rollback_tx(tx)?;
        result
    }

    pub fn replay_worker_log_batch(&self, batch: XLBatch) -> RS<()> {
        let max_xid = batch.entries.iter().map(|entry| entry.xid).max();
        if let Some(max_xid) = max_xid {
            self.snapshot_mgr.observe_committed_ts(max_xid);
        }
        self.storage.replay_batch(batch)
    }
}

fn default_worker_storage_data_dir() -> String {
    std::env::temp_dir()
        .join(format!(
            "mududb-worker-storage-{}",
            mudu::common::id::gen_oid()
        ))
        .to_string_lossy()
        .to_string()
}

impl IoUringXContract {
    fn _begin_tx(&self) -> Arc<dyn TxMgr> {
        Arc::new(WorkerTxManager::new(self.snapshot_mgr.begin_tx()))
    }

    async fn _insert(
        & self,
        desc: Arc<TableDesc>,
        tx_mgr: Arc<dyn TxMgr>,
        table_id: OID,
        keys: &VecDatum,
        values: &VecDatum,
        _opt_insert: &OptInsert,
    ) -> RS<()> {
        let key = build_key_tuple(keys, &desc)?;
        let value = build_value_tuple(values, &desc)?;
        let contain_key =  self.storage.get(table_id, &key, tx_mgr.as_ref()).await?;
        if contain_key.is_some() {
            Err(m_error!(EC::ExistingSuchElement, "existing key"))
        } else {
            self.storage.put(table_id, key, value, tx_mgr.as_ref()).await
        }
    }

    async fn _read_key(
        & self,
        desc: Arc<TableDesc>,
        tx_mgr: Arc<dyn TxMgr>,
        table_id: OID,
        pred_key: &VecDatum,
        select: &VecSelTerm,
        _opt_read: &OptRead,
    ) -> RS<Option<Vec<DatBin>>> {
        let key = build_key_tuple(pred_key, &desc)?;
        let opt_value = self.storage.get(table_id, &key, tx_mgr.as_ref()).await?;
        match opt_value {
            Some(value) => project_selected_fields(&desc, &key, &value, select).map(Some),
            None => Ok(None),
        }
    }

    async fn _read_range(
        & self,
        desc: Arc<TableDesc>,
        tx_mgr: Arc<dyn TxMgr>,
        table_id: OID,
        pred_key: &RangeData,
        pred_non_key: &Predicate,
        select: &VecSelTerm,
        _opt_read: &OptRead,
    ) -> RS<Arc<dyn RSCursor>> {
        ensure_supported_predicate(pred_non_key)?;
        let start = build_bound_key(pred_key.start(), &desc)?;
        let end = build_bound_key(pred_key.end(), &desc)?;
        let rows = self.storage.range(table_id, (start, end), tx_mgr.as_ref()).await?;
        let projected = rows
            .into_iter()
            .map(|(key, value)| {
                project_selected_fields(&desc, &key, &value, select).map(TupleRow::new)
            })
            .collect::<RS<Vec<_>>>()?;
        Ok(Arc::new(VecCursor {
            inner: Mutex::new(VecCursorInner {
                rows: projected,
                index: 0,
            }),
        }))
    }

    async fn _delete(
        & self,
        desc: Arc<TableDesc>,
        tx_mgr: Arc<dyn TxMgr>,
        table_id: OID,
        pred_key: &VecDatum,
        pred_non_key: &Predicate,
        _opt_delete: &OptDelete,
    ) -> RS<usize> {
        ensure_supported_predicate(pred_non_key)?;
        let key = build_key_tuple(pred_key, &desc)?;
        let deleted = self.storage.remove(table_id, &key, tx_mgr.as_ref()).await?;
        Ok(usize::from(deleted.is_some()))
    }

    async fn _update(
        & self,
        desc: Arc<TableDesc>,
        tx_mgr: Arc<dyn TxMgr>,
        table_id: OID,
        pred_key: &VecDatum,
        pred_non_key: &Predicate,
        values: &VecDatum,
        _opt_update: &OptUpdate,
    ) -> RS<usize> {
        ensure_supported_predicate(pred_non_key)?;
        let key = build_key_tuple(pred_key, &desc)?;
        let current = self.storage.get(table_id, &key, tx_mgr.as_ref()).await?;
        let Some(current) = current else {
            return Ok(0);
        };
        let updated = apply_value_update(&current, values, &desc)?;
        self.storage
            .put(table_id, key, updated, tx_mgr.as_ref())
            .await
            .map(|()| 1)
    }
}

#[async_trait]
impl XContract for IoUringXContract {
    async fn create_table(&self, _tx_mgr: Arc<dyn TxMgr>, schema: &SchemaTable) -> RS<()> {
        self.storage.create_table_async(schema).await
    }

    async fn drop_table(&self, _tx_mgr: Arc<dyn TxMgr>, oid: OID) -> RS<()> {
        self.storage.drop_table_async(oid).await
    }

    async fn alter_table(
        &self,
        _tx_mgr: Arc<dyn TxMgr>,
        _oid: OID,
        _alter_table: &AlterTable,
    ) -> RS<()> {
        Err(m_error!(
            EC::NotImplemented,
            "alter table is not implemented"
        ))
    }

    async fn begin_tx(&self) -> RS<Arc<dyn TxMgr>> {
        Ok(self._begin_tx())
    }

    async fn commit_tx(&self, tx_mgr: Arc<dyn TxMgr>) -> RS<()> {
        self.worker_commit_tx_async(tx_mgr).await
    }

    async fn abort_tx(&self, tx_mgr: Arc<dyn TxMgr>) -> RS<()> {
        self.worker_rollback_tx(tx_mgr)
    }

    async fn update(
        &self,
        tx_mgr: Arc<dyn TxMgr>,
        table_id: OID,
        pred_key: &VecDatum,
        pred_non_key: &Predicate,
        values: &VecDatum,
        opt_update: &OptUpdate,
    ) -> RS<usize> {
        let desc = self.meta_mgr.get_table_by_id(table_id).await?;
        self._update(
            desc,
            tx_mgr,
            table_id,
            pred_key,
            pred_non_key,
            values,
            opt_update,
        )
        .await
    }

    async fn read_key(
        &self,
        tx_mgr: Arc<dyn TxMgr>,
        table_id: OID,
        pred_key: &VecDatum,
        select: &VecSelTerm,
        opt_read: &OptRead,
    ) -> RS<Option<Vec<DatBin>>> {
        let desc = self.meta_mgr.get_table_by_id(table_id).await?;
        self._read_key(desc, tx_mgr, table_id, pred_key, select, opt_read)
            .await
    }

    async fn read_range(
        &self,
        tx_mgr: Arc<dyn TxMgr>,
        table_id: OID,
        pred_key: &RangeData,
        pred_non_key: &Predicate,
        select: &VecSelTerm,
        opt_read: &OptRead,
    ) -> RS<Arc<dyn RSCursor>> {
        let desc = self.meta_mgr.get_table_by_id(table_id).await?;
        self._read_range(
            desc,
            tx_mgr,
            table_id,
            pred_key,
            pred_non_key,
            select,
            opt_read,
        )
        .await
    }

    async fn delete(
        &self,
        tx_mgr: Arc<dyn TxMgr>,
        table_id: OID,
        pred_key: &VecDatum,
        pred_non_key: &Predicate,
        opt_delete: &OptDelete,
    ) -> RS<usize> {
        let desc = self.meta_mgr.get_table_by_id(table_id).await?;
        self._delete(desc, tx_mgr, table_id, pred_key, pred_non_key, opt_delete)
            .await
    }

    async fn insert(
        &self,
        tx_mgr: Arc<dyn TxMgr>,
        table_id: OID,
        keys: &VecDatum,
        values: &VecDatum,
        opt_insert: &OptInsert,
    ) -> RS<()> {
        let desc = self.meta_mgr.get_table_by_id(table_id).await?;
        self._insert(desc, tx_mgr, table_id, keys, values, opt_insert)
            .await
    }
}

impl IoUringXContract {
    pub fn meta_mgr(&self) -> Arc<dyn MetaMgr> {
        self.meta_mgr.clone()
    }
}

#[async_trait]
impl RSCursor for VecCursor {
    async fn next(&self) -> RS<Option<TupleRow>> {
        let mut inner = self
            .inner
            .lock()
            .map_err(|_| m_error!(EC::InternalErr, "range cursor lock poisoned"))?;
        if inner.index >= inner.rows.len() {
            return Ok(None);
        }
        let row = inner.rows[inner.index].clone();
        inner.index += 1;
        Ok(Some(row))
    }
}

fn ensure_supported_predicate(predicate: &Predicate) -> RS<()> {
    match predicate {
        Predicate::CNF(items) | Predicate::DNF(items) if items.is_empty() => Ok(()),
        Predicate::CNF(items) | Predicate::DNF(items) => {
            let _ = items
                .iter()
                .flatten()
                .map(|(_oid, _filter): &(AttrIndex, Filter)| ())
                .count();
            Err(m_error!(
                EC::NotImplemented,
                "non-key predicates are not implemented in io_uring xcontract"
            ))
        }
    }
}

fn build_key_tuple(data: &VecDatum, desc: &TableDesc) -> RS<Vec<u8>> {
    build_tuple_for::<true>(data.data(), desc)
}

fn build_value_tuple(data: &VecDatum, desc: &TableDesc) -> RS<Vec<u8>> {
    build_tuple_for::<false>(data.data(), desc)
}

fn build_tuple_for<const IS_KEY: bool>(
    data: &Vec<(AttrIndex, DatBin)>,
    desc: &TableDesc,
) -> RS<Vec<u8>> {
    let mut vec_data = data.clone();
    let mut ok = true;
    vec_data.sort_by(|(id1, _), (id2, _)| {
        let (f1, f2) = (desc.get_attr(*id1), desc.get_attr(*id2));
        if f1.primary_index().is_some() != IS_KEY || f2.primary_index().is_some() != IS_KEY {
            ok = false;
        }
        f1.datum_index().cmp(&f2.datum_index())
    });
    if !ok {
        return Err(m_error!(EC::TupleErr));
    }
    let values: Vec<_> = vec_data.into_iter().map(|(_, v)| v).collect();
    let tuple_desc = if IS_KEY {
        desc.key_desc()
    } else {
        desc.value_desc()
    };
    if tuple_desc.field_count() != values.len() {
        return Err(m_error!(EC::TupleErr));
    }
    build_tuple(&values, tuple_desc)
}

fn build_bound_key(
    bound: &Bound<Vec<(AttrIndex, DatBin)>>,
    desc: &TableDesc,
) -> RS<Bound<&'static [u8]>> {
    match bound {
        Bound::Included(values) => {
            let tuple = build_key_tuple(&VecDatum::new(values.clone()), desc)?;
            Ok(Bound::Included(Box::leak(tuple.into_boxed_slice())))
        }
        Bound::Excluded(values) => {
            let tuple = build_key_tuple(&VecDatum::new(values.clone()), desc)?;
            Ok(Bound::Excluded(Box::leak(tuple.into_boxed_slice())))
        }
        Bound::Unbounded => Ok(Bound::Unbounded),
    }
}

fn project_selected_fields(
    desc: &TableDesc,
    key: &[u8],
    value: &[u8],
    select: &VecSelTerm,
) -> RS<Vec<DatBin>> {
    let mut tuple_ret = vec![];
    for i in select.vec() {
        let f = desc.get_attr(*i);
        let index = f.datum_index();
        let field_desc = if f.primary_index().is_some() {
            desc.key_desc().get_field_desc(index)
        } else {
            desc.value_desc().get_field_desc(index)
        };
        let src = if f.primary_index().is_some() {
            key
        } else {
            value
        };
        let slice = field_desc.get(src)?;
        tuple_ret.push(slice.to_vec());
    }
    Ok(tuple_ret)
}

fn apply_value_update(current: &TupleRaw, values: &VecDatum, desc: &TableDesc) -> RS<Vec<u8>> {
    let mut updated = current.clone();
    let mut data = values.data().clone();
    data.sort_by_key(|(attr, _)| desc.get_attr(*attr).datum_index());
    for (id, dat) in data.iter() {
        let field = desc.get_attr(*id);
        let mut delta = vec![];
        update_tuple(
            field.datum_index() as usize,
            dat,
            desc.value_desc(),
            current,
            &mut delta,
        )?;
        for item in delta {
            item.apply_to(&mut updated);
        }
    }
    Ok(updated)
}

fn single_put_batch(xid: u64, key: Vec<u8>, value: Vec<u8>) -> XLBatch {
    XLBatch {
        entries: vec![crate::wal::xl_entry::XLEntry {
            xid,
            ops: vec![
                crate::wal::xl_entry::TxOp::Begin,
                crate::wal::xl_entry::TxOp::Insert(crate::wal::xl_data_op::XLInsert {
                    table_id: 0,
                    tuple_id: 0,
                    key,
                    value,
                }),
                crate::wal::xl_entry::TxOp::Commit,
            ],
        }],
    }
}

fn single_delete_batch(xid: u64, key: Vec<u8>) -> XLBatch {
    XLBatch {
        entries: vec![crate::wal::xl_entry::XLEntry {
            xid,
            ops: vec![
                crate::wal::xl_entry::TxOp::Begin,
                crate::wal::xl_entry::TxOp::Delete(crate::wal::xl_data_op::XLDelete {
                    table_id: 0,
                    tuple_id: 0,
                    key,
                }),
                crate::wal::xl_entry::TxOp::Commit,
            ],
        }],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contract::schema_column::SchemaColumn;
    use crate::contract::table_info::TableInfo;
    use crate::wal::worker_log::{decode_frames, ChunkedWorkerLogBackend, WorkerLogLayout};
    use crate::wal::xl_data_op::XLInsert;
    use crate::wal::xl_entry::TxOp;
    use futures::executor::block_on;
    use mudu::common::id::gen_oid;
    use mudu_type::dat_type_id::DatTypeID;
    use mudu_type::dt_info::DTInfo;
    use std::collections::HashMap;
    use std::env::temp_dir;

    struct TestMetaMgr {
        tables: Mutex<HashMap<OID, Arc<TableDesc>>>,
    }

    impl TestMetaMgr {
        fn new() -> Self {
            Self {
                tables: Mutex::new(HashMap::new()),
            }
        }
    }

    #[async_trait]
    impl MetaMgr for TestMetaMgr {
        async fn get_table_by_id(&self, oid: OID) -> RS<Arc<TableDesc>> {
            self.tables
                .lock()
                .unwrap()
                .get(&oid)
                .cloned()
                .ok_or_else(|| m_error!(EC::NoSuchElement, format!("no such table {}", oid)))
        }

        async fn get_table_by_name(&self, name: &String) -> RS<Option<Arc<TableDesc>>> {
            Ok(self
                .tables
                .lock()
                .unwrap()
                .values()
                .find(|table| table.name() == name)
                .cloned())
        }

        async fn create_table(&self, schema: &SchemaTable) -> RS<()> {
            let table = TableInfo::new(schema.clone())?.table_desc()?;
            self.tables.lock().unwrap().insert(schema.id(), table);
            Ok(())
        }

        async fn drop_table(&self, table_id: OID) -> RS<()> {
            self.tables.lock().unwrap().remove(&table_id);
            Ok(())
        }
    }

    fn test_schema() -> SchemaTable {
        SchemaTable::new(
            "t".to_string(),
            vec![
                SchemaColumn::new(
                    "id".to_string(),
                    DatTypeID::I32,
                    DTInfo::from_text(DatTypeID::I32, String::new()),
                ),
                SchemaColumn::new(
                    "v".to_string(),
                    DatTypeID::I32,
                    DTInfo::from_text(DatTypeID::I32, String::new()),
                ),
            ],
            vec![0],
            vec![1],
        )
    }

    fn datum(v: i32) -> Vec<u8> {
        v.to_be_bytes().to_vec()
    }

    fn key_row(v: i32) -> VecDatum {
        VecDatum::new(vec![(0, datum(v))])
    }

    fn value_row(v: i32) -> VecDatum {
        VecDatum::new(vec![(1, datum(v))])
    }

    #[test]
    fn relation_commit_log_round_trips() {
        let mgr = Arc::new(TestMetaMgr::new());
        let storage = WorkerStorage::new(
            mgr.clone(),
            0,
            std::env::temp_dir()
                .join(format!(
                    "xcontract_relation_log_{}",
                    mudu::common::id::gen_oid()
                ))
                .to_string_lossy()
                .to_string(),
        );
        let schema = test_schema();
        let table_id = schema.id();
        block_on(storage.create_table_async(&schema)).unwrap();
        let mut txm = WorkerTxManager::new(crate::server::worker_snapshot::WorkerSnapshot::new(
            9,
            vec![],
        ));
        block_on(storage.put(table_id, b"k1".to_vec(), b"v1".to_vec(), &mut txm)).unwrap();
        block_on(storage.remove(table_id, b"k1", &mut txm)).unwrap();
        let prepared = storage.prepare_commit(&txm).unwrap();

        assert_eq!(prepared.batch().entries.len(), 1);
        assert_eq!(prepared.batch().entries[0].xid, 9);
        assert!(matches!(prepared.batch().entries[0].ops[0], TxOp::Begin));
    }

    #[test]
    fn iouring_xcontract_commit_persists_relation_log() {
        let dir = temp_dir().join(format!("iouring_xcontract_log_{}", gen_oid()));
        let layout = WorkerLogLayout::new(dir, gen_oid(), 4096).unwrap();
        let log = ChunkedWorkerLogBackend::new(layout.clone()).unwrap();
        let meta_mgr = Arc::new(TestMetaMgr::new());
        let schema = test_schema();
        let table_id = schema.id();
        let contract = IoUringXContract::with_log(meta_mgr, Some(log));

        let ddl_tx = block_on(contract.begin_tx()).unwrap();
        block_on(contract.create_table(ddl_tx.clone(), &schema)).unwrap();
        block_on(contract.commit_tx(ddl_tx)).unwrap();
        let tx_mgr = block_on(contract.begin_tx()).unwrap();
        block_on(contract.insert(
            tx_mgr.clone(),
            table_id,
            &key_row(1),
            &value_row(10),
            &OptInsert::default(),
        ))
        .unwrap();
        block_on(contract.commit_tx(tx_mgr)).unwrap();

        let bytes = std::fs::read(layout.chunk_path(0)).unwrap();
        let frames = decode_frames(&bytes).unwrap();
        let decoded = crate::wal::xl_batch::decode_xl_batches(&frames).unwrap();
        assert_eq!(decoded.len(), 1);
        let insert = decoded[0].entries[0]
            .ops
            .iter()
            .find_map(|op| match op {
                TxOp::Insert(insert) => Some(insert),
                _ => None,
            })
            .unwrap();
        assert_eq!(insert.table_id, table_id);
        assert_eq!(
            insert.key,
            build_key_tuple(&key_row(1), &meta_table(&schema).unwrap()).unwrap()
        );
        assert_eq!(
            insert.value,
            build_value_tuple(&value_row(10), &meta_table(&schema).unwrap()).unwrap()
        );
    }

    #[test]
    fn iouring_xcontract_replay_restores_worker_kv_and_relation_rows() {
        let meta_mgr = Arc::new(TestMetaMgr::new());
        let schema = test_schema();
        let table_id = schema.id();
        let contract = IoUringXContract::with_log(meta_mgr, None);

        let tx_mgr = block_on(contract.begin_tx()).unwrap();
        block_on(contract.create_table(tx_mgr.clone(), &schema)).unwrap();
        block_on(contract.commit_tx(tx_mgr)).unwrap();
        let batch = XLBatch {
            entries: vec![crate::wal::xl_entry::XLEntry {
                xid: 11,
                ops: vec![
                    TxOp::Begin,
                    TxOp::Insert(XLInsert {
                        table_id: 0,
                        tuple_id: 0,
                        key: b"wk".to_vec(),
                        value: b"wv".to_vec(),
                    }),
                    TxOp::Insert(XLInsert {
                        table_id,
                        tuple_id: 0,
                        key: build_key_tuple(&key_row(3), &meta_table(&schema).unwrap()).unwrap(),
                        value: build_value_tuple(&value_row(30), &meta_table(&schema).unwrap())
                            .unwrap(),
                    }),
                    TxOp::Commit,
                ],
            }],
        };

        contract.replay_worker_log_batch(batch).unwrap();

        assert_eq!(contract.worker_get(b"wk").unwrap(), Some(b"wv".to_vec()));

        let xid = block_on(contract.begin_tx()).unwrap();
        let relation = block_on(contract.read_key(
            xid,
            table_id,
            &key_row(3),
            &VecSelTerm::new(vec![1]),
            &OptRead::default(),
        ))
        .unwrap();
        assert_eq!(relation, Some(vec![datum(30)]));
    }

    #[test]
    fn iouring_xcontract_replay_applies_worker_kv_delete() {
        let contract = IoUringXContract::with_worker_log(
            ChunkedWorkerLogBackend::new(
                WorkerLogLayout::new(
                    temp_dir().join(format!("iouring_xcontract_worker_log_{}", gen_oid())),
                    gen_oid(),
                    4096,
                )
                .unwrap(),
            )
            .unwrap(),
        );

        contract.worker_put(b"wk".to_vec(), b"wv".to_vec()).unwrap();
        let batch = XLBatch {
            entries: vec![crate::wal::xl_entry::XLEntry {
                xid: 7,
                ops: vec![
                    TxOp::Begin,
                    TxOp::Delete(crate::wal::xl_data_op::XLDelete {
                        table_id: 0,
                        tuple_id: 0,
                        key: b"wk".to_vec(),
                    }),
                    TxOp::Commit,
                ],
            }],
        };

        contract.replay_worker_log_batch(batch).unwrap();

        assert_eq!(contract.worker_get(b"wk").unwrap(), None);
    }

    #[test]
    fn iouring_xcontract_update_maps_table_attr_to_value_tuple_index() {
        let meta_mgr = Arc::new(TestMetaMgr::new());
        let schema = test_schema();
        let table_id = schema.id();
        let contract = IoUringXContract::with_log(meta_mgr, None);

        let ddl_tx = block_on(contract.begin_tx()).unwrap();
        block_on(contract.create_table(ddl_tx.clone(), &schema)).unwrap();
        block_on(contract.commit_tx(ddl_tx)).unwrap();

        let insert_tx = block_on(contract.begin_tx()).unwrap();
        block_on(contract.insert(
            insert_tx.clone(),
            table_id,
            &key_row(1),
            &value_row(10),
            &OptInsert::default(),
        ))
        .unwrap();
        block_on(contract.commit_tx(insert_tx)).unwrap();

        let update_tx = block_on(contract.begin_tx()).unwrap();
        let updated = block_on(contract.update(
            update_tx.clone(),
            table_id,
            &key_row(1),
            &Predicate::CNF(vec![]),
            &value_row(20),
            &OptUpdate {},
        ))
        .unwrap();
        assert_eq!(updated, 1);
        block_on(contract.commit_tx(update_tx)).unwrap();

        let read_tx = block_on(contract.begin_tx()).unwrap();
        let relation = block_on(contract.read_key(
            read_tx,
            table_id,
            &key_row(1),
            &VecSelTerm::new(vec![1]),
            &OptRead::default(),
        ))
        .unwrap();
        assert_eq!(relation, Some(vec![datum(20)]));
    }

    fn meta_table(schema: &SchemaTable) -> RS<Arc<TableDesc>> {
        TableInfo::new(schema.clone())?.table_desc()
    }
}
