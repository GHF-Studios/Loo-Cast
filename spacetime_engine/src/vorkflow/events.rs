use bevy::prelude::*;

use super::{stage::VorkflowStage, types::RawVorkflowData};

#[derive(Event)]
pub(in crate) struct VorkflowStageInitializationEvent {
    pub module_name: String,
    pub vorkflow_name: String,
    pub stage_input: RawVorkflowData,
}

#[derive(Event)]
pub(in crate) struct VorkflowStageCompletionEvent {
    pub module_name: String,
    pub vorkflow_name: String,
    pub current_stage: usize,
    pub stage_output: RawVorkflowData,
    pub stage_return: VorkflowStage,
}