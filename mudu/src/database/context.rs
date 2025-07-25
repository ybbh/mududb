use crate::common::result::RS;
use crate::database::db_conn::DBConn;
use crate::database::record::Record;
use crate::database::record_set::RecordSet;
use crate::database::sql_stmt::SQLStmt;
use crate::tuple::to_datum::ToDatum;
use std::sync::Arc;

pub struct Context {
  conn:Arc<dyn DBConn>,
}

impl Context {
    pub fn new(conn: Arc<dyn DBConn>) -> Self {
        
        Self { conn }
    }
    
    pub fn db_conn(&self) -> Arc<dyn DBConn> {
        self.conn.clone()
    }
    
    pub fn query<R:Record>(
        &self,         
        sql:&dyn SQLStmt,
        param:&[&dyn ToDatum]) -> RS<RecordSet<R>> {
        let (rs, ds) = self.conn.query(sql, param)?;
        let r = RecordSet::<R>::new(rs, ds);
        Ok(r)
    }
    
    pub fn command(
        &self,         
        sql:&dyn SQLStmt,
        param:&[&dyn ToDatum]) -> RS<usize>
    {
        self.conn.command(sql, param)
    }
}