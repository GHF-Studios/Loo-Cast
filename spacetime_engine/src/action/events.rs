use bevy::prelude::*;
use std::any::Any;
use std::any::TypeId;

#[derive(Event)]
pub(in super) struct StageProcessed {
    pub target_type: TypeId,
    pub target_entity: Entity,
    pub action_name: String,
    pub stage_name: String,
    pub stage_output: Option<Box<dyn Any + Send + Sync>>,
}

#[derive(Event)]
pub(in super) struct ActionProcessed {
    pub target_type: TypeId,
    pub target_entity: Entity,
    pub action_name: String,
    pub action_output: Option<Box<dyn Any + Send + Sync>>,
}
