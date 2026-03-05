use crate::bevy::prelude::*;
use core_mod_macros::{composite_workflow, composite_workflow_return};
use std::collections::HashSet;

use crate::chunk::components::ChunkLoader;
use crate::chunk::enums::ZoomState;
use crate::chunk::resources::ChunkManager;
use crate::chunk::types::ChunkActionWorkflowHandles;
use crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput;
use crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput;
use crate::config::statics::CONFIG;
use crate::usf::pos::grid::types::GridVec;
use crate::usf::scale::Scale;
use crate::workflow::functions::handle_composite_workflow_return_now;

use super::resources::ChunkRenderHandles;

#[tracing::instrument(skip_all)]
pub(crate) fn chunk_startup_system(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let quad = meshes.add(Mesh::from(Rectangle::new(1.0, 1.0)));
    let light_material: Handle<ColorMaterial> = materials.add(ColorMaterial::from_color(Color::srgb(0.75, 0.75, 0.75)));
    let dark_material = materials.add(ColorMaterial::from_color(Color::srgb(0.25, 0.25, 0.25)));

    commands.insert_resource(ChunkRenderHandles {
        quad,
        light_material,
        dark_material,
    });
}

#[tracing::instrument(skip_all)]
pub(crate) fn chunk_zoom_cooldown_system(time: Res<Time<Virtual>>, mut timer: Local<f32>, mut query: Query<&mut ChunkLoader>) {
    if *timer > 0.0 {
        *timer -= time.delta_secs();
        if *timer < 0.0 {
            *timer = 0.0;
        }
    }

    for mut chunk_loader in query.iter_mut() {
        if chunk_loader.zoom_state != ZoomState::None && *timer == 0.0 {
            chunk_loader.zoom_state = ZoomState::None;
            *timer = CONFIG().get::<f32>("chunk_loader/zoom_cooldown_secs");
        }
    }
}

#[tracing::instrument(skip_all)]
pub(crate) fn chunk_detection_system(
    chunk_loader_query: Query<&ChunkLoader>,
    chunk_manager: Res<ChunkManager>,
) -> (Vec<SpawnChunkInput>, Vec<DespawnChunkInput>) {
    let chunk_loader = match chunk_loader_query.single() {
        Ok(data) => data,
        Err(_) => {
            // warn!("No ChunkLoader found in the world; skipping chunk detection. Despawning all chunks.");
            return (Vec::new(), chunk_manager.chunks.iter().cloned().map(DespawnChunkInput::new).collect());
        }
    };

    let mut spawn_chunk_inputs = Vec::new();
    let mut despawn_chunk_inputs = Vec::new();
    let radius = chunk_manager.load_radius;
    let current_chunks = chunk_manager.chunks.clone();
    let mut chunk_loader_grid_coord_cursor = &chunk_loader.coord;
    let mut target_chunk_cone = Vec::new();

    // warn!("Starting Chunk Detection with current Chunks: {:?}", current_chunks);

    loop {
        let current_scale = chunk_loader_grid_coord_cursor.scale;
        let scale_diff = current_scale as i8 - chunk_loader.coord.scale as i8;
        if scale_diff > Scale::MAX_DIFF_SCALE_EXP {
            break;
        }

        // warn!("Chunk Detection at scale: {:?}", current_scale);
        let coords_in_radius = chunk_loader_grid_coord_cursor
            .query_grid_radius(radius)
            .into_iter()
            .collect::<HashSet<GridVec>>();
        // warn!("Detected Chunks: {:?}", coords_in_radius);
        target_chunk_cone.push((chunk_loader_grid_coord_cursor.clone(), coords_in_radius));

        if current_scale == Scale::MAX {
            break;
        }

        let Some(parent) = chunk_loader_grid_coord_cursor.parent.as_ref() else {
            break;
        };
        chunk_loader_grid_coord_cursor = parent;
    }

    target_chunk_cone.reverse();

    // for (chunk_loader_grid_coord, target_chunks) in &target_chunk_cone {
    //     warn!("Target Chunks at scale: {:?} => {{{}}} {:?}", chunk_loader_grid_coord.scale, target_chunks.len(), target_chunks);
    // }

    let mut final_target_chunks: HashSet<GridVec> = HashSet::new();

    for (_coord, target_chunks) in &target_chunk_cone {
        final_target_chunks.extend(target_chunks.iter().cloned());
    }

    let chunks_to_load = final_target_chunks.difference(&current_chunks).cloned();

    let chunks_to_unload = current_chunks.difference(&final_target_chunks).cloned();

    for chunk in chunks_to_load {
        spawn_chunk_inputs.push(SpawnChunkInput {
            grid_coord: chunk,
            metric_texture: Handle::default(),
        });
    }

    for chunk in chunks_to_unload {
        despawn_chunk_inputs.push(DespawnChunkInput { grid_coord: chunk });
    }

    // if !despawn_chunk_inputs.is_empty() {
    //     error!(
    //         "DespawnChunk-Inputs detected: {:?}",
    //         despawn_chunk_inputs.iter().map(|input| input.grid_coord.clone()).collect::<Vec<_>>()
    //     );
    // }

    // We now have `spawn_chunk_inputs` and `despawn_chunk_inputs` populated and ready to be used by the chunk management system

    (spawn_chunk_inputs, despawn_chunk_inputs)
}

