use bevy::prelude::*;

use super::stage::{Stage, StageType};
use crate::debug::types::AnySendSyncPremiumBox;

#[derive(Event)]
pub struct StageInitializationEvent {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub stage_input: Option<AnySendSyncPremiumBox>,
}

#[derive(Event)]
pub struct StageSetupEvent {
    pub ty: StageType,
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub current_stage: usize,
    pub stage_return: Stage,
    pub stage_state: Option<AnySendSyncPremiumBox>,
}

#[derive(Event)]
pub struct StageWaitEvent {
    pub ty: StageType,
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub current_stage: usize,
    pub stage_return: Stage,
    pub stage_state: Option<AnySendSyncPremiumBox>,
}

#[derive(Event)]
pub struct StageCompletionEvent {
    pub ty: StageType,
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub current_stage: usize,
    pub stage_return: Stage,
    pub stage_output: Option<AnySendSyncPremiumBox>,
}

#[derive(Event)]
pub struct StageFailureEvent {
    pub ty: StageType,
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub current_stage: usize,
    pub stage_return: Stage,
    pub stage_error: Option<AnySendSyncPremiumBox>,
}
