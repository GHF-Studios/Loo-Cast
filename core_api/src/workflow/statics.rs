use core_api_macros::export_static;
use std::sync::Mutex;
use tokio::runtime::Runtime;

use super::types::CompositeWorkflowRuntime;

export_static!(self, crate::workflow::statics::WORKFLOW_TOKIO_RUNTIME: Runtime = Runtime::new().unwrap());
export_static!(self, crate::workflow::statics::COMPOSITE_WORKFLOW_RUNTIME: Mutex<CompositeWorkflowRuntime> = Mutex::new(CompositeWorkflowRuntime::new()));
