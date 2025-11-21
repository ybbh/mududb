use crate::service::app_inst::AppInst;
use crate::service::runtime_simple::RuntimeSimple;
use crate::service::runtime::Runtime;
use mudu::common::result::RS;
use mudu::error::ec::EC;
use mudu::m_error;
use std::path::PathBuf;
use std::sync::Arc;
use mudu_utils::notifier::Notifier;

#[derive(Clone)]
struct RuntimeImpl {
    inner: Arc<RuntimeSimple>,
}

impl RuntimeImpl {
    pub fn new(package_path: &String, db_path: &String) -> RS<Self> {
        for ps in [package_path, db_path] {
            let path = PathBuf::from(ps);
            if !path.exists() {
                std::fs::create_dir_all(&path).map_err(|e| {
                    m_error!(EC::IOErr, format!("error creating database directory: {}", ps), e)
                })?
            } else {
                if !path.is_dir() {
                    return Err(m_error!(EC::IOErr, format!("{} is not a directory", ps)));
                }
            }
        }
        let mut runtime = RuntimeSimple::new(package_path, db_path);
        runtime.initialized()?;
        let ret = Self {
            inner: Arc::new(runtime),
        };
        Ok(ret)
    }
}

impl Runtime for RuntimeImpl {
    fn list(&self) -> Vec<String> {
        self.inner.list()
    }

    fn app(&self, app_name: &String) -> Option<Arc<dyn AppInst>> {
        self.inner.app(app_name)
    }

    fn install(&self, pkg_path: &String) -> RS<()> {
        self.inner.install(pkg_path)
    }
}

unsafe impl Sync for RuntimeImpl {}

unsafe impl Send for RuntimeImpl {}

pub fn create_runtime_service(
    package_path: &String,
    db_path: &String,
    opt_initialized_notifier:Option<Notifier>
) -> RS<Arc<dyn Runtime>> {
    let runtime = RuntimeImpl::new(package_path, db_path)?;
    match opt_initialized_notifier {
        Some(notifier) => {
            notifier.notify_all();
        }
        None => {

        }
    }
    Ok(Arc::new(runtime))
}
