use bevy::prelude::*;
use crate::entity::id::structs::EntityID;
use super::id::structs::*;

#[derive(Debug, Clone, Event)]
pub struct CreatePlayerEntity {
    pub player_event_id: PlayerEventID,
    pub world_position: Vec2
}

#[derive(Debug, Clone, Event)]
pub struct DestroyPlayerEntity {
    pub player_event_id: PlayerEventID,
    pub player_id: PlayerID
}

#[derive(Debug, Clone, Event)]
pub struct UpgradeToPlayerEntity {
    pub player_event_id: PlayerEventID,
    pub target_entity_id: EntityID,
}

#[derive(Debug, Clone, Event)]
pub enum StartedPlayer {
    Success {
        player_event_id: PlayerEventID,
        player_id: PlayerID
    },
    Failure {
        player_event_id: PlayerEventID,
        player_id: PlayerID
    }
}

#[derive(Debug, Clone, Event)]
pub enum CreatedPlayerEntity {
    Success {
        player_event_id: PlayerEventID,
        player_id: PlayerID,
        player_entity_id: EntityID,
        world_position: Vec2
    },
    Failure {
        player_event_id: PlayerEventID,
        world_position: Vec2
    }
}

#[derive(Debug, Clone, Event)]
pub enum DestroyedPlayerEntity {
    Success {
        player_event_id: PlayerEventID,
        player_id: PlayerID
    },
    Failure {
        player_event_id: PlayerEventID,
        player_id: PlayerID
    }
}

#[derive(Debug, Clone, Event)]
pub enum UpgradedToPlayerEntity {
    Success {
        player_event_id: PlayerEventID,
        player_id: PlayerID,
        target_entity_id: EntityID,
    },
    Failure {
        player_event_id: PlayerEventID,
        target_entity_id: EntityID
    }
}