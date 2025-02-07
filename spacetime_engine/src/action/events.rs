use bevy::prelude::*;
use std::any::Any;

#[derive(Event)]
pub(in crate) struct ActionStageProcessedEvent {
    pub module_name: String,
    pub action_name: String,
    pub stage_index: usize,
    pub stage_output: Box<dyn Any + Send + Sync>,
}