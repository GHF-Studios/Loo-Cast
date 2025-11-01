// Imports
use bevy::prelude::*;
use std::collections::HashSet;
use std::marker::PhantomData;

use crate::chunk::resources::ChunkManager;
use crate::chunk_loader::components::ChunkLoader;
use crate::chunk_loader::workflows::external::{load_chunks::LoadChunkInput, unload_chunks::UnloadChunkInput};
use crate::usf::pos::grid::types::GridVec;
use crate::usf::scale::Scale;
use crate::utils::lifecycle_hook::{DropHook, InitHook};

// Items

// Core Types
#[derive(bevy::ecs::system::SystemParam)]
pub struct MainAccess<'w, 's> {
    pub chunk_loader_query: Single<'w, (&'static Transform, &'static ChunkLoader, Option<&'static DropHook<ChunkLoader>>)>,
    pub chunk_manager: Res<'w, ChunkManager>,
    _phantom: PhantomData<&'s ()>,
}
pub struct Output {
    pub load_chunk_inputs: Vec<LoadChunkInput>,
    pub unload_chunk_inputs: Vec<UnloadChunkInput>,
}

// Core Functions
pub fn run_ecs(main_access: MainAccess) -> Output {
    let (transform, chunk_loader, drop_hook) = *main_access.chunk_loader_query;
    let chunk_manager = &main_access.chunk_manager;

    let mut load_chunk_inputs = Vec::new();
    let mut unload_chunk_inputs = Vec::new();

    let radius = chunk_loader.radius;

    let chunk_owner_id = chunk_loader.id();

    let target_chunk_cone = if drop_hook.is_some() {
        Vec::new()
    } else {
        let mut chunk_loader_scale_cursor = *chunk_loader.id().scale();
        let mut chunk_loader_grid_coord_cursor = &chunk_loader.origin_offset;
        let mut coords_in_cone = Vec::new();

        while chunk_loader_scale_cursor <= Scale::MAX {
            let coords_in_radius = chunk_loader_grid_coord_cursor.query_grid_radius(radius).into_iter().collect::<HashSet<GridVec>>();
            coords_in_cone.push((chunk_loader_grid_coord_cursor.clone(), coords_in_radius));

            if chunk_loader_scale_cursor != Scale::MAX {
                chunk_loader_scale_cursor.zoom_out();
                chunk_loader_grid_coord_cursor = &**chunk_loader_grid_coord_cursor.parent.as_ref().unwrap();
            }
        }

        coords_in_cone.reverse();
        coords_in_cone
    };

    let current_chunks: HashSet<GridVec> = chunk_manager
        .owned_chunks
        .iter()
        .filter_map(|(chunk, owner_id)| if owner_id == chunk_owner_id { Some(chunk.clone()) } else { None })
        .collect();

    for (chunk_loader_grid_coord, target_chunks) in target_chunk_cone {
        let chunks_to_load: Vec<_> = target_chunks.difference(&current_chunks).cloned().collect();
        let chunks_to_unload: Vec<_> = current_chunks.difference(&target_chunks).cloned().collect();

        for chunk_to_load in chunks_to_load {
            let chunk_loader_distance_squared = chunk_to_load.xy.distance_squared(chunk_loader_grid_coord.xy);
            let chunk_loader_radius_squared = radius * radius;

            load_chunk_inputs.push(LoadChunkInput {
                owner_id: chunk_owner_id.clone(),
                grid_coord: chunk_to_load,
                chunk_loader_distance_squared: chunk_loader_distance_squared.try_into().unwrap(),
                chunk_loader_radius_squared,
            });
        }

        for chunk_to_unload in chunks_to_unload {
            let chunk_loader_distance_squared = chunk_to_unload.xy.distance_squared(chunk_loader_grid_coord.xy);
            let chunk_loader_radius_squared = radius * radius;

            unload_chunk_inputs.push(UnloadChunkInput {
                owner_id: chunk_owner_id.clone(),
                grid_coord: chunk_to_unload,
                chunk_loader_distance_squared: chunk_loader_distance_squared.try_into().unwrap(),
                chunk_loader_radius_squared,
            });
        }
    }

    // warn!("Ran CategorizeChunks for {:?}", chunk_owner_id.id());

    Output {
        load_chunk_inputs,
        unload_chunk_inputs,
    }
}
