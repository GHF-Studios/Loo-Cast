// Imports
use crate::bevy::prelude::*;

use crate::chunk::{
    components::{Chunk, ChunkLoader},
    resources::ChunkManager,
};
use crate::config::statics::CONFIG;
use crate::render::{
    components::{MainCamera, RenderProxyHandle},
    functions::new_sprite_proxy_bundle,
};
use crate::usf::{pos::grid::types::GridVec, scale::Scale};
use crate::workflow::types::Outcome;

// Items
pub struct SpawnChunkInput {
    pub grid_coord: GridVec,
    pub metric_texture: Handle<Image>,
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
    pub camera_transform: Single<'w, 's, &'static Transform, With<MainCamera>>,
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
    let mut skipped_outside_window = 0usize;
    let mut skipped_example = None;

    for input in input.inputs {
        let scale = input.grid_coord.scale;
        let grid_coord = input.grid_coord;
        let scale_diff = scale as i8 - chunk_loader.coord.scale as i8;
        if !(0_i8..=Scale::MAX_DIFF_SCALE_EXP).contains(&scale_diff) {
            skipped_outside_window += 1;
            skipped_example.get_or_insert((scale, chunk_loader.coord.scale, scale_diff));
            continue;
        }
        let logical_world_coord = grid_coord.clone().to_native_logical(chunk_loader.coord.clone());
        let (visual_world_coord, visual_world_scale) = grid_coord.clone().to_native_visual(chunk_loader.coord.clone());
        let metric_texture = input.metric_texture.clone();

        if chunk_query.iter().any(|chunk| chunk.coord == grid_coord) {
            return Err(Error::ChunkAlreadyLoaded { grid_coord });
        }

        let chunk_z_offset = CONFIG().get::<i8>("chunk/z_offset") as f32;
        let chunk_z = scale.compute_z() + chunk_z_offset;

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

        let chunk_render_proxy_entity = commands
            .spawn(new_sprite_proxy_bundle(
                metric_texture,
                visual_world_coord,
                visual_world_scale,
                chunk_entity,
                chunk_z,
            ))
            .id();

        commands.entity(chunk_entity).insert((
            chunk_transform,
            Chunk { coord: grid_coord.clone() },
            RenderProxyHandle {
                proxy_entity: chunk_render_proxy_entity,
            },
            chunk_name,
        ));

        chunk_manager.chunks.insert(grid_coord.clone());

        spawn_chunk_states.push(SpawnChunkState {
            chunk_entity,
            is_spawned: false,
        });
    }

    if skipped_outside_window > 0 {
        let (coord_scale, loader_scale, scale_diff) = skipped_example.unwrap();
        warn!(
            "Skipped {} chunk spawns outside viewport scale window (example: coord_scale={:?}, loader_scale={:?}, scale_diff={}, max_diff={})",
            skipped_outside_window,
            coord_scale,
            loader_scale,
            scale_diff,
            Scale::MAX_DIFF_SCALE_EXP
        );
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
