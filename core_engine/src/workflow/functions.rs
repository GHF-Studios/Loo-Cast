use futures::FutureExt;
use crate::workflow::composite_workflow_context::{CURRENT_COMPOSITE_WORKFLOW_ID, ScopedCompositeWorkflowContext};
use crate::debug::types::AnySendSyncNamedBox;

use super::{channels::*, request::*, statics::PANIC_BUFFER, traits::*};

pub async fn run_workflow<W: WorkflowType>() {
    let module_name = W::MODULE_NAME;
    let workflow_name = W::WORKFLOW_NAME;
    let workflow_id = CURRENT_COMPOSITE_WORKFLOW_ID.with(|id| *id);

    get_request_sender()
        .send(TypedWorkflowRequest {
            module_name,
            workflow_name,
            workflow_id,
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
    let workflow_id = CURRENT_COMPOSITE_WORKFLOW_ID.with(|id| *id);

    get_request_e_sender()
        .send(TypedWorkflowRequestE {
            module_name,
            workflow_name,
            workflow_id,
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
    let workflow_id = CURRENT_COMPOSITE_WORKFLOW_ID.with(|id| *id);

    get_request_o_sender()
        .send(TypedWorkflowRequestO {
            module_name,
            workflow_name,
            workflow_id,
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
    let workflow_id = CURRENT_COMPOSITE_WORKFLOW_ID.with(|id| *id);

    get_request_oe_sender()
        .send(TypedWorkflowRequestOE {
            module_name,
            workflow_name,
            workflow_id,
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
    let workflow_id = CURRENT_COMPOSITE_WORKFLOW_ID.with(|id| *id);
    let input_type_name = format!("{}::{}::Input", module_name, workflow_name);

    get_request_i_sender()
        .send(TypedWorkflowRequestI {
            input: AnySendSyncNamedBox::new(input, input_type_name),
            module_name,
            workflow_name,
            workflow_id,
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
    let workflow_id = CURRENT_COMPOSITE_WORKFLOW_ID.with(|id| *id);
    let input_type_name = format!("{}::{}::Input", module_name, workflow_name);

    get_request_ie_sender()
        .send(TypedWorkflowRequestIE {
            input: AnySendSyncNamedBox::new(input, input_type_name),
            module_name,
            workflow_name,
            workflow_id,
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
    let workflow_id = CURRENT_COMPOSITE_WORKFLOW_ID.with(|id| *id);
    let input_type_name = format!("{}::{}::Input", module_name, workflow_name);

    get_request_io_sender()
        .send(TypedWorkflowRequestIO {
            input: AnySendSyncNamedBox::new(input, input_type_name),
            module_name,
            workflow_name,
            workflow_id,
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
    let workflow_id = CURRENT_COMPOSITE_WORKFLOW_ID.with(|id| *id);
    let input_type_name = format!("{}::{}::Input", module_name, workflow_name);

    get_request_ioe_sender()
        .send(TypedWorkflowRequestIOE {
            input: AnySendSyncNamedBox::new(input, input_type_name),
            module_name,
            workflow_name,
            workflow_id,
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

pub fn handle_composite_workflow_return_now<F>(handle: tokio::task::JoinHandle<ScopedCompositeWorkflowContext>, f: F)
where
    F: FnOnce(&ScopedCompositeWorkflowContext),
{
    match handle.now_or_never() {
        Some(Ok(ctx)) => f(&ctx),
        Some(Err(e)) => panic!("Workflow panicked: {:?}", e),
        None => panic!("Expected workflow to be finished but it was not."),
    }
}


pub fn handle_composite_workflow_return_later<F>(
    handle: tokio::task::JoinHandle<ScopedCompositeWorkflowContext>,
    f: F,
)
where
    F: FnOnce(&ScopedCompositeWorkflowContext) + Send + 'static,
{
    let panic_buffer = PANIC_BUFFER.clone();
    crate::workflow::statics::TOKIO_RUNTIME
        .lock()
        .unwrap()
        .handle()
        .spawn(async move {
            match handle.await {
                Ok(ctx) => f(&ctx),
                Err(e) => {
                    let mut buffer = panic_buffer.lock().unwrap();
                    buffer.push(format!("Composite workflow panicked: {e}"));
                }
            }
        });
}
