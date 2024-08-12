use bevy::prelude::*;
use bevy::ecs::system::SystemState;
use crate::entity::components::SpacetimeEntity;
use crate::entity::resources::*;
use crate::entity::events::*;

pub fn handle_create_entity(
    world: &mut World,
    event_parameters: &mut SystemState<
        EventReader<CreateEntity>,
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

        world.spawn((
            Transform::from_translation(world_position.extend(0.0)),
            SpacetimeEntity {
                id: entity_id,
            },
        ));
    }
}

pub fn handle_destroy_entity(
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

        // TODO: Panic if the entity still has any components left, except for 'Transform' and 'SpacetimeEntity'
        //use crate::component;
        //
        //for component_id in component::get_components_ids(world, &entity_reference)
        //{
        //    let component_info = component::component_id_to_component_info(world, component_id).unwrap();
        //    println!("{}", component::extract_component_name(component_info));
        //}

        world.despawn(entity_reference);
    }
}