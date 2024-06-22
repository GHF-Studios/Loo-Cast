use bevy::prelude::*;
use crate::{chunk::id::structs::ChunkID, entity::id::structs::*};
use super::id::structs::*;

// TODO: Read this (muy importánte)
// Hier, und bei den anderen Events für chunk loader die chunk id aus den non-internen events komplett entfernen
// und im handler für die externen events berechnet werden durch ggf. zu implementierende implementationen von dem From-Trait
#[derive(Debug, Clone, Event)]
pub struct CreateChunkActorEntity {
    pub chunk_actor_event_id: ChunkActorEventID,
    pub chunk_id: ChunkID,
    pub world_position: Vec2,
}

#[derive(Debug, Clone, Event)]
pub struct DestroyChunkActorEntity {
    pub chunk_actor_event_id: ChunkActorEventID,
    pub chunk_actor_id: ChunkActorID,
}

#[derive(Debug, Clone, Event)]
pub struct UpgradeToChunkActorEntity {
    pub chunk_actor_event_id: ChunkActorEventID,
    pub target_entity_id: EntityID,
    pub chunk_id: ChunkID,
}

#[derive(Debug, Clone, Event)]
pub enum StartedChunkActor {
    Success {
        chunk_actor_event_id: ChunkActorEventID,
        chunk_actor_id: ChunkActorID,
    },
    Failure {
        chunk_actor_event_id: ChunkActorEventID,
        chunk_actor_id: ChunkActorID,
    }
}

#[derive(Debug, Clone, Event)]
pub enum CreatedChunkActorEntity {
    Success {
        chunk_actor_event_id: ChunkActorEventID,
        chunk_actor_id: ChunkActorID,
        chunk_actor_entity_id: EntityID,
        chunk_id: ChunkID,
        world_position: Vec2,
    },
    Failure {
        chunk_actor_event_id: ChunkActorEventID,
        chunk_id: ChunkID,
        world_position: Vec2,
    }
}

#[derive(Debug, Clone, Event)]
pub enum DestroyedChunkActorEntity {
    Success {
        chunk_actor_event_id: ChunkActorEventID,
        chunk_actor_id: ChunkActorID,
    },
    Failure {
        chunk_actor_event_id: ChunkActorEventID,
        chunk_actor_id: ChunkActorID
    },
}

#[derive(Debug, Clone, Event)]
pub enum UpgradedToChunkActorEntity {
    Success {
        chunk_actor_event_id: ChunkActorEventID,
        chunk_actor_id: ChunkActorID,
        target_entity_id: EntityID,
        chunk_id: ChunkID,
    },
    Failure {
        chunk_actor_event_id: ChunkActorEventID,
        target_entity_id: EntityID,
        chunk_id: ChunkID,
    },
}

#[derive(Debug, Clone, Event)]
pub struct CreateChunkActorEntityInternal {
    pub chunk_actor_event_id: ChunkActorEventID,
    pub chunk_id: ChunkID,
    pub world_position: Vec2,
}

#[derive(Debug, Clone, Event)]
pub struct DestroyChunkActorEntityInternal {
    pub chunk_actor_event_id: ChunkActorEventID,
    pub chunk_actor_id: ChunkActorID,
}

#[derive(Debug, Clone, Event)]
pub struct UpgradeToChunkActorEntityInternal {
    pub chunk_actor_event_id: ChunkActorEventID,
    pub target_entity_id: EntityID,
    pub chunk_id: ChunkID,
}

#[derive(Debug, Clone, Event)]
pub enum CreatedChunkActorEntityInternal {
    Success {
        chunk_actor_event_id: ChunkActorEventID,
        chunk_actor_id: ChunkActorID,
        chunk_actor_entity_id: EntityID,
        chunk_id: ChunkID,
        world_position: Vec2,
    },
    Failure {
        chunk_actor_event_id: ChunkActorEventID,
        chunk_id: ChunkID,
        world_position: Vec2,
    }
}

#[derive(Debug, Clone, Event)]
pub enum DestroyedChunkActorEntityInternal {
    Success {
        chunk_actor_event_id: ChunkActorEventID,
        chunk_actor_id: ChunkActorID,
    },
    Failure {
        chunk_actor_event_id: ChunkActorEventID,
        chunk_actor_id: ChunkActorID
    },
}

#[derive(Debug, Clone, Event)]
pub enum UpgradedToChunkActorEntityInternal {
    Success {
        chunk_actor_event_id: ChunkActorEventID,
        chunk_actor_id: ChunkActorID,
        target_entity_id: EntityID,
        chunk_id: ChunkID,
    },
    Failure {
        chunk_actor_event_id: ChunkActorEventID,
        target_entity_id: EntityID,
        chunk_id: ChunkID,
    },
}