use bevy::prelude::*;
use crate::chunk::id::structs::*;
use crate::chunk::actor::id::structs::*;

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct ChunkActor {
    id: ChunkActorID,
    current_chunk: ChunkID,
}

impl ChunkActor {
    pub fn new(id: ChunkActorID, current_chunk: ChunkID) -> Self {
        Self {
            id,
            current_chunk,
        }
    }

    pub fn id(&self) -> ChunkActorID {
        self.id
    }

    pub(in crate) fn id_mut(&mut self) -> &mut ChunkActorID {
        &mut self.id
    }

    pub fn current_chunk(&self) -> ChunkID {
        self.current_chunk
    }

    pub(in crate) fn current_chunk_mut(&mut self) -> &mut ChunkID {
        &mut self.current_chunk
    }
}