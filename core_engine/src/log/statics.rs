use lazy_static::lazy_static;
use std::sync::{atomic::AtomicU64, Arc, Mutex, OnceLock};

use crate::log::{resources::LogRegistryHandle, types::LogRegistry};

pub(in super) static LOG_ID_COUNTER: OnceLock<AtomicU64> = OnceLock::new();

lazy_static! {
    pub static ref LOG_REGISTRY_HANDLE: LogRegistryHandle =
        LogRegistryHandle(Arc::new(Mutex::new(LogRegistry::default())));
}
