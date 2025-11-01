// Imports
use bevy::prelude::*;
use std::marker::PhantomData;

use crate::chunk::resources::ChunkManager;
use crate::chunk_loader::components::ChunkLoader;
use crate::chunk_loader::types::ChunkLoaderId;
use crate::chunk_loader::workflows::external::unload_chunks::UnloadChunkInput;
use crate::usf::pos::grid::types::GridVec;

// Items

// Core Types
#[derive(bevy::ecs::system::SystemParam)]
pub struct MainAccess<'w, 's> {
    pub chunk_loader: Single<'w, &'static ChunkLoader>,
    pub chunk_manager: Res<'w, ChunkManager>,
    pub phantom_data: PhantomData<&'s ()>,
}

pub struct Input {
    pub chunk_owner_id: ChunkLoaderId,
    pub chunk_loader_position: Vec2,
    pub chunk_loader_radius: u32,
}

pub struct Output {
    pub unload_chunk_inputs: Vec<UnloadChunkInput>,
}

// Core Functions
pub fn run_ecs(input: Input, main_access: MainAccess) -> Output {
    let chunk_manager = main_access.chunk_manager;
    let chunk_loader = *main_access.chunk_loader;

    let chunk_owner_id = input.chunk_owner_id;
    let position = input.chunk_loader_position;
    let radius = input.chunk_loader_radius;

    let mut unload_chunk_inputs = Vec::new();

    let chunks_to_despawn: Vec<&GridVec> = chunk_manager
        .owned_chunks
        .iter()
        .filter_map(|(chunk, owner_id)| if owner_id == &chunk_owner_id { Some(chunk) } else { None })
        .collect();

    for chunk_to_despawn in chunks_to_despawn {
        let chunk_loader_distance_squared = chunk_to_despawn.xy.distance_squared(chunk_loader.origin_offset.xy.clone()).try_into().unwrap();
        let chunk_loader_radius_squared = radius * radius;

        unload_chunk_inputs.push(UnloadChunkInput {
            owner_id: chunk_owner_id.clone(),
            grid_coord: chunk_to_despawn.clone(),
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
