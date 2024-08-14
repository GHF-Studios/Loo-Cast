use bevy::prelude::*;
use crate::{chunk::{actor::resources::ChunkActorRegistry, components::Chunk, functions::{checks::*, requests::*}, id::structs::ChunkID, loader::components::ChunkLoader, structs::ChunkResponse, ChunkRegistry, ChunkRequestRegistry, LoadChunk, LoadedChunk, SaveChunk, SavedChunk, UpgradedToChunk}, entity::{events::CreateEntity, functions::requests::request_create_entity, resources::{EntityRegistry, EntityRequestRegistry}}};

pub(in crate) fn start_chunks(
    load_chunk_event_writer: &mut EventWriter<LoadChunk>,
    create_entity_event_writer: &mut EventWriter<CreateEntity>,
    chunk_loader: &mut Mut<ChunkLoader>,
    chunk_registry: &mut ResMut<ChunkRegistry>,
    chunk_request_registry: &mut ResMut<ChunkRequestRegistry>,
    entity_registry: &mut EntityRegistry,
    entity_request_registry: &mut EntityRequestRegistry,
    start_chunk_ids: &Vec<ChunkID>,
) {
    for start_chunk_id in start_chunk_ids {
        debug!("Start chunk '{:?}' detected!", start_chunk_id);

        let chunk_id = *start_chunk_id;



        // TODO: RICHTIG IMPLEMENTIEREN //



        if !chunk_registry.is_chunk_registered(chunk_id) {
            let (entity_request_id, entity_id) = match request_create_entity(
                create_entity_event_writer, 
                entity_registry, 
                entity_request_registry
            ) {
                Some(entity_request_id) => entity_request_id,
                None => continue
            };

            chunk_loader.register_managed_chunk(chunk_id);

            continue;
        }

        request_load_chunk(load_chunk_event_writer, chunk_registry, chunk_request_registry, chunk_id);
    }
}

#[allow(clippy::too_many_arguments)]
pub(in crate) fn update_chunks(
    create_entity_event_writer: &mut EventWriter<CreateEntity>,
    load_chunk_event_writer: &mut EventWriter<LoadChunk>,
    save_chunk_event_writer: &mut EventWriter<SaveChunk>,
    chunk_loader: &mut Mut<ChunkLoader>,
    chunk_registry: &mut ResMut<ChunkRegistry>,
    chunk_request_registry: &mut ResMut<ChunkRequestRegistry>,
    chunk_actor_registry: &mut ChunkActorRegistry,
    entity_registry: &mut EntityRegistry,
    entity_request_registry: &mut EntityRequestRegistry,
    chunk_query: &Query<&Chunk>,
    old_chunk_ids: Vec<ChunkID>,
    new_chunk_ids: Vec<ChunkID>,
) {
    for old_chunk_id in old_chunk_ids.iter() {
        debug!("Old chunk '{:?}' detected!", old_chunk_id);
        
        let chunk_id = old_chunk_id;



        // TODO: RICHTIG IMPLEMENTIEREN //



        if !chunk_registry.is_chunk_registered(*chunk_id) { continue; }
        if !chunk_registry.is_chunk_loaded(*chunk_id) { continue; }
        if !chunk_loader.can_save_chunk(*chunk_id) { continue; }

        if request_save_chunk(
            save_chunk_event_writer, 
            chunk_registry, 
            chunk_request_registry,
            chunk_actor_registry,
            entity_registry, 
            chunk_query,
            *chunk_id
        ).is_none() { continue }

        chunk_loader.start_saving_chunk(*chunk_id)
    }

    for new_chunk_id in new_chunk_ids.iter() { 
        debug!("New chunk '{:?}' detected!", new_chunk_id);

        let chunk_id = new_chunk_id;



        // TODO: RICHTIG IMPLEMENTIEREN //



        if !chunk_registry.is_chunk_registered(*chunk_id) {
            let (entity_request_id, _) = request_create_entity(
                create_entity_event_writer, 
                entity_registry, 
                entity_request_registry
            );

            chunk_loader.start_preparing_entity_for_chunk_upgrade(*chunk_id, entity_request_id);

            continue;
        }

        if chunk_registry.is_chunk_loaded(*chunk_id) { continue; }
        if !chunk_loader.can_load_chunk(*chunk_id) { continue };
        if !can_request_load_chunk(chunk_registry, *chunk_id) { continue };

        match request_load_chunk(load_chunk_event_writer, chunk_registry, chunk_request_registry, *chunk_id) {
            Some(_) => chunk_loader.start_loading_chunk(*chunk_id),
            None => continue
        }
    }
}

pub(in crate) fn handle_updated_chunks(
    mut upgraded_to_chunk_event_reader: EventReader<UpgradedToChunk>,
    mut loaded_chunk_event_reader: EventReader<LoadedChunk>,
    mut saved_chunk_event_reader: EventReader<SavedChunk>,
    chunk_loader: &mut Mut<ChunkLoader>,
    chunk_registry: &mut ResMut<ChunkRegistry>,
) {
    let mut upgraded_to_chunk_events = Vec::new();
    for upgraded_to_chunk_event in upgraded_to_chunk_event_reader.read() {
        upgraded_to_chunk_events.push(upgraded_to_chunk_event);
    }

    let mut loaded_chunk_events = Vec::new();
    for loaded_chunk_event in loaded_chunk_event_reader.read() {
        loaded_chunk_events.push(loaded_chunk_event);
    }

    let mut saved_chunk_events = Vec::new();
    for saved_chunk_event in saved_chunk_event_reader.read() {
        saved_chunk_events.push(saved_chunk_event);
    }

    for upgraded_to_chunk_event in upgraded_to_chunk_events {
        let chunk_id = match upgraded_to_chunk_event.0 {
            ChunkResponse::Success { chunk_id, .. } => chunk_id,
            ChunkResponse::Failure { chunk_id, .. } => chunk_id,
        };

        if chunk_loader.currently_upgrading_to_chunks().contains(&chunk_id) {
            chunk_loader.stop_upgrading_to_chunk(chunk_id);
        }

        if chunk_registry.is_chunk_upgrading_to(chunk_id) {
            chunk_registry.stop_upgrading_to_chunk(chunk_id);
        }
    }

    for loaded_chunk_event in loaded_chunk_events {
        let chunk_id = match loaded_chunk_event.0 {
            ChunkResponse::Success { chunk_id, .. } => chunk_id,
            ChunkResponse::Failure { chunk_id, .. } => chunk_id,
        };

        if chunk_loader.currently_loading_chunks().contains(&chunk_id) {
            chunk_loader.stop_loading_chunk(chunk_id);
        }

        if chunk_registry.is_chunk_loading(chunk_id) {
            chunk_registry.stop_loading_chunk(chunk_id);
        }
    }

    for saved_chunk_event in saved_chunk_events {
        let chunk_id = match saved_chunk_event.0 {
            ChunkResponse::Success { chunk_id, .. } => chunk_id,
            ChunkResponse::Failure { chunk_id, .. } => chunk_id,
        };

        if chunk_loader.currently_saving_chunks().contains(&chunk_id) {
            chunk_loader.stop_saving_chunk(chunk_id);
        }

        if chunk_registry.is_chunk_saving(chunk_id) {
            chunk_registry.stop_saving_chunk(chunk_id);
        }
    }
}