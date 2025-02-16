use bevy::prelude::*;
use std::any::Any;

use super::{stage::WorkflowStage, types::RawWorkflowData};

#[derive(Event)]
pub(in crate) struct WorkflowStageCompletionEvent {
    pub module_name: String,
    pub workflow_name: String,
    pub current_stage: usize,
    pub stage_output: RawWorkflowData,
    pub stage_return: Option<WorkflowStage>,
}