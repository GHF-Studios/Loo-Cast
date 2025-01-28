use std::any::TypeId;
use std::pin::Pin;
use futures::future::BoxFuture;

use bevy::prelude::*;

use super::stage_io::{ActionStageIO, InputState, OutputState};

pub struct ActionTargetType {
    pub name: String,
    pub type_id: TypeId,
    pub ecs_action_types: Vec<ActionType>
}

pub enum ActionStage {
    Ecs(ActionStageEcs),
    Async(ActionStageAsync),
}

pub struct ActionStageEcs {
    pub name: String,
    pub function: Box<dyn FnMut(ActionStageIO<InputState>, &mut World) -> ActionStageIO<OutputState>>
}

pub struct ActionStageAsync {
    pub name: String,
    pub function: Pin<Box<dyn FnOnce(ActionStageIO<InputState>) -> BoxFuture<'static, ActionStageIO<OutputState>> + Send + Sync>>,
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
