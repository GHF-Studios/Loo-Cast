use bevy::prelude::*;
use std::any::Any;

use super::{stage::ActionStage, types::RawActionData};

#[derive(Event)]
pub(in crate) struct ActionStageProcessedEvent {
    pub module_name: String,
    pub action_name: String,
    pub stage_index: usize,
    pub stage_output: RawActionData,
    pub stage_return: Option<ActionStage>,
}