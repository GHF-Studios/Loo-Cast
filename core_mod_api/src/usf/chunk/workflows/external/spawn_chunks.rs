// Imports
use crate::bevy::prelude::*;

use crate::usf::chunk::{
    components::{Chunk, ChunkDebugWireframe, ChunkLoader},
    resources::ChunkManager,
};
use crate::usf::pos::grid::types::GridVec;
use crate::workflow::types::Outcome;

// Items
pub struct SpawnChunkInput {
    pub grid_coord: GridVec,
}

#[derive(Clone)]
pub struct SpawnChunkState {
    pub chunk_entity: Entity,
    pub is_spawned: bool,
}

// Core Types
#[derive(crate::bevy::ecs::system::SystemParam)]
pub struct MainAccess<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub chunk_query: Query<'w, 's, &'static Chunk>,
    pub chunk_loader_query: Single<'w, 's, &'static ChunkLoader>,
    pub chunk_manager: ResMut<'w, ChunkManager>,
}

pub struct Input {
    pub inputs: Vec<SpawnChunkInput>,
}

pub struct State {
    pub spawn_chunk_states: Vec<SpawnChunkState>,
}

pub struct Output {
    pub spawned_chunk_entities: Vec<Entity>,
}

#[derive(Debug)]
pub enum Error {
    ChunkAlreadyLoaded { grid_coord: GridVec },
}

// Core Functions
pub fn setup_ecs_while(input: Input, main_access: MainAccess) -> Result<State, Error> {
    let mut commands = main_access.commands;
    let chunk_query = main_access.chunk_query;
    let _chunk_loader = main_access.chunk_loader_query;
    let mut chunk_manager = main_access.chunk_manager;

    let mut spawn_chunk_states = Vec::new();

    for input in input.inputs {
        let grid_coord = input.grid_coord;

        if chunk_query.iter().any(|chunk| chunk.coord == grid_coord) {
            return Err(Error::ChunkAlreadyLoaded { grid_coord });
        }

        // let camera_pos = camera_transform.translation.truncate();
        // let camera_grid_extent = camera_pos.to_grid_coord(origin_offset.0);
        // let camera_world_coord = camera_pos.to_world_coord(scale, camera_grid_extent);
        // println!("camera_world_coord: {:?}", camera_world_coord);

        // Under the proxy contract, root transforms are intentionally non-authoritative.
        let chunk_transform = Transform::default();

        let chunk_name = Name::new(format!("chunk_entity({grid_coord:?})"));

        let chunk_entity = commands
            .spawn((chunk_transform, Chunk { coord: grid_coord.clone() }, ChunkDebugWireframe, chunk_name))
            .id();

        chunk_manager.chunks.insert(grid_coord.clone());

        spawn_chunk_states.push(SpawnChunkState {
            chunk_entity,
            is_spawned: false,
        });
    }

    Ok(State { spawn_chunk_states })
}

pub fn run_ecs_while(state: State, main_access: MainAccess) -> Result<Outcome<State, Output>, Error> {
    let mut commands = main_access.commands;

    let spawn_chunk_states = state
        .spawn_chunk_states
        .into_iter()
        .map(|mut spawn_chunk_state| {
            if commands.get_entity(spawn_chunk_state.chunk_entity).is_ok() {
                spawn_chunk_state.is_spawned = true;
            }

            spawn_chunk_state
        })
        .collect::<Vec<_>>();
    let is_done = spawn_chunk_states.iter().all(|spawn_chunk_state| spawn_chunk_state.is_spawned);

    if is_done {
        let spawned_chunk_entities = spawn_chunk_states.into_iter().map(|spawn_chunk_state| spawn_chunk_state.chunk_entity).collect();

        // warn!("All chunks spawned.");

        Ok(Outcome::Done(Output { spawned_chunk_entities }))
    } else {
        // let done_count = spawn_chunk_states.iter().filter(|spawn_chunk_state| spawn_chunk_state.is_spawned).count();
        // warn!("Waiting for chunks to spawn... {}/{}", done_count, spawn_chunk_states.len());

        Ok(Outcome::Wait(State { spawn_chunk_states }))
    }
}
