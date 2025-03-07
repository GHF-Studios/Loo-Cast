use tokio::sync::mpsc::UnboundedSender;
use std::any::Any;

use super::response::*;

pub enum WorkflowRequest {
    None(TypedWorkflowRequest),
    E(TypedWorkflowRequestE),
    O(TypedWorkflowRequestO),
    OE(TypedWorkflowRequestOE),
    I(TypedWorkflowRequestI),
    IE(TypedWorkflowRequestIE),
    IO(TypedWorkflowRequestIO),
    IOE(TypedWorkflowRequestIOE),
}

pub struct TypedWorkflowRequest {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub response_sender: UnboundedSender<TypedWorkflowResponse>,
}
pub struct TypedWorkflowRequestE {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub response_sender: UnboundedSender<TypedWorkflowResponseE>,
}
pub struct TypedWorkflowRequestO {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub response_sender: UnboundedSender<TypedWorkflowResponseO>,
}
pub struct TypedWorkflowRequestOE {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub response_sender: UnboundedSender<TypedWorkflowResponseOE>,
}
pub struct TypedWorkflowRequestI {
    pub input: Box<dyn Any + Send + Sync>,
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub response_sender: UnboundedSender<TypedWorkflowResponseI>,
}
pub struct TypedWorkflowRequestIE {
    pub input: Box<dyn Any + Send + Sync>,
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub response_sender: UnboundedSender<TypedWorkflowResponseIE>,
}
pub struct TypedWorkflowRequestIO {
    pub input: Box<dyn Any + Send + Sync>,
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub response_sender: UnboundedSender<TypedWorkflowResponseIO>,
}
pub struct TypedWorkflowRequestIOE {
    pub input: Box<dyn Any + Send + Sync>,
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub response_sender: UnboundedSender<TypedWorkflowResponseIOE>,
}
