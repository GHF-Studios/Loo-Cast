use bevy::prelude::*;
use crate::chunk::id::structs::*;
use crate::chunk::actor::id::structs::*;

#[derive(Component, Reflect)]
pub struct Chunk {
    pub id: ChunkID,
    pub chunk_actors: Vec<ChunkActorID>,
}