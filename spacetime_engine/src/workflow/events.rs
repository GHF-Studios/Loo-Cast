use bevy::prelude::*;
use std::any::Any;

use super::stage::{Stage, StageType};

#[derive(Event)]
pub(crate) struct StageInitializationEvent {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub stage_input: Option<Box<dyn Any + Send + Sync>>,
}

#[derive(Event)]
pub(crate) struct StageCompletionEvent {
    pub ty: StageType,
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub current_stage: usize,
    pub stage_output: Option<Box<dyn Any + Send + Sync>>,
    pub stage_return: Stage,
}

#[derive(Event)]
pub(crate) struct StageFailureEvent {
    pub ty: StageType,
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub current_stage: usize,
    pub stage_error: Option<Box<dyn Any + Send + Sync>>,
    pub stage_return: Stage,
}
