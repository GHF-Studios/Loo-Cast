// Imports
use bevy::prelude::*;
use std::collections::HashSet;

use crate::chunk::resources::ChunkManager;
use crate::chunk_loader::components::ChunkLoader;
use crate::chunk_loader::workflows::external::{load_chunks::LoadChunkInput, unload_chunks::UnloadChunkInput};
use crate::usf::pos::grid::types::GridVec;
use crate::usf::pos::components::OriginOffset;
use crate::usf::scale::Scale;
use crate::utils::lifecycle_hook::DropHook;

// Items

// Core Types
#[derive(bevy::ecs::system::SystemParam)]
pub struct MainAccess<'w, 's> {
    pub chunk_loader_query: Query<'w, 's, (&'static Transform, &'static ChunkLoader, Option<&'static DropHook<ChunkLoader>>)>,
    pub chunk_manager: Res<'w, ChunkManager>,
    pub origin_offset: Res<'w, OriginOffset>,
}
pub struct Output {
    pub load_chunk_inputs: Vec<LoadChunkInput>,
    pub unload_chunk_inputs: Vec<UnloadChunkInput>,
}

// Core Functions
pub fn run_ecs(main_access: MainAccess) -> Output {
    let chunk_loader_query = &main_access.chunk_loader_query;
    let chunk_manager = &main_access.chunk_manager;
    let origin_offset = main_access.origin_offset;

    let mut load_chunk_inputs = Vec::new();
    let mut unload_chunk_inputs = Vec::new();

    for (transform, chunk_loader, drop_hook) in chunk_loader_query.iter() {
        let chunk_loader_position = transform.translation.truncate();
        let chunk_loader_grid_coord = chunk_loader_position.to_grid_coord(*chunk_loader.id().scale(), origin_offset.0);
        let radius = chunk_loader.radius;

        let chunk_owner_id = chunk_loader.id();

        let target_chunks = if drop_hook.is_some() {
            HashSet::new()
        } else {
            let mut chunk_loader_scale_cursor = *chunk_loader.id().scale();
            let mut chunk_loader_grid_coord_cursor = chunk_loader_grid_coord.clone();
            let mut coords_in_cone = HashSet::new();

            while chunk_loader_scale_cursor != Scale::MAX {
                let coords_in_radius = chunk_loader_grid_coord_cursor.coords_in_radius(radius).into_iter().collect::<HashSet<GridVec>>();

                if !coords_in_cone.is_disjoint(&coords_in_radius) {
                    panic!("Duplicate coords!")
                }

                coords_in_cone.extend(coords_in_radius);
                chunk_loader_scale_cursor.zoom_out();
                chunk_loader_grid_coord_cursor.zoom_out();
            }

            coords_in_cone
        };

        let current_chunks: HashSet<GridVec> = chunk_manager
            .owned_chunks
            .iter()
            .filter_map(|(chunk, owner_id)| if owner_id == chunk_owner_id { Some(*chunk) } else { None })
            .collect();

        let chunks_to_load: Vec<_> = target_chunks.difference(&current_chunks).cloned().collect();
        let chunks_to_unload: Vec<_> = current_chunks.difference(&target_chunks).cloned().collect();

        for chunk_to_load in chunks_to_load {
            let chunk_loader_distance_squared = chunk_to_load.distance_squared(&chunk_loader_grid_coord);
            let chunk_loader_radius_squared = radius * radius;

            load_chunk_inputs.push(LoadChunkInput {
                owner_id: chunk_owner_id.clone(),
                grid_coord: chunk_to_load,
                chunk_loader_distance_squared: chunk_loader_distance_squared.try_into().unwrap(),
                chunk_loader_radius_squared,
            });
        }

        for chunk_to_unload in chunks_to_unload {
            let chunk_loader_distance_squared = chunk_to_unload.distance_squared(&chunk_loader_grid_coord);
            let chunk_loader_radius_squared = radius * radius;

            unload_chunk_inputs.push(UnloadChunkInput {
                owner_id: chunk_owner_id.clone(),
                grid_coord: chunk_to_unload,
                chunk_loader_distance_squared: chunk_loader_distance_squared.try_into().unwrap(),
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
