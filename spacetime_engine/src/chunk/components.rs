use bevy::prelude::*;
use crate::operations::InstanceID;

use super::actor::components::ChunkActor;
use super::loader::components::ChunkLoader;
use super::position::structs::ChunkPosition;

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct Chunk {
    id: InstanceID<Chunk>,
    position: ChunkPosition,
    registered_chunk_actors: Vec<InstanceID<ChunkActor>>,
    owner: Option<InstanceID<ChunkLoader>>
}

impl Chunk {
    pub fn new(id: InstanceID<Chunk>, position: ChunkPosition, owner: Option<InstanceID<ChunkLoader>>) -> Self {
        Self {
            id,
            position,
            registered_chunk_actors: Vec::new(),
            owner
        }
    }

    pub fn register_chunk_actor(&mut self, chunk_actor_id: InstanceID<ChunkActor>) {
        self.registered_chunk_actors.push(chunk_actor_id);
    }

    pub fn unregister_chunk_actor(&mut self, chunk_actor_id: InstanceID<ChunkActor>) {
        self.registered_chunk_actors.retain(|&id| id != chunk_actor_id);
    }

    pub fn id(&self) -> InstanceID<Chunk> {
        self.id
    }

    pub(in crate) fn id_mut(&mut self) -> &mut InstanceID<Chunk> {
        &mut self.id
    }

    pub fn registered_chunk_actors(&self) -> &Vec<InstanceID<ChunkActor>> {
        &self.registered_chunk_actors
    }

    pub(in crate) fn registered_chunk_actors_mut(&mut self) -> &mut Vec<InstanceID<ChunkActor>> {
        &mut self.registered_chunk_actors
    }

    pub fn owner(&self) -> Option<InstanceID<ChunkLoader>> {
        self.owner
    }

    pub(in crate) fn owner_mut(&mut self) -> &mut Option<InstanceID<ChunkLoader>> {
        &mut self.owner
    }
}