use std::any::Any;

pub struct TypedWorkflowRequest {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
}
pub struct TypedWorkflowRequestE {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
}
pub struct TypedWorkflowRequestO {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
}
pub struct TypedWorkflowRequestOE {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
}
pub struct TypedWorkflowRequestI {
    pub input: Box<dyn Any + Send + Sync>,
    pub module_name: &'static str,
    pub workflow_name: &'static str,
}
pub struct TypedWorkflowRequestIE {
    pub input: Box<dyn Any + Send + Sync>,
    pub module_name: &'static str,
    pub workflow_name: &'static str,
}
pub struct TypedWorkflowRequestIO {
    pub input: Box<dyn Any + Send + Sync>,
    pub module_name: &'static str,
    pub workflow_name: &'static str,
}
pub struct TypedWorkflowRequestIOE {
    pub input: Box<dyn Any + Send + Sync>,
    pub module_name: &'static str,
    pub workflow_name: &'static str,
}
