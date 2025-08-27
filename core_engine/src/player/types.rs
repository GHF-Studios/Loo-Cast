use bevy::prelude::*;

#[derive(Reflect)]
pub enum PlayerWorkflow {
    Spawn,
    Despawn(Entity),
}
