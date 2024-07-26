use bevy::ecs::component::ComponentId;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;
use super::components::*;
use super::events::*;
use super::resources::*;
use super::structs::*;
use super::id::structs::*;

pub(super) fn setup(world: &mut World) {
    world
        .register_component_hooks::<SpacetimeEntity>()
        .on_add(on_add_entity)
        .on_remove(on_remove_entity);
}

pub fn request_create_entity(
    create_entity_event_writer: &mut EventWriter<CreateEntity>,
    entity_registry: &mut EntityRegistry,
    entity_request_registry: &mut EntityRequestRegistry,
) -> (EntityRequestID, EntityID) {
    let entity_request_id = entity_request_registry.get_unused_entity_request_id();
    let entity_id = entity_registry.register_entity();

    let create_entity_request = EntityRequest {
        entity_request_id,
        entity_id,
    };

    entity_request_registry.register_entity_request(entity_request_id);

    if entity_registry.is_entity_registered(entity_id) {
        warn!("Entity '{:?}' is already registered!", entity_id);

        return (entity_request_id, entity_id);
    }

    if entity_registry.is_entity_loaded(entity_id) {
        warn!("Entity '{:?}' is already loaded!", entity_id);

        return (entity_request_id, entity_id);
    }

    entity_registry.start_creating_entity(create_entity_request.clone());
    entity_request_registry.load_entity_request(create_entity_request);
    create_entity_event_writer.send(CreateEntity(create_entity_request));

    (entity_request_id, entity_id)
}

pub fn request_destroy_entity(
    destroy_entity_event_writer: &mut EventWriter<DestroyEntity>,
    entity_registry: &mut EntityRegistry,
    entity_request_registry: &mut EntityRequestRegistry,
    entity_id: EntityID,
) -> EntityRequestID {
    let entity_request_id = entity_request_registry.get_unused_entity_request_id();

    let destroy_entity_request = EntityRequest {
        entity_request_id,
        entity_id,
    };

    entity_request_registry.register_entity_request(entity_request_id);

    if !entity_registry.is_entity_registered(entity_id) {
        warn!("Entity '{:?}' is not registered!", entity_id);

        return entity_request_id;
    }

    if !entity_registry.is_entity_loaded(entity_id) {
        warn!("Entity '{:?}' is not loaded!", entity_id);

        return entity_request_id;
    }

    entity_registry.start_destroying_entity(destroy_entity_request.clone());
    entity_request_registry.load_entity_request(destroy_entity_request);
    destroy_entity_event_writer.send(DestroyEntity(destroy_entity_request));

    entity_request_id
}

fn on_add_entity(
    mut world: DeferredWorld,
    entity: Entity,
    _component: ComponentId,
) {
    let entity_reference = entity;

    let world_position = match world.get::<Transform>(entity_reference) {
        Some(transform) => transform.translation.truncate(),
        None => {
            panic!("Failed to get transform component associated with entity '{:?}'!", entity_reference);
        }
    };

    let entity_id = {
        let mut entity_registry = match world.get_resource_mut::<EntityRegistry>() {
            Some(entity_registry) => entity_registry,
            None => {
                panic!("Failed to get entity registry!");
            }
        };

        let entity_id = match entity_registry.get_loaded_entity_id(&entity_reference) {
            Some(entity_id) => {
                panic!("Entity '{:?}' is already loaded!", entity_id);
            },
            None => {
                let entity_id = entity_registry.register_entity();
                entity_registry.load_entity(entity_id, entity_reference);

                entity_id
            }
        };

        entity_id
    };

    let entity_request_id = {
        let entity_request_registry = match world.get_resource::<EntityRequestRegistry>() {
            Some(entity_request_registry) => entity_request_registry,
            None => {
                panic!("Failed to get entity request registry!");
            }
        };

        match entity_request_registry
            .loaded_entity_requests()
            .values()
            .clone()
            .find(|request| request.entity_id == entity_id)
            .map(|request| {
                request.entity_request_id
            }) {
            Some(entity_request_id) => entity_request_id,
            None => {
                panic!("Failed to get entity request id currently associated with entity '{:?}'!", entity_reference);
            }
        }
    };

    {
        let mut entity_registry = match world.get_resource_mut::<EntityRegistry>() {
            Some(entity_registry) => entity_registry,
            None => {
                panic!("Failed to get entity registry!");
            }
        };

        let mut entity_request_registry = match world.get_resource_mut::<EntityRequestRegistry>() {
            Some(entity_request_registry) => entity_request_registry,
            None => {
                panic!("Failed to get entity request registry!");
            }
        };

        let is_creating_entity = entity_registry.is_creating_entity(entity_id);
        let is_loading_entity = entity_registry.is_loading_entity(entity_request_id);

        if is_creating_entity && is_loading_entity {
            panic!("Entity '{:?}' is both creating and loading!", entity_id);
        } else if is_creating_entity {
            entity_registry.stop_creating_entity(entity_id);
    
            entity_request_registry.unload_entity_request(entity_request_id);
    
            world.send_event(CreatedEntity(EntityResponse::Success {
                entity_request_id,
                entity_id,
                world_position,
            }));

        } else if is_loading_entity {
            entity_registry.stop_loading_entity(entity_id);
    
            entity_request_registry.unload_entity_request(entity_request_id);
    
            world.send_event(LoadedEntity(EntityResponse::Success {
                entity_request_id,
                entity_id,
                world_position,
            }));
        } else {
            panic!("Entity '{:?}' is neither creating nor loading!", entity_id);
        }
    }
}

