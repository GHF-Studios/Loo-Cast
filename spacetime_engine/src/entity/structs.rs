use bevy::prelude::*;
use super::id::structs::*;

#[derive(Debug, Clone, Copy)]
pub struct EntityRequest {
    pub entity_request_id: EntityRequestID,
    pub entity_id: EntityID,
}

impl PartialEq for EntityRequest {
    fn eq(&self, other: &Self) -> bool {
        self.entity_request_id == other.entity_request_id
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) struct InternalEntityRequest {
    pub entity_request_id: EntityRequestID,
    pub entity_id: EntityID,
    pub world_position: Vec2,
}

impl PartialEq for InternalEntityRequest {
    fn eq(&self, other: &Self) -> bool {
        self.entity_request_id == other.entity_request_id
    }
}

#[derive(Debug, Clone, Copy)]
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

impl PartialEq for InternalEntityResponse {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Success { entity_request_id: entity_request_id1, .. }, Self::Success { entity_request_id: entity_request_id2, .. }) => entity_request_id1 == entity_request_id2,
            (Self::Failure { entity_request_id: entity_request_id1, .. }, Self::Failure { entity_request_id: entity_request_id2, .. }) => entity_request_id1 == entity_request_id2,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
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

impl PartialEq for EntityResponse {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Success { entity_request_id: entity_request_id1, .. }, Self::Success { entity_request_id: entity_request_id2, .. }) => entity_request_id1 == entity_request_id2,
            (Self::Failure { entity_request_id: entity_request_id1, .. }, Self::Failure { entity_request_id: entity_request_id2, .. }) => entity_request_id1 == entity_request_id2,
            _ => false,
        }
    }
}