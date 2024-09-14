use bevy::prelude::*;
use crate::operations::structs::InstanceID;

use crate::chunk::structs::ChunkPosition;
use crate::chunk_actor::components::ChunkActor;
use crate::chunk_loader::components::ChunkLoader;

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct Chunk {
    id: InstanceID<Chunk>,
    position: ChunkPosition,
    owner: Option<InstanceID<ChunkLoader>>,
    registered_chunk_actors: Vec<InstanceID<ChunkActor>>,
}

impl Chunk {
    pub fn new(position: ChunkPosition, owner: Option<InstanceID<ChunkLoader>>) -> Self {
        Self {
            id: InstanceID::default(),
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

    pub fn position(&self) -> ChunkPosition {
        self.position
    }
    pub(in crate) fn position_mut(&mut self) -> &mut ChunkPosition {
        &mut self.position
    }

    pub fn owner(&self) -> Option<InstanceID<ChunkLoader>> {
        self.owner
    }
    pub(in crate) fn owner_mut(&mut self) -> &mut Option<InstanceID<ChunkLoader>> {
        &mut self.owner
    }

    pub fn registered_chunk_actors(&self) -> &Vec<InstanceID<ChunkActor>> {
        &self.registered_chunk_actors
    }
    pub(in crate) fn registered_chunk_actors_mut(&mut self) -> &mut Vec<InstanceID<ChunkActor>> {
        &mut self.registered_chunk_actors
    }
}