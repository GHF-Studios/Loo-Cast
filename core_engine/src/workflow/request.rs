use std::any::Any;
use uuid::Uuid;

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
    pub input: Box<dyn Any + Send + Sync>,
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub workflow_id: Uuid,
}
pub struct TypedWorkflowRequestIE {
    pub input: Box<dyn Any + Send + Sync>,
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub workflow_id: Uuid,
}
pub struct TypedWorkflowRequestIO {
    pub input: Box<dyn Any + Send + Sync>,
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub workflow_id: Uuid,
}
pub struct TypedWorkflowRequestIOE {
    pub input: Box<dyn Any + Send + Sync>,
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub workflow_id: Uuid,
}
