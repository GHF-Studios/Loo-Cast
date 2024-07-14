use bevy::prelude::*;
use super::id::structs::*;

#[derive(Debug, Clone)]
pub struct EntityRequest {
    pub entity_request_id: EntityRequestID,
    pub entity_id: EntityID,
}

#[derive(Debug, Clone)]
pub(super) struct InternalEntityRequest {
    pub entity_request_id: EntityRequestID,
    pub entity_id: EntityID,
    pub world_position: Vec2,
}

#[derive(Debug, Clone)]
pub(crate) enum InternalEntityResponse {
    Success {
        entity_request_id: EntityRequestID,
        entity_id: EntityID,
        world_position: Vec2,
    },
    Failure {
        entity_request_id: EntityRequestID,
        entity_id: EntityID,
        world_position: Vec2,
    },
}

#[derive(Debug, Clone)]
pub enum EntityResponse {
    Success {
        entity_request_id: EntityRequestID,
        entity_id: EntityID,
        world_position: Vec2,
    },
    Failure {
        entity_request_id: EntityRequestID,
        entity_id: EntityID,
        world_position: Vec2,
    },
}