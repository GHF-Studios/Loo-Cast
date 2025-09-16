use std::time::Instant;
use once_cell::sync::OnceCell;
use tokio::runtime::Runtime;

use crate::statics::get_ref;

pub fn init_tokio_runtime() -> Runtime {
    Runtime::new().unwrap()
}

pub fn init_start_time() -> Instant {
    Instant::now()
}

pub fn tokio_runtime() -> &'static Runtime {
    get_ref("tokio_runtime")
}

pub fn start_time() -> &'static Instant {
    get_ref("start_time")
}