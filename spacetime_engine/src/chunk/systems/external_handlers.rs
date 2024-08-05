use bevy::prelude::*;
use crate::chunk::events::*;
use crate::chunk::resources::*;
use crate::chunk::functions::requests::*;
use crate::entity::events::CreatedEntity;
use crate::entity::resources::*;
use crate::entity::structs::EntityResponse;
use crate::chunk::loader::components::ChunkLoader;
use crate::chunk::structs::ChunkResponse;

pub fn handle_created_entity(
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
                    let is_registry_capable = can_request_upgrade_to_chunk(&mut chunk_registry, &mut entity_registry, *chunk_id, entity_id);
    
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

pub fn handle_upgraded_to_chunk(
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

