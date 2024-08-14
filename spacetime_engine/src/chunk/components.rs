use bevy::prelude::*;
use crate::chunk::id::structs::*;
use crate::chunk::actor::id::structs::*;

use super::loader::id::structs::ChunkLoaderID;

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct Chunk {
    id: ChunkID,
    registered_chunk_actors: Vec<ChunkActorID>,
    owner: Option<ChunkLoaderID>
}

impl Chunk {
    pub fn new(id: ChunkID, owner: Option<ChunkLoaderID>) -> Self {
        Self {
            id,
            registered_chunk_actors: Vec::new(),
            owner
        }
    }

    pub fn register_chunk_actor(&mut self, chunk_actor_id: ChunkActorID) {
        self.registered_chunk_actors.push(chunk_actor_id);
    }

    pub fn unregister_chunk_actor(&mut self, chunk_actor_id: ChunkActorID) {
        self.registered_chunk_actors.retain(|&id| id != chunk_actor_id);
    }

    pub fn id(&self) -> ChunkID {
        self.id
    }

    pub(in crate) fn id_mut(&mut self) -> &mut ChunkID {
        &mut self.id
    }

    pub fn registered_chunk_actors(&self) -> &Vec<ChunkActorID> {
        &self.registered_chunk_actors
    }

    pub(in crate) fn registered_chunk_actors_mut(&mut self) -> &mut Vec<ChunkActorID> {
        &mut self.registered_chunk_actors
    }

    pub fn owner(&self) -> Option<ChunkLoaderID> {
        self.owner
    }

    pub(in crate) fn owner_mut(&mut self) -> &mut Option<ChunkLoaderID> {
        &mut self.owner
    }
}