use bevy::prelude::*;

use crate::chunk::types::ChunkOwnerId;

#[derive(Resource, Clone, Debug, Default)]
pub struct RemovedChunkLoaders(pub Vec<RemovedChunkLoader>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RemovedChunkLoader {
    pub id: ChunkOwnerId,
}