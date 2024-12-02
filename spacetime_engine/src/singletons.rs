use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use crate::structs::*;
use tokio::runtime::Runtime;

lazy_static! {
    pub static ref LOCKING_HIERARCHY: Arc<Mutex<LockingHierarchy>> = Arc::new(Mutex::new(LockingHierarchy::new()));
    pub static ref UNLOCK_QUEUE: Arc<Mutex<Vec<UnlockRequest>>> = Arc::new(Mutex::new(Vec::new()));
    pub static ref TOKIO_RUNTIME: Arc<Mutex<Runtime>> = Arc::new(Mutex::new(Runtime::new().expect("Failed to create Tokio runtime")));
    pub static ref OPERATION_QUEUE: Arc<Mutex<OperationQueue>> = Arc::new(Mutex::new(OperationQueue::new()));
}