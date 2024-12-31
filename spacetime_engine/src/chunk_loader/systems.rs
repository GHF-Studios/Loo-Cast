use bevy::prelude::*;
use std::collections::HashSet;

use crate::chunk::components::ChunkComponent;
use crate::chunk::functions::{calculate_chunks_in_radius, despawn_chunk, spawn_chunk};
use crate::chunk::statics::{CHUNK_OWNERSHIP, LOADED_CHUNKS, REQUESTED_CHUNK_ADDITIONS, REQUESTED_CHUNK_REMOVALS};

use super::components::ChunkLoaderComponent;

pub(in crate) fn update_chunk_loader_system(
    mut commands: Commands,
    chunk_loader_query: Query<(Entity, &Transform, &ChunkLoaderComponent)>,
    chunk_query: Query<(Entity, &ChunkComponent)>,
) {
    for (loader_entity, transform, chunk_loader) in chunk_loader_query.iter() {
        let position = transform.translation.truncate(); // 2D position
        let radius = chunk_loader.radius;

        let target_chunks = calculate_chunks_in_radius(position, radius)
            .into_iter()
            .collect::<HashSet<(i32, i32)>>();

        // Track chunks that should be loaded/unloaded
        let chunk_ownership = CHUNK_OWNERSHIP.lock().unwrap();
        let current_chunks: HashSet<(i32, i32)> = chunk_ownership
            .iter()
            .filter_map(|(chunk, &owner)| {
                if owner == loader_entity {
                    Some(*chunk)
                } else {
                    None
                }
            })
            .collect();
        drop(chunk_ownership);

        let chunks_to_spawn: Vec<&(i32, i32)> = target_chunks.difference(&current_chunks).collect();
        let chunks_to_despawn: Vec<&(i32, i32)> = current_chunks.difference(&target_chunks).collect();

        if !chunks_to_spawn.is_empty() {
            debug!("update_chunk_loader spawning chunks {:?}", chunks_to_spawn);
        }
        if !chunks_to_despawn.is_empty() {
            debug!("update_chunk_loader despawning chunks {:?}", chunks_to_despawn);
        }

        let loaded_chunks = LOADED_CHUNKS.lock().unwrap();
        
        // Spawn and claim ownership of new chunks
        for &chunk_coord in chunks_to_spawn {
            if loaded_chunks.contains(&chunk_coord) {
                // Skip if chunk already exists
                continue;
            }

            debug!("update_chunk_loader spawning chunk {:?}", chunk_coord);

            let requested_chunk_additions = REQUESTED_CHUNK_ADDITIONS.lock().unwrap();
            let requested_chunk_removals = REQUESTED_CHUNK_REMOVALS.lock().unwrap();
            spawn_chunk(&mut commands, requested_chunk_additions, requested_chunk_removals, chunk_coord, loader_entity);
        }

        // Release ownership of chunks no longer in range
        for &chunk_coord in chunks_to_despawn {
            // Check if another loader can claim ownership
            match chunk_loader_query
            .iter()
            .find(|(other_loader_entity, transform, loader)| {
                if *other_loader_entity == loader_entity {
                    return false;
                }

                let other_position = transform.translation.truncate();
                let other_radius = loader.radius;
                calculate_chunks_in_radius(other_position, other_radius).contains(&chunk_coord)
            }) {
                Some((loader_entity, _, _)) => {
                    debug!("Found a new owner for chunk {:?}, switching owner", chunk_coord);

                    let mut chunk_ownership = CHUNK_OWNERSHIP.lock().unwrap();
                    chunk_ownership.remove(&chunk_coord);
                    chunk_ownership.insert(chunk_coord, loader_entity);
                    drop(chunk_ownership);
                },
                None => {
                    debug!("Found no new owner for chunk {:?}, despawning chunk", chunk_coord);

                    if !loaded_chunks.contains(&chunk_coord) {
                        // Skip if chunk already does not exist
                        continue;
                    }

                    let (chunk_entity, _) = chunk_query
                        .iter()
                        .find(|(_, chunk)| {
                            chunk.coord == chunk_coord
                        })
                        .unwrap_or_else(|| { panic!("Failed to find the entity of chunk {:?}", chunk_coord) });

                    let requested_chunk_additions = REQUESTED_CHUNK_ADDITIONS.lock().unwrap();
                    let requested_chunk_removals = REQUESTED_CHUNK_REMOVALS.lock().unwrap();
                    despawn_chunk(&mut commands, requested_chunk_additions, requested_chunk_removals, chunk_coord, chunk_entity);
                }
            }
        }
    }
}