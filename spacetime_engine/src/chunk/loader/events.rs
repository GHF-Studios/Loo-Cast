use bevy::prelude::*;
use super::id::structs::*;

// TODO: Implement
#[derive(Debug, Clone, Event)]
pub(super) struct StartChunkLoader {
    pub chunk_loader_id: ChunkLoaderID
}

// TODO: Implement
#[derive(Debug, Clone, Event)]
pub(super) struct UpdateChunkLoader {
    pub chunk_loader_id: ChunkLoaderID
}

#[derive(Debug, Clone, Event)]
pub enum StartChunkLoaderResult {
    Success {
        chunk_loader_id: ChunkLoaderID
    },
    Failure {
        chunk_loader_id: ChunkLoaderID
    }
}

#[derive(Debug, Clone, Event)]
pub enum UpdateChunkLoaderResult {
    Success {
        chunk_loader_id: ChunkLoaderID
    },
    Failure {
        chunk_loader_id: ChunkLoaderID
    }
}