use bevy::prelude::*;

use crate::chunk::id::structs::*;
use crate::entity::id::structs::*;
use super::id::structs::*;

pub(in crate::chunk::actor) struct ChunkActorUpdateInfo {
    pub actor_entity: Entity,
    pub old_chunk_id: ChunkID,
    pub new_chunk_id: ChunkID,
    pub actor_id: ChunkActorID,
}

pub(in crate::chunk::actor) struct ChunkActorDespawnInfo {
    pub actor_entity: Entity,
    pub actor_id: ChunkActorID,
}

#[derive(Clone, Debug)]
pub(in crate::chunk::actor) struct ChunkActorCreateRequest {
    pub chunk_actor_id: ChunkActorID,
    pub chunk_actor_entity_id: EntityID,
    pub chunk_id: ChunkID,
    pub world_position: Vec2,
}