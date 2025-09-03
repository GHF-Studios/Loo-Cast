use bevy::prelude::*;
use std::any::Any;

use crate::{utils::premium_box::AnySendSyncPremiumBox, workflow::types::WorkflowID};

#[derive(Debug, Reflect)]
pub enum WorkflowResponse {
    None(TypedWorkflowResponse),
    E(TypedWorkflowResponseE),
    O(TypedWorkflowResponseO),
    OE(TypedWorkflowResponseOE),
}

#[derive(Debug, Reflect)]
pub struct TypedWorkflowResponse {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
}
#[derive(Debug, Reflect)]
pub struct TypedWorkflowResponseE {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub result: Result<(), AnySendSyncPremiumBox>,
}
#[derive(Debug, Reflect)]
pub struct TypedWorkflowResponseO {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub output: AnySendSyncPremiumBox,
}
#[derive(Debug, Reflect)]
pub struct TypedWorkflowResponseOE {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub result: Result<AnySendSyncPremiumBox, AnySendSyncPremiumBox>,
}

impl WorkflowResponse {
    pub fn get_worfklow_id(&self) -> WorkflowID {
        let (module, workflow) = match self {
            Self::None(r) => (r.module_name, r.workflow_name),
            Self::E(r) => (r.module_name, r.workflow_name),
            Self::O(r) => (r.module_name, r.workflow_name),
            Self::OE(r) => (r.module_name, r.workflow_name),
        };

        WorkflowID {
            module,
            workflow
        }
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
