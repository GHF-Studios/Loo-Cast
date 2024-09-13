use super::structs::*;
use super::wrappers::*;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref MAIN_TYPE_REGISTRY: Arc<Mutex<MainTypeRegistry>> = Arc::new(Mutex::new(MainTypeRegistry::new()));
    pub static ref OPERATION_QUEUE: Arc<Mutex<OperationQueue>> = Arc::new(Mutex::new(OperationQueue::new()));
}