use bevy::prelude::*;
use core_engine_macros::{composite_workflow, composite_workflow_return};

use crate::chunk::workflows::chunk::despawn_chunks::user_items::DespawnChunkInput;
use crate::chunk::workflows::chunk::spawn_chunks::user_items::SpawnChunkInput;
use crate::chunk::workflows::chunk::transfer_chunk_ownerships::user_items::TransferChunkOwnershipInput;
use crate::config::statics::CONFIG;
use crate::workflow::functions::handle_composite_workflow_return_now;

use super::components::ChunkComponent;
use super::enums::ChunkAction;
use super::functions::{chunk_pos_to_world, world_pos_to_chunk};
use super::resources::ChunkRenderHandles;
use super::types::ChunkActionWorkflowHandles;
use super::ChunkActionBuffer;

pub(crate) fn chunk_startup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let quad = meshes.add(Mesh::from(Rectangle::new(1.0, 1.0)));
    let light_material = materials.add(ColorMaterial::from_color(Color::srgb(0.75, 0.75, 0.75)));
    let dark_material = materials.add(ColorMaterial::from_color(Color::srgb(0.25, 0.25, 0.25)));

    commands.insert_resource(ChunkRenderHandles {
        quad,
        light_material,
        dark_material,
    });
}

pub(crate) fn chunk_update_system(chunk_query: Query<(Entity, &Transform, &ChunkComponent)>) {
    for (_, transform, chunk) in chunk_query.iter() {
        let world_pos = transform.translation.truncate();
        let chunk_pos = world_pos_to_chunk(world_pos);

        assert_eq!(chunk.coord, chunk_pos, "Attempted to move chunk entity");
        assert_eq!(
            chunk_pos_to_world(chunk.coord),
            world_pos,
            "Attempted to move chunk entity"
        );
    }
}

pub(crate) fn process_chunk_actions_system(
    mut chunk_action_buffer: ResMut<ChunkActionBuffer>,
    mut workflow_handles: Local<Option<ChunkActionWorkflowHandles>>,
) {
    let mut processed_coords = vec![];

    // Step 1: If workflows are running, wait for all to complete
    if let Some(handles) = &mut *workflow_handles {
        let spawn_done = handles.spawn.as_ref().map_or(true, |h| h.is_finished());
        let despawn_done = handles.despawn.as_ref().map_or(true, |h| h.is_finished());
        let transfer_done = handles.transfer.as_ref().map_or(true, |h| h.is_finished());

        if !spawn_done || !despawn_done || !transfer_done {
            return;
        }

        // Cleanup finished handles
        if let Some(handle) = handles.spawn.take() {
            handle_composite_workflow_return_now(handle, |_ctx| {
                composite_workflow_return!();
            });
        }
        if let Some(handle) = handles.despawn.take() {
            handle_composite_workflow_return_now(handle, |_ctx| {
                composite_workflow_return!();
            });
        }
        if let Some(handle) = handles.transfer.take() {
            handle_composite_workflow_return_now(handle, |_ctx| {
                composite_workflow_return!();
            });
        }

        *workflow_handles = None;
    }

    // Step 2: Drain the buffer
    let mut spawn_inputs = vec![];
    let mut spawn_coords = vec![];
    let mut despawn_inputs = vec![];
    let mut transfer_inputs = vec![];

    for (_, coords) in chunk_action_buffer.priority_buckets.iter() {
        for coord in coords {
            if let Some(action) = chunk_action_buffer.actions.get(coord).cloned() {
                match action {
                    ChunkAction::Spawn {
                        coord, new_owner, ..
                    } => {
                        spawn_coords.push(coord);
                        spawn_inputs.push(crate::chunk::workflows::chunk::spawn_chunks::user_items::SpawnChunkInput {
                            chunk_coord: coord,
                            chunk_owner: new_owner,
                            metric_texture: Default::default(), // placeholder
                        });
                        processed_coords.push(coord);
                    }
                    ChunkAction::Despawn { coord, .. } => {
                        despawn_inputs.push(crate::chunk::workflows::chunk::despawn_chunks::user_items::DespawnChunkInput {
                            chunk_coord: coord
                        });
                        processed_coords.push(coord);
                    }
                    ChunkAction::TransferOwnership {
                        coord, new_owner, ..
                    } => {
                        transfer_inputs.push(crate::chunk::workflows::chunk::transfer_chunk_ownerships::user_items::TransferChunkOwnershipInput {
                            chunk_coord: coord,
                            new_owner,
                        });
                        processed_coords.push(coord);
                    }
                }
            }
        }
    }

    if spawn_inputs.is_empty() && despawn_inputs.is_empty() && transfer_inputs.is_empty() {
        return; // Nothing to do
    }

    // Step 3: Build & launch composite workflows
    let spawn_handle = if !spawn_inputs.is_empty() {
        let texture_size = CONFIG.get::<f32>("chunk/size") as usize;
        let chunk_size = CONFIG.get::<f32>("chunk/size");

        let param_data = spawn_coords
            .iter()
            .map(|&(x, y)| {
                crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                    chunk_pos: [x, y],
                    chunk_size,
                    _padding: 0,
                }
            })
            .collect::<Vec<_>>();

        Some(composite_workflow!(
            move in texture_size: usize,
            move in spawn_inputs: Vec<SpawnChunkInput>,
            move in param_data: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in spawn_coords: Vec<(i32, i32)>,
        {
            let generate_output = workflow!(IO, Gpu::GenerateTextures, Input {
                shader_name: "texture_generators/example_compute_uv",
                texture_size,
                param_data,
            });

            let spawn_inputs_with_textures = spawn_coords.into_iter()
                .zip(generate_output.texture_handles.into_iter())
                .map(|(coord, tex)| crate::chunk::workflows::chunk::spawn_chunks::user_items::SpawnChunkInput {
                    chunk_coord: coord,
                    chunk_owner: spawn_inputs.iter().find(|i| i.chunk_coord == coord).unwrap().chunk_owner,
                    metric_texture: tex,
                })
                .collect::<Vec<_>>();

            let _ = workflow!(IOE, Chunk::SpawnChunks, Input {
                inputs: spawn_inputs_with_textures
            });
        }))
    } else {
        None
    };

    let despawn_handle = if !despawn_inputs.is_empty() {
        Some(
            composite_workflow!(move in despawn_inputs: Vec<DespawnChunkInput>, {
                let _ = workflow!(IOE, Chunk::DespawnChunks, Input {
                    inputs: despawn_inputs
                });
            }),
        )
    } else {
        None
    };

    let transfer_handle = if !transfer_inputs.is_empty() {
        Some(
            composite_workflow!(move in transfer_inputs: Vec<TransferChunkOwnershipInput>, {
                let _ = workflow!(IOE, Chunk::TransferChunkOwnerships, Input {
                    inputs: transfer_inputs
                });
            }),
        )
    } else {
        None
    };

    *workflow_handles = Some(ChunkActionWorkflowHandles {
        spawn: spawn_handle,
        despawn: despawn_handle,
        transfer: transfer_handle,
    });

    // Step 4: Mark all these actions as in-progress (done now)
    chunk_action_buffer.remove_actions(processed_coords);
}
