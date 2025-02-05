use std::{any::Any, pin::Pin};
use bevy::prelude::*;
use futures::future::BoxFuture;

use super::stage_io::*;

pub enum ActionStage {
    Ecs(ActionStageEcs),
    Async(ActionStageAsync),
}

// TODO: Ensure we correctly emit Action completion events and the full action lifecycle works as intended

/* TODO: Split up into a trait and specialized implementations,
*        just like the input and output types of actions,
*        to allow for type-safe system params so this behaves like a system
*/
pub struct ActionStageEcs {
    pub name: String,
    pub function: Box<dyn FnMut(ActionStageIO<InputState>, &mut World) -> ActionStageIO<OutputState> + Send + Sync>
}

/* TODO: Split up into a trait and specialized implementations,
*        just like the input and output types of actions,
*        to allow for type-safe system params so this behaves like a system
*/
pub struct ActionStageAsync {
    pub name: String,
    pub function: Pin<Box<dyn FnMut(ActionStageIO<InputState>) -> BoxFuture<'static, ActionStageIO<OutputState>> + Send + Sync>>,
}

pub struct ActionStageOutput {
    pub entity: Entity,
    pub target_type: String,
    pub action_name: String,
    pub output: Box<dyn Any + Send + Sync>,
}