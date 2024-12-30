use bevy::ecs::component::ComponentId;
use bevy::prelude::*;
use bevy::ecs::world::DeferredWorld;

use crate::chunk_loader::resources::ChunkOwnership;

use super::components::ChunkComponent;
use super::functions::grid_position;

pub(in crate) fn hook_on_add_chunk(mut world: DeferredWorld, entity: Entity, _component_id: ComponentId) {
    let chunk_coord = match world.get::<Transform>(entity) {
        Some(transform) => grid_position(transform.translation.truncate()),
        None => return
    };

    let chunk = world.get::<ChunkComponent>(entity).unwrap();
    let owner = match chunk.owner {
        Some(owner) => owner,
        None => {
            panic!("Attempted to add chunk {:?}, for which no owner was provided", chunk_coord)
        }
    };

    let mut chunk_ownership = match world.get_resource_mut::<ChunkOwnership>() {
        Some(chunk_ownership) => chunk_ownership,
        None => return
    };

    debug!("loaded_chunks {:?}", chunk_ownership.loaded_chunks);
    debug!("ownership {:?}", chunk_ownership.ownership);

    if chunk_ownership.loaded_chunks.contains(&chunk_coord) {
        panic!("Attempted to add chunk {:?} duplicate", chunk_coord)
    }

    if chunk_ownership.ownership.contains_key(&chunk_coord) {
        unreachable!("Attempted to add chunk {:?}, but it's ownership has already been claimed illegally", chunk_coord)
    }

    chunk_ownership.loaded_chunks.insert(chunk_coord);
    chunk_ownership.ownership.insert(chunk_coord, owner);
}

pub(in crate) fn hook_on_remove_chunk(mut world: DeferredWorld, entity: Entity, _component_id: ComponentId) {
    let chunk_coord = match world.get::<Transform>(entity) {
        Some(transform) => grid_position(transform.translation.truncate()),
        None => return
    };

    let mut chunk_ownership = match world.get_resource_mut::<ChunkOwnership>() {
        Some(chunk_ownership) => chunk_ownership,
        None => return
    };

    if chunk_ownership.loaded_chunks.contains(&chunk_coord) {
        panic!("Attempted to despawn already-non-existent chunk {:?}", chunk_coord)
    }

    if chunk_ownership.ownership.contains_key(&chunk_coord) {
        panic!("Attempted to release ownership of chunk {:?}, which nobody has ownership of to begin with", chunk_coord)
    }

    chunk_ownership.loaded_chunks.remove(&chunk_coord);
    chunk_ownership.ownership.remove(&chunk_coord);
}