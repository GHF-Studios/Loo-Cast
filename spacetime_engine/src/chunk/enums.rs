use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate) enum ChunkAction {
    Spawn {
        coord: (i32, i32),
        owner: Option<Entity>
    },
    Despawn {
        coord: (i32, i32)
    },
    TransferOwnership {
        coord: (i32, i32),
        new_owner: Entity
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ChunkState {
    Spawning,
    Loaded,
    Despawning,
}