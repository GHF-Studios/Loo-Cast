use std::any::{Any, TypeId};

use super::types::{ActionState, ActionType};

pub struct ActionTypeModule {
    pub name: String,
    pub action_types: Vec<ActionType>
}

pub enum ActionTargetState {
    Idle,
    Busy(ActionState),
}