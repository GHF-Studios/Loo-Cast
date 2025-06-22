use lazy_static::lazy_static;
use std::sync::Arc;

use crate::log_NEW::types::LogStorageHandle;

lazy_static! {
    pub static ref LOG_STORAGE_HANDLE: LogStorageHandle =
        LogStorageHandle(Arc::new(LogStorage::new()));
}
