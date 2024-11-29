use crate::operation::structs::OperationQueue;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref OPERATION_QUEUE: Arc<Mutex<OperationQueue>> = Arc::new(Mutex::new(OperationQueue::new()));
}