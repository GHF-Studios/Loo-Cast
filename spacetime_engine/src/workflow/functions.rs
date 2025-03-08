use tokio::sync::mpsc::{Sender, unbounded_channel};

use super::{channels::*, request::*, traits::*};

pub async fn run_workflow<W: WorkflowType>() {
    let module_name = W::MODULE_NAME;
    let workflow_name = W::WORKFLOW_NAME;
    let request_sender = get_request_sender();
    let mut response_receiver = get_response_receiver();

    request_sender.send(TypedWorkflowRequest {
        module_name,
        workflow_name,
    }).unwrap();

    response_receiver.recv().await.unwrap()
}
pub async fn run_workflow_e<W: WorkflowTypeE>() -> Result<(), W::Error> {
    let module_name = W::MODULE_NAME;
    let workflow_name = W::WORKFLOW_NAME;
    let request_sender = get_request_e_sender();
    let mut response_receiver = get_response_e_receiver();

    request_sender.send(TypedWorkflowRequestE {
        module_name,
        workflow_name,
    }).unwrap();

    response_receiver.recv().await.unwrap().unpack()
}
pub async fn run_workflow_o<W: WorkflowTypeO>() -> Result<W::Output, ()> {
    let module_name = W::MODULE_NAME;
    let workflow_name = W::WORKFLOW_NAME;
    let request_sender = get_request_o_sender();
    let mut response_receiver = get_response_o_receiver();

    request_sender.send(TypedWorkflowRequestO {
        module_name,
        workflow_name,
    }).unwrap();

    response_receiver.recv().await.unwrap().unpack()
}
pub async fn run_workflow_oe<W: WorkflowTypeOE>() -> Result<W::Output, W::Error> {
    let module_name = W::MODULE_NAME;
    let workflow_name = W::WORKFLOW_NAME;
    let request_sender = get_request_oe_sender();
    let mut response_receiver = get_response_oe_receiver();

    request_sender.send(TypedWorkflowRequestOE {
        module_name,
        workflow_name,
    }).unwrap();

    response_receiver.recv().await.unwrap().unpack()
}
pub async fn run_workflow_i<W: WorkflowTypeI>(input: W::Input) {
    let module_name = W::MODULE_NAME;
    let workflow_name = W::WORKFLOW_NAME;
    let request_sender = get_request_i_sender();
    let mut response_receiver = get_response_i_receiver();

    request_sender.send(TypedWorkflowRequestI {
        input: Box::new(input),
        module_name,
        workflow_name,
    }).unwrap();

    response_receiver.recv().await.unwrap()
}
pub async fn run_workflow_ie<W: WorkflowTypeIE>(input: W::Input) -> Result<(), W::Error> {
    let module_name = W::MODULE_NAME;
    let workflow_name = W::WORKFLOW_NAME;
    let request_sender = get_request_ie_sender();
    let mut response_receiver = get_response_ie_receiver();

    request_sender.send(TypedWorkflowRequestIE {
        input: Box::new(input),
        module_name,
        workflow_name,
    }).unwrap();

    response_receiver.recv().await.unwrap().unpack()
}
pub async fn run_workflow_io<W: WorkflowTypeIO>(input: W::Input) -> Result<W::Output, ()> {
    let module_name = W::MODULE_NAME;
    let workflow_name = W::WORKFLOW_NAME;
    let request_sender = get_request_io_sender();
    let mut response_receiver = get_response_io_receiver();

    request_sender.send(TypedWorkflowRequestIO {
        input: Box::new(input),
        module_name,
        workflow_name,
    }).unwrap();

    response_receiver.recv().await.unwrap().unpack()
}
pub async fn run_workflow_ioe<W: WorkflowTypeIOE>(input: W::Input) -> Result<W::Output, W::Error> {
    let module_name = W::MODULE_NAME;
    let workflow_name = W::WORKFLOW_NAME;
    let request_sender = get_request_ioe_sender();
    let mut response_receiver = get_response_ioe_receiver();

    request_sender.send(TypedWorkflowRequestIOE {
        input: Box::new(input),
        module_name,
        workflow_name,
    }).unwrap();

    response_receiver.recv().await.unwrap().unpack()
}
