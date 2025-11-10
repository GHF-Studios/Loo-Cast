// Imports
use bevy::prelude::*;

use crate::camera::components::MainCamera;
use crate::chunk::{components::Chunk, resources::ChunkManager};
use crate::chunk_loader::components::ChunkLoader;
use crate::chunk_loader::types::ChunkLoaderId;
use crate::config::statics::CONFIG;
use crate::render::components::RenderProxyHandle;
use crate::render::functions::make_sprite_proxy_bundle;
use crate::usf::pos::grid::types::GridVec;
use crate::usf::scale::Scale;
use crate::workflow::types::Outcome;

// Items
pub struct SpawnChunkInput {
    pub grid_coord: GridVec,
    pub chunk_owner_id: ChunkLoaderId,
    pub metric_texture: Handle<Image>
}

#[derive(Clone)]
pub struct SpawnChunkState {
    pub chunk_entity: Entity,
    pub is_spawned: bool,
}

// Core Types
#[derive(bevy::ecs::system::SystemParam)]
pub struct MainAccess<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub chunk_query: Query<'w, 's, &'static Chunk>,
    pub chunk_loader_query: Single<'w, &'static ChunkLoader>,
    pub chunk_manager: ResMut<'w, ChunkManager>,
    pub camera_transform: Single<'w, &'static Transform, With<MainCamera>>,
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
    let chunk_loader = main_access.chunk_loader_query;
    let mut chunk_manager = main_access.chunk_manager;
    let _camera_transform = main_access.camera_transform;

    let mut spawn_chunk_states = Vec::new();

    for input in input.inputs {
        let scale = input.grid_coord.scale;
        let grid_coord = input.grid_coord;
        let logical_world_coord = grid_coord.clone().to_native_logical(chunk_loader.origin_offset.clone());
        let (visual_world_coord, visual_world_scale) = grid_coord.clone().to_native_visual(chunk_loader.origin_offset.clone());
        let chunk_owner_id = input.chunk_owner_id;
        let metric_texture = input.metric_texture.clone();

        if chunk_query.iter().any(|chunk| chunk.coord == grid_coord) {
            return Err(Error::ChunkAlreadyLoaded { grid_coord });
        }

        let chunk_z_offset = CONFIG().get::<i8>("chunk/z_offset");
        let chunk_z = (-(Scale::MAX as i8 - scale as i8) + chunk_z_offset) as f32;

        // let camera_pos = camera_transform.translation.truncate();
        // let camera_grid_extent = camera_pos.to_grid_coord(origin_offset.0);
        // let camera_world_coord = camera_pos.to_world_coord(scale, camera_grid_extent);
        // println!("camera_world_coord: {:?}", camera_world_coord);

        let chunk_transform = Transform {
            translation: logical_world_coord.extend(chunk_z),
            ..Default::default()
        };

        let chunk_name = Name::new(format!("chunk_entity({grid_coord:?})"));

        let chunk_entity = commands.spawn(()).id();

        let chunk_render_proxy_entity = commands.spawn(make_sprite_proxy_bundle(
            metric_texture,
            visual_world_coord,
            visual_world_scale,
            chunk_entity,
            chunk_z,
        )).id();

        commands.entity(chunk_entity).insert((
            chunk_transform,
            Chunk {
                coord: grid_coord.clone(),
                owner_id: Some(chunk_owner_id.clone()),
            },
            RenderProxyHandle {
                proxy_entity: chunk_render_proxy_entity,
            },
            chunk_name
        ));

        chunk_manager.loaded_chunks.insert(grid_coord.clone());
        chunk_manager.owned_chunks.insert(grid_coord, chunk_owner_id.clone());

        spawn_chunk_states.push(SpawnChunkState {
            chunk_entity,
            is_spawned: false,
        });
    }

    Ok(State {
        spawn_chunk_states
    })
}

pub fn run_ecs_while(state: State, main_access: MainAccess) -> Result<Outcome<State, Output>, Error> {
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

        // warn!("All chunks spawned.");

        Ok(Outcome::Done(Output {
            spawned_chunk_entities
        }))
    } else {
        // let done_count = spawn_chunk_states.iter().filter(|spawn_chunk_state| spawn_chunk_state.is_spawned).count();
        // warn!("Waiting for chunks to spawn... {}/{}", done_count, spawn_chunk_states.len());

        Ok(Outcome::Wait(State {
            spawn_chunk_states
        }))
    }
}