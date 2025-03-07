use crossbeam_channel::Sender;

use super::{instance::*, request::*};

pub async fn request_workflow<W: WorkflowInstance>(
    request_sender: Sender<WorkflowRequest>,
) {
    let module_name = W::module_name();
    let workflow_name = W::workflow_name();
    let (response_sender, response_receiver) = crossbeam_channel::bounded(1);

    request_sender.send(WorkflowRequest::None(TypedWorkflowRequest {
        module_name,
        workflow_name,
        response_sender,
    })).unwrap();

    response_receiver.recv_async().await.unwrap().unpack()
}

pub async fn request_workflow_e<W: WorkflowInstanceE>(
    request_sender: Sender<WorkflowRequest>,
) -> Result<(), W::Error> {
    let module_name = W::module_name();
    let workflow_name = W::workflow_name();
    let (response_sender, response_receiver) = crossbeam_channel::bounded(1);

    request_sender.send(WorkflowRequest::E(TypedWorkflowRequestE {
        module_name,
        workflow_name,
        response_sender,
    })).unwrap();

    response_receiver.recv_async().await.unwrap().unpack()
}

pub async fn request_workflow_o<W: WorkflowInstanceO>(
    request_sender: Sender<WorkflowRequest>,
) -> Result<W::Output, ()> {
    let module_name = W::module_name();
    let workflow_name = W::workflow_name();
    let (response_sender, response_receiver) = crossbeam_channel::bounded(1);

    request_sender.send(WorkflowRequest::O(TypedWorkflowRequestO {
        module_name,
        workflow_name,
        response_sender,
    })).unwrap();

    response_receiver.recv_async().await.unwrap().unpack()
}

pub async fn request_workflow_oe<W: WorkflowInstanceOE>(
    request_sender: Sender<WorkflowRequest>,
) -> Result<W::Output, W::Error> {
    let module_name = W::module_name();
    let workflow_name = W::workflow_name();
    let (response_sender, response_receiver) = crossbeam_channel::bounded(1);

    request_sender.send(WorkflowRequest::OE(TypedWorkflowRequestOE {
        module_name,
        workflow_name,
        response_sender,
    })).unwrap();

    response_receiver.recv_async().await.unwrap().unpack()
}

pub async fn request_workflow_s<W: WorkflowInstanceS>(
    request_sender: Sender<WorkflowRequest>,
) -> Result<W::Output, W::Error> {
    let module_name = W::module_name();
    let workflow_name = W::workflow_name();
    let (response_sender, response_receiver) = crossbeam_channel::bounded(1);

    request_sender.send(WorkflowRequest::S(TypedWorkflowRequestS {
        module_name,
        workflow_name,
        response_sender,
    })).unwrap();

    response_receiver.recv_async().await.unwrap().unpack()
}

pub async fn request_workflow_se<W: WorkflowInstanceSE>(
    request_sender: Sender<WorkflowRequest>,
) -> Result<W::Output, W::Error> {
    let module_name = W::module_name();
    let workflow_name = W::workflow_name();
    let (response_sender, response_receiver) = crossbeam_channel::bounded(1);

    request_sender.send(WorkflowRequest::SE(TypedWorkflowRequestSE {
        module_name,
        workflow_name,
        response_sender,
    })).unwrap();

    response_receiver.recv_async().await.unwrap().unpack()
}

pub async fn request_workflow_so<W: WorkflowInstanceSO>(
    request_sender: Sender<WorkflowRequest>,
) -> Result<W::Output, ()> {
    let module_name = W::module_name();
    let workflow_name = W::workflow_name();
    let (response_sender, response_receiver) = crossbeam_channel::bounded(1);

    request_sender.send(WorkflowRequest::SO(TypedWorkflowRequestSO {
        module_name,
        workflow_name,
        response_sender,
    })).unwrap();

    response_receiver.recv_async().await.unwrap().unpack()
}

pub async fn request_workflow_soe<W: WorkflowInstanceSOE>(
    request_sender: Sender<WorkflowRequest>,
) -> Result<W::Output, W::Error> {
    let module_name = W::module_name();
    let workflow_name = W::workflow_name();
    let (response_sender, response_receiver) = crossbeam_channel::bounded(1);

    request_sender.send(WorkflowRequest::SOE(TypedWorkflowRequestSOE {
        module_name,
        workflow_name,
        response_sender,
    })).unwrap();

    response_receiver.recv_async().await.unwrap().unpack()
}

