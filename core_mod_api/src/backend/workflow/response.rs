use crate::bevy::prelude::Reflect;
use std::any::Any;
use uuid::Uuid;

use crate::{utils::premium_box::AnySendSyncPremiumBox, workflow::types::WorkflowID};

#[derive(Debug, Reflect)]
pub enum WorkflowResponse {
    None(TypedWorkflowResponseEnvelope),
    E(TypedWorkflowResponseEEnvelope),
    O(TypedWorkflowResponseOEnvelope),
    OE(TypedWorkflowResponseOEEnvelope),
}

#[derive(Debug, Reflect)]
pub struct TypedWorkflowResponse {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
}
#[derive(Debug, Reflect)]
pub struct TypedWorkflowResponseEnvelope {
    pub request_id: Uuid,
    pub response: TypedWorkflowResponse,
}
#[derive(Debug, Reflect)]
pub struct TypedWorkflowResponseE {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub result: Result<(), AnySendSyncPremiumBox>,
}
#[derive(Debug, Reflect)]
pub struct TypedWorkflowResponseEEnvelope {
    pub request_id: Uuid,
    pub response: TypedWorkflowResponseE,
}
#[derive(Debug, Reflect)]
pub struct TypedWorkflowResponseO {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub output: AnySendSyncPremiumBox,
}
#[derive(Debug, Reflect)]
pub struct TypedWorkflowResponseOEnvelope {
    pub request_id: Uuid,
    pub response: TypedWorkflowResponseO,
}
#[derive(Debug, Reflect)]
pub struct TypedWorkflowResponseOE {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub result: Result<AnySendSyncPremiumBox, AnySendSyncPremiumBox>,
}
#[derive(Debug, Reflect)]
pub struct TypedWorkflowResponseOEEnvelope {
    pub request_id: Uuid,
    pub response: TypedWorkflowResponseOE,
}

impl WorkflowResponse {
    pub fn get_worfklow_id(&self) -> WorkflowID {
        let (module, workflow, request_id) = match self {
            Self::None(r) => (r.response.module_name, r.response.workflow_name, r.request_id),
            Self::E(r) => (r.response.module_name, r.response.workflow_name, r.request_id),
            Self::O(r) => (r.response.module_name, r.response.workflow_name, r.request_id),
            Self::OE(r) => (r.response.module_name, r.response.workflow_name, r.request_id),
        };

        WorkflowID { module, workflow, request_id }
    }
}

impl TypedWorkflowResponseE {
    #[track_caller]
    pub fn unpack<E: 'static + Any + Send + Sync>(self) -> Result<(), E> {
        self.result.map_err(|e| e.into_inner())
    }
}

impl TypedWorkflowResponseO {
    #[track_caller]
    pub fn unpack<O: 'static + Any + Send + Sync>(self) -> O {
        self.output.into_inner()
    }
}

impl TypedWorkflowResponseOE {
    #[track_caller]
    pub fn unpack<O: 'static + Any + Send + Sync, E: 'static + Any + Send + Sync>(self) -> Result<O, E> {
        self.result.map(|o| o.into_inner()).map_err(|e| e.into_inner())
    }
}
