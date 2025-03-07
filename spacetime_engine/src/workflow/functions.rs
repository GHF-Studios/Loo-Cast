use tokio::sync::mpsc::{Sender, unbounded_channel};

use super::{request::*, traits::*};

pub async fn run_workflow<W: WorkflowType>(
    request_sender: Sender<WorkflowRequest>,
) {
    let module_name = W::MODULE_NAME;
    let workflow_name = W::WORKFLOW_NAME;
    let (response_sender, mut response_receiver) = unbounded_channel();

    request_sender.send(WorkflowRequest::None(TypedWorkflowRequest {
        module_name,
        workflow_name,
        response_sender,
    })).await.unwrap();

    response_receiver.recv().await.unwrap()
}
pub async fn run_workflow_e<W: WorkflowTypeE>(
    request_sender: Sender<WorkflowRequest>,
) -> Result<(), W::Error> {
    let module_name = W::MODULE_NAME;
    let workflow_name = W::WORKFLOW_NAME;
    let (response_sender, mut response_receiver) = unbounded_channel();

    request_sender.send(WorkflowRequest::E(TypedWorkflowRequestE {
        module_name,
        workflow_name,
        response_sender,
    })).await.unwrap();

    response_receiver.recv().await.unwrap().unpack()
}
pub async fn run_workflow_o<W: WorkflowTypeO>(
    request_sender: Sender<WorkflowRequest>,
) -> Result<W::Output, ()> {
    let module_name = W::MODULE_NAME;
    let workflow_name = W::WORKFLOW_NAME;
    let (response_sender, mut response_receiver) = unbounded_channel();

    request_sender.send(WorkflowRequest::O(TypedWorkflowRequestO {
        module_name,
        workflow_name,
        response_sender,
    })).await.unwrap();

    response_receiver.recv().await.unwrap().unpack()
}
pub async fn run_workflow_oe<W: WorkflowTypeOE>(
    request_sender: Sender<WorkflowRequest>,
) -> Result<W::Output, W::Error> {
    let module_name = W::MODULE_NAME;
    let workflow_name = W::WORKFLOW_NAME;
    let (response_sender, mut response_receiver) = unbounded_channel();

    request_sender.send(WorkflowRequest::OE(TypedWorkflowRequestOE {
        module_name,
        workflow_name,
        response_sender,
    })).await.unwrap();

    response_receiver.recv().await.unwrap().unpack()
}
pub async fn run_workflow_i<W: WorkflowTypeI>(
    request_sender: Sender<WorkflowRequest>,
    input: W::Input,
) {
    let module_name = W::MODULE_NAME;
    let workflow_name = W::WORKFLOW_NAME;
    let (response_sender, mut response_receiver) = unbounded_channel();

    request_sender.send(WorkflowRequest::I(TypedWorkflowRequestI {
        input: Box::new(input),
        module_name,
        workflow_name,
        response_sender,
    })).await.unwrap();

    response_receiver.recv().await.unwrap()
}
pub async fn run_workflow_ie<W: WorkflowTypeIE>(
    request_sender: Sender<WorkflowRequest>,
    input: W::Input,
) -> Result<(), W::Error> {
    let module_name = W::MODULE_NAME;
    let workflow_name = W::WORKFLOW_NAME;
    let (response_sender, mut response_receiver) = unbounded_channel();

    request_sender.send(WorkflowRequest::IE(TypedWorkflowRequestIE {
        input: Box::new(input),
        module_name,
        workflow_name,
        response_sender,
    })).await.unwrap();

    response_receiver.recv().await.unwrap().unpack()
}
pub async fn run_workflow_io<W: WorkflowTypeIO>(
    request_sender: Sender<WorkflowRequest>,
    input: W::Input,
) -> Result<W::Output, ()> {
    let module_name = W::MODULE_NAME;
    let workflow_name = W::WORKFLOW_NAME;
    let (response_sender, mut response_receiver) = unbounded_channel();

    request_sender.send(WorkflowRequest::IO(TypedWorkflowRequestIO {
        input: Box::new(input),
        module_name,
        workflow_name,
        response_sender,
    })).await.unwrap();

    response_receiver.recv().await.unwrap().unpack()
}
pub async fn run_workflow_ioe<W: WorkflowTypeIOE>(
    request_sender: Sender<WorkflowRequest>,
    input: W::Input,
) -> Result<W::Output, W::Error> {
    let module_name = W::MODULE_NAME;
    let workflow_name = W::WORKFLOW_NAME;
    let (response_sender, mut response_receiver) = unbounded_channel();

    request_sender.send(WorkflowRequest::IOE(TypedWorkflowRequestIOE {
        input: Box::new(input),
        module_name,
        workflow_name,
        response_sender,
    })).await.unwrap();

    response_receiver.recv().await.unwrap().unpack()
}
