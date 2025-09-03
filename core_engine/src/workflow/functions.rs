use bevy::prelude::*;
use futures::FutureExt;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::utils::premium_box::AnySendSyncPremiumBox;
use crate::workflow::composite_workflow_context::{ScopedCompositeWorkflowContext, CURRENT_COMPOSITE_WORKFLOW_ID};
use crate::workflow::response::{TypedWorkflowResponse, WorkflowResponse};
use crate::workflow::types::WorkflowID;

use super::{channels::*, request::*, traits::*};

static RESPONSE_INBOX: Lazy<Mutex<HashMap<WorkflowID, WorkflowResponse>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub async fn run_workflow<W: WorkflowType>() {
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
        if let Some(Some(response)) = {
            let mut receiver = get_response_receiver();
            receiver.recv().now_or_never()
        } {
            let key = WorkflowID {
                module: response.module_name,
                workflow: response.workflow_name,
            };

            if key == workflow_id {
                return;
            }

            RESPONSE_INBOX.lock().unwrap().insert(key, WorkflowResponse::None(response));
            tokio::task::yield_now().await;
            continue;
        }

        let response = RESPONSE_INBOX.lock().unwrap().remove(&workflow_id);
        let response = match response {
            Some(response) => response,
            None => {
                tokio::task::yield_now().await;
                continue;
            }
        };

        if let WorkflowResponse::None(_r) = response {
            return;
        }

        RESPONSE_INBOX.lock().unwrap().insert(response.get_worfklow_id(), response);
        tokio::task::yield_now().await;
    }
}

pub async fn run_workflow_e<W: WorkflowTypeE>() -> Result<(), W::Error> {
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
        if let Some(Some(response)) = {
            let mut receiver = get_response_e_receiver();
            receiver.recv().now_or_never()
        } {
            let key = WorkflowID {
                module: response.module_name,
                workflow: response.workflow_name,
            };

            if key == workflow_id {
                return response.unpack();
            }

            RESPONSE_INBOX.lock().unwrap().insert(key, WorkflowResponse::E(response));
            tokio::task::yield_now().await;
            continue;
        }

        let response = RESPONSE_INBOX.lock().unwrap().remove(&workflow_id);
        let response = match response {
            Some(response) => response,
            None => {
                tokio::task::yield_now().await;
                continue;
            }
        };

        if let WorkflowResponse::E(r) = response {
            return r.unpack();
        }

        RESPONSE_INBOX.lock().unwrap().insert(response.get_worfklow_id(), response);
        tokio::task::yield_now().await;
    }
}

pub async fn run_workflow_o<W: WorkflowTypeO>() -> W::Output {
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
        if let Some(Some(response)) = {
            let mut receiver = get_response_o_receiver();
            receiver.recv().now_or_never()
        } {
            let key = WorkflowID {
                module: response.module_name,
                workflow: response.workflow_name,
            };

            if key == workflow_id {
                return response.unpack();
            }

            RESPONSE_INBOX.lock().unwrap().insert(key, WorkflowResponse::O(response));
            tokio::task::yield_now().await;
            continue;
        }

        let response = RESPONSE_INBOX.lock().unwrap().remove(&workflow_id);
        let response = match response {
            Some(response) => response,
            None => {
                tokio::task::yield_now().await;
                continue;
            }
        };

        if let WorkflowResponse::O(r) = response {
            return r.unpack();
        }

        RESPONSE_INBOX.lock().unwrap().insert(response.get_worfklow_id(), response);
        tokio::task::yield_now().await;
    }
}

pub async fn run_workflow_oe<W: WorkflowTypeOE>() -> Result<W::Output, W::Error> {
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
        if let Some(Some(response)) = {
            let mut receiver = get_response_oe_receiver();
            receiver.recv().now_or_never()
        } {
            let key = WorkflowID {
                module: response.module_name,
                workflow: response.workflow_name,
            };

            if key == workflow_id {
                return response.unpack();
            }

            RESPONSE_INBOX.lock().unwrap().insert(key, WorkflowResponse::OE(response));
            tokio::task::yield_now().await;
            continue;
        }

        let response = RESPONSE_INBOX.lock().unwrap().remove(&workflow_id);
        let response = match response {
            Some(response) => response,
            None => {
                tokio::task::yield_now().await;
                continue;
            }
        };

        if let WorkflowResponse::OE(r) = response {
            return r.unpack();
        }

        RESPONSE_INBOX.lock().unwrap().insert(response.get_worfklow_id(), response);
        tokio::task::yield_now().await;
    }
}

