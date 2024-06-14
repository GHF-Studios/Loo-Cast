use bevy::prelude::*;
use crate::chunk::id::structs::*;

#[derive(Debug, Clone, Event)]
pub struct CreateChunkEntity {
    pub chunk_id: ChunkID,
}

#[derive(Debug, Clone, Event)]
pub struct CreatedChunkEntity {
    pub chunk_id: ChunkID,
    pub success: bool,
}

#[derive(Debug, Clone, Event)]
pub struct DestroyChunkEntity {
    pub chunk_id: ChunkID,
}

#[derive(Debug, Clone, Event)]
pub struct DestroyedChunkEntity {
    pub chunk_id: ChunkID,
    pub success: bool,
}

#[derive(Debug, Clone, Event)]
pub struct LoadChunkEntity {
    pub chunk_id: ChunkID,
}

#[derive(Debug, Clone, Event)]
pub struct LoadedChunkEntity {
    pub chunk_id: ChunkID,
    pub success: bool,
}

#[derive(Debug, Clone, Event)]
pub struct UnloadChunkEntity {
    pub chunk_id: ChunkID,
}

#[derive(Debug, Clone, Event)]
pub struct UnloadedChunkEntity {
    pub chunk_id: ChunkID,
    pub success: bool,
}

#[derive(Debug, Clone, Event)]
pub(in crate) struct CreateChunkEntityInternal {
    pub chunk_id: ChunkID,
}

#[derive(Debug, Clone, Event)]
pub(in crate) struct CreatedChunkEntityInternal {
    pub chunk_id: ChunkID,
    pub success: bool,
}

#[derive(Debug, Clone, Event)]
pub(in crate) struct DestroyChunkEntityInternal {
    pub chunk_id: ChunkID,
}

#[derive(Debug, Clone, Event)]
pub(in crate) struct DestroyedChunkEntityInternal {
    pub chunk_id: ChunkID,
    pub success: bool,
}

#[derive(Debug, Clone, Event)]
pub(in crate) struct LoadChunkEntityInternal {
    pub chunk_id: ChunkID,
}

#[derive(Debug, Clone, Event)]
pub(in crate) struct LoadedChunkEntityInternal {
    pub chunk_id: ChunkID,
    pub success: bool,
}

#[derive(Debug, Clone, Event)]
pub(in crate) struct UnloadChunkEntityInternal {
    pub chunk_id: ChunkID,
}

#[derive(Debug, Clone, Event)]
pub(in crate) struct UnloadedChunkEntityInternal {
    pub chunk_id: ChunkID,
    pub success: bool,
}