use bevy::prelude::*;
use crate::chunk::components::Chunk;
use crate::operations::InstanceID;

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct ChunkActor {
    id: InstanceID<ChunkActor>,
    current_chunk: InstanceID<Chunk>,
}

impl ChunkActor {
    pub fn new(start_chunk: InstanceID<Chunk>) -> Self {
        Self {
            id: InstanceID::default(),
            current_chunk: start_chunk,
        }
    }

    pub fn id(&self) -> InstanceID<ChunkActor> {
        self.id
    }

    pub(in crate) fn id_mut(&mut self) -> &mut InstanceID<ChunkActor> {
        &mut self.id
    }

    pub fn current_chunk(&self) -> InstanceID<Chunk> {
        self.current_chunk
    }

    pub(in crate) fn current_chunk_mut(&mut self) -> &mut InstanceID<Chunk> {
        &mut self.current_chunk
    }
}