pub async fn run_workflow_i<W: WorkflowTypeI>(input: W::Input) {
    bevy::prelude::warn!("Running run_workflow_i for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
    let composite_workflow_id = CURRENT_COMPOSITE_WORKFLOW_ID.with(|id| *id);
    let workflow_id = WorkflowID {
        module: W::MODULE_NAME,
        workflow: W::WORKFLOW_NAME,
    };

    get_request_i_sender()
        .send(TypedWorkflowRequestI {
            input: AnySendSyncPremiumBox::new(input, format!("{}::{}::Input", workflow_id.module, workflow_id.workflow)),
            module_name: workflow_id.module,
            workflow_name: workflow_id.workflow,
            composite_workflow_id,
        })
        .unwrap();

    bevy::prelude::warn!("Sent request for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
    loop {
        if let Some(Some(response)) = {
            let mut receiver = get_response_i_receiver();
            receiver.recv().now_or_never()
        } {
            let key = WorkflowID {
                module: response.module_name,
                workflow: response.workflow_name,
            };

            if key == workflow_id {
                bevy::prelude::warn!("Finished {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
                return;
            }

            RESPONSE_INBOX.lock().unwrap().insert(key, WorkflowResponse::None(response));
            tokio::task::yield_now().await;
            continue;
        }

        let response = RESPONSE_INBOX.lock().unwrap().remove(&workflow_id);
        let response = match response {
            Some(response) => response,
            None => {
                tokio::task::yield_now().await;
                continue;
            }
        };

        if let WorkflowResponse::None(_r) = response {
            return;
        }

        RESPONSE_INBOX.lock().unwrap().insert(response.get_worfklow_id(), response);
        tokio::task::yield_now().await;
    }
}

pub async fn run_workflow_ie<W: WorkflowTypeIE>(input: W::Input) -> Result<(), W::Error> {
    let composite_workflow_id = CURRENT_COMPOSITE_WORKFLOW_ID.with(|id| *id);
    let workflow_id = WorkflowID {
        module: W::MODULE_NAME,
        workflow: W::WORKFLOW_NAME,
    };

    get_request_ie_sender()
        .send(TypedWorkflowRequestIE {
            input: AnySendSyncPremiumBox::new(input, format!("{}::{}::Input", workflow_id.module, workflow_id.workflow)),
            module_name: workflow_id.module,
            workflow_name: workflow_id.workflow,
            composite_workflow_id,
        })
        .unwrap();

    loop {
        if let Some(Some(response)) = {
            let mut receiver = get_response_ie_receiver();
            receiver.recv().now_or_never()
        } {
            let key = WorkflowID {
                module: response.module_name,
                workflow: response.workflow_name,
            };

            if key == workflow_id {
                return response.unpack();
            }

            RESPONSE_INBOX.lock().unwrap().insert(key, WorkflowResponse::E(response));
            tokio::task::yield_now().await;
            continue;
        }

        let response = RESPONSE_INBOX.lock().unwrap().remove(&workflow_id);
        let response = match response {
            Some(response) => response,
            None => {
                tokio::task::yield_now().await;
                continue;
            }
        };

        if let WorkflowResponse::E(r) = response {
            return r.unpack();
        }

        RESPONSE_INBOX.lock().unwrap().insert(response.get_worfklow_id(), response);
        tokio::task::yield_now().await;
    }
}

pub async fn run_workflow_io<W: WorkflowTypeIO>(input: W::Input) -> W::Output {
    let composite_workflow_id = CURRENT_COMPOSITE_WORKFLOW_ID.with(|id| *id);
    let workflow_id = WorkflowID {
        module: W::MODULE_NAME,
        workflow: W::WORKFLOW_NAME,
    };

    get_request_io_sender()
        .send(TypedWorkflowRequestIO {
            input: AnySendSyncPremiumBox::new(input, format!("{}::{}::Input", workflow_id.module, workflow_id.workflow)),
            module_name: workflow_id.module,
            workflow_name: workflow_id.workflow,
            composite_workflow_id,
        })
        .unwrap();

    loop {
        if let Some(Some(response)) = {
            let mut receiver = get_response_io_receiver();
            receiver.recv().now_or_never()
        } {
            let key = WorkflowID {
                module: response.module_name,
                workflow: response.workflow_name,
            };

            if key == workflow_id {
                return response.unpack();
            }

            RESPONSE_INBOX.lock().unwrap().insert(key, WorkflowResponse::O(response));
            tokio::task::yield_now().await;
            continue;
        }

        let response = RESPONSE_INBOX.lock().unwrap().remove(&workflow_id);
        let response = match response {
            Some(response) => response,
            None => {
                tokio::task::yield_now().await;
                continue;
            }
        };

        if let WorkflowResponse::O(r) = response {
            return r.unpack();
        }

        RESPONSE_INBOX.lock().unwrap().insert(response.get_worfklow_id(), response);
        tokio::task::yield_now().await;
    }
}

pub async fn run_workflow_ioe<W: WorkflowTypeIOE>(input: W::Input) -> Result<W::Output, W::Error> {
    let composite_workflow_id = CURRENT_COMPOSITE_WORKFLOW_ID.with(|id| *id);
    let workflow_id = WorkflowID {
        module: W::MODULE_NAME,
        workflow: W::WORKFLOW_NAME,
    };

    get_request_ioe_sender()
        .send(TypedWorkflowRequestIOE {
            input: AnySendSyncPremiumBox::new(input, format!("{}::{}::Input", workflow_id.module, workflow_id.workflow)),
            module_name: workflow_id.module,
            workflow_name: workflow_id.workflow,
            composite_workflow_id,
        })
        .unwrap();

    loop {
        if let Some(Some(response)) = {
            let mut receiver = get_response_ioe_receiver();
            receiver.recv().now_or_never()
        } {
            let key = WorkflowID {
                module: response.module_name,
                workflow: response.workflow_name,
            };

            if key == workflow_id {
                return response.unpack();
            }

            RESPONSE_INBOX.lock().unwrap().insert(key, WorkflowResponse::OE(response));
            tokio::task::yield_now().await;
            continue;
        }

        let response = RESPONSE_INBOX.lock().unwrap().remove(&workflow_id);
        let response = match response {
            Some(response) => response,
            None => {
                tokio::task::yield_now().await;
                continue;
            }
        };

        if let WorkflowResponse::OE(r) = response {
            return r.unpack();
        }

        RESPONSE_INBOX.lock().unwrap().insert(response.get_worfklow_id(), response);
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
