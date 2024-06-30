use bevy::prelude::*;
use crate::{chunk::id::structs::ChunkID, entity::id::structs::*};
use super::id::structs::*;

#[derive(Debug, Clone, Event)]
pub struct CreateChunkActorEntity {
    pub chunk_actor_event_id: ChunkActorEventID,
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
}

#[derive(Debug, Clone, Event)]
pub enum StartedChunkActor {
    Success {
        chunk_actor_event_id: ChunkActorEventID,
        chunk_actor_id: ChunkActorID,
        chunk_actor_entity_id: EntityID,
        chunk_id: ChunkID,
        world_position: Vec2,
    },
    Failure {
        chunk_actor_event_id: ChunkActorEventID,
        chunk_actor_id: ChunkActorID,
        chunk_actor_entity_id: EntityID,
        chunk_id: ChunkID,
        world_position: Vec2,
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
        chunk_actor_id: ChunkActorID,
        chunk_actor_entity_id: EntityID,
        chunk_id: ChunkID,
        world_position: Vec2,
    }
}

#[derive(Debug, Clone, Event)]
pub enum DestroyedChunkActorEntity {
    Success {
        chunk_actor_event_id: ChunkActorEventID,
        chunk_actor_id: ChunkActorID,
        chunk_actor_entity_id: EntityID,
        chunk_id: ChunkID,
        world_position: Vec2,
    },
    Failure {
        chunk_actor_event_id: ChunkActorEventID,
        chunk_actor_id: ChunkActorID,
        chunk_actor_entity_id: EntityID,
        chunk_id: ChunkID,
        world_position: Vec2,
    },
}

#[derive(Debug, Clone, Event)]
pub enum UpgradedToChunkActorEntity {
    Success {
        chunk_actor_event_id: ChunkActorEventID,
        chunk_actor_id: ChunkActorID,
        target_entity_id: EntityID,
        chunk_id: ChunkID,
        world_position: Vec2,
    },
    Failure {
        chunk_actor_event_id: ChunkActorEventID,
        chunk_actor_id: ChunkActorID,
        target_entity_id: EntityID,
        chunk_id: ChunkID,
        world_position: Vec2,
    },
}

#[derive(Debug, Clone, Event)]
pub struct CreateChunkActorEntityInternal {
    pub chunk_actor_event_id: ChunkActorEventID,
    pub chunk_actor_id: ChunkActorID,
    pub chunk_actor_entity_id: EntityID,
    pub chunk_id: ChunkID,
    pub world_position: Vec2,
}

#[derive(Debug, Clone, Event)]
pub struct DestroyChunkActorEntityInternal {
    pub chunk_actor_event_id: ChunkActorEventID,
    pub chunk_actor_id: ChunkActorID,
    pub chunk_actor_entity_id: EntityID,
}

#[derive(Debug, Clone, Event)]
pub struct UpgradeToChunkActorEntityInternal {
    pub chunk_actor_event_id: ChunkActorEventID,
    pub chunk_actor_id: ChunkActorID,
    pub target_entity_id: EntityID,
    pub chunk_id: ChunkID,
    pub world_position: Vec2,
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
        chunk_actor_id: ChunkActorID,
        chunk_actor_entity_id: EntityID,
        chunk_id: ChunkID,
        world_position: Vec2,
    }
}

#[derive(Debug, Clone, Event)]
pub enum DestroyedChunkActorEntityInternal {
    Success {
        chunk_actor_event_id: ChunkActorEventID,
        chunk_actor_id: ChunkActorID,
        chunk_actor_entity_id: EntityID,
        chunk_id: ChunkID,
        world_position: Vec2,
    },
    Failure {
        chunk_actor_event_id: ChunkActorEventID,
        chunk_actor_id: ChunkActorID,
        chunk_actor_entity_id: EntityID,
        chunk_id: ChunkID,
        world_position: Vec2,
    },
}

#[derive(Debug, Clone, Event)]
pub enum UpgradedToChunkActorEntityInternal {
    Success {
        chunk_actor_event_id: ChunkActorEventID,
        chunk_actor_id: ChunkActorID,
        chunk_actor_entity_id: EntityID,
        chunk_id: ChunkID,
        world_position: Vec2,
    },
    Failure {
        chunk_actor_event_id: ChunkActorEventID,
        chunk_actor_id: ChunkActorID,
        target_entity_id: EntityID,
        chunk_id: ChunkID,
        world_position: Vec2,
    },
}