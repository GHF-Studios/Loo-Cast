use crate::bevy::prelude::{error, warn};
use futures::FutureExt;
use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::timeout;

use crate::config::statics::CONFIG;
use crate::time::functions::virtual_timeout;
use crate::utils::premium_box::AnySendSyncPremiumBox;
use crate::workflow::composite_workflow_context::{CURRENT_COMPOSITE_WORKFLOW_ID, ScopedCompositeWorkflowContext};
use crate::workflow::resources::{WorkflowTimeoutSignal, emit_workflow_timeout_signal};
use crate::workflow::response::WorkflowResponse;
use crate::workflow::types::{WorkflowID, WorkflowTimeoutMode};

use super::{channels::*, request::*, traits::*};

static RESPONSE_INBOX: Lazy<Mutex<HashMap<WorkflowID, WorkflowResponse>>> = Lazy::new(|| Mutex::new(HashMap::new()));
static IGNORED_WORKFLOW_LOGS: Lazy<HashMap<String, HashSet<String>>> = Lazy::new(|| {
    fn split_top_level_commas(s: &str) -> Vec<String> {
        let mut parts = Vec::new();
        let mut current = String::new();
        let mut brace_level = 0;

        for c in s.chars() {
            match c {
                '{' => {
                    brace_level += 1;
                    current.push(c);
                }
                '}' => {
                    if brace_level > 0 {
                        brace_level -= 1;
                    }
                    current.push(c);
                }
                ',' if brace_level == 0 => {
                    parts.push(current.trim().to_string());
                    current.clear();
                }
                _ => current.push(c),
            }
        }

        if !current.trim().is_empty() {
            parts.push(current.trim().to_string());
        }

        parts
    }

    let mut map = HashMap::new();
    let config = CONFIG().get::<&'static str>("workflow/runner_logging_blacklist");

    for entry in split_top_level_commas(config) {
        if let Some((module, suffix)) = entry.split_once("::") {
            let module = module.trim().to_string();

            // Wildcard: Module::*
            if suffix.trim() == "*" {
                map.entry(module).or_insert_with(|| {
                    let mut s = HashSet::new();
                    s.insert("*".to_string());
                    s
                });
                continue;
            }

            // Group: Module::{A, B, C}
            if suffix.starts_with('{') && suffix.ends_with('}') {
                let inner = &suffix[1..suffix.len() - 1];
                let set = inner
                    .split(',')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(str::to_string)
                    .collect::<HashSet<_>>();
                map.entry(module).or_insert_with(HashSet::new).extend(set);
                continue;
            }

            // Single item: Module::Something
            if !suffix.contains('{') && !suffix.contains('}') {
                map.entry(module).or_insert_with(HashSet::new).insert(suffix.trim().to_string());
                continue;
            }

            // Invalid suffix format
            panic!(
                "Invalid workflow blacklist entry: '{}'.
                Expected one of the following formats: 
                `Module::Workflow` / `Module::{{Workflow1, Workflow2, ...}}` / `Module::*`",
                entry
            );
        } else {
            panic!(
                "Invalid workflow blacklist entry: '{}'.
                Expected one of the following formats: 
                `Module::Workflow` / `Module::{{Workflow1, Workflow2, ...}}` / `Module::*`",
                entry
            );
        }
    }

    map
});

