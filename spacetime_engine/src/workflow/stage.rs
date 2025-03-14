use bevy::prelude::*;
use crossbeam_channel::Sender;
use futures::future::BoxFuture;
use std::any::Any;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorkflowStageSignature {
    None,
    E,
    O,
    OE,
    I,
    IE,
    IO,
    IOE,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorkflowStageType {
    Ecs,
    Render,
    Async,
    EcsWhile,
    RenderWhile,
}
impl std::fmt::Display for WorkflowStageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            WorkflowStageType::Ecs => write!(f, "WorkflowStageType(Ecs)"),
            WorkflowStageType::Render => write!(f, "WorkflowStageType(Render)"),
            WorkflowStageType::Async => write!(f, "WorkflowStageType(Async)"),
            WorkflowStageType::EcsWhile => write!(f, "WorkflowStageType(EcsWhile)"),
            WorkflowStageType::RenderWhile => write!(f, "WorkflowStageType(RenderWhile)"),
        }
    }
}

pub enum WorkflowStage {
    Ecs(WorkflowStageEcs),
    Render(WorkflowStageRender),
    Async(WorkflowStageAsync),
    EcsWhile(WorkflowStageEcsWhile),
    RenderWhile(WorkflowStageRenderWhile),
}

impl WorkflowStage {
    pub fn get_type(&self) -> WorkflowStageType {
        match self {
            WorkflowStage::Ecs(_) => WorkflowStageType::Ecs,
            WorkflowStage::Render(_) => WorkflowStageType::Render,
            WorkflowStage::Async(_) => WorkflowStageType::Async,
            WorkflowStage::EcsWhile(_) => WorkflowStageType::EcsWhile,
            WorkflowStage::RenderWhile(_) => WorkflowStageType::RenderWhile,
        }
    }

    pub fn get_stage_response_handler(
        &mut self,
    ) -> &mut Box<
        dyn FnMut(
            &'static str, 
            &'static str, 
            Option<Box<dyn Any + Send + Sync>>, 
            Sender<(&str, &str, usize, WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
            Sender<(&str, &str, usize, WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>)
            + Send
            + Sync,
    > {
        match self {
            WorkflowStage::Ecs(stage) => &mut stage.handle_ecs_response,
            WorkflowStage::Render(stage) => &mut stage.handle_render_response,
            WorkflowStage::Async(stage) => &mut stage.handle_async_response,
            WorkflowStage::EcsWhile(stage) => &mut stage.handle_ecs_while_response,
            WorkflowStage::RenderWhile(stage) => &mut stage.handle_render_while_response,
        }
    }

    pub fn get_signature(&self) -> WorkflowStageSignature {
        match self {
            WorkflowStage::Ecs(stage) => stage.signature,
            WorkflowStage::Render(stage) => stage.signature,
            WorkflowStage::Async(stage) => stage.signature,
            WorkflowStage::EcsWhile(stage) => stage.signature,
            WorkflowStage::RenderWhile(stage) => stage.signature,
        }
    }
}

pub enum WorkflowStageWhileOutcome {
    Waiting(Option<Box<dyn Any + Send + Sync>>),
    Completed(Option<Box<dyn Any + Send + Sync>>),
}

pub struct WorkflowStageEcs {
    pub name: &'static str,
    pub signature: WorkflowStageSignature,
    pub run_ecs: Box<
        dyn FnMut(
                Option<Box<dyn Any + Send + Sync>>,
                &mut World,
            ) -> Option<Box<dyn Any + Send + Sync>>
            + Send
            + Sync,
    >,
    pub handle_ecs_response: Box<
        dyn FnMut(
            &'static str, 
            &'static str, 
            Option<Box<dyn Any + Send + Sync>>, 
            Sender<(&str, &str, usize, WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
            Sender<(&str, &str, usize, WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>)
            + Send
            + Sync,
    >,
}

pub struct WorkflowStageRender {
    pub name: &'static str,
    pub signature: WorkflowStageSignature,
    pub run_render: Box<
        dyn FnMut(
                Option<Box<dyn Any + Send + Sync>>,
                &mut World,
            ) -> Option<Box<dyn Any + Send + Sync>>
            + Send
            + Sync,
    >,
    pub handle_render_response: Box<
        dyn FnMut(
            &'static str, 
            &'static str, 
            Option<Box<dyn Any + Send + Sync>>, 
            Sender<(&str, &str, usize, WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
            Sender<(&str, &str, usize, WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>)
            + Send
            + Sync,
    >,
}

pub struct WorkflowStageAsync {
    pub name: &'static str,
    pub signature: WorkflowStageSignature,
    pub run_async: Box<
        dyn FnMut(
                Option<Box<dyn Any + Send + Sync>>,
            ) -> BoxFuture<'static, Option<Box<dyn Any + Send + Sync>>>
            + Send
            + Sync,
    >,
    pub handle_async_response: Box<
        dyn FnMut(
            &'static str, 
            &'static str, 
            Option<Box<dyn Any + Send + Sync>>, 
            Sender<(&str, &str, usize, WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
            Sender<(&str, &str, usize, WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>)
            + Send
            + Sync,
    >,
}

pub struct WorkflowStageEcsWhile {
    pub name: &'static str,
    pub signature: WorkflowStageSignature,
    pub setup_ecs_while: Box<
        dyn FnMut(
                Option<Box<dyn Any + Send + Sync>>,
                &mut World,
            ) -> Option<Box<dyn Any + Send + Sync>>
            + Send
            + Sync,
    >,
    pub run_ecs_while: Box<
        dyn FnMut(Option<Box<dyn Any + Send + Sync>>, &mut World) -> Box<dyn Any + Send + Sync>
            + Send
            + Sync,
    >,
    pub handle_ecs_while_response: Box<
        dyn FnMut(
            &'static str, 
            &'static str, 
            Option<Box<dyn Any + Send + Sync>>, 
            Sender<(&str, &str, usize, WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
            Sender<(&str, &str, usize, WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>)
            + Send
            + Sync,
    >,
}

pub struct WorkflowStageRenderWhile {
    pub name: &'static str,
    pub signature: WorkflowStageSignature,
    pub setup_render_while: Box<
        dyn FnMut(
                Option<Box<dyn Any + Send + Sync>>,
                &mut World,
            ) -> Option<Box<dyn Any + Send + Sync>>
            + Send
            + Sync,
    >,
    pub run_render_while: Box<
        dyn FnMut(Option<Box<dyn Any + Send + Sync>>, &mut World) -> Box<dyn Any + Send + Sync>
            + Send
            + Sync,
    >,
    pub handle_render_while_response: Box<
        dyn FnMut(
            &'static str, 
            &'static str, 
            Option<Box<dyn Any + Send + Sync>>, 
            Sender<(&str, &str, usize, WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
            Sender<(&str, &str, usize, WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>)
            + Send
            + Sync,
    >,
}
