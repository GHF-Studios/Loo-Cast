use bevy::prelude::*;
use crate::{chunk::components::*, operations::structs::*};

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegisteredChunkInfo {
    Unmanaged(InstanceID<Chunk>),
    Managed(InstanceID<Chunk>),
}

impl RegisteredChunkInfo {
    pub fn chunk_id(&self) -> InstanceID<Chunk> {
        match self {
            Self::Unmanaged(chunk_id) => *chunk_id,
            Self::Managed(chunk_id) => *chunk_id,
        }
    }
}