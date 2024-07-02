use bevy::prelude::*;
use crate::{chunk::id::structs::*, entity::id::structs::EntityID};
use super::id::structs::*;

#[derive(Clone, Debug)]
pub(super) struct UpdateChunkActorInfo {
    pub actor_entity: Entity,
    pub old_chunk_id: ChunkID,
    pub new_chunk_id: ChunkID,
    pub actor_id: ChunkActorID,
}

#[derive(Clone, Debug)]
pub(super) struct DespawnChunkActorInfo {
    pub actor_entity: Entity,
    pub actor_id: ChunkActorID,
}

#[derive(Clone, Debug)]
pub(super) struct ChunkActorCreateRequest {
    pub chunk_actor_request_id: ChunkActorRequestID,
    pub chunk_actor_id: ChunkActorID,
    pub chunk_actor_entity_id: EntityID,
    pub chunk_id: ChunkID,
    pub world_position: Vec2,
}

#[derive(Clone, Debug)]
pub(super) struct ChunkActorUpgradeRequest {
    pub chunk_actor_request_id: ChunkActorRequestID,
    pub chunk_actor_id: ChunkActorID,
    pub target_entity_id: EntityID,
    pub chunk_id: ChunkID,
    pub world_position: Vec2,
}