#[tracing::instrument(skip_all)]
pub(crate) fn chunk_management_system(
    In(inputs): In<(Vec<SpawnChunkInput>, Vec<DespawnChunkInput>)>,
    mut workflow_handles: Local<Option<ChunkActionWorkflowHandles>>,
) {
    let (spawn_chunk_inputs, despawn_chunk_inputs) = inputs;

    // Step 1: If workflows are running, wait for all to complete
    if let Some(handles) = &mut *workflow_handles {
        let spawn_done = handles.spawn.as_ref().is_none_or(|h| h.is_finished());
        let despawn_done = handles.despawn.as_ref().is_none_or(|h| h.is_finished());

        if !spawn_done || !despawn_done {
            //warn!(
            //    "Waiting for chunk action workflows to finish... spawn_done: {}, despawn_done: {}",
            //    spawn_done, despawn_done
            //);
            return;
        }

        // Cleanup finished handles
        if let Some(handle) = handles.spawn.take() {
            handle_composite_workflow_return_now(handle, |_ctx| {
                composite_workflow_return!();

                warn!("Finished composite workflow 'SpawnChunks'");
            });
        }
        if let Some(handle) = handles.despawn.take() {
            handle_composite_workflow_return_now(handle, |_ctx| {
                composite_workflow_return!();

                warn!("Finished composite workflow 'DespawnChunks'");
            });
        }

        *workflow_handles = None;
    }

    if spawn_chunk_inputs.is_empty() && despawn_chunk_inputs.is_empty() {
        // warn!("No chunk actions to process");
        return;
    }

    if !spawn_chunk_inputs.is_empty() || !despawn_chunk_inputs.is_empty() {
        // warn!(
        //     "Chunk Detection: To Load: {:?}, To Unload: {:?}",
        //     spawn_chunk_inputs.iter().map(|input| input.grid_coord.clone()).collect::<Vec<_>>(),
        //     despawn_chunk_inputs.iter().map(|input| input.grid_coord.clone()).collect::<Vec<_>>()
        // );
    }

    // Step 2: Build & launch composite workflows
    let spawn_handle = if !spawn_chunk_inputs.is_empty() {
        let param_data = spawn_chunk_inputs
            .iter()
            .map(|input| {
                let coord = input.grid_coord.clone();

                crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                    chunk_pos: [coord.xy.x, coord.xy.y],
                    chunk_size: 1000,
                    chunk_scale: coord.scale as i32,
                    current_view_scale: 35,
                    _padding0: 0,
                    _padding1: [0, 0, 0, 0],
                }
            })
            .collect::<Vec<_>>();

        Some(composite_workflow!(
            SpawnChunks,
            move in spawn_chunk_inputs: Vec<SpawnChunkInput>,
            move in param_data: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
        {
            warn!("Running composite workflow 'SpawnChunks'");

            let shader_name = CONFIG().get::<&'static str>("chunk/texture_generator_shader");

            let generate_output = workflow!(IO, Gpu::GenerateChunkTextures, Input {
                shader_name,
                param_data,
            });

            let spawn_inputs_with_textures = spawn_chunk_inputs
                .into_iter()
                .zip(generate_output.render_executor.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();

            warn!("Finished GenerateChunkTextures workflow execution");

            let _ = workflow!(IOE, Chunk::SpawnChunks, Input {
                inner: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures },
            });
        }))
    } else {
        None
    };

    let despawn_handle = if !despawn_chunk_inputs.is_empty() {
        Some(composite_workflow!(
            DespawnChunks,
            move in despawn_chunk_inputs: Vec<DespawnChunkInput>,
        {
            warn!("Running composite workflow 'DespawnChunks'");

            let _ = workflow!(IOE, Chunk::DespawnChunks, Input {
                inner: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_chunk_inputs },
            });
        }))
    } else {
        None
    };

    *workflow_handles = Some(ChunkActionWorkflowHandles {
        spawn: spawn_handle,
        despawn: despawn_handle,
    });
}
