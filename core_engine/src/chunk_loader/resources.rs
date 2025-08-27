use bevy::prelude::*;

use crate::chunk::types::ChunkOwnerId;

#[derive(Resource, Reflect, Clone, Debug, Default)]
#[reflect(Resource)]
pub struct RemovedChunkLoaders(pub Vec<RemovedChunkLoader>);

#[derive(Clone, Debug, PartialEq, Eq, Reflect)]
pub struct RemovedChunkLoader {
    pub id: ChunkOwnerId,
}