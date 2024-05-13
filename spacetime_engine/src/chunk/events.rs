use bevy::prelude::*;
use crate::chunk::id::structs::*;

#[derive(Clone, Event)]
pub struct CreateChunk(pub ChunkID);

#[derive(Clone, Event)]
pub struct DestroyChunk(pub ChunkID);

#[derive(Clone, Event)]
pub struct LoadChunk(pub ChunkID);

#[derive(Clone, Event)]
pub struct UnloadChunk(pub ChunkID);

#[derive(Clone, Event)]
pub(in crate) struct CreateChunkInternal(pub ChunkID);

#[derive(Clone, Event)]
pub(in crate) struct DestroyChunkInternal(pub ChunkID);

#[derive(Clone, Event)]
pub(in crate) struct LoadChunkInternal(pub ChunkID);

#[derive(Clone, Event)]
pub(in crate) struct UnloadChunkInternal(pub ChunkID);