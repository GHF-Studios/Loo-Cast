use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use crate::chunk::events::*;
use crate::chunk::resources::*;
use crate::chunk::functions;
use crate::entity::resources::*;
use crate::chunk::components::Chunk;

pub fn handle_upgrade_to_chunk(
    world: &mut World,
    event_parameters: &mut SystemState<
        EventReader<UpgradeToChunk>,
    >,
    registry_parameters: &mut SystemState<(
        ResMut<EntityRegistry>,
        ResMut<ChunkRegistry>
    )>,
) {
    let mut upgrade_to_chunk_event_reader = event_parameters.get_mut(world);

    let mut upgrade_to_chunk_events = Vec::new();
    for upgrade_to_chunk_event in upgrade_to_chunk_event_reader.read() {
        upgrade_to_chunk_events.push(upgrade_to_chunk_event.clone());
    }

    for upgrade_to_chunk_event in upgrade_to_chunk_events {
        let upgrade_chunk_request = upgrade_to_chunk_event.0;

        let chunk_id = upgrade_chunk_request.chunk_id;
        let chunk_entity_id = upgrade_chunk_request.chunk_entity_id;

        let entity_reference = {
            let entity_registry = registry_parameters.get_mut(world).0;

            let entity_reference = match entity_registry.get_loaded_entity_reference(&chunk_entity_id) {
                Some(entity_reference) => entity_reference.clone(),
                None => {
                    panic!("Entity reference associated with entity id '{:?}' not found!", chunk_entity_id);
                }
            };

            entity_reference
        };

        world.entity_mut(entity_reference).insert(Chunk::new(chunk_id));
    }
}

pub fn handle_downgrade_from_chunk(
    world: &mut World,
    event_parameters: &mut SystemState<
        EventReader<DowngradeFromChunk>,
    >,
    registry_parameters: &mut SystemState<(
        ResMut<EntityRegistry>,
        ResMut<ChunkRegistry>
    )>,
) {
    let mut downgrade_from_chunk_event_reader = event_parameters.get_mut(world);

    let mut downgrade_from_chunk_events = Vec::new();
    for downgrade_from_chunk_event in downgrade_from_chunk_event_reader.read() {
        downgrade_from_chunk_events.push(downgrade_from_chunk_event.clone());
    }

    for downgrade_from_chunk_event in downgrade_from_chunk_events {
        let downgrade_chunk_request = downgrade_from_chunk_event.0;

        let chunk_id = downgrade_chunk_request.chunk_id;
        let entity_reference = {
            let chunk_registry = registry_parameters.get_mut(world).1;

            let entity_reference = match chunk_registry.get_loaded_chunk_entity(chunk_id) {
                Some(chunk_entity_reference) => chunk_entity_reference.clone(),
                None => {
                    panic!("Chunk entity reference associated with chunk id '{:?}' not found!", chunk_id);
                }
            };

            entity_reference
        };

        world.entity_mut(entity_reference).remove::<Chunk>();
    }
}

pub fn handle_load_chunk(
    world: &mut World,
    event_parameters: &mut SystemState<(
        EventReader<LoadChunk>,
    )>,
    registry_parameters: &mut SystemState<(
        ResMut<ChunkRegistry>,
        ResMut<EntityRegistry>,
    )>,
) {
    let mut load_chunk_event_reader = event_parameters.get_mut(world).0;

    let mut load_chunk_events = Vec::new();
    for load_chunk_event in load_chunk_event_reader.read() {
        load_chunk_events.push(load_chunk_event.clone());
    }

    for load_chunk_event in load_chunk_events {
        let load_chunk_request = load_chunk_event.0;

        let chunk_id = load_chunk_request.chunk_id;

        let (previously_serialized_chunk_entity_id, serialized_chunk) = {
            let chunk_registry = registry_parameters.get_mut(world).0;

            match chunk_registry.serialized_chunks().get(&chunk_id) {
                Some((chunk_entity_id, serialized_chunk)) => (chunk_entity_id.clone(), serialized_chunk.clone()),
                None => {
                    panic!("Chunk with id '{:?}' not found!", chunk_id);
                }
            }
        };

        let chunk_entity_reference = functions::deserialize_chunk(world, serialized_chunk.to_string());

        let assigned_chunk_entity_id = {
            let entity_registry = registry_parameters.get_mut(world).1;

            let assigned_entity_id = match entity_registry.get_loaded_entity_id(&chunk_entity_reference) {
                Some(entity_id) => entity_id.clone(),
                None => {
                    panic!("Entity id associated with entity reference '{:?}' not found!", chunk_entity_reference);
                }
            };

            assigned_entity_id
        };

        let mut chunk_registry = registry_parameters.get_mut(world).0;

        match chunk_registry.deserialize_chunk(chunk_id) {
            Some(_) => {},
            None => {
                panic!("Failed to deserialize chunk with id '{:?}'!", chunk_id);
            }
        }

        if assigned_chunk_entity_id != previously_serialized_chunk_entity_id {
            panic!("Assigned entity id '{:?}' does not match the previously serialized entity id '{:?}'!", assigned_chunk_entity_id, previously_serialized_chunk_entity_id);
        }
    }
}

pub fn handle_save_chunk(
    world: &mut World,
    event_parameters: &mut SystemState<(
        EventReader<SaveChunk>,
    )>,
    chunk_registry_parameter: &mut SystemState<
        ResMut<ChunkRegistry>,
    >,
    entity_registry_parameter: &mut SystemState<
        Res<EntityRegistry>,
    >,
) {
    let mut save_chunk_event_reader = event_parameters.get_mut(world).0;

    let mut save_chunk_events = Vec::new();
    for save_chunk_event in save_chunk_event_reader.read() {
        save_chunk_events.push(save_chunk_event.clone());
    }

    for save_chunk_event in save_chunk_events {
        let save_chunk_request = save_chunk_event.0;

        let chunk_id = save_chunk_request.chunk_id;
        let entity_id = {
            let chunk_registry = chunk_registry_parameter.get_mut(world);

            let entity_reference = match chunk_registry.get_loaded_chunk_entity(chunk_id) {
                Some(chunk_entity_reference) => chunk_entity_reference.clone(),
                None => {
                    panic!("Chunk entity reference associated with chunk id '{:?}' not found!", chunk_id);
                }
            };

            let entity_registry = entity_registry_parameter.get(world);

            let entity_id = match entity_registry.get_loaded_entity_id(&entity_reference) {
                Some(entity_id) => entity_id.clone(),
                None => {
                    panic!("Entity id associated with entity reference '{:?}' not found!", entity_reference);
                }
            };

            entity_id
        };

        let serialized_chunk = functions::serialize_chunk(world, chunk_registry_parameter, chunk_id);

        let mut chunk_registry = chunk_registry_parameter.get_mut(world);

        chunk_registry.serialize_chunk(chunk_id, serialized_chunk, entity_id);
    }
}