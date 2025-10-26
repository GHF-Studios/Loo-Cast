// Imports
use bevy::prelude::*;
use std::marker::PhantomData;

use crate::chunk::resources::{ChunkManager, GridOriginOffset};
use crate::chunk::traits::Vec2Ext;
use crate::chunk::types::{GridCoord, ChunkOwnerId};
use crate::chunk_loader::components::ChunkLoader;
use crate::chunk_loader::workflows::external::unload_chunks::UnloadChunkInput;

// Items

// Core Types
#[derive(bevy::ecs::system::SystemParam)]
pub struct MainAccess<'w, 's> {
    pub chunk_loader: Single<'w, &'static ChunkLoader>,
    pub chunk_manager: Res<'w, ChunkManager>,
    pub grid_origin_offset: Res<'w, GridOriginOffset>,
    pub phantom_data: PhantomData<&'s ()>,
}

pub struct Input {
    pub chunk_owner_id: ChunkOwnerId,
    pub chunk_loader_position: Vec2,
    pub chunk_loader_radius: u32,
}

pub struct Output {
    pub unload_chunk_inputs: Vec<UnloadChunkInput>,
}

// Core Functions
pub fn run_ecs(input: Input, main_access: MainAccess) -> Output {
    let chunk_manager = main_access.chunk_manager;
    let grid_origin_offset = main_access.grid_origin_offset;
    let chunk_loader = main_access.chunk_loader;

    let chunk_owner_id = input.chunk_owner_id;
    let position = input.chunk_loader_position;
    let chunk_loader_grid_extentition = position.to_grid_coord(*chunk_loader.id().scale(), grid_origin_offset.0);
    let radius = input.chunk_loader_radius;

    let mut unload_chunk_inputs = Vec::new();

    let chunks_to_despawn: Vec<&GridCoord> = chunk_manager
        .owned_chunks
        .iter()
        .filter_map(|(chunk, owner_id)| if owner_id == &chunk_owner_id { Some(chunk) } else { None })
        .collect();

    for chunk_to_despawn in chunks_to_despawn {
        let chunk_loader_distance_squared = chunk_to_despawn.xy.distance_squared(&chunk_loader_grid_extentition.xy).try_into().unwrap();
        let chunk_loader_radius_squared = radius * radius;

        unload_chunk_inputs.push(UnloadChunkInput {
            owner_id: chunk_owner_id.clone(),
            grid_coord: *chunk_to_despawn,
            chunk_loader_distance_squared,
            chunk_loader_radius_squared,
        });
    }

    warn!(
        "Ran OnRemoveChunkLoader for {:?} with # of unload targets: {}",
        chunk_owner_id.id(),
        unload_chunk_inputs.len()
    );

    Output { unload_chunk_inputs }
}
