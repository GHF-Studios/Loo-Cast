use std::{any::Any, pin::Pin};
use bevy::prelude::*;
use futures::future::BoxFuture;

use super::stage_io::*;

pub enum ActionStage {
    Ecs(ActionStageEcs),
    Async(ActionStageAsync),
}

pub struct ActionStageEcs {
    pub name: String,
    pub function: Box<dyn FnMut(ActionStageIO<InputState>, &mut World) -> ActionStageIO<OutputState> + Send + Sync>
}

pub struct ActionStageAsync {
    pub name: String,
    pub function: Pin<Box<dyn FnOnce(ActionStageIO<InputState>) -> BoxFuture<'static, ActionStageIO<OutputState>> + Send + Sync>>,
}

pub struct ActionStageOutput {
    pub entity: Entity,
    pub target_type: String,
    pub action_name: String,
    pub output: Box<dyn Any + Send + Sync>,
}