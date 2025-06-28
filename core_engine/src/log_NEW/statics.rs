use lazy_static::lazy_static;
use std::sync::Arc;

use crate::log_NEW::{resources::LogRegistryHandle, types::LogRegistry};

lazy_static! {
    pub static ref LOG_REGISTRY_HANDLE: LogRegistryHandle =
        LogRegistryHandle(Arc::new(LogRegistry::default()));
}
