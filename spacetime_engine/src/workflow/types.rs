use futures::future::BoxFuture;
use tokio::task::JoinHandle;

use super::{
    composite_workflow_context::ScopedCompositeWorkflowContext, resources::WorkflowMap, stage::{Stage, StageType}, statics::TOKIO_RUNTIME
};

pub struct CompositeWorkflowRuntime(tokio::runtime::Handle);
impl CompositeWorkflowRuntime {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn spawn(&mut self, future: BoxFuture<'static, ScopedCompositeWorkflowContext>) -> JoinHandle<ScopedCompositeWorkflowContext> {
        self.0.spawn(future)
    }

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
                    unreachable!("Composite workflow failed: {}", e)
                }
            };
            ctx
        })
    }
}

impl Default for CompositeWorkflowRuntime {
    fn default() -> Self {
        Self(TOKIO_RUNTIME.lock().unwrap().handle().clone())
    }
}

#[derive(Debug, Clone, Copy)]
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
                stage_completed: completed
            } => write!(f, "WorkflowState::Processing(current_stage: {}, current_stage_type: {}, initialized: {}, completed: {})", current_stage, current_stage_type, initialized, completed),
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

pub struct WorkflowTypeModule {
    pub name: &'static str,
    pub workflow_types: Vec<WorkflowType>,
}

pub struct WorkflowType {
    pub name: &'static str,
    pub stages: Vec<Stage>,
}

pub enum Outcome<S, O> {
    Wait(S),
    Done(O),
}

pub(super) struct RetryRequest {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub retry_count: usize,
    pub action: Box<dyn FnOnce(&mut WorkflowMap) + Send>,
}
