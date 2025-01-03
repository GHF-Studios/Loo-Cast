use bevy::prelude::*;

#[derive(Debug)]
pub(in crate) enum ChunkRetryAction {
    Spawn {
        chunk_coord: (i32, i32),
        chunk_owner: Entity,
    },
    Despawn {
        chunk_coord: (i32, i32),
        chunk_entity: Entity,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ChunkState {
    Spawning,
    Loaded,
    Despawning,
}