use bevy::prelude::*;

use crate::chunk::bundles::ChunkBundle;
use crate::chunk::components::ChunkComponent;
use crate::chunk::constants::HALF_CHUNK_SIZE;
use crate::chunk::functions::*;
use crate::chunk::statics::{CHUNK_OWNERSHIP, LOADED_CHUNKS, REQUESTED_CHUNK_ADDITIONS, REQUESTED_CHUNK_REMOVALS};

use super::components::ChunkLoaderComponent;

pub(in crate) fn observe_on_add_chunk_loader(
    trigger: Trigger<OnRemove, ChunkLoaderComponent>,
    mut commands: Commands,
    chunk_loader_query: Query<(&Transform, &ChunkLoaderComponent)>,
) {
    let entity = trigger.entity();
    let (transform, chunk_loader) = chunk_loader_query.get(entity).unwrap();
    let radius = chunk_loader.radius;
    let position = transform.translation.truncate();
    let chunks_to_load = calculate_chunks_in_radius(position, radius);

    for chunk_coord in chunks_to_load {
        let loaded_chunks = LOADED_CHUNKS.lock().unwrap();
        if loaded_chunks.contains(&chunk_coord) {
            continue;
        }

        let mut requested_chunk_additions = REQUESTED_CHUNK_ADDITIONS.lock().unwrap();
        if requested_chunk_additions.contains(&chunk_coord) {
            continue;
        }

        // TODO: Request the chunk (encapsulate/automate this somehow, maybe using event)
        requested_chunk_additions.insert(chunk_coord);
        // TODO: +
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
) {
    let mut chunk_ownership = CHUNK_OWNERSHIP.lock().unwrap();
    let chunks_to_release: Vec<(i32, i32)> = chunk_ownership
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
                    chunk_ownership.remove(&chunk_coord);
                    chunk_ownership.insert(chunk_coord, new_owner);
                },
                None => {
                    let mut requested_chunk_removals = REQUESTED_CHUNK_REMOVALS.lock().unwrap();
                    if requested_chunk_removals.contains(&chunk_coord) {
                        continue;
                    }

                    // TODO: Request the chunk (encapsulate/automate this somehow, maybe using event)
                    requested_chunk_removals.insert(chunk_coord);
                    // TODO: +
                    commands.entity(trigger.entity()).despawn_recursive();
                }
            }
    }
}
