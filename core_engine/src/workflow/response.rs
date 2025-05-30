use crate::debug::types::AnySendSyncNamedBox;
use std::any::Any;

pub enum WorkflowResponse {
    None(TypedWorkflowResponse),
    E(TypedWorkflowResponseE),
    O(TypedWorkflowResponseO),
    OE(TypedWorkflowResponseOE),
}

pub struct TypedWorkflowResponse {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
}
pub struct TypedWorkflowResponseE {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub result: Result<(), AnySendSyncNamedBox>,
}
pub struct TypedWorkflowResponseO {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub output: AnySendSyncNamedBox,
}
pub struct TypedWorkflowResponseOE {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub result: Result<AnySendSyncNamedBox, AnySendSyncNamedBox>,
}

impl TypedWorkflowResponseE {
    pub fn unpack<E: 'static + Any + Send + Sync>(self) -> Result<(), E> {
        self.result.map_err(|e| e.into_inner())
    }
}

impl TypedWorkflowResponseO {
    pub fn unpack<O: 'static + Any + Send + Sync>(self) -> O {
        self.output.into_inner()
    }
}

impl TypedWorkflowResponseOE {
    pub fn unpack<O: 'static + Any + Send + Sync, E: 'static + Any + Send + Sync>(self) -> Result<O, E> {
        self.result.map(|o| o.into_inner()).map_err(|e| e.into_inner())
    }
}
