use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct PlayerActionQueue(pub Vec<PlayerAction>);

pub enum PlayerAction {
    Spawn,
    Despawn(Entity),
}