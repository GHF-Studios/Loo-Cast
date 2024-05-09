use bevy::prelude::*;
use crate::chunk::actor::id::structs::*;

#[derive(Clone, Event)]
pub struct CreateChunkActor(pub ChunkActorID);

#[derive(Clone, Event)]
pub struct DestroyChunkActor(pub ChunkActorID);

#[derive(Clone, Event)]
pub struct LoadChunkActor(pub ChunkActorID);

#[derive(Clone, Event)]
pub struct UnloadChunkActor(pub ChunkActorID);