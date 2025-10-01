// Imports
use bevy::prelude::{ResMut, Commands, Query, Entity};

use crate::chunk::{components::Chunk, resources::ChunkManager, types::GridCoord};
use crate::workflow::types::Outcome;

// Items
pub struct DespawnChunkInput {
    pub grid_coord: GridCoord
}
#[derive(Clone)]
pub struct DespawnChunkState {
    pub entity: Entity,
    pub is_despawned: bool,
}

// Core Types
#[derive(bevy::ecs::system::SystemParam)]
pub struct MainAccess<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub chunk_query: Query<'w, 's, (Entity, &'static Chunk)>,
    pub chunk_manager: ResMut<'w, ChunkManager>,
}

pub struct Input {
    pub inputs: Vec<DespawnChunkInput>,
}

pub struct State {
    pub despawn_chunk_states: Vec<DespawnChunkState>,
}

pub struct Output {
    pub despawned_chunk_entities: Vec<Entity>,
}

#[derive(Debug)]
pub enum Error {
    ChunkNotLoaded { grid_coord: GridCoord },
}

// Core Functions
pub fn setup_ecs_while(input: Input, main_access: MainAccess) -> Result<State, Error> {
    let mut commands = main_access.commands;
    let chunk_query = main_access.chunk_query;
    let mut chunk_manager = main_access.chunk_manager;

    let mut despawn_chunk_states = Vec::new();

    for input in input.inputs {
        let grid_coord = input.grid_coord;

        if let Some((entity, _)) = chunk_query.iter().find(|(_, chunk)| chunk.coord == grid_coord) {
            chunk_manager.loaded_chunks.remove(&grid_coord);
            chunk_manager.owned_chunks.remove(&grid_coord);

            let mut chunk_entity_commands = commands.entity(entity);
            despawn_chunk_states.push(DespawnChunkState {
                entity: chunk_entity_commands.id(),
                is_despawned: false,
            });
            chunk_entity_commands.despawn();
        } else {
            return Err(Error::ChunkNotLoaded { grid_coord });
        }
    }

    Ok(State {
        despawn_chunk_states
    })
}

pub fn run_ecs_while(state: State, main_access: MainAccess) -> Result<Outcome<State, Output>, Error> {
    let mut commands = main_access.commands;

    let despawn_chunk_states = state.despawn_chunk_states.into_iter().map(|mut despawn_chunk_state| {
        if commands.get_entity(despawn_chunk_state.entity).is_err() {
            despawn_chunk_state.is_despawned = true;
        }

        despawn_chunk_state
    }).collect::<Vec<_>>();
    let is_done = despawn_chunk_states.iter().all(|despawn_chunk_state| despawn_chunk_state.is_despawned);

    if is_done {
        let despawned_chunk_entities = despawn_chunk_states.into_iter().map(|despawn_chunk_state| despawn_chunk_state.entity).collect();

        Ok(Outcome::Done(Output {
            despawned_chunk_entities
        }))
    } else {
        Ok(Outcome::Wait(State {
            despawn_chunk_states
        }))
    }
}