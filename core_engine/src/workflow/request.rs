use uuid::Uuid;

use crate::debug::types::AnySendSyncPremiumBox;

pub struct TypedWorkflowRequest {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub composite_workflow_id: Uuid,
}
pub struct TypedWorkflowRequestE {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub composite_workflow_id: Uuid,
}
pub struct TypedWorkflowRequestO {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub composite_workflow_id: Uuid,
}
pub struct TypedWorkflowRequestOE {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub composite_workflow_id: Uuid,
}
pub struct TypedWorkflowRequestI {
    pub input: AnySendSyncPremiumBox,
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub composite_workflow_id: Uuid,
}
pub struct TypedWorkflowRequestIE {
    pub input: AnySendSyncPremiumBox,
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub composite_workflow_id: Uuid,
}
pub struct TypedWorkflowRequestIO {
    pub input: AnySendSyncPremiumBox,
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub composite_workflow_id: Uuid,
}
pub struct TypedWorkflowRequestIOE {
    pub input: AnySendSyncPremiumBox,
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub composite_workflow_id: Uuid,
}
