use std::{any::Any, ops::Deref};

use futures::future::BoxFuture;
use tokio::task::JoinHandle;

use super::{
    stage::{Stage, StageType},
    statics::TOKIO_RUNTIME,
};

pub struct CompositeWorkflowRuntime(tokio::runtime::Handle);
impl CompositeWorkflowRuntime {
    pub fn new() -> Self {
        Self(TOKIO_RUNTIME.lock().unwrap().handle().clone())
    }

    pub fn spawn(&mut self, future: BoxFuture<'static, ()>) -> JoinHandle<()> {
        self.0.spawn(future)
    }

    pub fn spawn_fallible<E: 'static + Send + std::error::Error>(
        &mut self,
        future: BoxFuture<'static, Result<(), E>>,
    ) -> JoinHandle<()> {
        self.0.spawn(Self::wrap_fallible_with_error_handler(future))
    }

    fn wrap_fallible_with_error_handler<E: 'static + Send + std::error::Error>(
        future: BoxFuture<'static, Result<(), E>>,
    ) -> BoxFuture<'static, ()> {
        Box::pin(async move {
            match future.await {
                Ok(_) => bevy::prelude::debug!(
                    "Composite workflow `test_workflow_framework` completed successfully"
                ),
                Err(e) => {
                    unreachable!("Composite workflow `test_workflow_framework` failed: {}", e)
                }
            };
        })
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
    Waiting(S),
    Completed(O),
}