fn on_remove_entity(
    mut world: DeferredWorld,
    entity: Entity,
    _component: ComponentId,
) {
    let entity_reference = entity;

    let world_position = match world.get::<Transform>(entity_reference) {
        Some(transform) => transform.translation.truncate(),
        None => {
            panic!("Failed to get world position associated with entity '{:?}'!", entity_reference);
        }
    };

    let entity_id = {
        let mut entity_registry = match world.get_resource_mut::<EntityRegistry>() {
            Some(entity_registry) => entity_registry,
            None => {
                panic!("Failed to get entity registry!");
            }
        };

        match entity_registry.get_loaded_entity_id(&entity_reference) {
            Some(entity_id) => {
                entity_registry.unload_entity(entity_id);
                entity_registry.unregister_entity(entity_id);

                entity_id
            },
            None => {
                panic!("Failed to get entity id associated with entity '{:?}'!", entity_reference);
            }
        }
    };

    let entity_request_id =  {
        let entity_request_registry = match world.get_resource::<EntityRequestRegistry>() {
            Some(entity_request_registry) => entity_request_registry,
            None => {
                panic!("Failed to get entity request registry!");
            }
        };

        match entity_request_registry
            .loaded_entity_requests()
            .values()
            .clone()
            .find(|request| request.entity_id == entity_id)
            .map(|request| {
                request.entity_request_id
            }) {
                Some(entity_request_id) => entity_request_id,
                None => {
                    panic!("Failed to get entity request id currently associated with entity '{:?}'!", entity_reference);
                }
            }
    };

    {
        let mut entity_registry = match world.get_resource_mut::<EntityRegistry>() {
            Some(entity_registry) => entity_registry,
            None => {
                panic!("Failed to get entity registry!");
            }
        };

        let mut entity_request_registry = match world.get_resource_mut::<EntityRequestRegistry>() {
            Some(entity_request_registry) => entity_request_registry,
            None => {
                panic!("Failed to get entity request registry!");
            }
        };

        let is_destroying_entity = entity_registry.is_destroying_entity(entity_id);
        let is_saving_entity = entity_registry.is_saving_entity(entity_request_id);

        if is_destroying_entity && is_saving_entity {
            panic!("Entity '{:?}' is both destroying and saving!", entity_id);
        } else if is_destroying_entity {
            entity_registry.stop_destroying_entity(entity_id);
    
            entity_request_registry.unload_entity_request(entity_request_id);
    
            world.send_event(DestroyedEntity(EntityResponse::Success {
                entity_request_id,
                entity_id,
                world_position,
            }));
        } else if is_saving_entity {
            entity_registry.stop_saving_entity(entity_id);
    
            entity_request_registry.unload_entity_request(entity_request_id);
    
            world.send_event(SavedEntity(EntityResponse::Success {
                entity_request_id,
                entity_id,
                world_position,
            }));
        } else {
            panic!("Entity '{:?}' is neither destroying nor saving!", entity_id);
        }
    }
}