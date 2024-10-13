use bevy::prelude::*;
use bevy::ecs::{world::DeferredWorld, component::ComponentId};
use crate::chunk::components::Chunk;
use crate::chunk::wrappers::ChunkInstanceRegistry;
use crate::core::components::Serialized;
use crate::core::singletons::MAIN_TYPE_REGISTRY;
use super::components::ChunkLoader;
use super::wrappers::ChunkLoaderInstanceRegistry;

pub(in super) fn on_add_chunk_loader(
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

    let chunk_loader_instance_registry = match main_type_registry.get_data_mut::<ChunkLoader, ChunkLoaderInstanceRegistry>() {
        Some(chunk_loader_instance_registry) => chunk_loader_instance_registry,
        None => {
            return;
        },
    };

    match world.get::<Serialized>(entity) {
        Some(_) => {
            let chunk_loader_id = match world.get::<ChunkLoader>(entity) {
                Some(chunk_loader) => chunk_loader.id(),
                None => {
                    return;
                },
            };
            chunk_loader_instance_registry.manage(chunk_loader_id, entity);
        },
        None => {
            let chunk_loader_id = chunk_loader_instance_registry.register();
            chunk_loader_instance_registry.manage(chunk_loader_id, entity);

            let mut chunk_loader = match world.get_mut::<ChunkLoader>(entity) {
                Some(chunk_loader) => chunk_loader,
                None => {
                    return;
                },
            };

            *chunk_loader.id_mut() = chunk_loader_id;
        },
    }

    // TODO: Spawn the initial chunks
}

pub(in super) fn on_remove_chunk_loader(
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

    let chunk_loader_instance_registry = match main_type_registry.get_data_mut::<ChunkLoader, ChunkLoaderInstanceRegistry>() {
        Some(chunk_loader_instance_registry) => chunk_loader_instance_registry,
        None => {
            return;
        },
    };

    let chunk_loader_id = match chunk_loader_instance_registry.get_key(&entity) {
        Some(chunk_loader_id) => *chunk_loader_id,
        None => {
            return;
        },
    };

    match world.get::<Serialized>(entity) {
        Some(_) => {
            chunk_loader_instance_registry.unmanage(chunk_loader_id);
        },
        None => {
            chunk_loader_instance_registry.unmanage(chunk_loader_id);
            chunk_loader_instance_registry.unregister(chunk_loader_id);
        },
    };

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

    let chunk_loader = match world.get::<ChunkLoader>(entity) {
        Some(chunk_loader) => chunk_loader,
        None => {
            return;
        },
    };

    for registered_chunk_info in chunk_loader.registered_chunks().clone() {
        let chunk_entity = match chunk_instance_registry.get(registered_chunk_info.chunk_id()) {
            Some(chunk_entity) => *chunk_entity,
            None => {
                return;
            },
        };

        // TODO: Unload the chunk
    }
}
