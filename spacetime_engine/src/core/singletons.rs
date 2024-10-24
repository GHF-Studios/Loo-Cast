use crate::core::structs::LockingHierarchy;
use super::wrappers::*;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use tokio::runtime::Runtime;

lazy_static! {
    pub static ref LOCKING_HIERARCHY: Arc<Mutex<LockingHierarchy>> = Arc::new(Mutex::new(LockingHierarchy::new()));
    pub static ref TOKIO_RUNTIME: Arc<Mutex<Runtime>> = Arc::new(Mutex::new(Runtime::new().expect("Failed to create Tokio runtime")));
}