use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate) enum ChunkAction {
    SpawnChunk {
        coord: (i32, i32)
    },
    DespawnChunk {
        coord: (i32, i32)
    },
    TransferChunkOwnership {
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