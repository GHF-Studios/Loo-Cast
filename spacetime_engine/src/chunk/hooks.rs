use bevy::ecs::component::ComponentId;
use bevy::prelude::*;
use bevy::ecs::world::DeferredWorld;

use super::components::ChunkComponent;
use super::functions::world_pos_to_chunk;
use super::statics::{CHUNK_OWNERSHIP, LOADED_CHUNKS, REQUESTED_CHUNK_ADDITIONS, REQUESTED_CHUNK_REMOVALS};

pub(in crate) fn hook_on_add_chunk(world: DeferredWorld, entity: Entity, _component_id: ComponentId) {
    let chunk = world.get::<ChunkComponent>(entity).unwrap();
    let owner = match chunk.owner {
        Some(owner) => owner,
        None => {
            panic!("Attempted to add chunk, for which no owner was provided")
        }
    };

    let mut loaded_chunks = LOADED_CHUNKS.lock().unwrap();
    let mut chunk_ownership = CHUNK_OWNERSHIP.lock().unwrap();
    let mut requested_chunk_additions = REQUESTED_CHUNK_ADDITIONS.lock().unwrap();

    if loaded_chunks.contains(&chunk.coord) {
        panic!("Attempted to add chunk {:?} duplicate", chunk.coord);
    }

    if chunk_ownership.contains_key(&chunk.coord) {
        unreachable!("Attempted to add chunk {:?}, but it's ownership has already been claimed illegally", chunk.coord);
    }

    if !requested_chunk_additions.contains(&chunk.coord) {
        panic!("Attempted to illegally add chunk {:?}, which has not been requested yet", chunk.coord);
    }

    loaded_chunks.insert(chunk.coord);
    chunk_ownership.insert(chunk.coord, owner);
    requested_chunk_additions.remove(&chunk.coord);
}

pub(in crate) fn hook_on_remove_chunk(world: DeferredWorld, entity: Entity, _component_id: ComponentId) {
    let chunk = world.get::<ChunkComponent>(entity).unwrap();

    let mut loaded_chunks = LOADED_CHUNKS.lock().unwrap();
    let mut chunk_ownership = CHUNK_OWNERSHIP.lock().unwrap();
    let mut requested_chunk_removals = REQUESTED_CHUNK_REMOVALS.lock().unwrap();

    if !loaded_chunks.contains(&chunk.coord) {
        panic!("Attempted to despawn already-non-existent chunk {:?}", chunk.coord);
    }

    if !chunk_ownership.contains_key(&chunk.coord) {
        unreachable!("Attempted to release ownership of chunk {:?}, which nobody has ownership of to begin with", chunk.coord);
    }

    if !requested_chunk_removals.contains(&chunk.coord) {
        panic!("Attempted to illegally remove chunk {:?}, which has not been requested yet", chunk.coord);
    }

    loaded_chunks.remove(&chunk.coord);
    chunk_ownership.remove(&chunk.coord);
    requested_chunk_removals.remove(&chunk.coord);
}