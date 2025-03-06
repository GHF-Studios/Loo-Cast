use crossbeam_channel::Sender;
use std::any::Any;

use super::response::*;

pub enum WorkflowRequest {
    None(TypedWorkflowRequest),
    E(TypedWorkflowRequestE),
    O(TypedWorkflowRequestO),
    OE(TypedWorkflowRequestOE),
    S(TypedWorkflowRequestS),
    SE(TypedWorkflowRequestSE),
    SO(TypedWorkflowRequestSO),
    SOE(TypedWorkflowRequestSOE),
    I(TypedWorkflowRequestI),
    IE(TypedWorkflowRequestIE),
    IO(TypedWorkflowRequestIO),
    IOE(TypedWorkflowRequestIOE),
    IS(TypedWorkflowRequestIS),
    ISE(TypedWorkflowRequestISE),
    ISO(TypedWorkflowRequestISO),
    ISOE(TypedWorkflowRequestISOE),
}

pub struct TypedWorkflowRequest {
    pub module_name: String,
    pub workflow_name: String,
    pub response_sender: Sender<WorkflowResponse>,
}
pub struct TypedWorkflowRequestE {
    pub module_name: String,
    pub workflow_name: String,
    pub response_sender: Sender<WorkflowResponseE>,
}
pub struct TypedWorkflowRequestO {
    pub module_name: String,
    pub workflow_name: String,
    pub response_sender: Sender<WorkflowResponseO>,
}
pub struct TypedWorkflowRequestOE {
    pub module_name: String,
    pub workflow_name: String,
    pub response_sender: Sender<WorkflowResponseOE>,
}
pub struct TypedWorkflowRequestS {
    pub module_name: String,
    pub workflow_name: String,
    pub response_sender: Sender<WorkflowResponseS>,
}
pub struct TypedWorkflowRequestSE {
    pub module_name: String,
    pub workflow_name: String,
    pub response_sender: Sender<WorkflowResponseSE>,
}
pub struct TypedWorkflowRequestSO {
    pub module_name: String,
    pub workflow_name: String,
    pub response_sender: Sender<WorkflowResponseSO>,
}
pub struct TypedWorkflowRequestSOE {
    pub module_name: String,
    pub workflow_name: String,
    pub response_sender: Sender<WorkflowResponseSOE>,
}
pub struct TypedWorkflowRequestI {
    pub input: Box<dyn Any + Send + Sync>,
    pub module_name: String,
    pub workflow_name: String,
    pub response_sender: Sender<WorkflowResponseI>,
}
pub struct TypedWorkflowRequestIE {
    pub input: Box<dyn Any + Send + Sync>,
    pub module_name: String,
    pub workflow_name: String,
    pub response_sender: Sender<WorkflowResponseIE>,
}
pub struct TypedWorkflowRequestIO {
    pub input: Box<dyn Any + Send + Sync>,
    pub module_name: String,
    pub workflow_name: String,
    pub response_sender: Sender<WorkflowResponseIO>,
}
pub struct TypedWorkflowRequestIOE {
    pub input: Box<dyn Any + Send + Sync>,
    pub module_name: String,
    pub workflow_name: String,
    pub response_sender: Sender<WorkflowResponseIOE>,
}
pub struct TypedWorkflowRequestIS {
    pub input: Box<dyn Any + Send + Sync>,
    pub module_name: String,
    pub workflow_name: String,
    pub response_sender: Sender<WorkflowResponseIS>,
}
pub struct TypedWorkflowRequestISE {
    pub input: Box<dyn Any + Send + Sync>,
    pub module_name: String,
    pub workflow_name: String,
    pub response_sender: Sender<WorkflowResponseISE>,
}
pub struct TypedWorkflowRequestISO {
    pub input: Box<dyn Any + Send + Sync>,
    pub module_name: String,
    pub workflow_name: String,
    pub response_sender: Sender<WorkflowResponseISO>,
}
pub struct TypedWorkflowRequestISOE {
    pub input: Box<dyn Any + Send + Sync>,
    pub module_name: String,
    pub workflow_name: String,
    pub response_sender: Sender<WorkflowResponseISOE>,
}