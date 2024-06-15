use bevy::prelude::*;
use crate::entity::id::structs::*;
use super::id::structs::*;

#[derive(Clone, Debug)]
pub(super) struct CreateChunkLoaderEntityRequest {
    pub chunk_loader_id: ChunkLoaderID,
    pub chunk_loader_entity_id: EntityID,
    pub world_position: Vec2,
}

#[derive(Clone, Debug)]
pub(super) struct UpgradeToChunkLoaderEntityRequest {
    pub chunk_loader_id: ChunkLoaderID,
    pub target_entity_id: EntityID,
}