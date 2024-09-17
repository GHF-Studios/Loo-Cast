use super::components::*;
use super::wrappers::*;
use crate::operations::singletons::*;
use crate::operations::components::*;
use bevy::{ecs::{component::ComponentId, world::DeferredWorld}, prelude::*};

pub(in super) fn on_add_entity(
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

    let entity_instance_registry = match main_type_registry.get_data_mut::<Entity, EntityInstanceRegistry>() {
        Some(entity_instance_registry) => entity_instance_registry,
        None => {
            return;
        },
    };

    match world.get::<Serialized>(entity) {
        Some(_) => {
            let entity_id = match world.get::<SpacetimeEntity>(entity) {
                Some(spacetime_entity_component) => spacetime_entity_component.id(),
                None => {
                    return;
                },
            };
            entity_instance_registry.manage(entity_id, entity);
        },
        None => {
            let entity_id = entity_instance_registry.register();
            entity_instance_registry.manage(entity_id, entity);

            let mut spacetime_entity = match world.get_mut::<SpacetimeEntity>(entity) {
                Some(spacetime_entity_component) => spacetime_entity_component,
                None => {
                    return;
                },
            };
            
            *spacetime_entity.id_mut() = entity_id;
        },
    };
}

pub(in super) fn on_remove_entity(
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

    let entity_instance_registry = match main_type_registry.get_data_mut::<Entity, EntityInstanceRegistry>() {
        Some(entity_instance_registry) => entity_instance_registry,
        None => {
            return;
        },
    };

    let entity_id = match entity_instance_registry.get_key(&entity) {
        Some(entity_id) => *entity_id,
        None => {
            return;
        },
    };

    // TODO: Return if the entity still has any components left, except for 'Transform', 'SpacetimeEntity', and 'Serialized'.

    match world.get::<Serialized>(entity) {
        Some(_) => {
            entity_instance_registry.unmanage(entity_id);
            return;
        },
        None => {
            entity_instance_registry.unmanage(entity_id);
            entity_instance_registry.unregister(entity_id);
            return;
        },
    };
}
