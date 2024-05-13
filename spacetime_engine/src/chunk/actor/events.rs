use bevy::prelude::*;
use crate::chunk::actor::id::structs::*;

#[derive(Clone, Event)]
pub struct CreateChunkActor(pub ChunkActorID);

#[derive(Clone, Event)]
pub struct DestroyChunkActor(pub ChunkActorID);