use bevy::prelude::*;

use crate::chunk::bundles::ChunkBundle;
use crate::chunk::components::ChunkComponent;
use crate::chunk::constants::HALF_CHUNK_SIZE;
use crate::chunk::functions::*;

use super::components::ChunkLoaderComponent;
use super::resources::ChunkOwnership;

pub(in crate) fn observe_on_add_chunk_loader(
    trigger: Trigger<OnRemove, ChunkLoaderComponent>,
    mut commands: Commands,
    chunk_loader_query: Query<(&Transform, &ChunkLoaderComponent)>,
    chunk_ownership: ResMut<ChunkOwnership>,
) {
    let entity = trigger.entity();
    let (transform, chunk_loader) = chunk_loader_query.get(entity).unwrap();
    let radius = chunk_loader.radius;
    let position = transform.translation.truncate(); // 2D position
    let chunks_to_load = calculate_chunks_in_radius(position, radius);

    for chunk_coord in chunks_to_load {
        if chunk_ownership.loaded_chunks.contains(&chunk_coord) {
            continue;
        }

        commands.spawn(ChunkBundle {
            chunk: ChunkComponent {
                coord: chunk_coord,
                owner: Some(entity)
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
}

pub(in crate) fn observe_on_remove_chunk_loader(
    trigger: Trigger<OnRemove, ChunkLoaderComponent>,
    mut commands: Commands,
    chunk_loader_query: Query<(Entity, &Transform, &ChunkLoaderComponent)>,
    mut chunk_ownership: ResMut<ChunkOwnership>,
) {
    
    let chunks_to_release: Vec<(i32, i32)> = chunk_ownership
        .ownership
        .iter()
        .filter_map(|(&chunk, &owner)| if owner == trigger.entity() { Some(chunk) } else { None })
        .collect();

    for chunk_coord in chunks_to_release {
        match chunk_loader_query
            .iter()
            .find(|(_, transform, loader)| {
                let other_position = transform.translation.truncate();
                let other_radius = loader.radius;
                calculate_chunks_in_radius(other_position, other_radius).contains(&chunk_coord)
            }) {
                Some((new_owner, _, _)) => {
                    // A new owner has been found for the chunk. Ownership will be transfered accordingly
                    chunk_ownership.ownership.remove(&chunk_coord);
                    chunk_ownership.ownership.insert(chunk_coord, new_owner);
                },
                None => {
                    // No new owner could be found. The chunk will despawn immediately
                    commands.entity(trigger.entity()).despawn_recursive();
                }
            }
    }
}
