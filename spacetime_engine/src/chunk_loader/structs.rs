use bevy::prelude::*;
use crate::{chunk::components::*, core::structs::*};

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegisteredChunkInfo {
    Unmanaged(DynamicID<Chunk>),
    Managed(DynamicID<Chunk>),
}

impl RegisteredChunkInfo {
    pub fn chunk_id(&self) -> DynamicID<Chunk> {
        match self {
            Self::Unmanaged(chunk_id) => *chunk_id,
            Self::Managed(chunk_id) => *chunk_id,
        }
    }
}