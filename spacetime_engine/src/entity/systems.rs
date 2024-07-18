use bevy::prelude::*;
use bevy::ecs::system::SystemState;
use super::components::SpacetimeEntity;
use super::resources::*;
use super::events::*;

pub(super) fn handle_create_entity(
    world: &mut World,
    event_parameters: &mut SystemState<
        EventReader<CreateEntity>,
    >,
    registry_parameters: &mut SystemState<
        ResMut<EntityRegistry>,
    >,
) {
    let mut create_entity_event_reader = event_parameters.get_mut(world);

    let mut create_entity_events = Vec::new();
    for create_entity_event in create_entity_event_reader.read() {
        create_entity_events.push(create_entity_event.clone());
    }

    for create_entity_event in create_entity_events {
        let create_entity_request = &create_entity_event.0;
        
        let entity_id = create_entity_request.entity_id;
        let world_position = Vec2::new(0.0, 0.0);

        let mut entity = world.spawn(Transform::from_translation(world_position.extend(0.0)));
        let entity_reference = entity.id();
        entity.insert(SpacetimeEntity {
            entity_id,
        });

        let mut entity_registry = registry_parameters.get_mut(world);
        entity_registry.load_entity(entity_id, entity_reference);
    }
}

pub(super) fn handle_destroy_entity(
    world: &mut World,
    event_parameters: &mut SystemState<
        EventReader<DestroyEntity>,
    >,
    registry_parameters: &mut SystemState<
        ResMut<EntityRegistry>,
    >,
) {
    let mut destroy_entity_event_reader = event_parameters.get_mut(world);

    let mut destroy_entity_events = Vec::new();
    for destroy_entity_event in destroy_entity_event_reader.read() {
        destroy_entity_events.push(destroy_entity_event.clone());
    }

    for destroy_entity_event in destroy_entity_events {
        let destroy_entity_request = &destroy_entity_event.0;
        
        let entity_id = destroy_entity_request.entity_id;

        let entity_registry = registry_parameters.get_mut(world);
        let entity_reference = match entity_registry.get_loaded_entity_reference(&entity_id) {
            Some(entity_reference) => entity_reference.clone(),
            None => {
                panic!("Entity reference associated with entity id '{:?}' not found!", entity_id); 
            },
        };

        world.despawn(entity_reference);

        let mut entity_registry = registry_parameters.get_mut(world);
        entity_registry.unload_entity(entity_id);
    }
}