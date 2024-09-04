use bevy::{prelude::Entity, reflect::Reflect};

use crate::{chunk::components::Chunk, operations::InstanceID};

use super::{components::ChunkLoader, id::structs::ChunkLoaderRequestID};

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

#[derive(Debug, Clone, Copy)]
pub struct ChunkLoaderRequest {
    pub chunk_loader_request_id: ChunkLoaderRequestID,
    pub chunk_loader_id: InstanceID<ChunkLoader>,
    pub chunk_loader_entity_id: InstanceID<Entity>,
}

impl PartialEq for ChunkLoaderRequest {
    fn eq(&self, other: &Self) -> bool {
        self.chunk_loader_request_id == other.chunk_loader_request_id 
        && self.chunk_loader_id == other.chunk_loader_id 
        && self.chunk_loader_entity_id == other.chunk_loader_entity_id
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ChunkLoaderResponse {
    Success {
        chunk_loader_request_id: ChunkLoaderRequestID,
        chunk_loader_id: InstanceID<ChunkLoader>,
        chunk_loader_entity_id: InstanceID<Entity>,
    },
    Failure {
        chunk_loader_request_id: ChunkLoaderRequestID,
        chunk_loader_id: InstanceID<ChunkLoader>,
        chunk_loader_entity_id: InstanceID<Entity>,
    },
}

impl PartialEq for ChunkLoaderResponse {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Success { 
                    chunk_loader_request_id: chunk_loader_request_id1,
                    chunk_loader_id: chunk_loader_id1,
                    chunk_loader_entity_id: chunk_loader_entity_id1,
                }, 
                Self::Success { 
                    chunk_loader_request_id: chunk_loader_request_id2,
                    chunk_loader_id: chunk_loader_id2,
                    chunk_loader_entity_id: chunk_loader_entity_id2,
                }
            ) => {
                chunk_loader_request_id1 == chunk_loader_request_id2 
                && chunk_loader_id1 == chunk_loader_id2 
                && chunk_loader_entity_id1 == chunk_loader_entity_id2
            },
            (
                Self::Failure { 
                    chunk_loader_request_id: chunk_loader_request_id1,
                    chunk_loader_id: chunk_loader_id1,
                    chunk_loader_entity_id: chunk_loader_entity_id1,
                }, 
                Self::Failure { 
                    chunk_loader_request_id: chunk_loader_request_id2,
                    chunk_loader_id: chunk_loader_id2,
                    chunk_loader_entity_id: chunk_loader_entity_id2,
                }
            ) => {
                chunk_loader_request_id1 == chunk_loader_request_id2 
                && chunk_loader_id1 == chunk_loader_id2 
                && chunk_loader_entity_id1 == chunk_loader_entity_id2
            },
            _ => false,
        }
    }
}