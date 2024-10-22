use bevy::prelude::*;
use crate::chunk::components::Chunk;
use crate::core::structs::DynamicKey;

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct ChunkActor {
    id: DynamicKey<ChunkActor>,
    current_chunk: DynamicKey<Chunk>,
}

impl ChunkActor {
    pub fn new(start_chunk: DynamicKey<Chunk>) -> Self {
        Self {
            id: DynamicKey::default(),
            current_chunk: start_chunk,
        }
    }

    pub fn id(&self) -> DynamicKey<ChunkActor> {
        self.id
    }

    pub(in crate) fn id_mut(&mut self) -> &mut DynamicKey<ChunkActor> {
        &mut self.id
    }

    pub fn current_chunk(&self) -> DynamicKey<Chunk> {
        self.current_chunk
    }

    pub(in crate) fn current_chunk_mut(&mut self) -> &mut DynamicKey<Chunk> {
        &mut self.current_chunk
    }
}