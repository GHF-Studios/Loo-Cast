use bevy::prelude::*;
use crate::entity::events::*;
use crate::entity::resources::*;
use crate::entity::structs::*;
use crate::entity::id::structs::*;
use super::checks::*;

pub fn request_create_entity(
    create_entity_event_writer: &mut EventWriter<CreateEntity>,
    entity_registry: &mut EntityRegistry,
    entity_request_registry: &mut EntityRequestRegistry,
) -> Option<(EntityRequestID, EntityID)> {
    let entity_request_id = entity_request_registry.get_unused_entity_request_id();
    let entity_id = entity_registry.register_entity();

    if !can_request_create_entity(entity_registry, entity_id) {
        return None;
    }

    let create_entity_request = EntityRequest {
        entity_request_id,
        entity_id,
    };
    
    entity_registry.start_creating_entity(create_entity_request.clone());
    entity_request_registry.register_entity_request(entity_request_id);
    entity_request_registry.load_entity_request(entity_request_id, create_entity_request);
    create_entity_event_writer.send(CreateEntity(create_entity_request));

    Some((entity_request_id, entity_id))
}

pub fn request_destroy_entity(
    destroy_entity_event_writer: &mut EventWriter<DestroyEntity>,
    entity_registry: &mut EntityRegistry,
    entity_request_registry: &mut EntityRequestRegistry,
    entity_id: EntityID,
) -> Option<EntityRequestID> {
    let entity_request_id = entity_request_registry.get_unused_entity_request_id();

    if !can_request_destroy_entity(entity_registry, entity_id) {
        return None;
    }

    let destroy_entity_request = EntityRequest {
        entity_request_id,
        entity_id,
    };

    entity_registry.start_destroying_entity(destroy_entity_request.clone());
    entity_request_registry.register_entity_request(entity_request_id);
    entity_request_registry.load_entity_request(entity_request_id, destroy_entity_request);
    destroy_entity_event_writer.send(DestroyEntity(destroy_entity_request));

    Some(entity_request_id)
}
