use bevy::prelude::*;
use crate::entity::id::structs::EntityID;
use crate::chunk::id::structs::{ChunkID, ChunkRequestID};

#[derive(Debug, Clone)]
pub struct ChunkRequest {
    pub chunk_request_id: ChunkRequestID,
    pub target_entity_id: Entity,
}

#[derive(Debug, Clone)]
pub(super) struct InternalChunkRequest {
    pub chunk_request_id: ChunkRequestID,
    pub chunk_id: ChunkID,
    pub chunk_entity_id: EntityID,
    pub world_position: Vec2,
}

#[derive(Debug, Clone)]
pub(crate) enum InternalChunkResponse {
    Success {
        chunk_request_id: ChunkRequestID,
        chunk_id: ChunkID,
        chunk_entity_id: EntityID,
        world_position: Vec2,
    },
    Failure {
        chunk_request_id: ChunkRequestID,
        chunk_id: ChunkID,
        target_entity_id: EntityID,
        world_position: Vec2,
    },
}

#[derive(Debug, Clone)]
pub enum ChunkResponse {
    Success {
        chunk_request_id: ChunkRequestID,
        chunk_id: ChunkID,
        chunk_entity_id: EntityID,
        world_position: Vec2,
    },
    Failure {
        chunk_request_id: ChunkRequestID,
        chunk_id: ChunkID,
        target_entity_id: EntityID,
        world_position: Vec2,
    },
}