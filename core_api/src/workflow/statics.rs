use std::sync::Mutex;
use once_cell::sync::OnceCell;
use tokio::runtime::Runtime;

use crate::statics::get_ref;

use super::types::CompositeWorkflowRuntime;

pub fn init_workflow_tokio_runtime() -> Runtime {
    Runtime::new().unwrap()
}

pub fn init_composite_workflow_runtime() -> Mutex<CompositeWorkflowRuntime> {
    Mutex::new(CompositeWorkflowRuntime::new())
}

pub fn tokio_runtime() -> &'static Runtime {
    get_ref("workflow_tokio_runtime")
}

pub fn composite_workflow_runtime() -> &'static Mutex<CompositeWorkflowRuntime> {
    get_ref("composite_workflow_runtime")
}
