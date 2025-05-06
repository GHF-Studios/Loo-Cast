use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;

use super::types::CompositeWorkflowRuntime;

lazy_static! {
    pub static ref TOKIO_RUNTIME: Arc<Mutex<Runtime>> =
        Arc::new(Mutex::new(Runtime::new().unwrap()));
    pub static ref COMPOSITE_WORKFLOW_RUNTIME: Arc<Mutex<CompositeWorkflowRuntime>> =
        Arc::new(Mutex::new(CompositeWorkflowRuntime::new()));
}
