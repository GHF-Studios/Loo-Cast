use bevy::prelude::*;
use super::events::*;
use super::resources::*;
use super::structs::*;
use super::id::structs::*;

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

    if entity_registry.is_entity_registered(entity_id) {
        warn!("Entity '{:?}' is already registered!", entity_id);

        return (entity_request_id, entity_id);
    }

    if entity_registry.is_entity_loaded(entity_id) {
        warn!("Entity '{:?}' is already loaded!", entity_id);

        return (entity_request_id, entity_id);
    }

    entity_registry.start_creating_entity(create_entity_request.clone());
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

    if !entity_registry.is_entity_registered(entity_id) {
        warn!("Entity '{:?}' is not registered!", entity_id);

        return entity_request_id;
    }

    if !entity_registry.is_entity_loaded(entity_id) {
        warn!("Entity '{:?}' is not loaded!", entity_id);

        return entity_request_id;
    }

    entity_registry.start_destroying_entity(destroy_entity_request.clone());
    destroy_entity_event_writer.send(DestroyEntity(destroy_entity_request));

    entity_request_id
}