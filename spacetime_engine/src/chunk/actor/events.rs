use bevy::prelude::*;
use crate::{chunk::id::structs::ChunkID, entity::id::structs::*};
use super::id::structs::ChunkActorID;

#[derive(Clone, Event)]
pub struct CreateChunkActorEntity {
    pub chunk_id: ChunkID,
    pub world_position: Vec2,
}

#[derive(Clone, Event)]
pub struct CreatedChunkActorEntity {
    pub chunk_actor_id: ChunkActorID,
    pub chunk_actor_entity_id: EntityID,
    pub chunk_id: ChunkID,
    pub world_position: Vec2,
    pub success: bool,
}

#[derive(Clone, Event)]
pub struct DestroyChunkActorEntity {
    pub chunk_actor_id: ChunkActorID,
}

#[derive(Clone, Event)]
pub struct DestroyedChunkActorEntity {
    pub chunk_actor_id: ChunkActorID,
    pub success: bool,
}

#[derive(Clone, Event)]
pub struct ConvertToChunkActorEntity {
    pub target_entity_id: EntityID,
    pub chunk_id: ChunkID,
}

#[derive(Clone, Event)]
pub struct ConvertedToChunkActorEntity {
    pub chunk_actor_id: ChunkActorID,
    pub chunk_actor_entity_id: EntityID,
    pub chunk_id: ChunkID,
    pub success: bool,
}