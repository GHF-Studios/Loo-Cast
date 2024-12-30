use bevy::prelude::*;
use std::collections::HashSet;

use crate::chunk::bundles::ChunkBundle;
use crate::chunk::components::ChunkComponent;
use crate::chunk::constants::HALF_CHUNK_SIZE;
use crate::chunk::functions::calculate_chunks_in_radius;
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
        let mut chunk_ownership = CHUNK_OWNERSHIP.lock().unwrap();
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

        let chunks_to_spawn = target_chunks.difference(&current_chunks);
        let chunks_to_despawn = current_chunks.difference(&target_chunks);

        debug!("chunks_to_spawn {:?}", chunks_to_spawn);
        debug!("chunks_to_despawn {:?}", chunks_to_despawn);

        let loaded_chunks = LOADED_CHUNKS.lock().unwrap();
        
        // Spawn and claim ownership of new chunks
        for &chunk_coord in chunks_to_spawn {
            if loaded_chunks.contains(&chunk_coord) {
                // Skip if chunk already exists
                continue;
            }

            let mut requested_chunk_additions = REQUESTED_CHUNK_ADDITIONS.lock().unwrap();
            if requested_chunk_additions.contains(&chunk_coord) {
                // Skip if chunk is already requested
                continue;
            }

            // TODO: Request the chunk (encapsulate/automate this somehow, maybe using event)
            requested_chunk_additions.insert(chunk_coord);
            // TODO: +
            commands.spawn(ChunkBundle {
                chunk: ChunkComponent {
                    coord: chunk_coord,
                    owner: Some(loader_entity)
                },
                sprite_bundle: SpriteBundle {
                    sprite: Sprite {
                        rect: Some(Rect::new(-HALF_CHUNK_SIZE, -HALF_CHUNK_SIZE, HALF_CHUNK_SIZE, HALF_CHUNK_SIZE)),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            });
        }

        // Release ownership of chunks no longer in range
        for &chunk_coord in chunks_to_despawn {
            chunk_ownership.remove(&chunk_coord);

            // Check if another loader can claim ownership
            if !chunk_loader_query.iter().any(|(_, transform, chunk_loader)| {
                let other_position = transform.translation.truncate();
                let other_radius = chunk_loader.radius;
                calculate_chunks_in_radius(other_position, other_radius).contains(&chunk_coord)
            }) {
                if !loaded_chunks.contains(&chunk_coord) {
                    // Skip if chunk already does not exist
                    continue;
                }

                let mut requested_chunk_removals = REQUESTED_CHUNK_REMOVALS.lock().unwrap();
                if requested_chunk_removals.contains(&chunk_coord) {
                    // Skip if chunk is already requested
                    continue;
                }

                // No other loader can claim this chunk; resolve the chunk entity and despawn it
                let (chunk_entity, _) = chunk_query
                    .iter()
                    .find(|(_, chunk)| {
                        chunk.coord == chunk_coord
                    })
                    .expect("The entity of chunk {:?} could not be resolved");
                
                // TODO: Request the chunk (encapsulate/automate this somehow, maybe using event)
                requested_chunk_removals.insert(chunk_coord);
                // TODO: +
                commands.entity(chunk_entity).despawn_recursive();
            }
        }
    }
}