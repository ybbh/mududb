use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex as StdMutex, OnceLock, Weak};
use std::time::{SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use mudu::common::id::OID;
use mudu::common::result::RS;
use mudu::error::ec::EC as ER;
use mudu::m_error;

use crate::contract::meta_mgr::MetaMgr;
use crate::contract::schema_table::SchemaTable;
use crate::contract::table_desc::TableDesc;
use crate::contract::table_info::TableInfo;
use crate::meta::schema_catalog::{
    delete_schema_from_catalog, load_schemas_from_catalog, open_schema_catalog,
    write_schema_to_catalog,
};
use crate::storage::relation::relation::Relation;

type MetaMgrRegistry = HashMap<String, Vec<Weak<MetaMgrImpl>>>;

fn registry() -> &'static StdMutex<MetaMgrRegistry> {
    static REGISTRY: OnceLock<StdMutex<MetaMgrRegistry>> = OnceLock::new();
    REGISTRY.get_or_init(|| StdMutex::new(HashMap::new()))
}

fn ddl_lock() -> &'static tokio::sync::Mutex<()> {
    static DDL_LOCK: OnceLock<tokio::sync::Mutex<()>> = OnceLock::new();
    DDL_LOCK.get_or_init(|| tokio::sync::Mutex::new(()))
}

pub struct MetaMgrImpl {
    path: String,
    schema_catalog: Relation,
    next_catalog_xid: AtomicU64,
    id2table: scc::HashMap<OID, TableInfo>,
    name2id: scc::HashMap<String, OID>,
    table: scc::HashMap<String, TableInfo>,
}

impl MetaMgrImpl {
    pub fn new<P: AsRef<Path>>(path: P) -> RS<Self> {
        let path = PathBuf::from(path.as_ref());
        if fs::metadata(&path).is_err() {
            fs::create_dir_all(&path).map_err(|e| m_error!(ER::IOErr, "", e))?;
        }

        let path_string = path.to_string_lossy().to_string();
        let schema_catalog = open_schema_catalog(&path_string)?;
        let this = Self {
            path: path_string,
            schema_catalog,
            next_catalog_xid: AtomicU64::new(now_catalog_xid()),
            id2table: Default::default(),
            name2id: Default::default(),
            table: Default::default(),
        };
        for schema in load_schemas_from_catalog(&this.schema_catalog)? {
            this.apply_create_table_local(&schema)?;
        }
        Ok(this)
    }

    pub fn register_global(self: &Arc<Self>) {
        let mut guard = registry().lock().unwrap();
        guard
            .entry(self.path.clone())
            .or_default()
            .push(Arc::downgrade(self));
    }

    pub fn lookup_table_info_by_id(&self, oid: OID) -> Option<TableInfo> {
        let opt = self.id2table.get_sync(&oid);
        opt.map(|entry| entry.get().clone())
    }

    pub fn lookup_table_by_name(&self, name: &String) -> RS<Option<Arc<TableDesc>>> {
        let opt = self.table.get_sync(name);
        let table_desc = match opt {
            None => return Ok(None),
            Some(table) => table.get().table_desc()?,
        };
        Ok(Some(table_desc))
    }

    pub fn list_schemas_inner(&self) -> Vec<SchemaTable> {
        let mut schemas = Vec::new();
        self.table.iter_sync(|_table_name, table_info| {
            schemas.push(table_info.schema().as_ref().clone());
            true
        });
        schemas.sort_by_key(|schema| schema.id());
        schemas
    }

    pub async fn create_table_inner(&self, schema: &SchemaTable) -> RS<()> {
        let _ddl_guard = ddl_lock()
            .lock()
            .await;
        if self.table.contains_sync(schema.table_name()) {
            return Err(m_error!(ER::ExistingSuchElement, ""));
        }

        write_schema_to_catalog(&self.schema_catalog, schema, self.next_catalog_xid()).await?;
        self.broadcast_create(schema)
    }

    pub async fn drop_table_inner(&self, oid: OID) -> RS<()> {
        let _ddl_guard = ddl_lock()
            .lock()
            .await;
        let table = self
            .lookup_table_info_by_id(oid)
            .ok_or_else(|| m_error!(ER::NoSuchElement, format!("no such table {}", oid)))?;

        delete_schema_from_catalog(&self.schema_catalog, oid, self.next_catalog_xid()).await?;
        self.broadcast_drop(table.schema().table_name(), oid)
    }

    fn next_catalog_xid(&self) -> u64 {
        let mut next = self.next_catalog_xid.load(Ordering::Relaxed);
        loop {
            let candidate = now_catalog_xid().max(next.saturating_add(1));
            match self.next_catalog_xid.compare_exchange(
                next,
                candidate,
                Ordering::SeqCst,
                Ordering::SeqCst,
            ) {
                Ok(_) => return candidate,
                Err(actual) => next = actual,
            }
        }
    }

