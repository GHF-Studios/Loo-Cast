use bevy::prelude::*;

use crate::chunk::types::ChunkOwnerId;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Reflect)]
pub struct RemovedChunkLoader {
    pub id: ChunkOwnerId,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Reflect)]
pub struct RemovedChunkLoaderObservation {
    pub entity: Entity,
}