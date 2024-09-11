use crate::{chunk::id::structs::{ChunkID, ChunkRequestID}, entity::id::structs::EntityID};

#[derive(Debug, Clone, Copy)]
pub struct ChunkRequest {
    pub chunk_request_id: ChunkRequestID,
    pub chunk_id: ChunkID,
    pub chunk_entity_id: EntityID,
}

impl PartialEq for ChunkRequest {
    fn eq(&self, other: &Self) -> bool {
        self.chunk_request_id == other.chunk_request_id 
        && self.chunk_id == other.chunk_id 
        && self.chunk_entity_id == other.chunk_entity_id
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ChunkResponse {
    Success {
        chunk_request_id: ChunkRequestID,
        chunk_id: ChunkID,
        chunk_entity_id: EntityID,
    },
    Failure {
        chunk_request_id: ChunkRequestID,
        chunk_id: ChunkID,
        chunk_entity_id: EntityID,
    },
}

impl PartialEq for ChunkResponse {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Success { 
                    chunk_request_id: chunk_request_id1,
                    chunk_id: chunk_id1,
                    chunk_entity_id: chunk_entity_id1,
                }, 
                Self::Success { 
                    chunk_request_id: chunk_request_id2,
                    chunk_id: chunk_id2,
                    chunk_entity_id: chunk_entity_id2,
                }
            ) => {
                chunk_request_id1 == chunk_request_id2 
                && chunk_id1 == chunk_id2 
                && chunk_entity_id1 == chunk_entity_id2
            },
            (
                Self::Failure { 
                    chunk_request_id: chunk_request_id1,
                    chunk_id: chunk_id1,
                    chunk_entity_id: chunk_entity_id1,
                }, 
                Self::Failure { 
                    chunk_request_id: chunk_request_id2,
                    chunk_id: chunk_id2,
                    chunk_entity_id: chunk_entity_id2,
                }
            ) => {
                chunk_request_id1 == chunk_request_id2 
                && chunk_id1 == chunk_id2 
                && chunk_entity_id1 == chunk_entity_id2
            },
            _ => false,
        }
    }
}
