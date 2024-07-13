use bevy::prelude::*;
use crate::{chunk::id::structs::ChunkID, entity::id::structs::EntityID};
use super::id::structs::*;

#[derive(Debug, Clone)]
pub struct PlayerRequest {
    pub player_request_id: PlayerRequestID,
    pub target_entity_id: EntityID,
}

#[derive(Debug, Clone)]
pub struct InternalPlayerRequest {
    pub player_request_id: PlayerRequestID,
    pub player_id: PlayerID,
    pub player_entity_id: EntityID,
    pub chunk_id: ChunkID,
    pub world_position: Vec2
}

#[derive(Debug, Clone)]
pub enum InternalPlayerResponse {
    Success {
        player_request_id: PlayerRequestID,
        player_id: PlayerID,
        player_entity_id: EntityID,
        chunk_id: ChunkID,
        world_position: Vec2
    },
    Failure {
        player_request_id: PlayerRequestID,
        player_id: PlayerID,
        player_entity_id: EntityID,
        chunk_id: ChunkID,
        world_position: Vec2
    }
}

#[derive(Debug, Clone)]
pub enum PlayerResponse {
    Success {
        player_request_id: PlayerRequestID,
        player_id: PlayerID,
        player_entity_id: EntityID,
        chunk_id: ChunkID,
        world_position: Vec2
    },
    Failure {
        player_request_id: PlayerRequestID,
        player_id: PlayerID,
        player_entity_id: EntityID,
        chunk_id: ChunkID,
        world_position: Vec2
    }
}