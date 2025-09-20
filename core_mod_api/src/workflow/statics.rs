use core_mod_macros::export_static;
use std::sync::Mutex;
use tokio::runtime::Runtime;

use super::types::CompositeWorkflowRuntime;

export_static!(self, crate::core_mod_api::workflow::statics::WORKFLOW_TOKIO_RUNTIME: Runtime = Runtime::new().unwrap());
export_static!(self, crate::core_mod_api::workflow::statics::COMPOSITE_WORKFLOW_RUNTIME: Mutex<CompositeWorkflowRuntime> = Mutex::new(CompositeWorkflowRuntime::new()));
