// Imports
use bevy::prelude::*;
use std::collections::HashSet;

use crate::chunk::functions::{calculate_chunk_distance_from_owner, calculate_chunks_in_radius, world_pos_to_chunk};
use crate::chunk::intent::ActionIntent;
use crate::chunk::resources::ActionIntentCommitBuffer;
use crate::chunk::resources::ChunkManager;
use crate::chunk_loader::components::ChunkLoader;
use crate::chunk_loader::workflows::chunk_loader::{load_chunks::user_items::LoadChunkInput, unload_chunks::user_items::UnloadChunkInput};
use crate::usf::scale::Scale;
use crate::utils::components::DropHook;

// Items

// Core Types
pub struct MainAccess<'w, 's, S: Scale> {
    pub chunk_loader_query: Query<'w, 's, (&'static Transform, &'static ChunkLoader<S>, Option<&'static DropHook<ChunkLoader<S>>>)>,
    pub chunk_manager: Res<'w, ChunkManager<S>>,
}
pub struct Output<S: Scale> {
    pub load_chunk_inputs: Vec<LoadChunkInput<S>>,
    pub unload_chunk_inputs: Vec<UnloadChunkInput<S>>,
}

// Core Functions
pub fn run_ecs<S: Scale>(main_access: MainAccess<S>) -> Output<S> {
    let chunk_loader_query = &main_access.chunk_loader_query;
    let chunk_manager = &main_access.chunk_manager;

    let mut load_chunk_inputs = Vec::new();
    let mut unload_chunk_inputs = Vec::new();

    for (transform, chunk_loader, drop_hook) in chunk_loader_query.iter() {
        let position = transform.translation.truncate();
        let radius = chunk_loader.radius;

        let chunk_owner_id = chunk_loader.chunk_owner_id();

        let target_chunks = if drop_hook.is_some() {
            HashSet::new()
        } else {
            calculate_chunks_in_radius(position, radius).into_iter().collect::<HashSet<(i32, i32)>>()
        };

        let current_chunks: HashSet<(i32, i32)> = chunk_manager
            .owned_chunks
            .iter()
            .filter_map(|(chunk, owner_id)| if owner_id == chunk_owner_id { Some(*chunk) } else { None })
            .collect();

        let chunks_to_load: Vec<_> = target_chunks.difference(&current_chunks).cloned().collect();
        let chunks_to_unload: Vec<_> = current_chunks.difference(&target_chunks).cloned().collect();

        for chunk_coord in chunks_to_load {
            let chunk_loader_distance_squared = calculate_chunk_distance_from_owner(&chunk_coord, &world_pos_to_chunk(position));
            let chunk_loader_radius_squared = radius * radius;

            load_chunk_inputs.push(LoadChunkInput {
                owner_id: chunk_owner_id.clone(),
                chunk_coord,
                chunk_loader_distance_squared,
                chunk_loader_radius_squared,
            });
        }

        for chunk_coord in chunks_to_unload {
            let chunk_loader_distance_squared = calculate_chunk_distance_from_owner(&chunk_coord, &world_pos_to_chunk(position));
            let chunk_loader_radius_squared = radius * radius;

            unload_chunk_inputs.push(UnloadChunkInput {
                owner_id: chunk_owner_id.clone(),
                chunk_coord,
                chunk_loader_distance_squared,
                chunk_loader_radius_squared,
            });
        }

        // warn!("Ran CategorizeChunks for {:?}", chunk_owner_id.id());
    }

    Output {
        load_chunk_inputs,
        unload_chunk_inputs,
    }
}
