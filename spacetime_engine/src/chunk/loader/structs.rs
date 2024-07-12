use bevy::prelude::*;
use crate::entity::id::structs::EntityID;
use crate::chunk::id::structs::ChunkID;
use super::id::structs::{ChunkLoaderID, ChunkLoaderRequestID};

#[derive(Debug, Clone)]
pub struct ChunkLoaderRequest {
    pub chunk_loader_request_id: ChunkLoaderRequestID,
    pub target_entity_id: Entity,
}

#[derive(Debug, Clone)]
pub(super) struct InternalChunkLoaderRequest {
    pub chunk_loader_request_id: ChunkLoaderRequestID,
    pub chunk_loader_id: ChunkLoaderID,
    pub chunk_loader_entity_id: EntityID,
    pub chunk_id: ChunkID,
    pub world_position: Vec2,
}

#[derive(Debug, Clone)]
pub(crate) enum InternalChunkLoaderResponse {
    Success {
        chunk_loader_request_id: ChunkLoaderRequestID,
        chunk_loader_id: ChunkLoaderID,
        chunk_loader_entity_id: EntityID,
        chunk_id: ChunkID,
        world_position: Vec2,
    },
    Failure {
        chunk_loader_request_id: ChunkLoaderRequestID,
        chunk_loader_id: ChunkLoaderID,
        target_entity_id: EntityID,
        chunk_id: ChunkID,
        world_position: Vec2,
    },
}

#[derive(Debug, Clone)]
pub enum ChunkLoaderResponse {
    Success {
        chunk_loader_request_id: ChunkLoaderRequestID,
        chunk_loader_id: ChunkLoaderID,
        chunk_loader_entity_id: EntityID,
        chunk_id: ChunkID,
        world_position: Vec2,
    },
    Failure {
        chunk_loader_request_id: ChunkLoaderRequestID,
        chunk_loader_id: ChunkLoaderID,
        target_entity_id: EntityID,
        chunk_id: ChunkID,
        world_position: Vec2,
    },
}