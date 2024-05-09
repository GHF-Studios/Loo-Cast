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