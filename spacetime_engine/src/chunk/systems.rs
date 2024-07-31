use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use crate::chunk::events::*;
use crate::chunk::resources::*;
use crate::chunk::functions;
use crate::entity::events::CreatedEntity;
use crate::entity::resources::*;
use crate::entity::structs::EntityResponse;
use super::components::Chunk;
use super::loader::components::ChunkLoader;
use super::structs::ChunkResponse;

pub(super) fn handle_upgrade_to_chunk(
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

pub(super) fn handle_downgrade_from_chunk(
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

pub(super) fn handle_load_chunk(
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

pub(super) fn handle_save_chunk(
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
            let entity_registry = entity_registry_parameter.get(world);

            let entity_reference = match chunk_registry.get_loaded_chunk_entity(chunk_id) {
                Some(chunk_entity_reference) => chunk_entity_reference.clone(),
                None => {
                    panic!("Chunk entity reference associated with chunk id '{:?}' not found!", chunk_id);
                }
            };

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

pub(super) fn handle_created_entity(
    mut created_entity_event_reader: EventReader<CreatedEntity>,
    mut upgrade_to_chunk_event_writer: EventWriter<UpgradeToChunk>,
    mut chunk_loader_query: Query<&mut ChunkLoader>,
    mut chunk_registry: ResMut<ChunkRegistry>,
    mut chunk_request_registry: ResMut<ChunkRequestRegistry>,
    mut entity_registry: ResMut<EntityRegistry>,
) {
    let mut chunk_loader = chunk_loader_query.single_mut();

    let mut preparing_entities = Vec::new();
    for (chunk_id, entity_request_id) in chunk_loader.currently_preparing_entities_for_chunk_upgrade().iter() {
        preparing_entities.push((*chunk_id, *entity_request_id));
    }

    let mut created_entity_events = Vec::new();
    for created_entity_event in created_entity_event_reader.read() {
        created_entity_events.push(created_entity_event.clone());
    }

    for created_entity_event in created_entity_events {
        let prepared_entities = match created_entity_event.0 {
            EntityResponse::Success { entity_id, .. } => {
                let mut prepared_entities = Vec::new();
                
                for (chunk_id, entity_request_id) in preparing_entities.iter() {
                    let is_loader_capable = chunk_loader.can_upgrade_to_chunk(*chunk_id);
                    let is_registry_capable = functions::can_request_upgrade_to_chunk(&mut chunk_registry, &mut entity_registry, *chunk_id, entity_id);
    
                    if is_loader_capable && is_registry_capable {
                        prepared_entities.push((chunk_id, entity_request_id, entity_id))
                    } else {
                        continue;
                    }
                }

                prepared_entities
            },
            EntityResponse::Failure { entity_id, .. } => {
                panic!("Entity '{:?}' failed to create!", entity_id);
            },
        };

        for (chunk_id, _, entity_id) in prepared_entities {
            chunk_loader.stop_preparing_entity_for_chunk_upgrade(*chunk_id);
            chunk_loader.start_upgrading_to_chunk(*chunk_id);
            functions::request_upgrade_to_chunk(&mut upgrade_to_chunk_event_writer, &mut chunk_registry, &mut chunk_request_registry, &mut entity_registry, *chunk_id, entity_id);
        }
    }
}

pub(super) fn handle_upgraded_to_chunk(
    mut upgraded_to_chunk_event_reader: EventReader<UpgradedToChunk>,
    mut chunk_loader_query: Query<&mut ChunkLoader>,
    mut chunk_registry: ResMut<ChunkRegistry>,
) {
    let mut chunk_loader = chunk_loader_query.single_mut();

    let mut upgrading_to_chunks = Vec::new();
    for chunk_id in chunk_loader.currently_upgrading_to_chunks().iter() {
        upgrading_to_chunks.push(*chunk_id);
    }
    
    let mut upgraded_to_chunk_events = Vec::new();
    for upgraded_to_chunk_event in upgraded_to_chunk_event_reader.read() {
        upgraded_to_chunk_events.push(upgraded_to_chunk_event.clone());
    }

    for upgraded_to_chunk_event in upgraded_to_chunk_events {
        let upgraded_to_chunks = match upgraded_to_chunk_event.0 {
            ChunkResponse::Success { chunk_id, .. } => {
                let mut upgraded_to_chunks = Vec::new();
                
                for upgrading_to_chunk in upgrading_to_chunks.iter() {
                    if upgrading_to_chunk == &chunk_id {
                        upgraded_to_chunks.push(chunk_id)
                    } else {
                        continue;
                    }
                }

                upgraded_to_chunks
            },
            ChunkResponse::Failure { chunk_id, .. } => {
                panic!("Entity failed to upgrade to chunk '{:?}'!", chunk_id);
            },
        };

        for chunk_id in upgraded_to_chunks {
            if !chunk_registry.is_chunk_upgrading_to(chunk_id) { continue; }

            chunk_registry.stop_upgrading_to_chunk(chunk_id);
            chunk_loader.stop_upgrading_to_chunk(chunk_id);
        }
    }
}

