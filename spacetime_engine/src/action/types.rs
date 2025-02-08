use std::any::{type_name, Any};
use bevy::prelude::*;

use super::{stage::ActionStage, stage_io::{ActionIO, CallbackState, InputState, OutputState}};

pub struct RawActionData {
    pub data: Box<dyn Any + Send + Sync>,
    pub data_type_name: &'static str,
}
impl RawActionData {
    pub fn new<D: Any + Send + Sync>(raw_data: D) -> Self {
        Self {
            data: Box::new(raw_data),
            data_type_name: type_name::<D>(),
        }
    }
}

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
    pub validation: Box<dyn Fn(ActionIO<InputState>, &mut World) -> Result<ActionIO<OutputState>, String> + Send + Sync>,
    pub stages: Vec<ActionStage>
}

pub struct Action {
    pub action_type: ActionType,
}

pub struct ActionInstance {
    pub module_name: String,
    pub action_name: String,
    pub state: ActionState,
    pub data_buffer: Box<dyn Any + Send + Sync>,
    pub callback: Option<Box<dyn FnOnce(&mut World, ActionIO<CallbackState>) + Send + Sync>>,
    pub num_stages: usize,
    pub timeout_frames: usize,
}
impl std::fmt::Debug for ActionInstance{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "ActionInstance(module_name: {}, action_name: {}, state: {}, num_stages: {}, timeout_frames: {})", 
            self.module_name, self.action_name, self.state, self.num_stages, self.timeout_frames)
    }
}
impl ActionInstance {
    pub(in super) fn new_request(
        module_name: String, 
        action_name: String, 
        input_params: Box<dyn Any + Send + Sync>, 
        output_callback: Option<Box<dyn FnOnce(&mut World, ActionIO<CallbackState>) + Send + Sync>>,
        num_stages: usize,
    ) -> Self {
        let timeout_frames = num_stages * 30;

        Self {
            module_name,
            action_name,
            state: ActionState::Requested,
            data_buffer: input_params,
            callback: output_callback,
            num_stages,
            timeout_frames
        }
    }
}