    fn apply_create_table_local(&self, schema: &SchemaTable) -> RS<()> {
        let table_id = schema.id();
        let table_name = schema.table_name().clone();
        let table = TableInfo::new(schema.clone())?;
        let _ = self.table.insert_sync(table_name.clone(), table.clone());
        let _ = self.id2table.insert_sync(table_id, table);
        let _ = self.name2id.insert_sync(table_name, table_id);
        Ok(())
    }

    fn apply_drop_table_local(&self, table_name: &str, oid: OID) {
        let _ = self.id2table.remove_sync(&oid);
        let _ = self.name2id.remove_sync(table_name);
        let _ = self.table.remove_sync(table_name);
    }

    fn broadcast_create(&self, schema: &SchemaTable) -> RS<()> {
        let peers = self.peer_instances();
        if peers.is_empty() {
            return self.apply_create_table_local(schema);
        }
        for mgr in peers {
            mgr.apply_create_table_local(schema)?;
        }
        Ok(())
    }

    fn broadcast_drop(&self, table_name: &str, oid: OID) -> RS<()> {
        let peers = self.peer_instances();
        if peers.is_empty() {
            self.apply_drop_table_local(table_name, oid);
            return Ok(());
        }
        for mgr in peers {
            mgr.apply_drop_table_local(table_name, oid);
        }
        Ok(())
    }

    fn peer_instances(&self) -> Vec<Arc<MetaMgrImpl>> {
        let mut guard = registry().lock().unwrap();
        let peers = guard.entry(self.path.clone()).or_default();
        let mut live = Vec::with_capacity(peers.len());
        peers.retain(|weak| match weak.upgrade() {
            Some(peer) => {
                live.push(peer);
                true
            }
            None => false,
        });
        live
    }
}

fn now_catalog_xid() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
        .min(u64::MAX as u128) as u64
}

#[async_trait]
impl MetaMgr for MetaMgrImpl {
    async fn get_table_by_id(&self, oid: OID) -> RS<Arc<TableDesc>> {
        let opt = self.lookup_table_info_by_id(oid);
        match opt {
            Some(table) => table.table_desc(),
            None => Err(m_error!(
                ER::NoSuchElement,
                format!("no such table {}", oid)
            )),
        }
    }

    async fn get_table_by_name(&self, name: &String) -> RS<Option<Arc<TableDesc>>> {
        self.lookup_table_by_name(name)
    }

    async fn create_table(&self, schema: &SchemaTable) -> RS<()> {
        self.create_table_inner(schema).await
    }

    async fn drop_table(&self, table_id: OID) -> RS<()> {
        self.drop_table_inner(table_id).await
    }

    async fn list_schemas(&self) -> RS<Vec<SchemaTable>> {
        Ok(self.list_schemas_inner())
    }
}

unsafe impl Sync for MetaMgrImpl {}

unsafe impl Send for MetaMgrImpl {}

#[cfg(test)]
mod tests {
    use std::env::temp_dir;

    use mudu_type::dat_type_id::DatTypeID;
    use mudu_type::dt_info::DTInfo;

    use crate::contract::schema_column::SchemaColumn;

    use super::*;

    fn test_schema() -> SchemaTable {
        SchemaTable::new(
            "meta_recovery_t".to_string(),
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

    #[test]
    fn meta_mgr_recovers_schema_catalog_after_reopen() {
        let dir = temp_dir().join(format!("meta_mgr_catalog_{}", mudu::common::id::gen_oid()));
        let mgr = Arc::new(MetaMgrImpl::new(&dir).unwrap());
        mgr.register_global();

        let schema = test_schema();
        futures::executor::block_on(mgr.create_table(&schema)).unwrap();
        assert_eq!(
            crate::meta::schema_catalog::load_schemas_from_catalog(&mgr.schema_catalog)
                .unwrap()
                .len(),
            1
        );
        drop(mgr);

        let reopened = MetaMgrImpl::new(&dir).unwrap();
        let table = futures::executor::block_on(reopened.get_table_by_id(schema.id())).unwrap();
        assert_eq!(table.name(), schema.table_name());
    }

    #[test]
    fn meta_mgr_broadcasts_ddl_to_peer_instances() {
        let dir = temp_dir().join(format!("meta_mgr_peer_{}", mudu::common::id::gen_oid()));
        let mgr1 = Arc::new(MetaMgrImpl::new(&dir).unwrap());
        mgr1.register_global();
        let mgr2 = Arc::new(MetaMgrImpl::new(&dir).unwrap());
        mgr2.register_global();

        let schema = test_schema();
        futures::executor::block_on(mgr1.create_table(&schema)).unwrap();
        let table = futures::executor::block_on(mgr2.get_table_by_id(schema.id())).unwrap();
        assert_eq!(table.name(), schema.table_name());

        futures::executor::block_on(mgr2.drop_table(schema.id())).unwrap();
        assert!(futures::executor::block_on(mgr1.get_table_by_id(schema.id())).is_err());
    }
}
