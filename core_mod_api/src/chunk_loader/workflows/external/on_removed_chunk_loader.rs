// Imports
use bevy::prelude::*;
use std::marker::PhantomData;

use crate::chunk::types::ChunkOwnerId;

use super::super::super::types::RemovedChunkLoader;
use super::super::super::resources::RemovedChunkLoaders;

// Items

// Core Types
#[derive(bevy::ecs::system::SystemParam)]
pub struct MainAccess<'w, 's> {
    pub removed_chunk_loaders: ResMut<'w, RemovedChunkLoaders>,
    pub phantom_data: PhantomData<&'s ()>,
}

pub struct Input {
    pub chunk_owner_id: Option<ChunkOwnerId>,
}

// Core Functions
pub fn run_ecs(input: Input, mut main_access: MainAccess) {
    let removed_chunk_loaders = &mut main_access.removed_chunk_loaders;

    if let Some(id) = input.chunk_owner_id {
        removed_chunk_loaders.0.insert(RemovedChunkLoader { id });
    }
}
