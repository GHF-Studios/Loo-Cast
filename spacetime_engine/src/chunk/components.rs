use bevy::prelude::*;
use crate::chunk::id::structs::*;
use crate::chunk::actor::id::structs::*;

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct Chunk {
    id: ChunkID,
    chunk_actors: Vec<ChunkActorID>,
}

impl Chunk {
    pub fn new(id: ChunkID) -> Self {
        Self {
            id,
            chunk_actors: Vec::new(),
        }
    }

    pub fn add_chunk_actor(&mut self, chunk_actor_id: ChunkActorID) {
        self.chunk_actors.push(chunk_actor_id);
    }

    pub fn remove_chunk_actor(&mut self, chunk_actor_id: ChunkActorID) {
        self.chunk_actors.retain(|&id| id != chunk_actor_id);
    }

    pub fn id(&self) -> ChunkID {
        self.id
    }

    pub(in crate) fn id_mut(&mut self) -> &mut ChunkID {
        &mut self.id
    }

    pub fn chunk_actors(&self) -> &Vec<ChunkActorID> {
        &self.chunk_actors
    }

    pub(in crate) fn chunk_actors_mut(&mut self) -> &mut Vec<ChunkActorID> {
        &mut self.chunk_actors
    }
}