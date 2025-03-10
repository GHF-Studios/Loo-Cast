use bevy::prelude::*;
use std::any::Any;

use super::{stage::WorkflowStage, types::RawWorkflowData};

#[derive(Event)]
pub(in crate) struct WorkflowStageInitializationEvent {
    pub module_name: String,
    pub workflow_name: String,
    pub stage_input: Option<Box<dyn Any + Send + Sync>>,
}

#[derive(Event)]
pub(in crate) struct WorkflowStageCompletionEvent {
    pub module_name: String,
    pub workflow_name: String,
    pub current_stage: usize,
    pub stage_output: Option<Box<dyn Any + Send + Sync>>,
    pub stage_return: WorkflowStage,
}