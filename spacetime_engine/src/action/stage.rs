use std::any::Any;
use bevy::prelude::*;
use futures::future::BoxFuture;

use super::stage_io::*;

pub enum ActionStage {
    Ecs(ActionStageEcs),
    Async(ActionStageAsync),
    EcsWhile(ActionStageEcsWhile),
}

pub struct ActionStageEcs {
    pub name: String,
    pub function: Box<dyn FnMut(ActionIO<InputState>, &mut World) -> ActionIO<OutputState> + Send + Sync>
}

pub struct ActionStageAsync {
    pub name: String,
    pub function: Box<dyn FnMut(ActionIO<InputState>) -> BoxFuture<'static, ActionIO<OutputState>> + Send + Sync>,
}

pub struct ActionStageEcsWhile {
    pub name: String,
    pub function: Box<dyn FnMut(ActionIO<InputState>, &mut World) -> Result<ActionIO<InputState>, ActionIO<OutputState>> + Send + Sync>,
}

pub struct ActionStageOutput {
    pub entity: Entity,
    pub module_type: String,
    pub action_name: String,
    pub output: Box<dyn Any + Send + Sync>,
}