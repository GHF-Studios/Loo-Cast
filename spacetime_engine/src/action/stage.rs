use std::any::Any;
use bevy::prelude::*;
use futures::future::BoxFuture;

use super::stage_io::*;

pub enum ActionStage {
    Ecs(ActionStageEcs),
    EcsWhile(ActionStageEcsWhile),
    EcsRender(ActionStageEcsRender),
    EcsRenderWhile(ActionStageEcsRenderWhile),
    Async(ActionStageAsync),
}

// -- RETURN TYPES --
pub struct ActionStageOutput {
    pub entity: Entity,
    pub module_name: String,
    pub action_name: String,
    pub output: Box<dyn Any + Send + Sync>,
}

pub enum ActionStageEcsWhileOutcome {
    Waiting(ActionIO<InputState>),
    Completed(ActionIO<OutputState>),
}

// -- STAGE TYPES --
pub struct ActionStageEcs {
    pub name: String,
    pub function: Box<dyn FnMut(ActionIO<InputState>, &mut World) -> ActionIO<OutputState> + Send + Sync>,
}

pub struct ActionStageEcsWhile {
    pub name: String,
    pub function: Box<dyn FnMut(ActionIO<InputState>, &mut World) -> ActionStageEcsWhileOutcome + Send + Sync>,
}

pub struct ActionStageEcsRender {
    pub name: String,
    pub function: Box<dyn FnMut(ActionIO<InputState>, &mut World) -> ActionIO<OutputState> + Send + Sync>,
}

pub struct ActionStageEcsRenderWhile {
    pub name: String,
    pub function: Box<dyn FnMut(ActionIO<InputState>, &mut World) -> ActionStageEcsWhileOutcome + Send + Sync>,
}

pub struct ActionStageAsync {
    pub name: String,
    pub function: Box<dyn FnMut(ActionIO<InputState>) -> BoxFuture<'static, ActionIO<OutputState>> + Send + Sync>,
}