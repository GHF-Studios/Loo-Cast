// Imports
use bevy::prelude::{Commands, Entity, Query, ResMut, Handle, Image, Transform, Sprite, Name, warn};

use crate::chunk::{components::Chunk, resources::ChunkManager, functions::chunk_pos_to_world, types::ChunkOwnerId};
use crate::config::statics::CONFIG;
use crate::debug::observers::on_click_select;
use crate::usf::scale::Scale;
use crate::workflow::types::Outcome;

// Items
pub struct SpawnChunkInput<S: Scale> {
    pub chunk_coord: (i32, i32),
    pub chunk_owner_id: ChunkOwnerId<S>,
    pub metric_texture: Handle<Image>
}

#[derive(Clone)]
pub struct SpawnChunkState {
    pub chunk_entity: Entity,
    pub is_spawned: bool,
}

// Core Types
#[derive(bevy::ecs::system::SystemParam)]
pub struct MainAccess<'w, 's, S: Scale> {
    pub commands: Commands<'w, 's>,
    pub chunk_query: Query<'w, 's, &'static Chunk<S>>,
    pub chunk_manager: ResMut<'w, ChunkManager<S>>,
}

pub struct Input<S: Scale> {
    pub inputs: Vec<SpawnChunkInput<S>>,
}

pub struct State {
    pub spawn_chunk_states: Vec<SpawnChunkState>,
}

pub struct Output {
    pub spawned_chunk_entities: Vec<Entity>,
}

#[derive(Debug)]
pub enum Error {
    ChunkAlreadyLoaded { chunk_coord: (i32, i32) },
}

// Core Functions
pub fn setup_ecs_while<S: Scale>(input: Input<S>, main_access: MainAccess<S>) -> Result<State, Error> {
    let mut commands = main_access.commands;
    let chunk_query = main_access.chunk_query;
    let mut chunk_manager = main_access.chunk_manager;

    let mut spawn_chunk_states = Vec::new();

    for input in input.inputs {
        let chunk_coord = input.chunk_coord;
        let chunk_owner_id = input.chunk_owner_id;
        let metric_texture = input.metric_texture.clone();

        if chunk_query.iter().any(|chunk| chunk.coord == chunk_coord) {
            return Err(Error::ChunkAlreadyLoaded { chunk_coord });
        }

        let default_chunk_z = CONFIG().get::<f32>("chunk/default_z");

        let chunk_transform = Transform {
            translation: chunk_pos_to_world(chunk_coord).extend(default_chunk_z),
            ..Default::default()
        };

        let chunk_name = Name::new(format!("chunk_entity({}, {})", chunk_coord.0, chunk_coord.1));

        let chunk_entity = commands.spawn((
            chunk_transform,
            Sprite {
                image: metric_texture,
                ..Default::default()
            },
            Chunk {
                coord: chunk_coord,
                owner_id: Some(chunk_owner_id.clone()),
                phantom_scale: std::marker::PhantomData,
            },
            chunk_name
        )).observe(on_click_select).id();

        // warn!("Spawning chunk at coord ({}, {})", chunk_coord.0, chunk_coord.1);

        chunk_manager.loaded_chunks.insert(chunk_coord);
        chunk_manager.owned_chunks.insert(chunk_coord, chunk_owner_id.clone());

        spawn_chunk_states.push(SpawnChunkState {
            chunk_entity,
            is_spawned: false,
        });
    }

    warn!("Spawning {} chunks", spawn_chunk_states.len());

    Ok(State {
        spawn_chunk_states
    })
}

pub fn run_ecs_while<S: Scale>(state: State, main_access: MainAccess<S>) -> Result<Outcome<State, Output>, Error> {
    let mut commands = main_access.commands;

    let spawn_chunk_states = state.spawn_chunk_states.into_iter().map(|mut spawn_chunk_state| {
        if commands.get_entity(spawn_chunk_state.chunk_entity).is_ok() {
            spawn_chunk_state.is_spawned = true;
        }

        spawn_chunk_state
    }).collect::<Vec<_>>();
    let is_done = spawn_chunk_states.iter().all(|spawn_chunk_state| spawn_chunk_state.is_spawned);

    if is_done {
        let spawned_chunk_entities = spawn_chunk_states.into_iter().map(|spawn_chunk_state| spawn_chunk_state.chunk_entity).collect();

        warn!("All chunks spawned.");

        Ok(Outcome::Done(Output {
            spawned_chunk_entities
        }))
    } else {
        let done_count = spawn_chunk_states.iter().filter(|spawn_chunk_state| spawn_chunk_state.is_spawned).count();
        warn!("Waiting for chunks to spawn... {}/{}", done_count, spawn_chunk_states.len());

        Ok(Outcome::Wait(State {
            spawn_chunk_states
        }))
    }
}