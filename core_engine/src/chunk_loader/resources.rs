use bevy::prelude::*;
use std::collections::HashSet;

use crate::chunk::types::ChunkOwnerId;

#[derive(Resource, Reflect, Clone, Debug, Default)]
#[reflect(Resource)]
pub struct RemovedChunkLoaders(pub HashSet<RemovedChunkLoader>);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Reflect)]
pub struct RemovedChunkLoader {
    pub id: ChunkOwnerId,
}
