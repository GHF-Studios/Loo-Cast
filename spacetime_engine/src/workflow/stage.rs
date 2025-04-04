use bevy::prelude::*;
use crossbeam_channel::Sender;
use futures::future::BoxFuture;
use std::any::Any;

use super::events::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StageSignature {
    None,
    E,
    O,
    OE,
    I,
    IE,
    IO,
    IOE,
}
impl StageSignature {
    pub fn has_input(&self) -> bool {
        match self {
            StageSignature::I | StageSignature::IE | StageSignature::IO | StageSignature::IOE => {
                true
            }
            _ => false,
        }
    }
    pub fn has_output(&self) -> bool {
        match self {
            StageSignature::O | StageSignature::OE | StageSignature::IO | StageSignature::IOE => {
                true
            }
            _ => false,
        }
    }
    pub fn has_error(&self) -> bool {
        match self {
            StageSignature::E | StageSignature::OE | StageSignature::IE | StageSignature::IOE => {
                true
            }
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StageType {
    Ecs,
    Render,
    Async,
    EcsWhile,
    RenderWhile,
}
impl std::fmt::Display for StageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            StageType::Ecs => write!(f, "WorkflowStageType(Ecs)"),
            StageType::Render => write!(f, "WorkflowStageType(Render)"),
            StageType::Async => write!(f, "WorkflowStageType(Async)"),
            StageType::EcsWhile => write!(f, "WorkflowStageType(EcsWhile)"),
            StageType::RenderWhile => write!(f, "WorkflowStageType(RenderWhile)"),
        }
    }
}

pub enum Stage {
    Ecs(StageEcs),
    Render(StageRender),
    Async(StageAsync),
    EcsWhile(StageEcsWhile),
    RenderWhile(StageRenderWhile),
}

impl Stage {
    pub fn get_type(&self) -> StageType {
        match self {
            Stage::Ecs(_) => StageType::Ecs,
            Stage::Render(_) => StageType::Render,
            Stage::Async(_) => StageType::Async,
            Stage::EcsWhile(_) => StageType::EcsWhile,
            Stage::RenderWhile(_) => StageType::RenderWhile,
        }
    }

    pub fn get_index(&self) -> usize {
        match self {
            Stage::Ecs(stage) => stage.signature as usize,
            Stage::Render(stage) => stage.signature as usize,
            Stage::Async(stage) => stage.signature as usize,
            Stage::EcsWhile(stage) => stage.signature as usize,
            Stage::RenderWhile(stage) => stage.signature as usize,
        }
    }

    pub fn get_signature(&self) -> StageSignature {
        match self {
            Stage::Ecs(stage) => stage.signature,
            Stage::Render(stage) => stage.signature,
            Stage::Async(stage) => stage.signature,
            Stage::EcsWhile(stage) => stage.signature,
            Stage::RenderWhile(stage) => stage.signature,
        }
    }

    pub fn get_wait_sender(&self) -> Option<Sender<StageWaitEvent>> {
        match self {
            Stage::Ecs(_) => None,
            Stage::Render(_) => None,
            Stage::Async(_) => None,
            Stage::EcsWhile(stage) => Some(stage.wait_sender.clone()),
            Stage::RenderWhile(stage) => Some(stage.wait_sender.clone()),
        }
    }

    pub fn get_completion_sender(&self) -> Sender<StageCompletionEvent> {
        match self {
            Stage::Ecs(stage) => stage.completion_sender.clone(),
            Stage::Render(stage) => stage.completion_sender.clone(),
            Stage::Async(stage) => stage.completion_sender.clone(),
            Stage::EcsWhile(stage) => stage.completion_sender.clone(),
            Stage::RenderWhile(stage) => stage.completion_sender.clone(),
        }
    }

    pub fn get_failure_sender(&self) -> Option<Sender<StageFailureEvent>> {
        match self {
            Stage::Ecs(stage) => stage.failure_sender.clone(),
            Stage::Render(stage) => stage.failure_sender.clone(),
            Stage::Async(stage) => stage.failure_sender.clone(),
            Stage::EcsWhile(stage) => stage.failure_sender.clone(),
            Stage::RenderWhile(stage) => stage.failure_sender.clone(),
        }
    }
}

