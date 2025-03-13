use bevy::prelude::*;
use std::any::Any;

use super::stage::{WorkflowStage, WorkflowStageType};

#[derive(Event)]
pub(crate) struct WorkflowStageInitializationEvent {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub stage_input: Option<Box<dyn Any + Send + Sync>>,
}

#[derive(Event)]
pub(crate) struct WorkflowStageCompletionEvent {
    pub ty: WorkflowStageType,
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub current_stage: usize,
    pub stage_output: Option<Box<dyn Any + Send + Sync>>,
    pub stage_return: WorkflowStage,
}
