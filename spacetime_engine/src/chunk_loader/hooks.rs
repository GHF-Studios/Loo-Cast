use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;
use bevy::ecs::component::ComponentId;

use crate::chunk::functions::*;

use super::components::ChunkLoaderComponent;
use super::resources::ChunkOwnership;

pub(in crate) fn on_add_chunk_loader(mut world: DeferredWorld, entity: Entity, _component_id: ComponentId) {
    let component_value = world
        .get::<ChunkLoaderComponent>(entity)
        .unwrap();
    let transform = world.get::<Transform>(entity).unwrap();

    let radius = component_value.radius;
    let position = transform.translation.truncate(); // 2D position
    let chunks_to_load = calculate_chunks_in_range(position, radius);

    if let Some(mut chunk_ownership) = world.get_resource_mut::<ChunkOwnership>() {
        for chunk_coord in chunks_to_load {
            // Skip spawning if the chunk already exists
            if chunk_ownership.loaded_chunks.contains(&chunk_coord) {
                continue;
            }

            // Claim ownership of the chunk
            chunk_ownership.ownership.insert(chunk_coord, entity);
            chunk_ownership.loaded_chunks.insert(chunk_coord);
        }
    }
}

pub(in crate) fn on_remove_chunk_loader(mut world: DeferredWorld, entity: Entity, _component_id: ComponentId, mut chunk_loader_query: QueryState<(&Transform, &ChunkLoaderComponent)>) {
    let mut chunk_ownership = match world.get_resource_mut::<ChunkOwnership>() {
        Some(chunk_ownership) => chunk_ownership.clone(),
        None => {
            return;
        }
    }; 
    
    // Release ownership of chunks owned by this loader
    let chunks_to_release: Vec<(i32, i32)> = chunk_ownership
        .ownership
        .iter()
        .filter_map(|(&chunk, &owner)| if owner == entity { Some(chunk) } else { None })
        .collect();

    for chunk_coord in chunks_to_release {
        chunk_ownership.ownership.remove(&chunk_coord);

        // Check if another loader can take ownership
        if !world
            .query(&mut chunk_loader_query)
            .iter()
            .any(|(transform, loader)| {
                let other_position = transform.translation.truncate();
                let other_radius = loader.radius;
                calculate_chunks_in_range(other_position, other_radius).contains(&chunk_coord)
            })
        {
            // No other loader can claim this chunk; despawn it
            chunk_ownership.loaded_chunks.remove(&chunk_coord);
        }
    }
}

