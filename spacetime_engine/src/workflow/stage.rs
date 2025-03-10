use std::any::Any;
use bevy::prelude::*;
use futures::future::BoxFuture;

pub enum WorkflowStage {
    Ecs(WorkflowStageEcs),
    EcsWhile(WorkflowStageEcsWhile),
    Render(WorkflowStageRender),
    RenderWhile(WorkflowStageRenderWhile),
    Async(WorkflowStageAsync),
}

pub enum WorkflowStageWhileOutcome {
    Waiting(Option<Box<dyn Any + Send + Sync>>),
    Completed(Option<Box<dyn Any + Send + Sync>>),
}

pub struct WorkflowStageEcs {
    pub name: String,
    pub function: Box<dyn FnMut(Option<Box<dyn Any + Send + Sync>>, &mut World) -> Option<Box<dyn Any + Send + Sync>> + Send + Sync>,
}

pub struct WorkflowStageEcsWhile {
    pub name: String,
    pub function: Box<dyn FnMut(Option<Box<dyn Any + Send + Sync>>, &mut World) -> WorkflowStageWhileOutcome + Send + Sync>,
}

pub struct WorkflowStageRender {
    pub name: String,
    pub function: Box<dyn FnMut(Option<Box<dyn Any + Send + Sync>>, &mut World) -> Option<Box<dyn Any + Send + Sync>> + Send + Sync>,
}

pub struct WorkflowStageRenderWhile {
    pub name: String,
    pub function: Box<dyn FnMut(Option<Box<dyn Any + Send + Sync>>, &mut World) -> WorkflowStageWhileOutcome + Send + Sync>,
}

pub struct WorkflowStageAsync {
    pub name: String,
    pub function: Box<dyn FnMut(Option<Box<dyn Any + Send + Sync>>) -> BoxFuture<'static, Option<Box<dyn Any + Send + Sync>>> + Send + Sync>,
}