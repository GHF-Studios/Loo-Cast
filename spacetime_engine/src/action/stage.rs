use std::any::Any;
use bevy::prelude::*;
use futures::future::BoxFuture;

use super::stage_io::*;

pub enum ActionStage {
    Ecs(ActionStageEcs),
    EcsWhile(ActionStageEcsWhile),
    Render(ActionStageRender),
    RenderWhile(ActionStageRenderWhile),
    Async(ActionStageAsync),
}

// -- RETURN TYPES --
pub struct ActionStageOutput {
    pub entity: Entity,
    pub module_name: String,
    pub action_name: String,
    pub output: Box<dyn Any + Send + Sync>,
}

pub enum ActionStageWhileOutcome {
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
    pub function: Box<dyn FnMut(ActionIO<InputState>, &mut World) -> ActionStageWhileOutcome + Send + Sync>,
}

pub struct ActionStageRender {
    pub name: String,
    pub function: Box<dyn FnMut(ActionIO<InputState>, &mut World) -> ActionIO<OutputState> + Send + Sync>,
}

pub struct ActionStageRenderWhile {
    pub name: String,
    pub function: Box<dyn FnMut(ActionIO<InputState>, &mut World) -> ActionStageWhileOutcome + Send + Sync>,
}

pub struct ActionStageAsync {
    pub name: String,
    pub function: Box<dyn FnMut(ActionIO<InputState>) -> BoxFuture<'static, ActionIO<OutputState>> + Send + Sync>,
}