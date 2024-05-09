use bevy::prelude::*;
use crate::chunk::id::structs::*;
use crate::chunk::actor::id::structs::*;

#[derive(Component, Reflect, Debug, PartialEq, Eq)]
pub struct ChunkActor {
    pub(in crate) id: ChunkActorID,
    pub(in crate) current_chunk: ChunkID,
}