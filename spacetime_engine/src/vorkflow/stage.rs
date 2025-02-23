use std::any::Any;
use bevy::prelude::*;
use futures::future::BoxFuture;

use super::io::*;

pub enum VorkflowStage {
    Ecs(VorkflowStageEcs),
    EcsWhile(VorkflowStageEcsWhile),
    Render(VorkflowStageRender),
    RenderWhile(VorkflowStageRenderWhile),
    Async(VorkflowStageAsync),
}

// -- RETURN TYPES --
pub struct VorkflowStageOutput {
    pub entity: Entity,
    pub module_name: String,
    pub vorkflow_name: String,
    pub output: Box<dyn Any + Send + Sync>,
}

pub enum VorkflowStageWhileOutcome {
    Waiting(VorkflowIO<InputState>),
    Completed(VorkflowIO<OutputState>),
}

// -- STAGE TYPES --
pub struct VorkflowStageEcs {
    pub name: String,
    pub function: Box<dyn FnMut(VorkflowIO<InputState>, &mut World) -> VorkflowIO<OutputState> + Send + Sync>,
}

pub struct VorkflowStageEcsWhile {
    pub name: String,
    pub function: Box<dyn FnMut(VorkflowIO<InputState>, &mut World) -> VorkflowStageWhileOutcome + Send + Sync>,
}

pub struct VorkflowStageRender {
    pub name: String,
    pub function: Box<dyn FnMut(VorkflowIO<InputState>, &mut World) -> VorkflowIO<OutputState> + Send + Sync>,
}

pub struct VorkflowStageRenderWhile {
    pub name: String,
    pub function: Box<dyn FnMut(VorkflowIO<InputState>, &mut World) -> VorkflowStageWhileOutcome + Send + Sync>,
}

pub struct VorkflowStageAsync {
    pub name: String,
    pub function: Box<dyn FnMut(VorkflowIO<InputState>) -> BoxFuture<'static, VorkflowIO<OutputState>> + Send + Sync>,
}