use bevy::prelude::*;

#[derive(Event)]
pub struct ChunkSpawnEvent {
    pub chunk: (i32, i32),
}

#[derive(Event)]
pub struct ChunkDespawnEvent {
    pub chunk: (i32, i32),
}