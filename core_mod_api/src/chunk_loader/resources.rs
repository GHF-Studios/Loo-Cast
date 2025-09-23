use bevy::prelude::*;
use std::collections::HashSet;

use crate::chunk::types::ChunkOwnerId;
use crate::usf::scale::Scale;

#[derive(Resource, Reflect, Clone, Debug, Default)]
#[reflect(Resource)]
pub struct RemovedChunkLoaders<S: Scale>(pub HashSet<RemovedChunkLoader<S>>);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Reflect)]
pub struct RemovedChunkLoader<S: Scale> {
    pub id: ChunkOwnerId<S>,
}
