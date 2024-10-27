use crate::core::structs::NumericID;
use crate::chunk::components::*;
use crate::chunk_actor::components::*;
use bevy::prelude::*;

#[derive(Clone, Debug)]
pub(super) struct UpdateChunkActorInfo {
    pub actor_entity: Entity,
    pub old_chunk_id: NumericID<Chunk>,
    pub new_chunk_id: NumericID<Chunk>,
    pub actor_id: NumericID<ChunkActor>,
}

#[derive(Clone, Debug)]
pub(super) struct DespawnChunkActorInfo {
    pub actor_entity: Entity,
    pub actor_id: NumericID<ChunkActor>,
}