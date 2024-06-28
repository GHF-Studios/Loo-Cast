use bevy::prelude::*;
use crate::chunk::id::structs::*;
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