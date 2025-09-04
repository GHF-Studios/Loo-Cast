use bevy::prelude::*;
use futures::FutureExt;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use tokio::sync::Mutex;

use crate::utils::premium_box::AnySendSyncPremiumBox;
use crate::workflow::composite_workflow_context::{ScopedCompositeWorkflowContext, CURRENT_COMPOSITE_WORKFLOW_ID};
use crate::workflow::response::WorkflowResponse;
use crate::workflow::types::WorkflowID;
use tokio::time::{timeout, Duration};

use super::{channels::*, request::*, traits::*};

static RESPONSE_INBOX: Lazy<Mutex<HashMap<WorkflowID, WorkflowResponse>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub async fn run_workflow<W: WorkflowType>() {
    warn!("Running run_workflow for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
    let composite_workflow_id = CURRENT_COMPOSITE_WORKFLOW_ID.with(|id| *id);
    let workflow_id = WorkflowID {
        module: W::MODULE_NAME,
        workflow: W::WORKFLOW_NAME,
    };

    get_request_sender()
        .send(TypedWorkflowRequest {
            module_name: workflow_id.module,
            workflow_name: workflow_id.workflow,
            composite_workflow_id,
        })
        .unwrap();

    loop {
        let mut receiver = get_response_receiver().await;

        match timeout(Duration::from_secs(5), receiver.recv()).await {
            Ok(Some(response)) => {
                let key = WorkflowID {
                    module: response.module_name,
                    workflow: response.workflow_name,
                };

                if key == workflow_id {
                    warn!("Finished run_workflow for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
                    return;
                }

                RESPONSE_INBOX.lock().await.insert(key, WorkflowResponse::None(response));
            },
            Ok(None) => {
                panic!("Channel closed while waiting for response to {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            },
            Err(_) => {
                panic!("Timeout while waiting for response to {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            },
        }

        if let Some(WorkflowResponse::None(_response)) = RESPONSE_INBOX.lock().await.remove(&workflow_id) {
            warn!("Finished run_workflow for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            return;
        }

        tokio::task::yield_now().await;
    }
}

pub async fn run_workflow_e<W: WorkflowTypeE>() -> Result<(), W::Error> {
    warn!("Running run_workflow_e for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
    let composite_workflow_id = CURRENT_COMPOSITE_WORKFLOW_ID.with(|id| *id);
    let workflow_id = WorkflowID {
        module: W::MODULE_NAME,
        workflow: W::WORKFLOW_NAME,
    };

    get_request_e_sender()
        .send(TypedWorkflowRequestE {
            module_name: workflow_id.module,
            workflow_name: workflow_id.workflow,
            composite_workflow_id,
        })
        .unwrap();

    loop {
        let mut receiver = get_response_e_receiver().await;

        match timeout(Duration::from_secs(5), receiver.recv()).await {
            Ok(Some(response)) => {
                let key = WorkflowID {
                    module: response.module_name,
                    workflow: response.workflow_name,
                };

                if key == workflow_id {
                    warn!("Finished run_workflow_e for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
                    return response.unpack();
                }

                RESPONSE_INBOX.lock().await.insert(key, WorkflowResponse::E(response));
            },
            Ok(None) => {
                panic!("Channel closed while waiting for response to {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            },
            Err(_) => {
                panic!("Timeout while waiting for response to {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            },
        }

        if let Some(WorkflowResponse::E(response)) = RESPONSE_INBOX.lock().await.remove(&workflow_id) {
            warn!("Finished run_workflow_e for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            return response.unpack();
        }

        tokio::task::yield_now().await;
    }
}

pub async fn run_workflow_o<W: WorkflowTypeO>() -> W::Output {
    warn!("Running run_workflow_o for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
    let composite_workflow_id = CURRENT_COMPOSITE_WORKFLOW_ID.with(|id| *id);
    let workflow_id = WorkflowID {
        module: W::MODULE_NAME,
        workflow: W::WORKFLOW_NAME,
    };

    get_request_o_sender()
        .send(TypedWorkflowRequestO {
            module_name: workflow_id.module,
            workflow_name: workflow_id.workflow,
            composite_workflow_id,
        })
        .unwrap();

    loop {
        let mut receiver = get_response_o_receiver().await;

        match timeout(Duration::from_secs(5), receiver.recv()).await {
            Ok(Some(response)) => {
                let key = WorkflowID {
                    module: response.module_name,
                    workflow: response.workflow_name,
                };

                if key == workflow_id {
                    warn!("Finished run_workflow_o for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
                    return response.unpack();
                }

                RESPONSE_INBOX.lock().await.insert(key, WorkflowResponse::O(response));
            },
            Ok(None) => {
                panic!("Channel closed while waiting for response to {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            },
            Err(_) => {
                panic!("Timeout while waiting for response to {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            },
        }

        if let Some(WorkflowResponse::O(response)) = RESPONSE_INBOX.lock().await.remove(&workflow_id) {
            warn!("Finished run_workflow_o for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            return response.unpack();
        }

        tokio::task::yield_now().await;
    }
}

pub async fn run_workflow_oe<W: WorkflowTypeOE>() -> Result<W::Output, W::Error> {
    warn!("Running run_workflow_oe for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
    let composite_workflow_id = CURRENT_COMPOSITE_WORKFLOW_ID.with(|id| *id);
    let workflow_id = WorkflowID {
        module: W::MODULE_NAME,
        workflow: W::WORKFLOW_NAME,
    };

    get_request_oe_sender()
        .send(TypedWorkflowRequestOE {
            module_name: workflow_id.module,
            workflow_name: workflow_id.workflow,
            composite_workflow_id,
        })
        .unwrap();

    loop {
        let mut receiver = get_response_oe_receiver().await;

        match timeout(Duration::from_secs(5), receiver.recv()).await {
            Ok(Some(response)) => {
                let key = WorkflowID {
                    module: response.module_name,
                    workflow: response.workflow_name,
                };

                if key == workflow_id {
                    warn!("Finished run_workflow_oe for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
                    return response.unpack();
                }

                RESPONSE_INBOX.lock().await.insert(key, WorkflowResponse::OE(response));
            },
            Ok(None) => {
                panic!("Channel closed while waiting for response to {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            },
            Err(_) => {
                panic!("Timeout while waiting for response to {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            },
        }

        if let Some(WorkflowResponse::OE(response)) = RESPONSE_INBOX.lock().await.remove(&workflow_id) {
            warn!("Finished run_workflow_oe for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            return response.unpack();
        }

        tokio::task::yield_now().await;
    }
}

pub async fn run_workflow_i<W: WorkflowTypeI>(input: W::Input) {
    warn!("Running run_workflow_i for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
    let composite_workflow_id = CURRENT_COMPOSITE_WORKFLOW_ID.with(|id| *id);
    let workflow_id = WorkflowID {
        module: W::MODULE_NAME,
        workflow: W::WORKFLOW_NAME,
    };

    get_request_i_sender()
        .send(TypedWorkflowRequestI {
            input: AnySendSyncPremiumBox::new(input, format!("{}::{}::Input", W::MODULE_NAME, W::WORKFLOW_NAME)),
            module_name: workflow_id.module,
            workflow_name: workflow_id.workflow,
            composite_workflow_id,
        })
        .unwrap();

    loop {
        let mut receiver = get_response_i_receiver().await;

        match timeout(Duration::from_secs(5), receiver.recv()).await {
            Ok(Some(response)) => {
                let key = WorkflowID {
                    module: response.module_name,
                    workflow: response.workflow_name,
                };

                if key == workflow_id {
                    warn!("Finished run_workflow_i for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
                    return;
                }

                RESPONSE_INBOX.lock().await.insert(key, WorkflowResponse::None(response));
            },
            Ok(None) => {
                panic!("Channel closed while waiting for response to {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            },
            Err(_) => {
                panic!("Timeout while waiting for response to {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            },
        }

        if let Some(WorkflowResponse::None(_response)) = RESPONSE_INBOX.lock().await.remove(&workflow_id) {
            warn!("Finished run_workflow_i for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            return;
        }

        tokio::task::yield_now().await;
    }
}

pub async fn run_workflow_ie<W: WorkflowTypeIE>(input: W::Input) -> Result<(), W::Error> {
    warn!("Running run_workflow_ie for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
    let composite_workflow_id = CURRENT_COMPOSITE_WORKFLOW_ID.with(|id| *id);
    let workflow_id = WorkflowID {
        module: W::MODULE_NAME,
        workflow: W::WORKFLOW_NAME,
    };

    get_request_ie_sender()
        .send(TypedWorkflowRequestIE {
            input: AnySendSyncPremiumBox::new(input, format!("{}::{}::Input", W::MODULE_NAME, W::WORKFLOW_NAME)),
            module_name: workflow_id.module,
            workflow_name: workflow_id.workflow,
            composite_workflow_id,
        })
        .unwrap();

    loop {
        let mut receiver = get_response_ie_receiver().await;

        match timeout(Duration::from_secs(5), receiver.recv()).await {
            Ok(Some(response)) => {
                let key = WorkflowID {
                    module: response.module_name,
                    workflow: response.workflow_name,
                };

                if key == workflow_id {
                    warn!("Finished run_workflow_ie for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
                    return response.unpack();
                }

                RESPONSE_INBOX.lock().await.insert(key, WorkflowResponse::E(response));
            },
            Ok(None) => {
                panic!("Channel closed while waiting for response to {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            },
            Err(_) => {
                panic!("Timeout while waiting for response to {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            },
        }

        if let Some(WorkflowResponse::E(response)) = RESPONSE_INBOX.lock().await.remove(&workflow_id) {
            warn!("Finished run_workflow_ie for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            return response.unpack();
        }

        tokio::task::yield_now().await;
    }
}

pub async fn run_workflow_io<W: WorkflowTypeIO>(input: W::Input) -> W::Output {
    warn!("Running run_workflow_io for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
    let composite_workflow_id = CURRENT_COMPOSITE_WORKFLOW_ID.with(|id| *id);
    let workflow_id = WorkflowID {
        module: W::MODULE_NAME,
        workflow: W::WORKFLOW_NAME,
    };

    get_request_io_sender()
        .send(TypedWorkflowRequestIO {
            input: AnySendSyncPremiumBox::new(input, format!("{}::{}::Input", W::MODULE_NAME, W::WORKFLOW_NAME)),
            module_name: workflow_id.module,
            workflow_name: workflow_id.workflow,
            composite_workflow_id,
        })
        .unwrap();

    loop {
        let mut receiver = get_response_io_receiver().await;

        match timeout(Duration::from_secs(5), receiver.recv()).await {
            Ok(Some(response)) => {
                let key = WorkflowID {
                    module: response.module_name,
                    workflow: response.workflow_name,
                };

                if key == workflow_id {
                    warn!("Finished run_workflow_io for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
                    return response.unpack();
                }

                RESPONSE_INBOX.lock().await.insert(key, WorkflowResponse::O(response));
            },
            Ok(None) => {
                panic!("Channel closed while waiting for response to {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            },
            Err(_) => {
                panic!("Timeout while waiting for response to {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            },
        }

        if let Some(WorkflowResponse::O(response)) = RESPONSE_INBOX.lock().await.remove(&workflow_id) {
            warn!("Finished run_workflow_io for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            return response.unpack();
        }

        tokio::task::yield_now().await;
    }
}

pub async fn run_workflow_ioe<W: WorkflowTypeIOE>(input: W::Input) -> Result<W::Output, W::Error> {
    warn!("Running run_workflow_ioe for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
    let composite_workflow_id = CURRENT_COMPOSITE_WORKFLOW_ID.with(|id| *id);
    let workflow_id = WorkflowID {
        module: W::MODULE_NAME,
        workflow: W::WORKFLOW_NAME,
    };

    get_request_ioe_sender()
        .send(TypedWorkflowRequestIOE {
            input: AnySendSyncPremiumBox::new(input, format!("{}::{}::Input", W::MODULE_NAME, W::WORKFLOW_NAME)),
            module_name: workflow_id.module,
            workflow_name: workflow_id.workflow,
            composite_workflow_id,
        })
        .unwrap();

    loop {
        let mut receiver = get_response_ioe_receiver().await;

        match timeout(Duration::from_secs(5), receiver.recv()).await {
            Ok(Some(response)) => {
                let key = WorkflowID {
                    module: response.module_name,
                    workflow: response.workflow_name,
                };

                if key == workflow_id {
                    warn!("Finished run_workflow_ioe for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
                    return response.unpack();
                }

                RESPONSE_INBOX.lock().await.insert(key, WorkflowResponse::OE(response));
            },
            Ok(None) => {
                panic!("Channel closed while waiting for response to {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            },
            Err(_) => {
                panic!("Timeout while waiting for response to {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            },
        }

        if let Some(WorkflowResponse::OE(response)) = RESPONSE_INBOX.lock().await.remove(&workflow_id) {
            warn!("Finished run_workflow_ioe for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            return response.unpack();
        }

        tokio::task::yield_now().await;
    }
}

#[track_caller]
pub fn handle_composite_workflow_return_now<F>(handle: tokio::task::JoinHandle<ScopedCompositeWorkflowContext>, f: F)
where
    F: FnOnce(&ScopedCompositeWorkflowContext),
{
    match handle.now_or_never() {
        Some(Ok(ctx)) => f(&ctx),
        Some(Err(_join_error)) => error!("Composite workflow return handling failed because the composite workflow failed"),
        None => unreachable!("Expected workflow to be finished but it was not."),
    }
}

#[track_caller]
pub fn handle_composite_workflow_return_later<F>(handle: tokio::task::JoinHandle<ScopedCompositeWorkflowContext>, f: F)
where
    F: FnOnce(&ScopedCompositeWorkflowContext) + Send + 'static,
{
    crate::workflow::statics::TOKIO_RUNTIME.lock().unwrap().handle().spawn(async move {
        match handle.await {
            Ok(ctx) => f(&ctx),
            Err(_join_error) => {
                error!("Composite workflow return handling failed because the composite workflow failed");
            }
        }
    });
}
