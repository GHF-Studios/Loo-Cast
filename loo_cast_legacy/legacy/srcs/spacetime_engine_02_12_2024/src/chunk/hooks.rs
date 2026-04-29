use bevy::{ecs::{component::ComponentId, world::DeferredWorld}, prelude::*};
use crate::core::{components::*, singletons::*};
use super::{components::Chunk, wrappers::ChunkInstanceRegistry};

pub(in super) fn on_add_chunk(
    mut world: DeferredWorld,
    entity: Entity,
    _component: ComponentId,
) {
    let mut main_type_registry = match MAIN_TYPE_REGISTRY.lock() {
        Ok(main_type_registry) => main_type_registry,
        Err(_) => {
            return;
        },
    };

    let chunk_instance_registry = match main_type_registry.get_data_mut::<Chunk, ChunkInstanceRegistry>() {
        Some(chunk_instance_registry) => chunk_instance_registry,
        None => {
            return;
        },
    };

    match world.get::<Serialized>(entity) {
        Some(_) => {
            let chunk_id = match world.get::<Chunk>(entity) {
                Some(chunk) => chunk.id(),
                None => {
                    return;
                },
            };
            chunk_instance_registry.manage(chunk_id, entity);
        },
        None => {
            let chunk_id = chunk_instance_registry.register();
            chunk_instance_registry.manage(chunk_id, entity);

            let mut chunk = match world.get_mut::<Chunk>(entity) {
                Some(chunk) => chunk,
                None => {
                    return;
                },
            };

            *chunk.id_mut() = chunk_id;
        },
    };
}

pub(in super) fn on_remove_chunk(
    world: DeferredWorld,
    entity: Entity,
    _component: ComponentId,
) {
    let mut main_type_registry = match MAIN_TYPE_REGISTRY.lock() {
        Ok(main_type_registry) => main_type_registry,
        Err(_) => {
            return;
        },
    };

    let chunk_instance_registry = match main_type_registry.get_data_mut::<Chunk, ChunkInstanceRegistry>() {
        Some(chunk_instance_registry) => chunk_instance_registry,
        None => {
            return;
        },
    };

    let chunk_id = match chunk_instance_registry.get_key(&entity) {
        Some(chunk_id) => *chunk_id,
        None => {
            return;
        },
    };

    match world.get::<Serialized>(entity) {
        Some(_) => {
            chunk_instance_registry.unmanage(chunk_id);
        },
        None => {
            chunk_instance_registry.unmanage(chunk_id);
            chunk_instance_registry.unregister(chunk_id);
        },
    };
}
