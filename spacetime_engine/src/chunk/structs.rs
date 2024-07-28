use crate::chunk::id::structs::{ChunkID, ChunkRequestID};

#[derive(Debug, Clone, Copy)]
pub struct ChunkRequest {
    pub chunk_request_id: ChunkRequestID,
    pub chunk_id: ChunkID,
}

impl PartialEq for ChunkRequest {
    fn eq(&self, other: &Self) -> bool {
        self.chunk_request_id == other.chunk_request_id && self.chunk_id == other.chunk_id
    }

}

#[derive(Debug, Clone, Copy)]
pub enum ChunkResponse {
    Success {
        chunk_request_id: ChunkRequestID,
        chunk_id: ChunkID,
    },
    Failure {
        chunk_request_id: ChunkRequestID,
        chunk_id: ChunkID,
    },
}

impl PartialEq for ChunkResponse {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Success { chunk_request_id: chunk_request_id1, .. }, Self::Success { chunk_request_id: chunk_request_id2, .. }) => chunk_request_id1 == chunk_request_id2,
            (Self::Failure { chunk_request_id: chunk_request_id1, .. }, Self::Failure { chunk_request_id: chunk_request_id2, .. }) => chunk_request_id1 == chunk_request_id2,
            _ => false,
        }
    }
}