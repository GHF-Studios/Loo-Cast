use bevy::prelude::*;
use std::any::Any;
use std::any::TypeId;

#[derive(Event)]
pub(in crate) struct ActionStageProcessedEvent {
    pub target_entity: Entity,
    pub target_type: String,
    pub action_name: String,
    pub stage_index: usize,
    pub stage_output: Box<dyn Any + Send + Sync>,
}

#[derive(Event)]
pub(in crate) struct ActionProcessedEvent {
    pub target_entity: Entity,
    pub target_type: String,
    pub action_name: String,
    pub action_output: Box<dyn Any + Send + Sync>,
}
