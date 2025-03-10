use futures::future::BoxFuture;
use tokio::task::JoinHandle;

use super::{stage::WorkflowStage, statics::TOKIO_RUNTIME};

pub struct WorkflowTaskRuntime(tokio::runtime::Handle);
impl WorkflowTaskRuntime {
    pub fn new() -> Self {
        Self(TOKIO_RUNTIME.lock().unwrap().handle().clone())
    }

    pub fn spawn_composite_workflow<T: 'static + Send>(
        &mut self, 
        future: BoxFuture<'static, T>
    ) -> JoinHandle<T> {
        self.0.spawn(future)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum WorkflowState {
    Requested,
    Processing {
        current_stage: usize,
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
                stage_initialized: initialized, 
                stage_completed: completed 
            } => write!(f, "WorkflowState::Processing(current_stage: {}, initialized: {}, completed: {})", current_stage, initialized, completed),
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
    pub workflow_types: Vec<WorkflowType>
}

pub struct WorkflowType {
    pub name: &'static str,
    pub stages: Vec<WorkflowStage>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkflowStageOutcome<S, O> {
    Wait(S),
    Done(O)
}