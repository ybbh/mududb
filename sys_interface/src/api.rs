#[cfg(target_arch = "wasm32")]
use crate::inner;
use mudu::common::result::RS;
use mudu::common::xid::XID;
use mudu::database::entity::Entity;
use mudu::database::entity_set::RecordSet;
use mudu::database::sql_params::SQLParams;
use mudu::database::sql_stmt::SQLStmt;

#[cfg(target_arch = "wasm32")]
pub fn mudu_query<
    R: Entity
>(
    xid: XID,
    sql: &dyn SQLStmt,
    params: &dyn SQLParams,
) -> RS<RecordSet<R>> {
    inner::inner_query(xid, sql, params)
}

#[cfg(target_arch = "x86_64")]
pub fn mudu_query<
    R: Entity
>(
    _xid: XID,
    _sql: &dyn SQLStmt,
    _params: &dyn SQLParams,
) -> RS<RecordSet<R>> {
    Err(mudu::m_error!(mudu::error::ec::EC::NotImplemented, "mudu_query"))
}


#[cfg(target_arch = "wasm32")]
pub fn mudu_command(
    xid: XID,
    sql: &dyn SQLStmt,
    params: &dyn SQLParams,
) -> RS<u64> {
    inner::inner_command(xid, sql, params)
}

#[cfg(target_arch = "x86_64")]
pub fn mudu_command(
    _xid: XID,
    _sql: &dyn SQLStmt,
    _params: &dyn SQLParams,
) -> RS<u64> {
    Err(mudu::m_error!(mudu::error::ec::EC::NotImplemented, "mudu_command"))
}


