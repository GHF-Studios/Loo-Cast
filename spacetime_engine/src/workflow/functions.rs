use futures::FutureExt;

use super::{channels::*, request::*, traits::*};

pub async fn run_workflow<W: WorkflowType>() {
    let module_name = W::MODULE_NAME;
    let workflow_name = W::WORKFLOW_NAME;

    get_request_sender()
        .send(TypedWorkflowRequest {
            module_name,
            workflow_name,
        })
        .unwrap();

    loop {
        if let Some(Some(response)) = {
            let mut receiver = get_response_receiver();
            receiver.recv().now_or_never()
        } {
            return response;
        } else {
            tokio::task::yield_now().await;
        }
    }
}
pub async fn run_workflow_e<W: WorkflowTypeE>() -> Result<(), W::Error> {
    let module_name = W::MODULE_NAME;
    let workflow_name = W::WORKFLOW_NAME;

    get_request_e_sender()
        .send(TypedWorkflowRequestE {
            module_name,
            workflow_name,
        })
        .unwrap();

    loop {
        if let Some(Some(response)) = {
            let mut receiver = get_response_e_receiver();
            receiver.recv().now_or_never()
        } {
            return response.unpack();
        } else {
            tokio::task::yield_now().await;
        }
    }
}
pub async fn run_workflow_o<W: WorkflowTypeO>() -> W::Output {
    let module_name = W::MODULE_NAME;
    let workflow_name = W::WORKFLOW_NAME;

    get_request_o_sender()
        .send(TypedWorkflowRequestO {
            module_name,
            workflow_name,
        })
        .unwrap();

    loop {
        if let Some(Some(response)) = {
            let mut receiver = get_response_o_receiver();
            receiver.recv().now_or_never()
        } {
            return response.unpack();
        } else {
            tokio::task::yield_now().await;
        }
    }
}
pub async fn run_workflow_oe<W: WorkflowTypeOE>() -> Result<W::Output, W::Error> {
    let module_name = W::MODULE_NAME;
    let workflow_name = W::WORKFLOW_NAME;

    get_request_oe_sender()
        .send(TypedWorkflowRequestOE {
            module_name,
            workflow_name,
        })
        .unwrap();

    loop {
        if let Some(Some(response)) = {
            let mut receiver = get_response_oe_receiver();
            receiver.recv().now_or_never()
        } {
            return response.unpack();
        } else {
            tokio::task::yield_now().await;
        }
    }
}
pub async fn run_workflow_i<W: WorkflowTypeI>(input: W::Input) {
    let module_name = W::MODULE_NAME;
    let workflow_name = W::WORKFLOW_NAME;

    get_request_i_sender()
        .send(TypedWorkflowRequestI {
            input: Box::new(input),
            module_name,
            workflow_name,
        })
        .unwrap();

    loop {
        if let Some(Some(response)) = {
            let mut receiver = get_response_i_receiver();
            receiver.recv().now_or_never()
        } {
            return response;
        } else {
            tokio::task::yield_now().await;
        }
    }
}
pub async fn run_workflow_ie<W: WorkflowTypeIE>(input: W::Input) -> Result<(), W::Error> {
    let module_name = W::MODULE_NAME;
    let workflow_name = W::WORKFLOW_NAME;

    get_request_ie_sender()
        .send(TypedWorkflowRequestIE {
            input: Box::new(input),
            module_name,
            workflow_name,
        })
        .unwrap();

    loop {
        if let Some(Some(response)) = {
            let mut receiver = get_response_ie_receiver();
            receiver.recv().now_or_never()
        } {
            return response.unpack();
        } else {
            tokio::task::yield_now().await;
        }
    }
}
pub async fn run_workflow_io<W: WorkflowTypeIO>(input: W::Input) -> W::Output {
    let module_name = W::MODULE_NAME;
    let workflow_name = W::WORKFLOW_NAME;

    get_request_io_sender()
        .send(TypedWorkflowRequestIO {
            input: Box::new(input),
            module_name,
            workflow_name,
        })
        .unwrap();

    loop {
        if let Some(Some(response)) = {
            let mut response_receiver = get_response_io_receiver();
            response_receiver.recv().now_or_never()
        } {
            return response.unpack();
        } else {
            tokio::task::yield_now().await;
        }
    }
}
pub async fn run_workflow_ioe<W: WorkflowTypeIOE>(input: W::Input) -> Result<W::Output, W::Error> {
    let module_name = W::MODULE_NAME;
    let workflow_name = W::WORKFLOW_NAME;

    get_request_ioe_sender()
        .send(TypedWorkflowRequestIOE {
            input: Box::new(input),
            module_name,
            workflow_name,
        })
        .unwrap();

    loop {
        if let Some(Some(response)) = {
            let mut receiver = get_response_ioe_receiver();
            receiver.recv().now_or_never()
        } {
            return response.unpack();
        } else {
            tokio::task::yield_now().await;
        }
    }
}

pub fn handle_composite_workflow_return(handle: tokio::task::JoinHandle<()>, handler: fn()) {
    let inner_handler = move |handle: tokio::task::JoinHandle<()>| {
        crate::workflow::statics::COMPOSITE_WORKFLOW_RUNTIME
            .lock()
            .unwrap()
            .spawn(Box::pin(async move {
                match handle.await {
                    Err(e) => {
                        unreachable!("{}", e);
                    }
                    Ok(_) => {
                        handler();
                    }
                }
            }));
    };

    inner_handler(handle);
}
