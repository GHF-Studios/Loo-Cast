use bevy::prelude::*;
use crate::chunk::components::Chunk;
use crate::core::structs::DynamicID;

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct ChunkActor {
    id: DynamicID<ChunkActor>,
    current_chunk: DynamicID<Chunk>,
}

impl ChunkActor {
    pub fn new(start_chunk: DynamicID<Chunk>) -> Self {
        Self {
            id: DynamicID::default(),
            current_chunk: start_chunk,
        }
    }

    pub fn id(&self) -> DynamicID<ChunkActor> {
        self.id
    }

    pub(in crate) fn id_mut(&mut self) -> &mut DynamicID<ChunkActor> {
        &mut self.id
    }

    pub fn current_chunk(&self) -> DynamicID<Chunk> {
        self.current_chunk
    }

    pub(in crate) fn current_chunk_mut(&mut self) -> &mut DynamicID<Chunk> {
        &mut self.current_chunk
    }
}