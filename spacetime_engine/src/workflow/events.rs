use bevy::prelude::*;
use std::any::Any;

use super::stage::{Stage, StageType};

#[derive(Event)]
pub struct StageInitializationEvent {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub stage_input: Option<Box<dyn Any + Send + Sync>>,
}

#[derive(Event)]
pub struct StageWaitEvent {
    pub ty: StageType,
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub current_stage: usize,
    pub stage_return: Stage,
    pub stage_state: Option<Box<dyn Any + Send + Sync>>,
}

#[derive(Event)]
pub struct StageCompletionEvent {
    pub ty: StageType,
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub current_stage: usize,
    pub stage_return: Stage,
    pub stage_output: Option<Box<dyn Any + Send + Sync>>,
}

#[derive(Event)]
pub struct StageFailureEvent {
    pub ty: StageType,
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub current_stage: usize,
    pub stage_return: Stage,
    pub stage_error: Option<Box<dyn Any + Send + Sync>>,
}
