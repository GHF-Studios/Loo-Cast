use crate::statics::get_ref;
use std::sync::Mutex;
use tokio::runtime::Runtime;

use super::types::CompositeWorkflowRuntime;

/// Registry key: "workflow_tokio_runtime"
pub fn tokio_runtime() -> &'static Runtime {
    get_ref("workflow_tokio_runtime")
}

/// Registry key: "composite_workflow_runtime"
pub fn composite_workflow_runtime() -> &'static Mutex<CompositeWorkflowRuntime> {
    get_ref("composite_workflow_runtime")
}
