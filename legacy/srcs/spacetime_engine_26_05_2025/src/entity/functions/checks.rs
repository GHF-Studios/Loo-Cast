use crate::entity::resources::EntityRegistry;
use crate::entity::id::structs::EntityID;

pub fn can_request_create_entity(
    entity_registry: &EntityRegistry,
    entity_id: EntityID,
) -> bool {
    if entity_registry.is_entity_registered(entity_id) {
        return false;
    }
    if entity_registry.is_entity_loaded(entity_id) {
        return false;
    }

    if entity_registry.is_entity_creating(entity_id) {
        return false;
    }
    if entity_registry.is_entity_destroying(entity_id) {
        return false;
    }
    if entity_registry.is_entity_loading(entity_id) {
        return false;
    }
    if entity_registry.is_entity_saving(entity_id) {
        return false;
    }

    true
}

pub fn can_request_destroy_entity(
    entity_registry: &EntityRegistry,
    entity_id: EntityID,
) -> bool {
    if !entity_registry.is_entity_registered(entity_id) {
        return false;
    }
    if !entity_registry.is_entity_loaded(entity_id) {
        return false;
    }

    if entity_registry.is_entity_creating(entity_id) {
        return false;
    }
    if entity_registry.is_entity_destroying(entity_id) {
        return false;
    }
    if entity_registry.is_entity_loading(entity_id) {
        return false;
    }
    if entity_registry.is_entity_saving(entity_id) {
        return false;
    }

    true
}