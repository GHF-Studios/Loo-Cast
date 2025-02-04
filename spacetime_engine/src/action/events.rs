use bevy::prelude::*;
use std::any::Any;
use std::any::TypeId;

#[derive(Event)]
pub(in crate) struct ActionStageProcessed {
    pub target_entity: Entity,
    pub target_type: TypeId,
    pub action_name: String,
    pub stage_name: String,
    pub stage_output: Option<Box<dyn Any + Send + Sync>>,
}

#[derive(Event)]
pub(in crate) struct ActionProcessed {
    pub target_type: TypeId,
    pub target_entity: Entity,
    pub action_name: String,
    pub action_output: Option<Box<dyn Any + Send + Sync>>,
}