pub struct StageEcs {
    pub index: usize,
    pub name: &'static str,
    pub signature: StageSignature,
    pub run_ecs: Box<
        dyn FnMut(
                Option<Box<dyn Any + Send + Sync>>,
                &mut World,
            ) -> Option<Box<dyn Any + Send + Sync>>
            + Send
            + Sync,
    >,
    pub handle_ecs_run_response: Box<
        dyn FnMut(
                &'static str,
                &'static str,
                Option<Box<dyn Any + Send + Sync>>,
                Sender<StageCompletionEvent>,
                Option<Sender<StageFailureEvent>>,
            ) -> Box<dyn FnOnce(StageEcs)>
            + Send
            + Sync,
    >,
    pub completion_sender: Sender<StageCompletionEvent>,
    pub failure_sender: Option<Sender<StageFailureEvent>>,
}

pub struct StageRender {
    pub index: usize,
    pub name: &'static str,
    pub signature: StageSignature,
    pub run_render: Box<
        dyn FnMut(
                Option<Box<dyn Any + Send + Sync>>,
                &mut World,
            ) -> Option<Box<dyn Any + Send + Sync>>
            + Send
            + Sync,
    >,
    pub handle_render_run_response: Box<
        dyn FnMut(
                &'static str,
                &'static str,
                Option<Box<dyn Any + Send + Sync>>,
                Sender<StageCompletionEvent>,
                Option<Sender<StageFailureEvent>>,
            ) -> Box<dyn FnOnce(StageRender)>
            + Send
            + Sync,
    >,
    pub completion_sender: Sender<StageCompletionEvent>,
    pub failure_sender: Option<Sender<StageFailureEvent>>,
}

pub struct StageAsync {
    pub index: usize,
    pub name: &'static str,
    pub signature: StageSignature,
    pub run_async: Box<
        dyn FnMut(
                Option<Box<dyn Any + Send + Sync>>,
            ) -> BoxFuture<'static, Option<Box<dyn Any + Send + Sync>>>
            + Send
            + Sync,
    >,
    pub handle_async_run_response: Box<
        dyn FnMut(
                &'static str,
                &'static str,
                Option<Box<dyn Any + Send + Sync>>,
                Sender<StageCompletionEvent>,
                Option<Sender<StageFailureEvent>>,
            ) -> Box<dyn FnOnce(StageAsync)>
            + Send
            + Sync,
    >,
    pub completion_sender: Sender<StageCompletionEvent>,
    pub failure_sender: Option<Sender<StageFailureEvent>>,
}

pub struct StageEcsWhile {
    pub index: usize,
    pub name: &'static str,
    pub signature: StageSignature,
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
    pub handle_ecs_while_setup_response: Box<
        dyn FnMut(
                &'static str,
                &'static str,
                Option<Box<dyn Any + Send + Sync>>,
                Sender<StageSetupEvent>,
                Option<Sender<StageFailureEvent>>,
            ) + Send
            + Sync,
    >,
    pub handle_ecs_while_run_response: Box<
        dyn FnMut(
                &'static str,
                &'static str,
                Option<Box<dyn Any + Send + Sync>>,
                Sender<StageWaitEvent>,
                Sender<StageCompletionEvent>,
                Option<Sender<StageFailureEvent>>,
            ) -> Box<dyn FnOnce(StageEcsWhile)>
            + Send
            + Sync,
    >,
    pub setup_sender: Sender<StageSetupEvent>,
    pub wait_sender: Sender<StageWaitEvent>,
    pub completion_sender: Sender<StageCompletionEvent>,
    pub failure_sender: Option<Sender<StageFailureEvent>>,
}

pub struct StageRenderWhile {
    pub index: usize,
    pub name: &'static str,
    pub signature: StageSignature,
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
    pub handle_render_while_setup_response: Box<
        dyn FnMut(
                &'static str,
                &'static str,
                Option<Box<dyn Any + Send + Sync>>,
                Sender<StageSetupEvent>,
                Option<Sender<StageFailureEvent>>,
            ) + Send
            + Sync,
    >,
    pub handle_render_while_run_response: Box<
        dyn FnMut(
                &'static str,
                &'static str,
                Option<Box<dyn Any + Send + Sync>>,
                Sender<StageWaitEvent>,
                Sender<StageCompletionEvent>,
                Option<Sender<StageFailureEvent>>,
            ) -> Box<dyn FnOnce(StageRenderWhile)>
            + Send
            + Sync,
    >,
    pub setup_sender: Sender<StageSetupEvent>,
    pub wait_sender: Sender<StageWaitEvent>,
    pub completion_sender: Sender<StageCompletionEvent>,
    pub failure_sender: Option<Sender<StageFailureEvent>>,
}
