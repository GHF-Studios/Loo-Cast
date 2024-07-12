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
pub(super) struct ChunkActorRequest {
    pub chunk_actor_request_id: ChunkActorRequestID,
    pub target_entity_id: EntityID,
}

#[derive(Clone, Debug)]
pub(super) struct InternalChunkActorRequest {
    chunk_actor_request_id: ChunkActorRequestID,
    chunk_actor_id: ChunkActorID,
    chunk_actor_entity_id: EntityID,
    chunk_id: ChunkID,
    world_position: Vec2,
}

#[derive(Clone, Debug)]
pub(super) enum InternalChunkActorResponse {
    Success {
        chunk_actor_request_id: ChunkActorRequestID,
        chunk_actor_id: ChunkActorID,
        chunk_actor_entity_id: EntityID,
        chunk_id: ChunkID,
        world_position: Vec2,
    },
    Failure {
        chunk_actor_request_id: ChunkActorRequestID,
        chunk_actor_id: ChunkActorID,
        target_entity_id: EntityID,
        chunk_id: ChunkID,
        world_position: Vec2,
    },
}

#[derive(Clone, Debug)]
pub(super) enum ChunkActorResponse {
    Success {
        chunk_actor_request_id: ChunkActorRequestID,
        chunk_actor_id: ChunkActorID,
        chunk_actor_entity_id: EntityID,
        chunk_id: ChunkID,
        world_position: Vec2,
    },
    Failure {
        chunk_actor_request_id: ChunkActorRequestID,
        chunk_actor_id: ChunkActorID,
        target_entity_id: EntityID,
        chunk_id: ChunkID,
        world_position: Vec2,
    },
}