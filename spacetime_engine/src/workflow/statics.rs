use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;

use super::types::WorkflowTaskRuntime;

lazy_static! {
    pub(in super) static ref TOKIO_RUNTIME: Arc<Mutex<Runtime>> = Arc::new(Mutex::new(Runtime::new().unwrap()));
    pub static ref COMPOSITE_WORKFLOW_RUNTIME: Arc<Mutex<WorkflowTaskRuntime>> = Arc::new(Mutex::new(WorkflowTaskRuntime::new()));
}