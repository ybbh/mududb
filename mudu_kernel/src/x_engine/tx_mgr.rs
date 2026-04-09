use crate::server::worker_snapshot::WorkerSnapshot;
use crate::wal::xl_batch::XLBatch;
use mudu::common::id::OID;
use std::collections::BTreeMap;

pub trait TxMgr: Send + Sync {
    fn xid(&self) -> u64;

    fn snapshot(&self) -> WorkerSnapshot;

    fn put(&self, key: Vec<u8>, value: Vec<u8>);

    fn delete(&self, key: Vec<u8>);

    fn get(&self, key: &[u8]) -> Option<Option<Vec<u8>>>;

    fn put_relation(&self, oid: OID, key: Vec<u8>, value: Vec<u8>);

    fn delete_relation(&self, oid: OID, key: Vec<u8>);

    fn get_relation(&self, oid: OID, key: &[u8]) -> Option<Option<Vec<u8>>>;

    fn staged_relation_items_in_range(
        &self,
        oid: OID,
        start_key: &[u8],
        end_key: &[u8],
    ) -> Vec<(Vec<u8>, Option<Vec<u8>>)>;

    fn staged_relation_ops(&self) -> BTreeMap<OID, BTreeMap<Vec<u8>, Option<Vec<u8>>>>;

    fn staged_items_in_range(
        &self,
        start_key: &[u8],
        end_key: &[u8],
    ) -> Vec<(Vec<u8>, Option<Vec<u8>>)>;

    fn staged_put_items(&self) -> BTreeMap<Vec<u8>, Option<Vec<u8>>>;

    fn is_empty(&self) -> bool;

    fn write_ops(&self) -> Vec<(OID, Vec<u8>)>;

    fn build_write_ops(&self);

    fn xl_batch(&self) -> XLBatch;
}

