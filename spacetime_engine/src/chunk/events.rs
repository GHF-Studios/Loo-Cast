use bevy::prelude::*;
use super::structs::*;

#[derive(Debug, Clone, Event)]
pub struct CreateChunk(pub ChunkRequest);

#[derive(Debug, Clone, Event)]
pub struct DestroyChunk(pub ChunkRequest);

#[derive(Debug, Clone, Event)]
pub struct LoadChunk(pub ChunkRequest);

#[derive(Debug, Clone, Event)]
pub struct UnloadChunk(pub ChunkRequest);

#[derive(Debug, Clone, Event)]
pub(super) struct CreateChunkInternal(pub InternalChunkRequest);

#[derive(Debug, Clone, Event)]
pub(super) struct DestroyChunkInternal(pub InternalChunkRequest);

#[derive(Debug, Clone, Event)]
pub(super) struct LoadChunkInternal(pub InternalChunkRequest);

#[derive(Debug, Clone, Event)]
pub(super) struct UnloadChunkInternal(pub InternalChunkRequest);

#[derive(Debug, Clone, Event)]
pub(crate) struct CreatedChunkInternal(pub InternalChunkResponse);

#[derive(Debug, Clone, Event)]
pub(crate) struct DestroyedChunkInternal(pub InternalChunkResponse);

#[derive(Debug, Clone, Event)]
pub(crate) struct LoadedChunkInternal(pub InternalChunkResponse);

#[derive(Debug, Clone, Event)]
pub(crate) struct UnloadedChunkInternal(pub InternalChunkResponse);

#[derive(Debug, Clone, Event)]
pub struct CreatedChunk(pub ChunkResponse);

#[derive(Debug, Clone, Event)]
pub struct DestroyedChunk(pub ChunkResponse);

#[derive(Debug, Clone, Event)]
pub struct LoadedChunk(pub ChunkResponse);

#[derive(Debug, Clone, Event)]
pub struct UnloadedChunk(pub ChunkResponse);

#[derive(Debug, Clone, Event)]
pub struct StartedChunk(pub ChunkResponse);

#[derive(Debug, Clone, Event)]
pub struct StoppedChunk(pub ChunkResponse);