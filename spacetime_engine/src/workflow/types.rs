use bevy::prelude::*;
use futures::future::BoxFuture;
use std::any::{type_name, Any};
use tokio::{runtime::Runtime, task::JoinHandle};

use super::{io::{InputState, WorkflowIO}, stage::WorkflowStage, statics::TOKIO_RUNTIME};

pub struct RawWorkflowData {
    pub data: Box<dyn Any + Send + Sync>,
    pub data_type_name: &'static str,
}
impl RawWorkflowData {
    pub fn new<D: Any + Send + Sync>(value: D) -> Self {
        let wrapped_value = Self {
            data: Box::new(value),
            data_type_name: type_name::<D>(),
        };

        // TODO: Inefficient! Cache the type name.
        if wrapped_value.data_type_name == type_name::<RawWorkflowData>() {
            panic!("Attempted to create a RawWorkflowData with a RawWorkflowData data type.")
        }

        wrapped_value
    }
}

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
        stage_completed: bool,
    },
}
impl std::fmt::Display for WorkflowState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Requested => write!(f, "WorkflowState::Requested"),
            Self::Processing { current_stage, stage_completed: completed } => write!(f, "WorkflowState::Processing(current_stage: {}, completed: {})", current_stage, completed),
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
    pub name: String,
    pub workflow_types: Vec<WorkflowType>
}

pub struct WorkflowType {
    pub name: String,
    pub primary_validation: Box<dyn Fn(WorkflowIO<InputState>) -> Result<WorkflowIO<InputState>, String> + Send + Sync>,
    pub secondary_validation: Box<dyn Fn(WorkflowIO<InputState>, &mut World) -> Result<WorkflowIO<InputState>, String> + Send + Sync>,
    pub stages: Vec<WorkflowStage>
}

pub struct Workflow {
    pub workflow_type: WorkflowType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome<S, O> {
    Wait(S),
    Done(O)
}

pub enum WorkflowResult<T> {
    Ok(T),
    Err(String)
}