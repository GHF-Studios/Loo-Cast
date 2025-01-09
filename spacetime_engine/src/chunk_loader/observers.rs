use bevy::prelude::*;

use crate::chunk::components::ChunkComponent;
use crate::chunk::functions::*;

use super::components::ChunkLoaderComponent;

pub(in crate) fn observe_on_add_chunk_loader(
    trigger: Trigger<OnRemove, ChunkLoaderComponent>,
    mut commands: Commands,
    chunk_loader_query: Query<(&Transform, &ChunkLoaderComponent)>,
) {
    let mut requested_chunk_additions = REQUESTED_CHUNK_ADDITIONS.lock().unwrap();
    let requested_chunk_removals = REQUESTED_CHUNK_REMOVALS.lock().unwrap();
    let loaded_chunks = LOADED_CHUNKS.lock().unwrap();
    
    let loader_entity = trigger.entity();
    let (transform, chunk_loader) = chunk_loader_query.get(loader_entity).unwrap();
    let radius = chunk_loader.radius;
    let position = transform.translation.truncate();
    let chunks_to_load = calculate_chunks_in_radius(position, radius);

    for chunk_coord in chunks_to_load {
        if loaded_chunks.contains(&chunk_coord) {
            continue;
        }

        debug!("on_remove_chunk_loader spawning chunk {:?}", chunk_coord);

        match spawn_chunk(&mut commands, &mut requested_chunk_additions, &requested_chunk_removals, chunk_coord, loader_entity) {
            Ok(_) => {},
            Err(err) => {
                panic!("{:?}", err)
            }
        }
    }
}

pub(in crate) fn observe_on_remove_chunk_loader(
    trigger: Trigger<OnRemove, ChunkLoaderComponent>,
    mut commands: Commands,
    chunk_loader_query: Query<(Entity, &Transform, &ChunkLoaderComponent)>,
    chunk_query: Query<(Entity, &ChunkComponent)>
) {
    let loader_entity = trigger.entity();
    let requested_chunk_additions = REQUESTED_CHUNK_ADDITIONS.lock().unwrap();
    let mut requested_chunk_removals = REQUESTED_CHUNK_REMOVALS.lock().unwrap();
    let mut chunk_ownership = CHUNK_OWNERSHIP.lock().unwrap();

    let chunks_to_release: Vec<(i32, i32)> = chunk_ownership
        .iter()
        .filter_map(|(&chunk, &owner)| if owner == loader_entity { Some(chunk) } else { None })
        .collect();

    if !chunks_to_release.is_empty() {
        debug!("on_remove_chunk_loader releasing chunks {:?}", chunks_to_release);
    }

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
                    debug!("on_remove_chunk_loader Found a new owner for chunk {:?}, switching owner", chunk_coord);
                    
                    chunk_ownership.remove(&chunk_coord);
                    chunk_ownership.insert(chunk_coord, new_owner);
                },
                None => {
                    debug!("on_remove_chunk_loader Found no new owner for chunk {:?}, despawning chunk", chunk_coord);

                    let (chunk_entity, _) = chunk_query
                        .iter()
                        .find(|(_, chunk)| {
                            chunk.coord == chunk_coord
                        })
                        .unwrap_or_else(|| { panic!("Failed to find the entity of chunk {:?}", chunk_coord) });

                    match despawn_chunk(&mut commands, &requested_chunk_additions, &mut requested_chunk_removals, chunk_coord, chunk_entity) {
                        Ok(_) => {},
                        Err(err) => {
                            panic!("{:?}", err)
                        }
                    }
                }
            }
    }
}
