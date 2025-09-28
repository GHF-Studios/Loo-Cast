// Imports
use bevy::prelude::*;
use std::collections::HashSet;

use crate::chunk::functions::{calculate_chunk_distance_from_owner, calculate_chunks_in_radius, world_pos_to_chunk};
use crate::chunk::resources::ChunkManager;
use crate::chunk::traits::ChunkCoordTupleExt;
use crate::chunk::types::ChunkCoord;
use crate::chunk_loader::components::ChunkLoader;
use crate::chunk_loader::workflows::external::{load_chunks::LoadChunkInput, unload_chunks::UnloadChunkInput};
use crate::utils::components::DropHook;

// Items

// Core Types
#[derive(bevy::ecs::system::SystemParam)]
pub struct MainAccess<'w, 's> {
    pub chunk_loader_query: Query<'w, 's, (&'static Transform, &'static ChunkLoader, Option<&'static DropHook<ChunkLoader>>)>,
    pub chunk_manager: Res<'w, ChunkManager>,
}
pub struct Output {
    pub load_chunk_inputs: Vec<LoadChunkInput>,
    pub unload_chunk_inputs: Vec<UnloadChunkInput>,
}

// Core Functions
pub fn run_ecs(main_access: MainAccess) -> Output {
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
            calculate_chunks_in_radius(position, radius).into_iter().map(|coord| coord.scaled(chunk_loader.chunk_owner_id().scale())).collect::<HashSet<ChunkCoord>>()
        };

        let current_chunks: HashSet<ChunkCoord> = chunk_manager
            .owned_chunks
            .iter()
            .filter_map(|(chunk, owner_id)| if owner_id == chunk_owner_id { Some(*chunk) } else { None })
            .collect();

        let chunks_to_load: Vec<_> = target_chunks.difference(&current_chunks).cloned().collect();
        let chunks_to_unload: Vec<_> = current_chunks.difference(&target_chunks).cloned().collect();

        for chunk_coord in chunks_to_load {
            let chunk_loader_distance_squared = calculate_chunk_distance_from_owner(&chunk_coord.unscaled(), &world_pos_to_chunk(position));
            let chunk_loader_radius_squared = radius * radius;

            load_chunk_inputs.push(LoadChunkInput {
                owner_id: chunk_owner_id.clone(),
                chunk_coord,
                chunk_loader_distance_squared,
                chunk_loader_radius_squared,
            });
        }

        for chunk_coord in chunks_to_unload {
            let chunk_loader_distance_squared = calculate_chunk_distance_from_owner(&chunk_coord.unscaled(), &world_pos_to_chunk(position));
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
