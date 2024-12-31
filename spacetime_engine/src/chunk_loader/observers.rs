use bevy::prelude::*;

use crate::chunk::components::ChunkComponent;
use crate::chunk::functions::*;
use crate::chunk::resources::{ChunkRetryAction, ChunkRetryQueue};
use crate::chunk::statics::{CHUNK_OWNERSHIP, LOADED_CHUNKS, REQUESTED_CHUNK_ADDITIONS, REQUESTED_CHUNK_REMOVALS};

use super::components::ChunkLoaderComponent;

pub(in crate) fn observe_on_add_chunk_loader(
    trigger: Trigger<OnRemove, ChunkLoaderComponent>,
    mut commands: Commands,
    chunk_loader_query: Query<(&Transform, &ChunkLoaderComponent)>,
    mut retry_queue: ResMut<ChunkRetryQueue>, // Retry resource
) {
    let loader_entity = trigger.entity();
    let (transform, chunk_loader) = chunk_loader_query.get(loader_entity).unwrap();
    let radius = chunk_loader.radius;
    let position = transform.translation.truncate();
    let chunks_to_load = calculate_chunks_in_radius(position, radius);

    for chunk_coord in chunks_to_load {
        let loaded_chunks = LOADED_CHUNKS.lock().unwrap();
        if loaded_chunks.contains(&chunk_coord) {
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
}

pub(in crate) fn observe_on_remove_chunk_loader(
    trigger: Trigger<OnRemove, ChunkLoaderComponent>,
    mut commands: Commands,
    chunk_loader_query: Query<(Entity, &Transform, &ChunkLoaderComponent)>,
    chunk_query: Query<(Entity, &ChunkComponent)>,
    mut retry_queue: ResMut<ChunkRetryQueue>, // Retry resource
) {
    let loader_entity = trigger.entity();
    let mut chunk_ownership = CHUNK_OWNERSHIP.lock().unwrap();
    let chunks_to_release: Vec<(i32, i32)> = chunk_ownership
        .iter()
        .filter_map(|(&chunk, &owner)| if owner == loader_entity { Some(chunk) } else { None })
        .collect();

    for chunk_coord in chunks_to_release {
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
            Some((new_owner, _, _)) => {
                chunk_ownership.remove(&chunk_coord);
                chunk_ownership.insert(chunk_coord, new_owner);
            }
            None => {
                if let Some((chunk_entity, _)) = chunk_query
                    .iter()
                    .find(|(_, chunk)| chunk.coord == chunk_coord)
                {
                    let result = despawn_chunk(
                        &mut commands,
                        REQUESTED_CHUNK_ADDITIONS.lock().unwrap(),
                        REQUESTED_CHUNK_REMOVALS.lock().unwrap(),
                        chunk_coord,
                        chunk_entity,
                    );

                    if let Err(err) = result {
                        warn!(
                            "Failed to despawn chunk {:?}: {:?}. Retrying later.",
                            chunk_coord, err
                        );
                        retry_queue.actions.push_back(ChunkRetryAction::Despawn {
                            chunk_coord,
                            chunk_entity,
                        });
                    }
                }
            }
        }
    }
}
