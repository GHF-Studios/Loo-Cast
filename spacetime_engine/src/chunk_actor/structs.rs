use crate::operations::structs::InstanceID;
use crate::chunk::components::*;
use crate::chunk_actor::components::*;
use bevy::prelude::*;

#[derive(Clone, Debug)]
pub(super) struct UpdateChunkActorInfo {
    pub actor_entity: Entity,
    pub old_chunk_id: InstanceID<Chunk>,
    pub new_chunk_id: InstanceID<Chunk>,
    pub actor_id: InstanceID<ChunkActor>,
}

#[derive(Clone, Debug)]
pub(super) struct DespawnChunkActorInfo {
    pub actor_entity: Entity,
    pub actor_id: InstanceID<ChunkActor>,
}