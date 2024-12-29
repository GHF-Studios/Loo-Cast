use bevy::prelude::*;

#[derive(Event)]
pub struct InitializedChunkEvent(Entity);