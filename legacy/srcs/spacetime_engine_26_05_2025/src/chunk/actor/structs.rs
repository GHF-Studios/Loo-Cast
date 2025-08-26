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

#[derive(Clone, Debug, Copy)]
pub struct ChunkActorRequest {
    pub chunk_actor_request_id: ChunkActorRequestID,
    pub chunk_actor_id: ChunkActorID,
    pub chunk_actor_entity_id: EntityID,
}

impl PartialEq for ChunkActorRequest {
    fn eq(&self, other: &Self) -> bool {
        self.chunk_actor_request_id == other.chunk_actor_request_id 
        && self.chunk_actor_id == other.chunk_actor_id
        && self.chunk_actor_entity_id == other.chunk_actor_entity_id
    }
}

#[derive(Clone, Debug, Copy)]
pub enum ChunkActorResponse {
    Success {
        chunk_actor_request_id: ChunkActorRequestID,
        chunk_actor_id: ChunkActorID,
        chunk_actor_entity_id: EntityID,
    },
    Failure {
        chunk_actor_request_id: ChunkActorRequestID,
        chunk_actor_id: ChunkActorID,
        chunk_actor_entity_id: EntityID,
    },
}

impl PartialEq for ChunkActorResponse {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Success { 
                    chunk_actor_request_id: chunk_actor_request_id1,
                    chunk_actor_id: chunk_actor_id1,
                    chunk_actor_entity_id: chunk_actor_entity_id1,
                }, 
                Self::Success { 
                    chunk_actor_request_id: chunk_actor_request_id2,
                    chunk_actor_id: chunk_actor_id2,
                    chunk_actor_entity_id: chunk_actor_entity_id2,
                }
            ) => {
                chunk_actor_request_id1 == chunk_actor_request_id2 && chunk_actor_id1 == chunk_actor_id2 && chunk_actor_entity_id1 == chunk_actor_entity_id2
            },
            (
                Self::Failure { 
                    chunk_actor_request_id: chunk_actor_request_id1,
                    chunk_actor_id: chunk_actor_id1,
                    chunk_actor_entity_id: chunk_actor_entity_id1,
                }, 
                Self::Failure { 
                    chunk_actor_request_id: chunk_actor_request_id2,
                    chunk_actor_id: chunk_actor_id2,
                    chunk_actor_entity_id: chunk_actor_entity_id2,
                }
            ) => {
                chunk_actor_request_id1 == chunk_actor_request_id2 && chunk_actor_id1 == chunk_actor_id2 && chunk_actor_entity_id1 == chunk_actor_entity_id2
            },
            _ => false,
        }
    }
}