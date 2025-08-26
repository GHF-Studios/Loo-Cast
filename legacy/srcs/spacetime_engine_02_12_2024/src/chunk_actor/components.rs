use bevy::prelude::*;
use crate::chunk::components::Chunk;
use crate::core::structs::NumericID;

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct ChunkActor {
    id: NumericID<ChunkActor>,
    current_chunk: NumericID<Chunk>,
}

impl ChunkActor {
    pub fn new(start_chunk: NumericID<Chunk>) -> Self {
        Self {
            id: NumericID::default(),
            current_chunk: start_chunk,
        }
    }

    pub fn id(&self) -> NumericID<ChunkActor> {
        self.id
    }

    pub(in crate) fn id_mut(&mut self) -> &mut NumericID<ChunkActor> {
        &mut self.id
    }

    pub fn current_chunk(&self) -> NumericID<Chunk> {
        self.current_chunk
    }

    pub(in crate) fn current_chunk_mut(&mut self) -> &mut NumericID<Chunk> {
        &mut self.current_chunk
    }
}