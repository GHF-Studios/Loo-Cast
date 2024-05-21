use bevy::prelude::*;
use super::id::structs::*;

#[derive(Debug, Clone, Event)]
pub struct StartedChunkLoader {
    pub chunk_loader_id: ChunkLoaderID
}

#[derive(Debug, Clone, Event)]
pub struct UpdatedChunkLoader {
    pub chunk_loader_id: ChunkLoaderID
}