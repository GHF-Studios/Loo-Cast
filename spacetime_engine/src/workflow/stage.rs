use std::any::Any;
use bevy::prelude::*;
use futures::future::BoxFuture;

use super::io::*;

pub enum WorkflowStage {
    Ecs(WorkflowStageEcs),
    EcsWhile(WorkflowStageEcsWhile),
    Render(WorkflowStageRender),
    RenderWhile(WorkflowStageRenderWhile),
    Async(WorkflowStageAsync),
}

// -- RETURN TYPES --
pub struct WorkflowStageOutput {
    pub entity: Entity,
    pub module_name: String,
    pub workflow_name: String,
    pub output: Box<dyn Any + Send + Sync>,
}

pub enum WorkflowStageWhileOutcome {
    Waiting(WorkflowIO<InputState>),
    Completed(WorkflowIO<OutputState>),
}

// -- STAGE TYPES --
pub struct WorkflowStageEcs {
    pub name: String,
    pub function: Box<dyn FnMut(WorkflowIO<InputState>, &mut World) -> WorkflowIO<OutputState> + Send + Sync>,
}

pub struct WorkflowStageEcsWhile {
    pub name: String,
    pub function: Box<dyn FnMut(WorkflowIO<InputState>, &mut World) -> WorkflowStageWhileOutcome + Send + Sync>,
}

pub struct WorkflowStageRender {
    pub name: String,
    pub function: Box<dyn FnMut(WorkflowIO<InputState>, &mut World) -> WorkflowIO<OutputState> + Send + Sync>,
}

pub struct WorkflowStageRenderWhile {
    pub name: String,
    pub function: Box<dyn FnMut(WorkflowIO<InputState>, &mut World) -> WorkflowStageWhileOutcome + Send + Sync>,
}

pub struct WorkflowStageAsync {
    pub name: String,
    pub function: Box<dyn FnMut(WorkflowIO<InputState>) -> BoxFuture<'static, WorkflowIO<OutputState>> + Send + Sync>,
}