use bevy::prelude::*;
use std::collections::HashSet;

use crate::chunk::components::ChunkComponent;
use crate::chunk::functions::{calculate_chunks_in_radius, despawn_chunk, spawn_chunk};
use crate::chunk::resources::{ChunkRetryAction, ChunkRetryQueue};
use crate::chunk::statics::{CHUNK_OWNERSHIP, LOADED_CHUNKS, REQUESTED_CHUNK_ADDITIONS, REQUESTED_CHUNK_REMOVALS};

use super::components::ChunkLoaderComponent;

pub(in crate) fn update_chunk_loader_system(
    mut commands: Commands,
    chunk_loader_query: Query<(Entity, &Transform, &ChunkLoaderComponent)>,
    chunk_query: Query<(Entity, &ChunkComponent)>,
    mut retry_queue: ResMut<ChunkRetryQueue>, // Retry resource
) {
    for (loader_entity, transform, chunk_loader) in chunk_loader_query.iter() {
        let position = transform.translation.truncate();
        let radius = chunk_loader.radius;

        let target_chunks = calculate_chunks_in_radius(position, radius)
            .into_iter()
            .collect::<HashSet<(i32, i32)>>();

        let chunk_ownership = CHUNK_OWNERSHIP.lock().unwrap();
        let current_chunks: HashSet<(i32, i32)> = chunk_ownership
            .iter()
            .filter_map(|(chunk, &owner)| if owner == loader_entity { Some(*chunk) } else { None })
            .collect();
        drop(chunk_ownership);

        let chunks_to_spawn: Vec<&(i32, i32)> = target_chunks.difference(&current_chunks).collect();
        let chunks_to_despawn: Vec<&(i32, i32)> = current_chunks.difference(&target_chunks).collect();

        for &chunk_coord in chunks_to_spawn {
            if LOADED_CHUNKS.lock().unwrap().contains(&chunk_coord) {
                continue;
            }

            let result = spawn_chunk(
                &mut commands,
                REQUESTED_CHUNK_ADDITIONS.lock().unwrap(),
                REQUESTED_CHUNK_REMOVALS.lock().unwrap(),
                chunk_coord,
                loader_entity,
            );

            if let Err(err) = result {
                warn!("Failed to spawn chunk {:?}: {:?}. Retrying later.", chunk_coord, err);
                retry_queue.actions.push_back(ChunkRetryAction::Spawn {
                    chunk_coord,
                    chunk_owner: loader_entity,
                });
            }
        }

        for &chunk_coord in chunks_to_despawn {
            if !LOADED_CHUNKS.lock().unwrap().contains(&chunk_coord) {
                continue;
            }

            if let Some((chunk_entity, _)) = chunk_query.iter().find(|(_, chunk)| chunk.coord == chunk_coord) {
                let result = despawn_chunk(
                    &mut commands,
                    REQUESTED_CHUNK_ADDITIONS.lock().unwrap(),
                    REQUESTED_CHUNK_REMOVALS.lock().unwrap(),
                    chunk_coord,
                    chunk_entity,
                );

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
}

pub(in crate) fn process_chunk_retry_queue_system(
    mut commands: Commands,
    mut retry_queue: ResMut<ChunkRetryQueue>,
) {
    let mut successful_retries = Vec::new();

    while let Some(action) = retry_queue.actions.pop_front() {
        match action {
            ChunkRetryAction::Spawn { chunk_coord, chunk_owner } => {
                let result = spawn_chunk(
                    &mut commands,
                    REQUESTED_CHUNK_ADDITIONS.lock().unwrap(),
                    REQUESTED_CHUNK_REMOVALS.lock().unwrap(),
                    chunk_coord,
                    chunk_owner,
                );

                if result.is_err() {
                    retry_queue.actions.push_back(action); // Re-queue failed action
                } else {
                    successful_retries.push(action);
                }
            }
            ChunkRetryAction::Despawn { chunk_coord, chunk_entity } => {
                let result = despawn_chunk(
                    &mut commands,
                    REQUESTED_CHUNK_ADDITIONS.lock().unwrap(),
                    REQUESTED_CHUNK_REMOVALS.lock().unwrap(),
                    chunk_coord,
                    chunk_entity,
                );

                if result.is_err() {
                    retry_queue.actions.push_back(action); // Re-queue failed action
                } else {
                    successful_retries.push(action);
                }
            }
        }
    }
}