fn is_ignored_workflow(module: &str, workflow: &str) -> bool {
    IGNORED_WORKFLOW_LOGS
        .get(module)
        .map(|set| set.contains(workflow) || set.contains("*"))
        .unwrap_or(false)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkflowTimeoutControlDecision {
    Retry,
    Abort,
    Panic,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WorkflowTimeoutContext {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub timeout_count: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkflowRunnerControlError {
    TimeoutAborted {
        module_name: &'static str,
        workflow_name: &'static str,
        timeout_count: usize,
    },
}

#[derive(Debug)]
pub enum WorkflowControlledRunError<E> {
    Workflow(E),
    Control(WorkflowRunnerControlError),
}

#[inline]
fn publish_workflow_timeout_signal(module_name: &'static str, workflow_name: &'static str, timeout_count: usize) {
    emit_workflow_timeout_signal(WorkflowTimeoutSignal {
        module_name,
        workflow_name,
        timeout_count,
    });
}

pub async fn run_workflow<W: WorkflowType>(timeout_duration: Duration, timeout_mode: WorkflowTimeoutMode) {
    if !is_ignored_workflow(W::MODULE_NAME, W::WORKFLOW_NAME) {
        warn!("Running run_workflow for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
    }
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

        let result = match timeout_mode {
            WorkflowTimeoutMode::RealTime => timeout(timeout_duration, receiver.recv()).await.map_err(|_| ()),
            WorkflowTimeoutMode::VirtualTime => virtual_timeout(timeout_duration, receiver.recv()).await.map_err(|_| ()),
        };

        match result {
            Ok(Some(response)) => {
                let key = WorkflowID {
                    module: response.module_name,
                    workflow: response.workflow_name,
                };

                if key == workflow_id {
                    if !is_ignored_workflow(W::MODULE_NAME, W::WORKFLOW_NAME) {
                        warn!("Finished run_workflow for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
                    }
                    return;
                }

                RESPONSE_INBOX.lock().await.insert(key, WorkflowResponse::None(response));
            }
            Ok(None) => {
                panic!("Channel closed while waiting for response to {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            }
            Err(_) => {
                publish_workflow_timeout_signal(W::MODULE_NAME, W::WORKFLOW_NAME, 1);
                panic!("Timeout while waiting for response to {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            }
        }

        if let Some(WorkflowResponse::None(_response)) = RESPONSE_INBOX.lock().await.remove(&workflow_id) {
            if !is_ignored_workflow(W::MODULE_NAME, W::WORKFLOW_NAME) {
                warn!("Finished run_workflow for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            }
            return;
        }

        tokio::task::yield_now().await;
    }
}

pub async fn run_workflow_e<W: WorkflowTypeE>(timeout_duration: Duration, timeout_mode: WorkflowTimeoutMode) -> Result<(), W::Error> {
    if !is_ignored_workflow(W::MODULE_NAME, W::WORKFLOW_NAME) {
        warn!("Running run_workflow_e for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
    }
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

        let result = match timeout_mode {
            WorkflowTimeoutMode::RealTime => timeout(timeout_duration, receiver.recv()).await.map_err(|_| ()),
            WorkflowTimeoutMode::VirtualTime => virtual_timeout(timeout_duration, receiver.recv()).await.map_err(|_| ()),
        };

        match result {
            Ok(Some(response)) => {
                let key = WorkflowID {
                    module: response.module_name,
                    workflow: response.workflow_name,
                };

                if key == workflow_id {
                    if !is_ignored_workflow(W::MODULE_NAME, W::WORKFLOW_NAME) {
                        warn!("Finished run_workflow_e for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
                    }
                    return <Result<(), W::Error>>::from_response(response);
                }

                RESPONSE_INBOX.lock().await.insert(key, WorkflowResponse::E(response));
            }
            Ok(None) => {
                panic!("Channel closed while waiting for response to {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            }
            Err(_) => {
                publish_workflow_timeout_signal(W::MODULE_NAME, W::WORKFLOW_NAME, 1);
                panic!("Timeout while waiting for response to {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            }
        }

        if let Some(WorkflowResponse::E(response)) = RESPONSE_INBOX.lock().await.remove(&workflow_id) {
            if !is_ignored_workflow(W::MODULE_NAME, W::WORKFLOW_NAME) {
                warn!("Finished run_workflow_e for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            }
            return <Result<(), W::Error>>::from_response(response);
        }

        tokio::task::yield_now().await;
    }
}

pub async fn run_workflow_o<W: WorkflowTypeO>(timeout_duration: Duration, timeout_mode: WorkflowTimeoutMode) -> W::Output {
    if !is_ignored_workflow(W::MODULE_NAME, W::WORKFLOW_NAME) {
        warn!("Running run_workflow_o for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
    }
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

        let result = match timeout_mode {
            WorkflowTimeoutMode::RealTime => timeout(timeout_duration, receiver.recv()).await.map_err(|_| ()),
            WorkflowTimeoutMode::VirtualTime => virtual_timeout(timeout_duration, receiver.recv()).await.map_err(|_| ()),
        };

        match result {
            Ok(Some(response)) => {
                let key = WorkflowID {
                    module: response.module_name,
                    workflow: response.workflow_name,
                };

                if key == workflow_id {
                    if !is_ignored_workflow(W::MODULE_NAME, W::WORKFLOW_NAME) {
                        warn!("Finished run_workflow_o for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
                    }
                    return W::Output::from_boxed(response.output);
                }

                RESPONSE_INBOX.lock().await.insert(key, WorkflowResponse::O(response));
            }
            Ok(None) => {
                panic!("Channel closed while waiting for response to {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            }
            Err(_) => {
                publish_workflow_timeout_signal(W::MODULE_NAME, W::WORKFLOW_NAME, 1);
                panic!("Timeout while waiting for response to {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            }
        }

        if let Some(WorkflowResponse::O(response)) = RESPONSE_INBOX.lock().await.remove(&workflow_id) {
            if !is_ignored_workflow(W::MODULE_NAME, W::WORKFLOW_NAME) {
                warn!("Finished run_workflow_o for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            }
            return W::Output::from_boxed(response.output);
        }

        tokio::task::yield_now().await;
    }
}

pub async fn run_workflow_oe<W: WorkflowTypeOE>(timeout_duration: Duration, timeout_mode: WorkflowTimeoutMode) -> Result<W::Output, W::Error> {
    if !is_ignored_workflow(W::MODULE_NAME, W::WORKFLOW_NAME) {
        warn!("Running run_workflow_oe for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
    }
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

        let result = match timeout_mode {
            WorkflowTimeoutMode::RealTime => timeout(timeout_duration, receiver.recv()).await.map_err(|_| ()),
            WorkflowTimeoutMode::VirtualTime => virtual_timeout(timeout_duration, receiver.recv()).await.map_err(|_| ()),
        };

        match result {
            Ok(Some(response)) => {
                let key = WorkflowID {
                    module: response.module_name,
                    workflow: response.workflow_name,
                };

                if key == workflow_id {
                    if !is_ignored_workflow(W::MODULE_NAME, W::WORKFLOW_NAME) {
                        warn!("Finished run_workflow_oe for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
                    }
                    return <Result<W::Output, W::Error>>::from_response(response);
                }

                RESPONSE_INBOX.lock().await.insert(key, WorkflowResponse::OE(response));
            }
            Ok(None) => {
                panic!("Channel closed while waiting for response to {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            }
            Err(_) => {
                publish_workflow_timeout_signal(W::MODULE_NAME, W::WORKFLOW_NAME, 1);
                panic!("Timeout while waiting for response to {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            }
        }

        if let Some(WorkflowResponse::OE(response)) = RESPONSE_INBOX.lock().await.remove(&workflow_id) {
            if !is_ignored_workflow(W::MODULE_NAME, W::WORKFLOW_NAME) {
                warn!("Finished run_workflow_oe for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            }
            return <Result<W::Output, W::Error>>::from_response(response);
        }

        tokio::task::yield_now().await;
    }
}

pub async fn run_workflow_i<W: WorkflowTypeI>(timeout_duration: Duration, timeout_mode: WorkflowTimeoutMode, input: W::Input) {
    if !is_ignored_workflow(W::MODULE_NAME, W::WORKFLOW_NAME) {
        warn!("Running run_workflow_i for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
    }
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

        let result = match timeout_mode {
            WorkflowTimeoutMode::RealTime => timeout(timeout_duration, receiver.recv()).await.map_err(|_| ()),
            WorkflowTimeoutMode::VirtualTime => virtual_timeout(timeout_duration, receiver.recv()).await.map_err(|_| ()),
        };

        match result {
            Ok(Some(response)) => {
                let key = WorkflowID {
                    module: response.module_name,
                    workflow: response.workflow_name,
                };

                if key == workflow_id {
                    if !is_ignored_workflow(W::MODULE_NAME, W::WORKFLOW_NAME) {
                        warn!("Finished run_workflow_i for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
                    }
                    return;
                }

                RESPONSE_INBOX.lock().await.insert(key, WorkflowResponse::None(response));
            }
            Ok(None) => {
                panic!("Channel closed while waiting for response to {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            }
            Err(_) => {
                publish_workflow_timeout_signal(W::MODULE_NAME, W::WORKFLOW_NAME, 1);
                panic!("Timeout while waiting for response to {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            }
        }

        if let Some(WorkflowResponse::None(_response)) = RESPONSE_INBOX.lock().await.remove(&workflow_id) {
            if !is_ignored_workflow(W::MODULE_NAME, W::WORKFLOW_NAME) {
                warn!("Finished run_workflow_i for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            }
            return;
        }

        tokio::task::yield_now().await;
    }
}

pub async fn run_workflow_ie<W: WorkflowTypeIE>(timeout_duration: Duration, timeout_mode: WorkflowTimeoutMode, input: W::Input) -> Result<(), W::Error> {
    if !is_ignored_workflow(W::MODULE_NAME, W::WORKFLOW_NAME) {
        warn!("Running run_workflow_ie for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
    }
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

        let result = match timeout_mode {
            WorkflowTimeoutMode::RealTime => timeout(timeout_duration, receiver.recv()).await.map_err(|_| ()),
            WorkflowTimeoutMode::VirtualTime => virtual_timeout(timeout_duration, receiver.recv()).await.map_err(|_| ()),
        };

        match result {
            Ok(Some(response)) => {
                let key = WorkflowID {
                    module: response.module_name,
                    workflow: response.workflow_name,
                };

                if key == workflow_id {
                    if !is_ignored_workflow(W::MODULE_NAME, W::WORKFLOW_NAME) {
                        warn!("Finished run_workflow_ie for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
                    }
                    return <Result<(), W::Error>>::from_response(response);
                }

                RESPONSE_INBOX.lock().await.insert(key, WorkflowResponse::E(response));
            }
            Ok(None) => {
                panic!("Channel closed while waiting for response to {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            }
            Err(_) => {
                publish_workflow_timeout_signal(W::MODULE_NAME, W::WORKFLOW_NAME, 1);
                panic!("Timeout while waiting for response to {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            }
        }

        if let Some(WorkflowResponse::E(response)) = RESPONSE_INBOX.lock().await.remove(&workflow_id) {
            if !is_ignored_workflow(W::MODULE_NAME, W::WORKFLOW_NAME) {
                warn!("Finished run_workflow_ie for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            }
            return <Result<(), W::Error>>::from_response(response);
        }

        tokio::task::yield_now().await;
    }
}

pub async fn run_workflow_io<W: WorkflowTypeIO>(timeout_duration: Duration, timeout_mode: WorkflowTimeoutMode, input: W::Input) -> W::Output {
    if !is_ignored_workflow(W::MODULE_NAME, W::WORKFLOW_NAME) {
        warn!("Running run_workflow_io for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
    }
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

        let result = match timeout_mode {
            WorkflowTimeoutMode::RealTime => timeout(timeout_duration, receiver.recv()).await.map_err(|_| ()),
            WorkflowTimeoutMode::VirtualTime => virtual_timeout(timeout_duration, receiver.recv()).await.map_err(|_| ()),
        };

        match result {
            Ok(Some(response)) => {
                let key = WorkflowID {
                    module: response.module_name,
                    workflow: response.workflow_name,
                };

                if key == workflow_id {
                    if !is_ignored_workflow(W::MODULE_NAME, W::WORKFLOW_NAME) {
                        warn!("Finished run_workflow_io for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
                    }
                    return W::Output::from_boxed(response.output);
                }

                RESPONSE_INBOX.lock().await.insert(key, WorkflowResponse::O(response));
            }
            Ok(None) => {
                panic!("Channel closed while waiting for response to {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            }
            Err(_) => {
                publish_workflow_timeout_signal(W::MODULE_NAME, W::WORKFLOW_NAME, 1);
                panic!("Timeout while waiting for response to {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            }
        }

        if let Some(WorkflowResponse::O(response)) = RESPONSE_INBOX.lock().await.remove(&workflow_id) {
            if !is_ignored_workflow(W::MODULE_NAME, W::WORKFLOW_NAME) {
                warn!("Finished run_workflow_io for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            }
            return W::Output::from_boxed(response.output);
        }

        tokio::task::yield_now().await;
    }
}

pub async fn run_workflow_io_with_timeout_control<W, F>(
    timeout_duration: Duration,
    timeout_mode: WorkflowTimeoutMode,
    input: W::Input,
    mut on_timeout: F,
) -> Result<W::Output, WorkflowRunnerControlError>
where
    W: WorkflowTypeIO,
    F: FnMut(WorkflowTimeoutContext) -> WorkflowTimeoutControlDecision,
{
    if !is_ignored_workflow(W::MODULE_NAME, W::WORKFLOW_NAME) {
        warn!("Running run_workflow_io_with_timeout_control for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
    }
    let composite_workflow_id = CURRENT_COMPOSITE_WORKFLOW_ID.with(|id| *id);
    let workflow_id = WorkflowID {
        module: W::MODULE_NAME,
        workflow: W::WORKFLOW_NAME,
    };
    let mut timeout_count = 0usize;

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

        let result = match timeout_mode {
            WorkflowTimeoutMode::RealTime => timeout(timeout_duration, receiver.recv()).await.map_err(|_| ()),
            WorkflowTimeoutMode::VirtualTime => virtual_timeout(timeout_duration, receiver.recv()).await.map_err(|_| ()),
        };

        match result {
            Ok(Some(response)) => {
                let key = WorkflowID {
                    module: response.module_name,
                    workflow: response.workflow_name,
                };

                if key == workflow_id {
                    if !is_ignored_workflow(W::MODULE_NAME, W::WORKFLOW_NAME) {
                        warn!("Finished run_workflow_io_with_timeout_control for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
                    }
                    return Ok(W::Output::from_boxed(response.output));
                }

                RESPONSE_INBOX.lock().await.insert(key, WorkflowResponse::O(response));
            }
            Ok(None) => {
                panic!("Channel closed while waiting for response to {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            }
            Err(_) => {
                timeout_count += 1;
                publish_workflow_timeout_signal(W::MODULE_NAME, W::WORKFLOW_NAME, timeout_count);
                let decision = on_timeout(WorkflowTimeoutContext {
                    module_name: W::MODULE_NAME,
                    workflow_name: W::WORKFLOW_NAME,
                    timeout_count,
                });
                match decision {
                    WorkflowTimeoutControlDecision::Retry => {
                        if !is_ignored_workflow(W::MODULE_NAME, W::WORKFLOW_NAME) {
                            warn!(
                                "Timeout while waiting for {}::{}, retrying (timeout_count={})",
                                W::MODULE_NAME,
                                W::WORKFLOW_NAME,
                                timeout_count
                            );
                        }
                    }
                    WorkflowTimeoutControlDecision::Abort => {
                        return Err(WorkflowRunnerControlError::TimeoutAborted {
                            module_name: W::MODULE_NAME,
                            workflow_name: W::WORKFLOW_NAME,
                            timeout_count,
                        });
                    }
                    WorkflowTimeoutControlDecision::Panic => {
                        panic!("Timeout while waiting for response to {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
                    }
                }
            }
        }

        if let Some(WorkflowResponse::O(response)) = RESPONSE_INBOX.lock().await.remove(&workflow_id) {
            if !is_ignored_workflow(W::MODULE_NAME, W::WORKFLOW_NAME) {
                warn!("Finished run_workflow_io_with_timeout_control for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            }
            return Ok(W::Output::from_boxed(response.output));
        }

        tokio::task::yield_now().await;
    }
}

pub async fn run_workflow_ioe<W: WorkflowTypeIOE>(
    timeout_duration: Duration,
    timeout_mode: WorkflowTimeoutMode,
    input: W::Input,
) -> Result<W::Output, W::Error> {
    if !is_ignored_workflow(W::MODULE_NAME, W::WORKFLOW_NAME) {
        warn!("Running run_workflow_ioe for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
    }
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

        let result = match timeout_mode {
            WorkflowTimeoutMode::RealTime => timeout(timeout_duration, receiver.recv()).await.map_err(|_| ()),
            WorkflowTimeoutMode::VirtualTime => virtual_timeout(timeout_duration, receiver.recv()).await.map_err(|_| ()),
        };

        match result {
            Ok(Some(response)) => {
                let key = WorkflowID {
                    module: response.module_name,
                    workflow: response.workflow_name,
                };

                if key == workflow_id {
                    if !is_ignored_workflow(W::MODULE_NAME, W::WORKFLOW_NAME) {
                        warn!("Finished run_workflow_ioe for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
                    }
                    return <Result<W::Output, W::Error>>::from_response(response);
                }

                RESPONSE_INBOX.lock().await.insert(key, WorkflowResponse::OE(response));
            }
            Ok(None) => {
                panic!("Channel closed while waiting for response to {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            }
            Err(_) => {
                publish_workflow_timeout_signal(W::MODULE_NAME, W::WORKFLOW_NAME, 1);
                panic!("Timeout while waiting for response to {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            }
        }

        if let Some(WorkflowResponse::OE(response)) = RESPONSE_INBOX.lock().await.remove(&workflow_id) {
            if !is_ignored_workflow(W::MODULE_NAME, W::WORKFLOW_NAME) {
                warn!("Finished run_workflow_ioe for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            }
            return <Result<W::Output, W::Error>>::from_response(response);
        }

        tokio::task::yield_now().await;
    }
}

pub async fn run_workflow_ioe_with_timeout_control<W, F>(
    timeout_duration: Duration,
    timeout_mode: WorkflowTimeoutMode,
    input: W::Input,
    mut on_timeout: F,
) -> Result<W::Output, WorkflowControlledRunError<W::Error>>
where
    W: WorkflowTypeIOE,
    F: FnMut(WorkflowTimeoutContext) -> WorkflowTimeoutControlDecision,
{
    if !is_ignored_workflow(W::MODULE_NAME, W::WORKFLOW_NAME) {
        warn!("Running run_workflow_ioe_with_timeout_control for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
    }
    let composite_workflow_id = CURRENT_COMPOSITE_WORKFLOW_ID.with(|id| *id);
    let workflow_id = WorkflowID {
        module: W::MODULE_NAME,
        workflow: W::WORKFLOW_NAME,
    };
    let mut timeout_count = 0usize;

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

        let result = match timeout_mode {
            WorkflowTimeoutMode::RealTime => timeout(timeout_duration, receiver.recv()).await.map_err(|_| ()),
            WorkflowTimeoutMode::VirtualTime => virtual_timeout(timeout_duration, receiver.recv()).await.map_err(|_| ()),
        };

        match result {
            Ok(Some(response)) => {
                let key = WorkflowID {
                    module: response.module_name,
                    workflow: response.workflow_name,
                };

                if key == workflow_id {
                    if !is_ignored_workflow(W::MODULE_NAME, W::WORKFLOW_NAME) {
                        warn!("Finished run_workflow_ioe_with_timeout_control for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
                    }
                    return <Result<W::Output, W::Error>>::from_response(response).map_err(WorkflowControlledRunError::Workflow);
                }

                RESPONSE_INBOX.lock().await.insert(key, WorkflowResponse::OE(response));
            }
            Ok(None) => {
                panic!("Channel closed while waiting for response to {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            }
            Err(_) => {
                timeout_count += 1;
                publish_workflow_timeout_signal(W::MODULE_NAME, W::WORKFLOW_NAME, timeout_count);
                let decision = on_timeout(WorkflowTimeoutContext {
                    module_name: W::MODULE_NAME,
                    workflow_name: W::WORKFLOW_NAME,
                    timeout_count,
                });
                match decision {
                    WorkflowTimeoutControlDecision::Retry => {
                        if !is_ignored_workflow(W::MODULE_NAME, W::WORKFLOW_NAME) {
                            warn!(
                                "Timeout while waiting for {}::{}, retrying (timeout_count={})",
                                W::MODULE_NAME,
                                W::WORKFLOW_NAME,
                                timeout_count
                            );
                        }
                    }
                    WorkflowTimeoutControlDecision::Abort => {
                        return Err(WorkflowControlledRunError::Control(WorkflowRunnerControlError::TimeoutAborted {
                            module_name: W::MODULE_NAME,
                            workflow_name: W::WORKFLOW_NAME,
                            timeout_count,
                        }));
                    }
                    WorkflowTimeoutControlDecision::Panic => {
                        panic!("Timeout while waiting for response to {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
                    }
                }
            }
        }

        if let Some(WorkflowResponse::OE(response)) = RESPONSE_INBOX.lock().await.remove(&workflow_id) {
            if !is_ignored_workflow(W::MODULE_NAME, W::WORKFLOW_NAME) {
                warn!("Finished run_workflow_ioe_with_timeout_control for {}::{}", W::MODULE_NAME, W::WORKFLOW_NAME);
            }
            return <Result<W::Output, W::Error>>::from_response(response).map_err(WorkflowControlledRunError::Workflow);
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
    crate::workflow::statics::WORKFLOW_TOKIO_RUNTIME().handle().spawn(async move {
        match handle.await {
            Ok(ctx) => f(&ctx),
            Err(_join_error) => {
                error!("Composite workflow return handling failed because the composite workflow failed");
            }
        }
    });
}
