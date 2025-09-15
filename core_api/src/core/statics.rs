use crate::statics::get_ref;
use std::time::Instant;
use tokio::runtime::Runtime;

/// Wrapper around registry key "tokio_runtime"
pub fn tokio_runtime() -> &'static Runtime {
    get_ref("tokio_runtime")
}

/// Wrapper around registry key "start_time"
pub fn start_time() -> &'static Instant {
    get_ref("start_time")
}