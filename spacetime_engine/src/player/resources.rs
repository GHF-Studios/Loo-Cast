use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct PlayerWorkflowQueue(pub Vec<PlayerWorkflow>);

pub enum PlayerWorkflow {
    Spawn,
    Despawn(Entity),
}