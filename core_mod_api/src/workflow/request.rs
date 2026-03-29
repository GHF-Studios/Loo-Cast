use crate::bevy::prelude::Reflect;
use uuid::Uuid;

use crate::utils::premium_box::AnySendSyncPremiumBox;

#[derive(Reflect)]
pub struct TypedWorkflowRequest {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub composite_workflow_id: Uuid,
    pub request_id: Uuid,
}
#[derive(Reflect)]
pub struct TypedWorkflowRequestE {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub composite_workflow_id: Uuid,
    pub request_id: Uuid,
}
#[derive(Reflect)]
pub struct TypedWorkflowRequestO {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub composite_workflow_id: Uuid,
    pub request_id: Uuid,
}
#[derive(Reflect)]
pub struct TypedWorkflowRequestOE {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub composite_workflow_id: Uuid,
    pub request_id: Uuid,
}
#[derive(Reflect)]
pub struct TypedWorkflowRequestI {
    pub input: AnySendSyncPremiumBox,
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub composite_workflow_id: Uuid,
    pub request_id: Uuid,
}
#[derive(Reflect)]
pub struct TypedWorkflowRequestIE {
    pub input: AnySendSyncPremiumBox,
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub composite_workflow_id: Uuid,
    pub request_id: Uuid,
}
#[derive(Reflect)]
pub struct TypedWorkflowRequestIO {
    pub input: AnySendSyncPremiumBox,
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub composite_workflow_id: Uuid,
    pub request_id: Uuid,
}
#[derive(Reflect)]
pub struct TypedWorkflowRequestIOE {
    pub input: AnySendSyncPremiumBox,
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub composite_workflow_id: Uuid,
    pub request_id: Uuid,
}
