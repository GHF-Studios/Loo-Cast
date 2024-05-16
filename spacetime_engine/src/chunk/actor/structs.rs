use bevy::prelude::*;

use crate::chunk::id::structs::ChunkID;
use super::id::structs::ChunkActorID;

pub(in crate::chunk::actor) struct ActorUpdateInfo {
    pub actor_entity: Entity,
    pub old_chunk_id: ChunkID,
    pub new_chunk_id: ChunkID,
    pub actor_id: ChunkActorID,
}

pub(in crate::chunk::actor) struct ActorDespawnInfo {
    pub actor_entity: Entity,
    pub actor_id: ChunkActorID,
}