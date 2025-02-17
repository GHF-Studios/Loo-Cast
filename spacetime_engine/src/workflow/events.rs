use bevy::prelude::*;

use super::{stage::WorkflowStage, types::RawWorkflowData};

#[derive(Event)]
pub(in crate) struct WorkflowStageInitializationEvent {
    pub module_name: String,
    pub workflow_name: String,
    pub stage_input: RawWorkflowData,
}

#[derive(Event)]
pub(in crate) struct WorkflowStageCompletionEvent {
    pub module_name: String,
    pub workflow_name: String,
    pub current_stage: usize,
    pub stage_output: RawWorkflowData,
    pub stage_return: WorkflowStage,
}