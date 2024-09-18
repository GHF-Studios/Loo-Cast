use bevy::prelude::*;
use bevy::ecs::world::DeferredWorld;
use bevy::ecs::component::ComponentId;
use crate::chunk::components::Chunk;
use crate::chunk::wrappers::ChunkInstanceRegistry;
use crate::operations::components::Serialized;
use crate::operations::singletons::MAIN_TYPE_REGISTRY;
use super::components::ChunkActor;
use super::wrappers::ChunkActorInstanceRegistry;

pub(in super) fn on_add_chunk_actor(
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

    let chunk_actor_instance_registry = match main_type_registry.get_data_mut::<ChunkActor, ChunkActorInstanceRegistry>() {
        Some(chunk_actor_instance_registry) => chunk_actor_instance_registry,
        None => {
            return;
        },
    };

    match world.get::<Serialized>(entity) {
        Some(_) => {
            let chunk_actor_id = match world.get::<ChunkActor>(entity) {
                Some(chunk_actor) => chunk_actor.id(),
                None => {
                    return;
                },
            };
            chunk_actor_instance_registry.manage(chunk_actor_id, entity);
        },
        None => {
            let chunk_actor_id = chunk_actor_instance_registry.register();
            chunk_actor_instance_registry.manage(chunk_actor_id, entity);

            let mut chunk_actor = match world.get_mut::<ChunkActor>(entity) {
                Some(chunk_actor) => chunk_actor,
                None => {
                    return;
                },
            };

            *chunk_actor.id_mut() = chunk_actor_id;
        },
    }

    let chunk_actor = match world.get::<ChunkActor>(entity) {
        Some(chunk_actor) => chunk_actor,
        None => {
            return;
        },
    };

    let chunk_actor_id = chunk_actor.id();

    let chunk_id = chunk_actor.current_chunk();

    let chunk_instance_registry = match main_type_registry.get_data_mut::<Chunk, ChunkInstanceRegistry>() {
        Some(chunk_instance_registry) => chunk_instance_registry,
        None => {
            return;
        },
    };

    let chunk_entity = match chunk_instance_registry.get(chunk_id) {
        Some(chunk_entity) => *chunk_entity,
        None => {
            return;
        },
    };

    let mut chunk = match world.get_mut::<Chunk>(chunk_entity) {
        Some(chunk) => chunk,
        None => {
            return;
        },
    };

    chunk.register_chunk_actor(chunk_actor_id);
}

pub(in super) fn on_remove_chunk_actor(
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

    let chunk_actor_instance_registry = match main_type_registry.get_data_mut::<ChunkActor, ChunkActorInstanceRegistry>() {
        Some(chunk_actor_instance_registry) => chunk_actor_instance_registry,
        None => {
            return;
        },
    };

    let chunk_actor_id = match chunk_actor_instance_registry.get_key(&entity) {
        Some(chunk_actor_id) => *chunk_actor_id,
        None => {
            return;
        },
    };

    match world.get::<Serialized>(entity) {
        Some(_) => {
            chunk_actor_instance_registry.unmanage(chunk_actor_id);
        },
        None => {
            chunk_actor_instance_registry.unmanage(chunk_actor_id);
            chunk_actor_instance_registry.unregister(chunk_actor_id);
        },
    };

    let chunk_instance_registry = match main_type_registry.get_data_mut::<Chunk, ChunkInstanceRegistry>() {
        Some(chunk_instance_registry) => chunk_instance_registry,
        None => {
            return;
        },
    };

    let chunk_actor = match world.get_mut::<ChunkActor>(entity) {
        Some(chunk_actor) => chunk_actor,
        None => {
            return;
        },
    };

    let chunk_id = chunk_actor.current_chunk();

    let chunk_entity = match chunk_instance_registry.get(chunk_id) {
        Some(chunk_entity) => *chunk_entity,
        None => {
            return;
        },
    };

    let mut chunk = match world.get_mut::<Chunk>(chunk_entity) {
        Some(chunk) => chunk,
        None => {
            return;
        },
    };

    chunk.unregister_chunk_actor(chunk_actor_id);
}
