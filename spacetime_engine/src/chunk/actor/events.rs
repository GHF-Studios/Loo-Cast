use bevy::prelude::*;
use crate::{chunk::id::structs::ChunkID, entity::id::structs::*};

#[derive(Clone, Event)]
pub struct CreateChunkActorEntity {
    pub chunk_actor_entity_id: EntityID,
    pub chunk_id: ChunkID,
    pub world_position: Vec2,
}

#[derive(Clone, Event)]
pub struct CreatedChunkActorEntity {
    pub chunk_actor_entity_id: EntityID,
    pub chunk_id: ChunkID,
    pub world_position: Vec2,
    pub success: bool,
}

#[derive(Clone, Event)]
pub(in crate) struct CreateChunkActorEntityInternal {
    pub chunk_actor_entity_id: EntityID,
    pub chunk_id: ChunkID,
    pub world_position: Vec2,
}

#[derive(Clone, Event)]
pub(in crate) struct CreatedChunkActorEntityInternal {
    pub chunk_actor_entity_id: EntityID,
    pub chunk_id: ChunkID,
    pub world_position: Vec2,
    pub success: bool,
}