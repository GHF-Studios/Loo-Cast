use bevy::prelude::*;

use crate::chunk::id::structs::*;
use crate::entity::id::structs::*;
use super::id::structs::*;

#[derive(Clone, Debug)]
pub(in crate::chunk::actor) struct UpdateChunkActorInfo {
    pub actor_entity: Entity,
    pub old_chunk_id: ChunkID,
    pub new_chunk_id: ChunkID,
    pub actor_id: ChunkActorID,
}

#[derive(Clone, Debug)]
pub(in crate::chunk::actor) struct DespawnChunkActorInfo {
    pub actor_entity: Entity,
    pub actor_id: ChunkActorID,
}

#[derive(Clone, Debug)]
pub(in crate::chunk::actor) struct CreateChunkActorEntityRequest {
    pub chunk_actor_id: ChunkActorID,
    pub chunk_actor_entity_id: EntityID,
    pub chunk_id: ChunkID,
    pub world_position: Vec2,
}

#[derive(Clone, Debug)]
pub(in crate::chunk::actor) struct UpgradeToChunkActorEntityRequest {
    pub chunk_actor_id: ChunkActorID,
    pub target_entity_id: EntityID,
    pub chunk_id: ChunkID,
}