// Imports
use bevy::prelude::*;
use std::marker::PhantomData;

use crate::chunk::components::Chunk;
use crate::chunk::functions::*;
use crate::chunk::intent::ActionIntent;
use crate::chunk::resources::{ActionIntentCommitBuffer, ChunkManager};
use crate::chunk::types::ChunkOwnerId;
use crate::chunk_loader::components::ChunkLoader;
use crate::chunk_loader::workflows::external::unload_chunks::UnloadChunkInput;
use crate::usf::scale::Scale;

// Items

// Core Types
#[derive(bevy::ecs::system::SystemParam)]
pub struct MainAccess<'w, 's, S: Scale> {
    pub chunk_manager: Res<'w, ChunkManager<S>>,
    pub phantom_data: PhantomData<&'s ()>
}

pub struct Input<S: Scale> {
    pub chunk_owner_id: ChunkOwnerId<S>,
    pub chunk_loader_position: Vec2,
    pub chunk_loader_radius: u32,
}

pub struct Output<S: Scale> {
    pub unload_chunk_inputs: Vec<UnloadChunkInput<S>>,
}

// Core Functions
pub fn run_ecs<S: Scale>(input: Input<S>, main_access: MainAccess<S>) -> Output<S> {
    let chunk_manager = main_access.chunk_manager;

    let chunk_owner_id = input.chunk_owner_id;
    let position = input.chunk_loader_position;
    let radius = input.chunk_loader_radius;

    let mut unload_chunk_inputs = Vec::new();

    let chunks_to_despawn: Vec<&(i32, i32)> = chunk_manager
        .owned_chunks
        .iter()
        .filter_map(|(chunk, owner_id)| if owner_id == &chunk_owner_id { Some(chunk) } else { None })
        .collect();

    for chunk_coord in chunks_to_despawn {
        let chunk_loader_distance_squared = calculate_chunk_distance_from_owner(chunk_coord, &world_pos_to_chunk(position));
        let chunk_loader_radius_squared = radius * radius;

        unload_chunk_inputs.push(UnloadChunkInput {
            owner_id: chunk_owner_id.clone(),
            chunk_coord: *chunk_coord,
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
