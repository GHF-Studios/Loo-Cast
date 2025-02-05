use std::any::Any;
use bevy::prelude::*;

use super::{stage::ActionStage, target::ActionTargetRef};

#[derive(Debug)]
pub enum ActionState {
    Requested,
    Processing {
        current_stage: usize
    },
}
impl std::fmt::Display for ActionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Requested => write!(f, "ActionState::Requested"),
            Self::Processing { current_stage } => write!(f, "ActionState::Processing(current_stage: {})", current_stage),
        }
    }
}
impl ActionState {
    pub fn is_requested(&self) -> bool {
        matches!(self, Self::Requested)
    }
}

pub struct ActionType {
    pub name: String,
    pub validation: Box<dyn Fn(ActionTargetRef) -> Result<(), String> + Send + Sync>,
    pub stages: Vec<ActionStage>
}

pub struct Action {
    pub action_type: ActionType,
}

pub struct ActionInstance {
    pub entity: Entity,
    pub target_type: String,
    pub action_name: String,
    pub state: ActionState,
    pub data_buffer: Box<dyn Any + Send + Sync>,
    pub callback: Option<Box<dyn FnOnce(&mut World, Box<dyn Any + Send + Sync>) + Send + Sync>>,
    pub num_stages: usize,
    pub timeout_frames: usize,
}
impl std::fmt::Debug for ActionInstance{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "ActionInstance(entity: {}, target_type: {}, action_name: {}, state: {})", 
            self.entity, self.target_type, self.action_name, self.state)
    }
}
impl ActionInstance {
    pub(in super) fn new_request(
        entity: Entity, 
        target_type: String, 
        action_name: String, 
        input_params: Box<dyn Any + Send + Sync>, 
        output_callback: Option<Box<dyn FnOnce(&mut World, Box<dyn Any + Send + Sync>) + Send + Sync>>,
        num_stages: usize,
    ) -> Self {
        let timeout_frames = num_stages * 30;

        Self {
            entity,
            target_type,
            action_name,
            state: ActionState::Requested,
            data_buffer: input_params,
            callback: output_callback,
            num_stages,
            timeout_frames
        }
    }
}