pub async fn request_workflow_i<W: WorkflowInstanceI>(
    request_sender: Sender<WorkflowRequest>,
    input: W::Input,
) {
    let module_name = W::module_name();
    let workflow_name = W::workflow_name();
    let (response_sender, response_receiver) = crossbeam_channel::bounded(1);

    request_sender.send(WorkflowRequest::I(TypedWorkflowRequestI {
        input: Box::new(input),
        module_name,
        workflow_name,
        response_sender,
    })).unwrap();

    response_receiver.recv_async().await.unwrap().unpack()
}

pub async fn request_workflow_ie<W: WorkflowInstanceIE>(
    request_sender: Sender<WorkflowRequest>,
    input: W::Input,
) -> Result<(), W::Error> {
    let module_name = W::module_name();
    let workflow_name = W::workflow_name();
    let (response_sender, response_receiver) = crossbeam_channel::bounded(1);

    request_sender.send(WorkflowRequest::IE(TypedWorkflowRequestIE {
        input: Box::new(input),
        module_name,
        workflow_name,
        response_sender,
    })).unwrap();

    response_receiver.recv_async().await.unwrap().unpack()
}

pub async fn request_workflow_io<W: WorkflowInstanceIO>(
    request_sender: Sender<WorkflowRequest>,
    input: W::Input,
) -> Result<W::Output, ()> {
    let module_name = W::module_name();
    let workflow_name = W::workflow_name();
    let (response_sender, response_receiver) = crossbeam_channel::bounded(1);

    request_sender.send(WorkflowRequest::IO(TypedWorkflowRequestIO {
        input: Box::new(input),
        module_name,
        workflow_name,
        response_sender,
    })).unwrap();

    response_receiver.recv_async().await.unwrap().unpack()
}

pub async fn request_workflow_ioe<W: WorkflowInstanceIOE>(
    request_sender: Sender<WorkflowRequest>,
    input: W::Input,
) -> Result<W::Output, W::Error> {
    let module_name = W::module_name();
    let workflow_name = W::workflow_name();
    let (response_sender, response_receiver) = crossbeam_channel::bounded(1);

    request_sender.send(WorkflowRequest::IOE(TypedWorkflowRequestIOE {
        input: Box::new(input),
        module_name,
        workflow_name,
        response_sender,
    })).unwrap();

    response_receiver.recv_async().await.unwrap().unpack()
}

pub async fn request_workflow_is<W: WorkflowInstanceIS>(
    request_sender: Sender<WorkflowRequest>,
    input: W::Input,
) -> Result<W::Output, W::Error> {
    let module_name = W::module_name();
    let workflow_name = W::workflow_name();
    let (response_sender, response_receiver) = crossbeam_channel::bounded(1);

    request_sender.send(WorkflowRequest::IS(TypedWorkflowRequestIS {
        input: Box::new(input),
        module_name,
        workflow_name,
        response_sender,
    })).unwrap();

    response_receiver.recv_async().await.unwrap().unpack()
}

pub async fn request_workflow_ise<W: WorkflowInstanceISE>(
    request_sender: Sender<WorkflowRequest>,
    input: W::Input,
) -> Result<W::Output, W::Error> {
    let module_name = W::module_name();
    let workflow_name = W::workflow_name();
    let (response_sender, response_receiver) = crossbeam_channel::bounded(1);

    request_sender.send(WorkflowRequest::ISE(TypedWorkflowRequestISE {
        input: Box::new(input),
        module_name,
        workflow_name,
        response_sender,
    })).unwrap();

    response_receiver.recv_async().await.unwrap().unpack()
}

pub async fn request_workflow_iso<W: WorkflowInstanceISO>(
    request_sender: Sender<WorkflowRequest>,
    input: W::Input,
) -> Result<W::Output, ()> {
    let module_name = W::module_name();
    let workflow_name = W::workflow_name();
    let (response_sender, response_receiver) = crossbeam_channel::bounded(1);

    request_sender.send(WorkflowRequest::ISO(TypedWorkflowRequestISO {
        input: Box::new(input),
        module_name,
        workflow_name,
        response_sender,
    })).unwrap();

    response_receiver.recv_async().await.unwrap().unpack()
}

pub async fn request_workflow_isoe<W: WorkflowInstanceISOE>(
    request_sender: Sender<WorkflowRequest>,
    input: W::Input,
) -> Result<W::Output, W::Error> {
    let module_name = W::module_name();
    let workflow_name = W::workflow_name();
    let (response_sender, response_receiver) = crossbeam_channel::bounded(1);

    request_sender.send(WorkflowRequest::ISOE(TypedWorkflowRequestISOE {
        input: Box::new(input),
        module_name,
        workflow_name,
        response_sender,
    })).unwrap();

    response_receiver.recv_async().await.unwrap().unpack()
}
