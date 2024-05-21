use bevy::prelude::*;
use crate::chunk::id::structs::*;

#[derive(Clone, Event)]
pub struct CreateChunk {
    pub chunk_id: ChunkID,
}

#[derive(Clone, Event)]
pub struct CreatedChunk {
    pub chunk_id: ChunkID,
    pub success: bool,
}

#[derive(Clone, Event)]
pub struct DestroyChunk {
    pub chunk_id: ChunkID,
}

#[derive(Clone, Event)]
pub struct DestroyedChunk {
    pub chunk_id: ChunkID,
    pub success: bool,
}

#[derive(Clone, Event)]
pub struct LoadChunk {
    pub chunk_id: ChunkID,
}

#[derive(Clone, Event)]
pub struct LoadedChunk {
    pub chunk_id: ChunkID,
    pub success: bool,
}

#[derive(Clone, Event)]
pub struct UnloadChunk {
    pub chunk_id: ChunkID,
}

#[derive(Clone, Event)]
pub struct UnloadedChunk {
    pub chunk_id: ChunkID,
    pub success: bool,
}

#[derive(Clone, Event)]
pub(in crate) struct CreateChunkInternal {
    pub chunk_id: ChunkID,
}

#[derive(Clone, Event)]
pub(in crate) struct CreatedChunkInternal {
    pub chunk_id: ChunkID,
    pub success: bool,
}

#[derive(Clone, Event)]
pub(in crate) struct DestroyChunkInternal {
    pub chunk_id: ChunkID,
}

#[derive(Clone, Event)]
pub(in crate) struct DestroyedChunkInternal {
    pub chunk_id: ChunkID,
    pub success: bool,
}

#[derive(Clone, Event)]
pub(in crate) struct LoadChunkInternal {
    pub chunk_id: ChunkID,
}

#[derive(Clone, Event)]
pub(in crate) struct LoadedChunkInternal {
    pub chunk_id: ChunkID,
    pub success: bool,
}

#[derive(Clone, Event)]
pub(in crate) struct UnloadChunkInternal {
    pub chunk_id: ChunkID,
}

#[derive(Clone, Event)]
pub(in crate) struct UnloadedChunkInternal {
    pub chunk_id: ChunkID,
    pub success: bool,
}