use async_trait::async_trait;
use mudu::common::id::OID;
use mudu::common::result::RS;
use mudu::common::xid::XID;
use mudu::error::ec::EC;
use mudu::m_error;
use mudu_contract::database::db_conn::DBConnAsync;
use mudu_contract::database::prepared_stmt::PreparedStmt;
use mudu_contract::database::result_set::ResultSetAsync;
use mudu_contract::database::sql_params::SQLParams;
use mudu_contract::database::sql_stmt::SQLStmt;
use sql_parser::ast::parser::SQLParser;
use sql_parser::ast::stmt_type::StmtType;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::mudu_conn::mudu_prepared_stmt::MuduPreparedStmt;
use crate::server::worker_local::{current_worker_local, WorkerExecute, WorkerLocalRef};
use crate::sql::describer::Describer;

pub struct MuduConnAsync {
    worker_local: WorkerLocalRef,
    parser: Arc<SQLParser>,
    session_id: Arc<Mutex<Option<OID>>>,
}

impl MuduConnAsync {
    pub fn new() -> Self {
        Self {
            worker_local: current_worker_local(),
            parser: Arc::new(SQLParser::new()),
            session_id: Arc::new(Mutex::new(None)),
        }
    }

    fn parse_one(&self, sql: &dyn SQLStmt) -> RS<StmtType> {
        let stmt_list = self.parser.parse(&sql.to_sql_string())?;
        let mut stmts = stmt_list.into_stmts();
        if stmts.len() != 1 {
            return Err(m_error!(EC::ParseErr, "expected exactly one statement"));
        }
        Ok(stmts.remove(0))
    }

    async fn ensure_session_id(&self) -> RS<OID> {
        let mut guard = self.session_id.lock().await;
        if let Some(session_id) = *guard {
            return Ok(session_id);
        }
        let session_id = self.worker_local.open_async().await?;
        *guard = Some(session_id);
        Ok(session_id)
    }

    async fn active_session_id(&self) -> RS<OID> {
        let guard = self.session_id.lock().await;
        guard.ok_or_else(|| m_error!(EC::NoSuchElement, "no active session"))
    }
}

#[async_trait]
impl DBConnAsync for MuduConnAsync {
    async fn prepare(&self, stmt: Box<dyn SQLStmt>) -> RS<Arc<dyn PreparedStmt>> {
        let parsed = self.parse_one(stmt.as_ref())?;
        let desc = Describer::
            describe(self.worker_local.meta_mgr().as_ref(), parsed)
            .await?;
        Ok(Arc::new(MuduPreparedStmt::new(
            self.worker_local.clone(),
            self.session_id.clone(),
            stmt,
            Arc::new(desc),
        )))
    }

    async fn exec_silent(&self, sql_text: String) -> RS<()> {
        let session_id = self.ensure_session_id().await?;
        let _ = self
            .worker_local
            .batch(session_id, Box::new(sql_text), Box::new(()))
            .await?;
        Ok(())
    }

    async fn begin_tx(&self) -> RS<XID> {
        let session_id = self.ensure_session_id().await?;
        self.worker_local
            .execute_async(session_id, WorkerExecute::BeginTx)
            .await?;
        Ok(session_id)
    }

    async fn rollback_tx(&self) -> RS<()> {
        let session_id = self.active_session_id().await?;
        self.worker_local
            .execute_async(session_id, WorkerExecute::RollbackTx)
            .await
    }

    async fn commit_tx(&self) -> RS<()> {
        let session_id = self.active_session_id().await?;
        self.worker_local
            .execute_async(session_id, WorkerExecute::CommitTx)
            .await
    }

    async fn query(
        &self,
        sql: Box<dyn SQLStmt>,
        param: Box<dyn SQLParams>,
    ) -> RS<Arc<dyn ResultSetAsync>> {
        let session_id = self.ensure_session_id().await?;
        self.worker_local.query(session_id, sql, param).await
    }

    async fn execute(&self, sql: Box<dyn SQLStmt>, param: Box<dyn SQLParams>) -> RS<u64> {
        let session_id = self.ensure_session_id().await?;
        self.worker_local.execute(session_id, sql, param).await
    }

    async fn batch(&self, sql: Box<dyn SQLStmt>, param: Box<dyn SQLParams>) -> RS<u64> {
        let session_id = self.ensure_session_id().await?;
        self.worker_local.batch(session_id, sql, param).await
    }
}
