use bevy::prelude::*;
use crate::entity::id::structs::EntityID;
use super::id::structs::*;

#[derive(Debug, Clone, Event)]
pub struct CreatePlayerEntity {
    pub player_request_id: PlayerRequestID,
    pub world_position: Vec2
}

#[derive(Debug, Clone, Event)]
pub struct DestroyPlayerEntity {
    pub player_request_id: PlayerRequestID,
    pub player_id: PlayerID
}

#[derive(Debug, Clone, Event)]
pub struct UpgradeToPlayerEntity {
    pub player_request_id: PlayerRequestID,
    pub target_entity_id: EntityID,
}

#[derive(Debug, Clone, Event)]
pub enum StartedPlayer {
    Success {
        player_request_id: PlayerRequestID,
        player_id: PlayerID
    },
    Failure {
        player_request_id: PlayerRequestID,
        player_id: PlayerID
    }
}

#[derive(Debug, Clone, Event)]
pub enum CreatedPlayerEntity {
    Success {
        player_request_id: PlayerRequestID,
        player_id: PlayerID,
        player_entity_id: EntityID,
        world_position: Vec2
    },
    Failure {
        player_request_id: PlayerRequestID,
        world_position: Vec2
    }
}

#[derive(Debug, Clone, Event)]
pub enum DestroyedPlayerEntity {
    Success {
        player_request_id: PlayerRequestID,
        player_id: PlayerID
    },
    Failure {
        player_request_id: PlayerRequestID,
        player_id: PlayerID
    }
}

#[derive(Debug, Clone, Event)]
pub enum UpgradedToPlayerEntity {
    Success {
        player_request_id: PlayerRequestID,
        player_id: PlayerID,
        target_entity_id: EntityID,
    },
    Failure {
        player_request_id: PlayerRequestID,
        target_entity_id: EntityID
    }
}