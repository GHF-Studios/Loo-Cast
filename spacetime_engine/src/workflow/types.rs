use std::any::{type_name, Any};
use bevy::prelude::*;

use crate::config::statics::CONFIG;

use super::{stage::WorkflowStage, stage_io::{WorkflowIO, CallbackState, InputState, OutputState}};

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

#[derive(Debug)]
pub enum WorkflowState {
    Requested,
    Processing {
        current_stage: usize
    },
    Processed {
        current_stage: usize
    }
}
impl std::fmt::Display for WorkflowState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Requested => write!(f, "WorkflowState::Requested"),
            Self::Processing { current_stage } => write!(f, "WorkflowState::Processing(current_stage: {})", current_stage),
            Self::Processed { current_stage } => write!(f, "WorkflowState::Processed(current_stage: {})", current_stage),
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
            Self::Processing { current_stage } => *current_stage,
            Self::Processed { current_stage } => *current_stage,
        }
    }
}

pub struct WorkflowType {
    pub name: String,
    pub primary_validation: Box<dyn Fn(WorkflowIO<InputState>) -> Result<WorkflowIO<InputState>, String> + Send + Sync>,
    pub secondary_validation: Box<dyn Fn(WorkflowIO<InputState>, &mut World) -> Result<WorkflowIO<InputState>, String> + Send + Sync>,
    pub stages: Vec<Option<WorkflowStage>>
}

pub struct Workflow {
    pub workflow_type: WorkflowType,
}

pub struct WorkflowInstance {
    pub module_name: String,
    pub workflow_name: String,
    pub state: WorkflowState,
    pub data_buffer: RawWorkflowData,
    pub callback: Option<Box<dyn FnOnce(&mut World, WorkflowIO<CallbackState>) + Send + Sync>>,
    pub num_stages: usize,
    pub timeout_frames: usize,
}
impl std::fmt::Debug for WorkflowInstance{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "WorkflowInstance(module_name: {}, workflow_name: {}, state: {}, num_stages: {}, timeout_frames: {})", 
            self.module_name, self.workflow_name, self.state, self.num_stages, self.timeout_frames)
    }
}
impl WorkflowInstance {
    pub(in super) fn new_request(
        module_name: String, 
        workflow_name: String, 
        input_params: RawWorkflowData, 
        output_callback: Option<Box<dyn FnOnce(&mut World, WorkflowIO<CallbackState>) + Send + Sync>>,
        num_stages: usize,
    ) -> Self {
        let timeout_frames = num_stages * CONFIG.get::<usize>("workflow/timeout_frames_per_stage");

        Self {
            module_name,
            workflow_name,
            state: WorkflowState::Requested,
            data_buffer: input_params,
            callback: output_callback,
            num_stages,
            timeout_frames
        }
    }
}
