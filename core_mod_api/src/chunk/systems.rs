use bevy::prelude::*;
use core_mod_macros::{composite_workflow, composite_workflow_return};

use crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput;
use crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput;
use crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput;
use crate::chunk_loader::components::ChunkLoader;
use crate::chunk_loader::resources::RemovedChunkLoaders;
use crate::chunk_loader::types::ChunkLoaderId;
use crate::config::statics::CONFIG;
use crate::utils::lifecycle_hook::InitHook;
use crate::workflow::functions::handle_composite_workflow_return_now;

use super::components::Chunk;
use super::intent::ActionIntent;
use super::resources::ChunkRenderHandles;
use super::types::ChunkActionWorkflowHandles;
use super::ActionIntentCommitBuffer;

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
pub(crate) fn chunk_update_system(
    mut commands: Commands,
    chunk_query: Query<(Entity, &Chunk)>,
    removed_chunk_loaders: Res<RemovedChunkLoaders>,
) {
    for (entity, chunk) in chunk_query.iter() {
        if let Some(chunk_owner_id) = chunk.owner_id.clone() {
            if removed_chunk_loaders.0.iter().any(|rcl| rcl.id == chunk_owner_id) {
                commands.entity(entity).despawn();
            }
        } else {
            commands.entity(entity).despawn();
        }
    }
}

#[tracing::instrument(skip_all)]
pub(crate) fn process_chunk_actions_system(
    mut chunk_loader_init_hook_query: Query<&mut InitHook<ChunkLoader>>,
    mut action_intent_commit_buffer: ResMut<ActionIntentCommitBuffer>,
    mut workflow_handles: Local<Option<ChunkActionWorkflowHandles>>,
) {
    // Step 1: If workflows are running, wait for all to complete
    if let Some(handles) = &mut *workflow_handles {
        let spawn_done = handles.spawn.as_ref().is_none_or(|h| h.is_finished());
        let despawn_done = handles.despawn.as_ref().is_none_or(|h| h.is_finished());
        let transfer_done = handles.transfer.as_ref().is_none_or(|h| h.is_finished());

        if !spawn_done || !despawn_done || !transfer_done {
            //warn!(
            //    "Waiting for chunk action workflows to finish... spawn_done: {}, despawn_done: {}, transfer_done: {}",
            //    spawn_done, despawn_done, transfer_done
            //);
            return;
        }

        // Cleanup finished handles
        if let Some(handle) = handles.spawn.take() {
            handle_composite_workflow_return_now(handle, |ctx| {
                composite_workflow_return!(
                    new_chunk_loaders: Vec<Entity>,
                );

                for entity in new_chunk_loaders {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_query.get_mut(entity) {
                        init_hook.fire();
                    }
                }

                warn!("Finished composite workflow 'SpawnChunks'");
            });
        }
        if let Some(handle) = handles.despawn.take() {
            handle_composite_workflow_return_now(handle, |_ctx| {
                composite_workflow_return!();

                warn!("Finished composite workflow 'DespawnChunks'");
            });
        }
        if let Some(handle) = handles.transfer.take() {
            handle_composite_workflow_return_now(handle, |ctx| {
                composite_workflow_return!(
                    new_chunk_loaders: Vec<Entity>,
                );

                for entity in new_chunk_loaders {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_query.get_mut(entity) {
                        init_hook.fire();
                    }
                }

                warn!("Finished composite workflow 'TransferChunkOwnerships'");
            });
        }

        *workflow_handles = None;
    }

    // Step 2: Drain the buffer
    let mut processed_coords = vec![];
    let mut spawn_inputs = vec![];
    let mut spawn_coords = vec![];
    let mut despawn_inputs = vec![];
    let mut transfer_inputs = vec![];

    let mut chunk_loaders_performing_chunk_loads: Vec<ChunkLoaderId> = Vec::new();

    for (_, coords) in action_intent_commit_buffer.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffer.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffer)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords.push(coord.clone());
                    spawn_inputs.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        grid_coord: coord.clone(),
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords.push(coord);
                    chunk_loaders_performing_chunk_loads.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { grid_coord: coord.clone() });
                    processed_coords.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            grid_coord: coord.clone(),
                        },
                    );
                    processed_coords.push(coord);
                    chunk_loaders_performing_chunk_loads.push(new_owner_id);
                }
            }
        }
    }

    if spawn_inputs.is_empty() 
        && despawn_inputs.is_empty() 
        && transfer_inputs.is_empty()
    {
        // warn!("No chunk actions to process");
        return;
    }

    let mut new_chunk_loaders = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_query.get(*loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders.push(*loader_entity);
            }
        }
    }

    // Step 3: Build & launch composite workflows
    let spawn_handle = if !spawn_inputs.is_empty()
    {
        let param_data = spawn_coords
            .iter()
            .map(|coord| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [coord.xy.x.try_into().unwrap(), coord.xy.y.try_into().unwrap()],
                chunk_size: 1000,
                chunk_scale: coord.scale as i32,
                current_view_scale: 35,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();

        let new_chunk_loaders = new_chunk_loaders.clone();

        Some(composite_workflow!(
            SpawnChunks,
            move in spawn_inputs: Vec<SpawnChunkInput>,
            move in param_data: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            new_chunk_loaders: Vec<Entity>,
        {
            warn!("Running composite workflow 'SpawnChunks'");

            let shader_name = CONFIG().get::<&'static str>("chunk/texture_generator_shader");

            let generate_output = workflow!(IO, Gpu::GenerateChunkTextures, Input {
                shader_name,
                param_data,
            });

            let spawn_inputs_with_textures = spawn_inputs
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

    let despawn_handle = if !despawn_inputs.is_empty() {
        Some(composite_workflow!(
            DespawnChunks,
            move in despawn_inputs: Vec<DespawnChunkInput>,
        {
            warn!("Running composite workflow 'DespawnChunks'");

            let _ = workflow!(IOE, Chunk::DespawnChunks, Input {
                inner: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs },
            });
        }))
    } else {
        None
    };

    let transfer_handle = if !transfer_inputs.is_empty()
    {
        Some(composite_workflow!(
            TransferChunkOwnerships,
            move in transfer_inputs: Vec<TransferChunkOwnershipInput>,
            new_chunk_loaders: Vec<Entity>,
        {
            warn!("Running composite workflow 'TransferChunkOwnerships'");

            let _ = workflow!(IOE, Chunk::TransferChunkOwnerships, Input {
                inner: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs },
            });
        }))
    } else {
        None
    };

    *workflow_handles = Some(ChunkActionWorkflowHandles {
        spawn: spawn_handle,
        despawn: despawn_handle,
        transfer: transfer_handle,
    });

    // Step 4: Mark all these actions as in-progress (remove them from the commit buffer)
    action_intent_commit_buffer.remove_intents(processed_coords);
}
