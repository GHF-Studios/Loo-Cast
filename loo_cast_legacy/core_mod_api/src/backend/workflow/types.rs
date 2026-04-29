use crate::bevy::prelude::Reflect;
use futures::future::BoxFuture;
use tokio::task::JoinHandle;
use uuid::Uuid;

use crate::utils::progress::Progress;

use super::{
    composite_workflow_context::ScopedCompositeWorkflowContext,
    stage::{Stage, StageType},
    statics::WORKFLOW_TOKIO_RUNTIME,
};

#[derive(Reflect)]
pub struct CompositeWorkflowRuntime(#[reflect(ignore, default = "tokio_runtime_handle")] tokio::runtime::Handle);
impl CompositeWorkflowRuntime {
    pub fn new() -> Self {
        Self::default()
    }

    #[track_caller]
    pub fn spawn(&mut self, future: BoxFuture<'static, ScopedCompositeWorkflowContext>) -> JoinHandle<ScopedCompositeWorkflowContext> {
        self.0.spawn(future)
    }

    #[track_caller]
    pub fn spawn_fallible<E: 'static + Send + std::error::Error>(
        &mut self,
        future: BoxFuture<'static, (ScopedCompositeWorkflowContext, Result<(), E>)>,
    ) -> JoinHandle<ScopedCompositeWorkflowContext> {
        self.0.spawn(Self::wrap_fallible_with_error_handler(future))
    }

    fn wrap_fallible_with_error_handler<E: 'static + Send + std::error::Error>(
        future: BoxFuture<'static, (ScopedCompositeWorkflowContext, Result<(), E>)>,
    ) -> BoxFuture<'static, ScopedCompositeWorkflowContext> {
        Box::pin(async move {
            let (ctx, result) = future.await;
            match result {
                Ok(_) => {}
                Err(e) => {
                    panic!("Composite workflow failed: {}", e)
                }
            };
            ctx
        })
    }
}
impl Default for CompositeWorkflowRuntime {
    fn default() -> Self {
        Self(WORKFLOW_TOKIO_RUNTIME().handle().clone())
    }
}

fn tokio_runtime_handle() -> tokio::runtime::Handle {
    WORKFLOW_TOKIO_RUNTIME().handle().clone()
}

#[derive(Debug, Clone, Copy, Reflect)]
pub enum WorkflowState {
    Requested,
    Processing {
        current_stage: usize,
        current_stage_type: StageType,
        stage_initialized: bool,
        stage_completed: bool,
    },
}
impl std::fmt::Display for WorkflowState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Requested => write!(f, "WorkflowState::Requested"),
            Self::Processing {
                current_stage,
                current_stage_type,
                stage_initialized: initialized,
                stage_completed: completed,
            } => write!(
                f,
                "WorkflowState::Processing(current_stage: {}, current_stage_type: {}, initialized: {}, completed: {})",
                current_stage, current_stage_type, initialized, completed
            ),
        }
    }
}
impl WorkflowState {
    pub fn is_requested(&self) -> bool {
        matches!(self, Self::Requested)
    }

    pub fn current_stage(&self) -> usize {
        match self {
            Self::Requested => 0,
            Self::Processing { current_stage, .. } => *current_stage,
        }
    }
}

#[derive(Reflect)]
pub struct WorkflowTypeModule {
    pub name: &'static str,
    pub workflow_types: Vec<WorkflowType>,
}

#[derive(Reflect)]
pub struct WorkflowType {
    pub name: &'static str,
    #[reflect(ignore)]
    pub stages: Vec<Stage>,
}

#[derive(Reflect)]
pub enum Outcome<S, O> {
    Wait(S),
    Done(O),
}
impl<S, O> Outcome<S, O> {
    pub fn is_wait(&self) -> bool {
        matches!(self, Outcome::Wait(_))
    }

    pub fn is_done(&self) -> bool {
        matches!(self, Outcome::Done(_))
    }

    pub fn map_wait<T, F: FnOnce(S) -> T>(self, f: F) -> Outcome<T, O> {
        match self {
            Outcome::Wait(s) => Outcome::Wait(f(s)),
            Outcome::Done(o) => Outcome::Done(o),
        }
    }

    pub fn map_done<T, F: FnOnce(O) -> T>(self, f: F) -> Outcome<S, T> {
        match self {
            Outcome::Wait(s) => Outcome::Wait(s),
            Outcome::Done(o) => Outcome::Done(f(o)),
        }
    }

    pub fn unwrap_wait(self) -> S {
        match self {
            Outcome::Wait(s) => s,
            Outcome::Done(_) => panic!("Called unwrap_wait on Outcome::Done"),
        }
    }

    pub fn unwrap_done(self) -> O {
        match self {
            Outcome::Wait(_) => panic!("Called unwrap_done on Outcome::Wait"),
            Outcome::Done(o) => o,
        }
    }

    pub fn into_progress(self) -> Progress<S, O> {
        match self {
            Outcome::Wait(s) => Progress::Unfinished(s),
            Outcome::Done(o) => Progress::Finished(o),
        }
    }
}

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorkflowID {
    pub module: &'static str,
    pub workflow: &'static str,
    pub request_id: Uuid,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect)]
pub enum WorkflowTimeoutMode {
    RealTime,
    VirtualTime,
}
