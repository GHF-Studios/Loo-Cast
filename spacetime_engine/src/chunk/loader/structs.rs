use crate::entity::id::structs::EntityID;

use super::id::structs::{ChunkLoaderID, ChunkLoaderRequestID};

#[derive(Debug, Clone, Copy)]
pub struct ChunkLoaderRequest {
    pub chunk_loader_request_id: ChunkLoaderRequestID,
    pub chunk_loader_id: ChunkLoaderID,
    pub chunk_loader_entity_id: EntityID,
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
        chunk_loader_id: ChunkLoaderID,
        chunk_loader_entity_id: EntityID,
    },
    Failure {
        chunk_loader_request_id: ChunkLoaderRequestID,
        chunk_loader_id: ChunkLoaderID,
        chunk_loader_entity_id: EntityID,
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