use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use crate::chunk::events::*;
use crate::chunk::resources::*;
use crate::chunk::functions;
use crate::entity::resources::*;
use super::components::Chunk;

pub(super) fn handle_upgrade_to_chunk(
    world: &mut World,
    event_parameters: &mut SystemState<
        EventReader<UpgradeToChunk>,
    >,
    registry_parameters: &mut SystemState<
        ResMut<EntityRegistry>,
    >,
) {
    let mut upgrade_to_chunk_event_reader = event_parameters.get_mut(world);

    let mut upgrade_to_chunk_events = Vec::new();
    for upgrade_to_chunk_event in upgrade_to_chunk_event_reader.read() {
        upgrade_to_chunk_events.push(upgrade_to_chunk_event.clone());
    }

    for upgrade_to_chunk_event in upgrade_to_chunk_events {
        let upgrade_chunk_request = upgrade_to_chunk_event.0;

        let chunk_id = upgrade_chunk_request.chunk_id;
        let entity_id = upgrade_chunk_request.entity_id;

        let entity_reference = {
            let entity_registry = registry_parameters.get_mut(world);
            match entity_registry.get_loaded_entity_reference(&entity_id) {
                Some(entity_reference) => entity_reference.clone(),
                None => {
                    panic!("Entity reference associated with entity id '{:?}' not found!", entity_id);
                }
            }
        };

        world.entity_mut(entity_reference).insert(Chunk::new(chunk_id));
    }
}

pub(super) fn handle_downgrade_from_chunk(
    world: &mut World,
    event_parameters: &mut SystemState<
        EventReader<DowngradeFromChunk>,
    >,
    registry_parameters: &mut SystemState<
        ResMut<EntityRegistry>,
    >,
) {
    let mut downgrade_from_chunk_event_reader = event_parameters.get_mut(world);

    let mut downgrade_from_chunk_events = Vec::new();
    for downgrade_from_chunk_event in downgrade_from_chunk_event_reader.read() {
        downgrade_from_chunk_events.push(downgrade_from_chunk_event.clone());
    }

    for downgrade_from_chunk_event in downgrade_from_chunk_events {
        let downgrade_chunk_request = downgrade_from_chunk_event.0;

        let entity_id = downgrade_chunk_request.entity_id;

        let entity_reference = {
            let entity_registry = registry_parameters.get_mut(world);
            match entity_registry.get_loaded_entity_reference(&entity_id) {
                Some(entity_reference) => entity_reference.clone(),
                None => {
                    panic!("Entity reference associated with entity id '{:?}' not found!", entity_id);
                }
            }
        };

        world.entity_mut(entity_reference).remove::<Chunk>();
    }
}

pub(super) fn handle_load_chunk(
    world: &mut World,
    event_parameters: &mut SystemState<(
        EventReader<LoadChunk>,
    )>,
    registry_parameters: &mut SystemState<
        ResMut<ChunkRegistry>,
    >,
) {
    let mut load_chunk_event_reader = event_parameters.get_mut(world).0;

    let mut load_chunk_events = Vec::new();
    for load_chunk_event in load_chunk_event_reader.read() {
        load_chunk_events.push(load_chunk_event.clone());
    }

    for load_chunk_event in load_chunk_events {
        let load_chunk_request = load_chunk_event.0;

        let chunk_id = load_chunk_request.chunk_id;

        let serialized_chunk = {
            let mut chunk_registry = registry_parameters.get_mut(world);

            match chunk_registry.deserialize_chunk(chunk_id) {
                Some(serialized_chunk) => serialized_chunk,
                None => {
                    panic!("Chunk with id '{:?}' not found!", chunk_id);
                }
            }
        };

        let _chunk_entity = functions::deserialize_chunk(world, serialized_chunk);
    }
}

pub(super) fn handle_save_chunk(
    world: &mut World,
    event_parameters: &mut SystemState<(
        EventReader<SaveChunk>,
    )>,
    registry_parameter: &mut SystemState<
        ResMut<ChunkRegistry>,
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

        let serialized_chunk = functions::serialize_chunk(world, registry_parameter, chunk_id);

        let mut chunk_registry = registry_parameter.get_mut(world);

        chunk_registry.serialize_chunk(chunk_id, serialized_chunk);
    }
}
