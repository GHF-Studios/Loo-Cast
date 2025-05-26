use std::any::Any;
use uuid::Uuid;

use crate::debug::types::AnySendSyncNamedBox;

pub struct TypedWorkflowRequest {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub workflow_id: Uuid,
}
pub struct TypedWorkflowRequestE {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub workflow_id: Uuid,
}
pub struct TypedWorkflowRequestO {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub workflow_id: Uuid,
}
pub struct TypedWorkflowRequestOE {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub workflow_id: Uuid,
}
pub struct TypedWorkflowRequestI {
    pub input: AnySendSyncNamedBox,
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub workflow_id: Uuid,
}
pub struct TypedWorkflowRequestIE {
    pub input: AnySendSyncNamedBox,
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub workflow_id: Uuid,
}
pub struct TypedWorkflowRequestIO {
    pub input: AnySendSyncNamedBox,
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub workflow_id: Uuid,
}
pub struct TypedWorkflowRequestIOE {
    pub input: AnySendSyncNamedBox,
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub workflow_id: Uuid,
}
