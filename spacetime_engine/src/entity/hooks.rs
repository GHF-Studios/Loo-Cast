use super::components::*;
use super::wrappers::*;
use crate::operations::singletons::*;
use crate::operations::components::*;
use bevy::{ecs::{component::ComponentId, world::DeferredWorld}, prelude::*};

pub(in super) fn on_add_entity(
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

            // TODO: Remove
            warn!("Hook: Created entity: {:?}!", entity_id);
        },
    };

    // TODO: NEW COMMENT
    // Trigger an 'on_add' observer for this entity

    // TODO: DUST ASS OLD ASS COMMENT
    // If the entity has a 'ReactOnAdd<SpacetimeEntity>' component, remove the component and call the callback.

    let react_on_add_spacetime_entity_component = match world.get::<ReactOnAdd<SpacetimeEntity>>(entity) {
        Some(react_on_add_spacetime_entity_component) => react_on_add_spacetime_entity_component,
        None => {
            return;
        },
    };

    let spacetime_entity_component = world.get::<SpacetimeEntity>(entity).unwrap();

    react_on_add_spacetime_entity_component.call(spacetime_entity_component);

    // TODO: This shit ain't work here!!!!!!!! We can't modify the world from a hook, we should use an observer for that!!!!!!!!
    //world.entity_mut(entity).remove::<ReactOnAdd<SpacetimeEntity>>();
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
