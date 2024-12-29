use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;
use bevy::ecs::component::ComponentId;
use std::collections::HashSet;

use crate::chunk::components::ChunkComponent;
use crate::chunk::functions::calculate_chunks_in_range;

use super::components::ChunkLoaderComponent;
use super::resources::ChunkOwnership;

pub fn update_chunk_loader(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &ChunkLoaderComponent)>,
    mut chunk_ownership: ResMut<ChunkOwnership>,
) {
    for (loader_entity, transform, chunk_loader) in query.iter() {
        let position = transform.translation.truncate(); // 2D position
        let range = chunk_loader.range;

        let target_chunks = calculate_chunks_in_range(position, range)
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

        // Spawn and claim ownership of new chunks
        for &chunk_coord in chunks_to_spawn {
            if chunk_ownership.loaded_chunks.contains(&chunk_coord) {
                // Skip if chunk already exists
                continue;
            }

            // Spawn the chunk
            commands.spawn(ChunkComponent { coordinates: chunk_coord, owner: Some(loader_entity) });

            // Claim ownership
            chunk_ownership.ownership.insert(chunk_coord, loader_entity);
            chunk_ownership.loaded_chunks.insert(chunk_coord);
        }

        // Release ownership of chunks no longer in range
        for &chunk_coord in chunks_to_despawn {
            chunk_ownership.ownership.remove(&chunk_coord);

            // Check if another loader can claim ownership
            if !query.iter().any(|(_, transform, chunk_loader)| {
                let other_position = transform.translation.truncate();
                let other_radius = chunk_loader.radius;
                calculate_chunks_in_range(other_position, other_radius).contains(&chunk_coord)
            }) {
                // No other loader can claim this chunk; despawn it
                chunk_ownership.loaded_chunks.remove(&chunk_coord);
                commands.entity(loader_entity).despawn_recursive();
            }
        }
    }
}