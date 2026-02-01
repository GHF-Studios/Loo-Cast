use bevy::prelude::*;

use super::stage::{Stage, StageType};
use crate::{
    utils::premium_box::AnySendSyncPremiumBox,
    workflow::stage::{StageEcs, StageSignature},
};

#[derive(Message, Reflect)]
pub struct StageInitializationMessage {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub stage_input: Option<AnySendSyncPremiumBox>,
}

#[allow(clippy::missing_transmute_annotations)]
fn placeholder_stage() -> Stage {
    Stage::Ecs(StageEcs {
        index: 0,
        name: "PLACEHOLDER",
        stage_signature: StageSignature::None,
        handle_ecs_run_response: Box::new(|_, _, _, _, _| Box::new(|_| {})),
        completion_sender: unsafe { std::mem::transmute(0u128) },
        failure_sender: None,
    })
}

#[derive(Message, Reflect)]
pub struct StageSetupMessage {
    pub ty: StageType,
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub current_stage: usize,
    #[reflect(ignore, default = "placeholder_stage")]
    pub stage_return: Stage,
    pub stage_state: Option<AnySendSyncPremiumBox>,
}

#[derive(Message, Reflect)]
pub struct StageWaitMessage {
    pub ty: StageType,
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub current_stage: usize,
    #[reflect(ignore, default = "placeholder_stage")]
    pub stage_return: Stage,
    pub stage_state: Option<AnySendSyncPremiumBox>,
}

#[derive(Message, Reflect)]
pub struct StageCompletionMessage {
    pub ty: StageType,
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub current_stage: usize,
    #[reflect(ignore, default = "placeholder_stage")]
    pub stage_return: Stage,
    pub stage_output: Option<AnySendSyncPremiumBox>,
}

#[derive(Message, Reflect)]
pub struct StageFailureMessage {
    pub ty: StageType,
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub current_stage: usize,
    #[reflect(ignore, default = "placeholder_stage")]
    pub stage_return: Stage,
    pub stage_error: Option<AnySendSyncPremiumBox>,
}
