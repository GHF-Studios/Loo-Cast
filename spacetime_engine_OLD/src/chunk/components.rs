use bevy::prelude::*;
use crate::core::structs::NumericID;

use crate::chunk::structs::ChunkPosition;
use crate::chunk_actor::components::ChunkActor;
use crate::chunk_loader::components::ChunkLoader;

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct Chunk {
    id: NumericID<Chunk>,
    position: ChunkPosition,
    owner: Option<NumericID<ChunkLoader>>,
    registered_chunk_actors: Vec<NumericID<ChunkActor>>,
}

impl Chunk {
    pub fn new(position: ChunkPosition, owner: Option<NumericID<ChunkLoader>>) -> Self {
        Self {
            id: NumericID::default(),
            position,
            registered_chunk_actors: Vec::new(),
            owner
        }
    }

    pub fn register_chunk_actor(&mut self, chunk_actor_id: NumericID<ChunkActor>) {
        self.registered_chunk_actors.push(chunk_actor_id);
    }
    pub fn unregister_chunk_actor(&mut self, chunk_actor_id: NumericID<ChunkActor>) {
        self.registered_chunk_actors.retain(|&id| id != chunk_actor_id);
    }

    pub fn id(&self) -> NumericID<Chunk> {
        self.id
    }
    pub(in crate) fn id_mut(&mut self) -> &mut NumericID<Chunk> {
        &mut self.id
    }

    pub fn position(&self) -> ChunkPosition {
        self.position
    }
    pub(in crate) fn position_mut(&mut self) -> &mut ChunkPosition {
        &mut self.position
    }

    pub fn owner(&self) -> Option<NumericID<ChunkLoader>> {
        self.owner
    }
    pub(in crate) fn owner_mut(&mut self) -> &mut Option<NumericID<ChunkLoader>> {
        &mut self.owner
    }

    pub fn registered_chunk_actors(&self) -> &Vec<NumericID<ChunkActor>> {
        &self.registered_chunk_actors
    }
    pub(in crate) fn registered_chunk_actors_mut(&mut self) -> &mut Vec<NumericID<ChunkActor>> {
        &mut self.registered_chunk_actors
    }
}