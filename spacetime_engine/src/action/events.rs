use bevy::prelude::*;
use std::any::Any;

use super::{stage::ActionStage, types::RawActionData};

#[derive(Event)]
pub(in crate) struct ActionStageCompletionEvent {
    pub module_name: String,
    pub action_name: String,
    pub current_stage: usize,
    pub stage_output: RawActionData,
    pub stage_return: Option<ActionStage>,
}