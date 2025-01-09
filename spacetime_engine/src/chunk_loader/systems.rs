use bevy::prelude::*;
use std::collections::HashSet;

use crate::chunk::components::ChunkComponent;
use crate::chunk::enums::ChunkAction;
use crate::chunk::errors::SpawnError;
use crate::chunk::functions::{calculate_chunks_in_radius, despawn_chunk, spawn_chunk};
use crate::chunk::resources::ChunkManager;

use super::components::ChunkLoaderComponent;
use super::resources::ChunkLoaderActionBuffer;



pub(in crate) fn update_chunk_loader_system_NEW(
    mut commands: Commands,
    chunk_loader_query: Query<(Entity, &Transform, &ChunkLoaderComponent)>,
    chunk_query: Query<(Entity, &ChunkComponent)>,
    chunk_manager: Res<ChunkManager>,
    mut chunk_loader_action_buffer: ResMut<ChunkLoaderActionBuffer>,
) {
    for (loader_entity, transform, chunk_loader) in chunk_loader_query.iter() {
        let position = transform.translation.truncate();
        let radius = chunk_loader.radius;

        let target_chunks = calculate_chunks_in_radius(position, radius)
            .into_iter()
            .collect::<HashSet<(i32, i32)>>();

        let current_chunks: HashSet<(i32, i32)> = chunk_manager.get_owned_chunks()
            .iter()
            .filter_map(|(chunk, &owner)| if owner.unwrap() == loader_entity { Some(*chunk) } else { None })
            .collect();

        let potential_chunks_to_spawn: Vec<&(i32, i32)> = target_chunks.difference(&current_chunks).collect();
        let potential_chunks_to_despawn: Vec<&(i32, i32)> = current_chunks.difference(&target_chunks).collect();

        for chunk_coord in potential_chunks_to_spawn {
            let loaded = chunk_manager.is_loaded(chunk_coord);
            let owned = chunk_manager.is_owned(chunk_coord);
            let spawning = chunk_manager.is_spawning(chunk_coord);
            let despawning = chunk_manager.is_despawning(chunk_coord);
            let transfering_ownership = chunk_manager.is_transfering_ownership(chunk_coord);

            if !loaded {
                if !spawning && !despawning && !transfering_ownership { 
                    chunk_loader_action_buffer.0.insert(*chunk_coord, ChunkAction::SpawnChunk { coord: *chunk_coord });
                }
            } else if !owned && !despawning && !transfering_ownership {
                chunk_loader_action_buffer.0.insert(*chunk_coord, ChunkAction::TransferChunkOwnership { coord: *chunk_coord, new_owner: loader_entity });
            }
        }

        for chunk_coord in potential_chunks_to_despawn {
            let loaded = chunk_manager.is_loaded(chunk_coord);
            let despawning = chunk_manager.is_despawning(chunk_coord);

            if loaded && !despawning {
                chunk_loader_action_buffer.0.insert(*chunk_coord, ChunkAction::DespawnChunk { coord: *chunk_coord });
            }
        }
    }
}

pub(in crate) fn update_chunk_loader_system(
    mut commands: Commands,
    chunk_loader_query: Query<(Entity, &Transform, &ChunkLoaderComponent)>,
    chunk_query: Query<(Entity, &ChunkComponent)>,
    mut retry_queue: ResMut<ChunkRetryQueue>,
) {
    let chunk_ownership = CHUNK_OWNERSHIP.lock().unwrap();
    let loaded_chunks = LOADED_CHUNKS.lock().unwrap();
    let mut requested_chunk_additions = REQUESTED_CHUNK_ADDITIONS.lock().unwrap();
    let mut requested_chunk_removals = REQUESTED_CHUNK_REMOVALS.lock().unwrap();

    for (loader_entity, transform, chunk_loader) in chunk_loader_query.iter() {
        let position = transform.translation.truncate();
        let radius = chunk_loader.radius;

        let target_chunks = calculate_chunks_in_radius(position, radius)
            .into_iter()
            .collect::<HashSet<(i32, i32)>>();

        let current_chunks: HashSet<(i32, i32)> = chunk_ownership
            .iter()
            .filter_map(|(chunk, &owner)| if owner == loader_entity { Some(*chunk) } else { None })
            .collect();

        let chunks_to_spawn: Vec<&(i32, i32)> = target_chunks.difference(&current_chunks).collect();
        let chunks_to_despawn: Vec<&(i32, i32)> = current_chunks.difference(&target_chunks).collect();

        for &chunk_coord in chunks_to_spawn {
            // Perform pre-spawn checks
            if loaded_chunks.contains(&chunk_coord) {
                continue;
            }

            // Spawn the chunk
            let result = spawn_chunk(
                &mut commands,
                &mut requested_chunk_additions,
                &requested_chunk_removals,
                chunk_coord,
                loader_entity,
            );

            // Handle the result
            if let Err(err) = result {
                match err {
                    SpawnError::AlreadyBeingSpawned { .. } => {
                        panic!("Failed to spawn chunk {:?}: {:?}.", chunk_coord, err);
                    },
                    SpawnError::AlreadyBeingDespawned { .. } => {
                        debug!("Failed to spawn chunk {:?}: {:?}. Retrying later.", chunk_coord, err);
                        retry_queue.actions.push_back(ChunkRetryAction::Spawn {
                            chunk_coord,
                            chunk_owner: loader_entity,
                        });
                    }
                }
            }
        }

        for &chunk_coord in chunks_to_despawn {
            // Perform pre-spawn checks
            if !loaded_chunks.contains(&chunk_coord) {
                continue;
            }

            // Find the chunk entity
            let chunk_entity = match chunk_query.iter().find(|(_, chunk)| chunk.coord == chunk_coord) {
                Some((chunk_entity, _)) => chunk_entity,
                None => {
                    continue;
                }
            };

            // Despawn the chunk
            let result = despawn_chunk(
                &mut commands,
                &requested_chunk_additions,
                &mut requested_chunk_removals,
                chunk_coord,
                chunk_entity,
            );

            // Handle result
            if let Err(err) = result {
                warn!("Failed to despawn chunk {:?}: {:?}. Retrying later.", chunk_coord, err);
                retry_queue.actions.push_back(ChunkRetryAction::Despawn {
                    chunk_coord,
                    chunk_entity,
                });
            }
        }
    }
}

pub(in crate) fn process_chunk_retry_queue_system(
    mut commands: Commands,
    mut retry_queue: ResMut<ChunkRetryQueue>,
) {
    let mut requested_chunk_additions = REQUESTED_CHUNK_ADDITIONS.lock().unwrap();
    let mut requested_chunk_removals = REQUESTED_CHUNK_REMOVALS.lock().unwrap();

    let mut successful_retries = Vec::new();
    while let Some(action) = retry_queue.actions.pop_front() {
        match action {
            ChunkRetryAction::Spawn { chunk_coord, chunk_owner } => {
                let result = spawn_chunk(
                    &mut commands,
                    &mut requested_chunk_additions,
                    &requested_chunk_removals,
                    chunk_coord,
                    chunk_owner,
                );

                if result.is_err() {
                    retry_queue.actions.push_back(action);
                } else {
                    successful_retries.push(action);
                }
            }
            ChunkRetryAction::Despawn { chunk_coord, chunk_entity } => {
                let result = despawn_chunk(
                    &mut commands,
                    &requested_chunk_additions,
                    &mut requested_chunk_removals,
                    chunk_coord,
                    chunk_entity,
                );

                if result.is_err() {
                    retry_queue.actions.push_back(action);
                } else {
                    successful_retries.push(action);
                }
            }
        }
    }
}
