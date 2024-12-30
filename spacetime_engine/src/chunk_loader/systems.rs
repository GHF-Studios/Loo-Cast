use bevy::prelude::*;
use std::collections::HashSet;

use crate::chunk::bundles::ChunkBundle;
use crate::chunk::components::ChunkComponent;
use crate::chunk::constants::HALF_CHUNK_SIZE;
use crate::chunk::functions::calculate_chunks_in_radius;

use super::components::ChunkLoaderComponent;
use super::resources::ChunkOwnership;

pub(in crate) fn update_chunk_loader_system(
    mut commands: Commands,
    chunk_loader_query: Query<(Entity, &Transform, &ChunkLoaderComponent)>,
    chunk_query: Query<(Entity, &ChunkComponent)>,
    mut chunk_ownership: ResMut<ChunkOwnership>,
) {
    for (loader_entity, transform, chunk_loader) in chunk_loader_query.iter() {
        let position = transform.translation.truncate(); // 2D position
        let radius = chunk_loader.radius;

        let target_chunks = calculate_chunks_in_radius(position, radius)
            .into_iter()
            .collect::<HashSet<(i32, i32)>>();

        // Track chunks that should be loaded/unloaded
        let current_chunks: HashSet<(i32, i32)> = chunk_ownership
            .ownership
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

        // Spawn and claim ownership of new chunks
        for &chunk_coord in chunks_to_spawn {
            if chunk_ownership.loaded_chunks.contains(&chunk_coord) {
                // Skip if chunk already exists
                continue;
            }

            // Spawn the chunk
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
            chunk_ownership.ownership.remove(&chunk_coord);

            // Check if another loader can claim ownership
            if !chunk_loader_query.iter().any(|(_, transform, chunk_loader)| {
                let other_position = transform.translation.truncate();
                let other_radius = chunk_loader.radius;
                calculate_chunks_in_radius(other_position, other_radius).contains(&chunk_coord)
            }) {
                // No other loader can claim this chunk; resolve the chunk entity and despawn it
                let (chunk_entity, _) = chunk_query
                    .iter()
                    .find(|(_, chunk)| {
                        chunk.coord == chunk_coord
                    })
                    .expect("The entity of chunk {:?} could not be resolved");
                commands.entity(chunk_entity).despawn_recursive();
            }
        }
    }
}