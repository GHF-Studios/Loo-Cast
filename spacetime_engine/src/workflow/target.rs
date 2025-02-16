use std::any::{Any, TypeId};

use super::types::{WorkflowState, WorkflowType};

pub struct WorkflowTypeModule {
    pub name: String,
    pub workflow_types: Vec<WorkflowType>
}

pub enum WorkflowTargetState {
    Idle,
    Busy(WorkflowState),
}