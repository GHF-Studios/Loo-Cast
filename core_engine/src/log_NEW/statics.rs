use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

use crate::log_NEW::{resources::LogRegistryHandle, types::LogRegistry};

lazy_static! {
    pub static ref LOG_REGISTRY_HANDLE: LogRegistryHandle =
        LogRegistryHandle(Arc::new(Mutex::new(LogRegistry::default())));
}
