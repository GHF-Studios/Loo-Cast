use std::any::{Any, TypeId};

use bevy::prelude::*;

use super::stage_io::{ActionStageIO, InputState, OutputState};

pub struct ActionTargetType {
    pub name: String,
    pub type_id: TypeId,
    pub actions_types: Vec<ActionType>
}

pub enum ActionStage {
    Ecs {
        name: String,
        function: Box<dyn FnMut(ActionStageIO<InputState>, &mut Commands) -> ActionStageIO<OutputState>>
    },
    NonEcs {
        name: String,
        function: Box<dyn FnMut(ActionStageIO<InputState>) -> ActionStageIO<OutputState>>
    },
}

pub struct ActionType {
    pub name: String,
    pub stages: Vec<ActionStage>
}

pub enum ActionState {
    Queued,
    Processing,
    Completed,
    Failed
}

pub enum ActionTargetState {
    Idle,
    Busy(ActionState),
}

pub struct Action {
    pub action_type: ActionType,
    pub current_stage: usize,
    pub state: ActionState,
}
