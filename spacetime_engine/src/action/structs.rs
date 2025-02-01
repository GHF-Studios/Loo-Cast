use std::any::Any;
use std::{any::TypeId, error::Error};
use std::pin::Pin;
use futures::future::BoxFuture;

use bevy::prelude::*;

use super::stage_io::{ActionStageIO, InputState, OutputState};

pub struct ActionTargetType {
    pub name: String,
    pub type_id: TypeId,
    pub action_types: Vec<ActionType>
}

pub enum ActionStage {
    Ecs(ActionStageEcs),
    Async(ActionStageAsync),
}

pub struct ActionStageEcs {
    pub name: String,
    pub function: Box<dyn FnMut(ActionStageIO<InputState>, &mut World) -> ActionStageIO<OutputState> + Send + Sync>
}

pub struct ActionStageAsync {
    pub name: String,
    pub function: Pin<Box<dyn FnOnce(ActionStageIO<InputState>) -> BoxFuture<'static, ActionStageIO<OutputState>> + Send + Sync>>,
}

pub struct ActionType {
    pub name: String,
    pub validation: Box<dyn Fn(&dyn Any) -> Result<(), String> + Send + Sync>,
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
