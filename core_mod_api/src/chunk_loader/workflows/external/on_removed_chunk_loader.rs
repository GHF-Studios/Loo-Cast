// Imports
use bevy::prelude::*;
use std::marker::PhantomData;

use crate::chunk::types::ChunkOwnerId;
use crate::usf::scale::ConstScale;

use super::super::super::types::RemovedChunkLoader;
use super::super::super::resources::RemovedChunkLoaders;

// Items

// Core Types
#[derive(bevy::ecs::system::SystemParam)]
pub struct MainAccess<'w, 's, S: ConstScale> {
    pub removed_chunk_loaders: ResMut<'w, RemovedChunkLoaders<S>>,
    pub phantom_data: PhantomData<&'s ()>,
}

pub struct Input<S: ConstScale> {
    pub chunk_owner_id: Option<ChunkOwnerId<S>>,
}

// Core Functions
pub fn run_ecs<S: ConstScale>(input: Input<S>, mut main_access: MainAccess<S>) {
    let removed_chunk_loaders = &mut main_access.removed_chunk_loaders;

    if let Some(id) = input.chunk_owner_id {
        removed_chunk_loaders.0.insert(RemovedChunkLoader { id });
    }
}
