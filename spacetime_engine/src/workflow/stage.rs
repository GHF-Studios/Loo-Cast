use std::any::Any;
use bevy::prelude::*;
use futures::future::BoxFuture;

pub enum WorkflowStage {
    Ecs(WorkflowStageEcs),
    Render(WorkflowStageRender),
    Async(WorkflowStageAsync),
    EcsWhile(WorkflowStageEcsWhile),
    RenderWhile(WorkflowStageRenderWhile),
}

pub enum WorkflowStageWhileOutcome {
    Waiting(Option<Box<dyn Any + Send + Sync>>),
    Completed(Option<Box<dyn Any + Send + Sync>>),
}

pub struct WorkflowStageEcs {
    pub name: &'static str,
    pub run_ecs: Box<dyn FnMut(Option<Box<dyn Any + Send + Sync>>, &mut World) -> Option<Box<dyn Any + Send + Sync>> + Send + Sync>,
}

pub struct WorkflowStageRender {
    pub name: &'static str,
    pub run_render: Box<dyn FnMut(Option<Box<dyn Any + Send + Sync>>, &mut World) -> Option<Box<dyn Any + Send + Sync>> + Send + Sync>,
}

pub struct WorkflowStageAsync {
    pub name: &'static str,
    pub run_async: Box<dyn FnMut(Option<Box<dyn Any + Send + Sync>>) -> BoxFuture<'static, Option<Box<dyn Any + Send + Sync>>> + Send + Sync>,
}

pub struct WorkflowStageEcsWhile {
    pub name: &'static str,
    pub setup_ecs_while: Box<dyn FnMut(Option<Box<dyn Any + Send + Sync>>, &mut World) -> WorkflowStageWhileOutcome + Send + Sync>,
    pub run_ecs_while: Box<dyn FnMut(Option<Box<dyn Any + Send + Sync>>, &mut World) -> WorkflowStageWhileOutcome + Send + Sync>,
}

pub struct WorkflowStageRenderWhile {
    pub name: &'static str,
    pub setup_render_while: Box<dyn FnMut(Option<Box<dyn Any + Send + Sync>>, &mut World) -> Option<Box<dyn Any + Send + Sync>> + Send + Sync>,
    pub run_render_while: Box<dyn FnMut(Option<Box<dyn Any + Send + Sync>>, &mut World) -> WorkflowStageWhileOutcome + Send + Sync>,
}
