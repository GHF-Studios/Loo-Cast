use bevy::prelude::*;

use crate::chunk::types::ChunkOwnerId;
use crate::usf::scale::Scale;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Reflect)]
pub struct RemovedChunkLoader<S: Scale> {
    pub id: ChunkOwnerId<S>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Reflect)]
pub struct RemovedChunkLoaderObservation {
    pub entity: Entity,
    pub scale: i8,
}