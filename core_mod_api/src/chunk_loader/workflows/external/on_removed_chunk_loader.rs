// Imports
use bevy::prelude::*;

use crate::chunk::components::Chunk;
use crate::chunk::functions::*;
use crate::chunk::intent::ActionIntent;
use crate::chunk::resources::{ActionIntentCommitBuffer, ChunkManager};
use crate::chunk::types::ChunkOwnerId;
use crate::chunk_loader::components::ChunkLoader;
use crate::chunk_loader::resources::{RemovedChunkLoader, RemovedChunkLoaders};
use crate::chunk_loader::workflows::chunk_loader::unload_chunks::user_items::UnloadChunkInput;

// Items

// Core Types
pub struct MainAccess<'w, 's, S: Scale> {
    pub removed_chunk_loaders: ResMut<'w, RemovedChunkLoaders<S>>,
}

pub struct Input<S: Scale> {
    pub chunk_owner_id: ChunkOwnerId,
}

// Core Functions
pub fn run_ecs<S: Scale>(input: Input<S>, mut main_access: MainAccess<S>) {
    let mut removed_chunk_loaders = &mut main_access.removed_chunk_loaders;

    let chunk_owner_id = input.chunk_owner_id;

    removed_chunk_loaders.0.insert(RemovedChunkLoader { id: chunk_owner_id });
}
