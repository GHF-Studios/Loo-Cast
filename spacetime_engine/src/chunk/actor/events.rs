use bevy::prelude::*;
use crate::{chunk::id::structs::ChunkID, entity::id::structs::*};
use super::id::structs::ChunkActorID;

// TODO: Remove the chunk_id from this event. 
// It should be computed from the world_position, as to keep the event's fields simple and intuitive to understand and use.
#[derive(Clone, Event)]
pub struct CreateChunkActorEntity {
    pub chunk_id: ChunkID,
    pub world_position: Vec2,
}

#[derive(Clone, Event)]
pub struct DestroyChunkActorEntity {
    pub chunk_actor_id: ChunkActorID,
}

#[derive(Clone, Event)]
pub struct UpgradeToChunkActorEntity {
    pub target_entity_id: EntityID,
    pub chunk_id: ChunkID,
}

#[derive(Debug, Clone, Event)]
pub enum StartChunkActorResult {
    Success {
        chunk_actor_id: ChunkActorID,
    },
    Failure {
        chunk_actor_id: ChunkActorID,
    }
}

#[derive(Clone, Event)]
pub enum CreateChunkActorEntityResult {
    Success {
        chunk_actor_id: ChunkActorID,
        chunk_actor_entity_id: EntityID,
        chunk_id: ChunkID,
        world_position: Vec2,
    },
    Failure {
        chunk_id: ChunkID,
        world_position: Vec2,
    }
}

#[derive(Clone, Event)]
pub enum DestroyChunkActorEntityResult {
    Success {
        chunk_actor_id: ChunkActorID,
    },
    Failure {
        chunk_actor_id: ChunkActorID
    },
}

#[derive(Clone, Event)]
pub enum UpgradeToChunkActorEntityResult {
    Success {
        chunk_actor_id: ChunkActorID,
        target_entity_id: EntityID,
        chunk_id: ChunkID,
    },
    Failure {
        target_entity_id: EntityID,
        chunk_id: ChunkID,
    },
}