use crate::server::session::{DummyAuthSource, Session};
use crate::x_engine::thd_ctx::ThdCtx;
use pgwire::api::auth::md5pass::Md5PasswordAuthStartupHandler;
use pgwire::api::auth::{DefaultServerParameterProvider, StartupHandler};
use pgwire::api::copy::CopyHandler;
use pgwire::api::query::{ExtendedQueryHandler, SimpleQueryHandler};
use pgwire::api::{ErrorHandler, NoopHandler, PgWireServerHandlers};
use std::sync::Arc;

#[derive(Clone)]
pub struct SessionMgr {
    ctx: ThdCtx,
}

impl SessionMgr {
    pub fn new(ctx: ThdCtx) -> Self {
        Self { ctx }
    }
}

impl PgWireServerHandlers for SessionMgr {
    fn simple_query_handler(&self) -> Arc<impl SimpleQueryHandler> {
        Arc::new(Session::new(self.ctx.clone()))
    }

    fn extended_query_handler(&self) -> Arc<impl ExtendedQueryHandler> {
        Arc::new(Session::new(self.ctx.clone()))
    }

    fn startup_handler(&self) -> Arc<impl StartupHandler> {
        Arc::new(Md5PasswordAuthStartupHandler::new(
            Arc::new(DummyAuthSource),
            Arc::new(DefaultServerParameterProvider::default()),
        ))
    }

    fn copy_handler(&self) -> Arc<impl CopyHandler> {
        Arc::new(NoopHandler)
    }

    fn error_handler(&self) -> Arc<impl ErrorHandler> {
        Arc::new(NoopHandler)
    }
}
