use crate::debug::types::AnySendSyncPremiumBox;
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
    pub result: Result<(), AnySendSyncPremiumBox>,
}
pub struct TypedWorkflowResponseO {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub output: AnySendSyncPremiumBox,
}
pub struct TypedWorkflowResponseOE {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub result: Result<AnySendSyncPremiumBox, AnySendSyncPremiumBox>,
}

impl TypedWorkflowResponseE {
    #[track_caller]
    pub fn unpack<E: 'static + Any + Send + Sync>(self) -> Result<(), E> {
        self.result.map_err(|e| e.into_inner())
    }
}

impl TypedWorkflowResponseO {
    #[track_caller]
    pub fn unpack<O: 'static + Any + Send + Sync>(self) -> O {
        self.output.into_inner()
    }
}

impl TypedWorkflowResponseOE {
    #[track_caller]
    pub fn unpack<O: 'static + Any + Send + Sync, E: 'static + Any + Send + Sync>(self) -> Result<O, E> {
        self.result.map(|o| o.into_inner()).map_err(|e| e.into_inner())
    }
}
