use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tokio::runtime::Runtime;

lazy_static! {
    pub static ref TOKIO_RUNTIME: Arc<Mutex<Runtime>> = Arc::new(Mutex::new(Runtime::new().unwrap()));
    pub static ref START_TIME: Arc<Instant> = Arc::new(Instant::now());
}
