use bevy::prelude::*;
use crate::{chunk::components::*, core::structs::*};

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegisteredChunkInfo {
    Unmanaged(NumericID<Chunk>),
    Managed(NumericID<Chunk>),
}

impl RegisteredChunkInfo {
    pub fn chunk_id(&self) -> NumericID<Chunk> {
        match self {
            Self::Unmanaged(chunk_id) => *chunk_id,
            Self::Managed(chunk_id) => *chunk_id,
        }
    }
}