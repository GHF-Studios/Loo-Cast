use bevy::prelude::*;
use crate::chunk::actor::resources::ChunkActorRegistry;
use crate::chunk::components::Chunk;
use crate::chunk::id::structs::ChunkID as ChunkID;
use crate::chunk::events::*;
use crate::entity::id::structs::EntityID;
use crate::entity::resources::EntityRegistry;
use crate::chunk::id::structs::ChunkRequestID;
use crate::chunk::functions::checks::*;
use crate::chunk::structs::ChunkRequest;
use crate::chunk::{ChunkRequestRegistry, ChunkRegistry};

pub fn request_upgrade_to_chunk(
    upgrade_to_chunk_event_writer: &mut EventWriter<UpgradeToChunk>,
    chunk_registry: &mut ChunkRegistry,
    chunk_request_registry: &mut ChunkRequestRegistry,
    entity_registry: &mut EntityRegistry,
    chunk_id: ChunkID,
    chunk_entity_id: EntityID,
) -> Option<ChunkRequestID> {
    let chunk_request_id = chunk_request_registry.register_chunk_request();

    if !can_request_upgrade_to_chunk(chunk_registry, entity_registry, chunk_id, chunk_entity_id) {
        return None;
    }

    let upgrade_to_chunk_request = ChunkRequest {
        chunk_request_id,
        chunk_id,
        chunk_entity_id,
    };

    chunk_registry.start_upgrading_to_chunk(chunk_id);
    chunk_request_registry.load_chunk_request(chunk_request_id, upgrade_to_chunk_request);
    upgrade_to_chunk_event_writer.send(UpgradeToChunk(upgrade_to_chunk_request));

    Some(chunk_request_id)
}

pub fn request_downgrade_from_chunk(
    downgrade_from_chunk_event_writer: &mut EventWriter<DowngradeFromChunk>,
    chunk_registry: &mut ChunkRegistry,
    chunk_request_registry: &mut ChunkRequestRegistry,
    chunk_actor_registry: &mut ChunkActorRegistry,
    entity_registry: &mut EntityRegistry,
    chunk_query: &Query<&Chunk>,
    chunk_id: ChunkID,
) -> Option<ChunkRequestID> {
    let chunk_request_id = chunk_request_registry.register_chunk_request();

    if !can_request_downgrade_from_chunk(chunk_registry, chunk_actor_registry, entity_registry, chunk_query, chunk_id) {
        return None;
    }

    let chunk_entity_id = {
        let chunk_entity = match chunk_registry.get_loaded_chunk_entity(chunk_id) {
            Some(chunk_entity) => chunk_entity,
            None => panic!("Chunk entity '{:?}' is not loaded!", chunk_id)
        };

        match entity_registry.get_loaded_entity_id(&chunk_entity) {
            Some(chunk_entity_id) => chunk_entity_id,
            None => panic!("Entity ID associated with chunk entity '{:?}' is not loaded!", chunk_entity)
        }
    };

    let downgrade_from_chunk_request = ChunkRequest {
        chunk_request_id,
        chunk_id,
        chunk_entity_id
    };

    chunk_registry.start_downgrading_from_chunk(chunk_id);
    chunk_request_registry.load_chunk_request(chunk_request_id, downgrade_from_chunk_request);
    downgrade_from_chunk_event_writer.send(DowngradeFromChunk(downgrade_from_chunk_request));

    Some(chunk_request_id)
}

pub fn request_load_chunk(
    load_chunk_event_writer: &mut EventWriter<LoadChunk>,
    chunk_registry: &mut ChunkRegistry,
    chunk_request_registry: &mut ChunkRequestRegistry,
    chunk_id: ChunkID,
) -> Option<ChunkRequestID> {
    let chunk_request_id = chunk_request_registry.register_chunk_request();

    if !can_request_load_chunk(chunk_registry, chunk_id) {
        return None;
    }

    let chunk_entity_id = match chunk_registry.serialized_chunks().get(&chunk_id).map(|(entity_id, _)| {
        *entity_id
    }) {
        Some(chunk_entity_id) => chunk_entity_id,
        None => panic!("Chunk entity ID associated with chunk '{:?}' not found!", chunk_id)
    };

    let load_chunk_request = ChunkRequest {
        chunk_request_id,
        chunk_id,
        chunk_entity_id
    };

    chunk_registry.start_loading_chunk(chunk_id);
    chunk_request_registry.load_chunk_request(chunk_request_id, load_chunk_request);
    load_chunk_event_writer.send(LoadChunk(load_chunk_request));

    Some(chunk_request_id)
}

pub fn request_save_chunk(
    save_chunk_event_writer: &mut EventWriter<SaveChunk>,
    chunk_registry: &mut ChunkRegistry,
    chunk_request_registry: &mut ChunkRequestRegistry,
    chunk_actor_registry: &mut ChunkActorRegistry,
    entity_registry: &mut EntityRegistry,
    chunk_query: &Query<&Chunk>,
    chunk_id: ChunkID,
) -> Option<ChunkRequestID> {
    let chunk_request_id = chunk_request_registry.register_chunk_request();

    if !can_request_save_chunk(chunk_registry, chunk_actor_registry, entity_registry, chunk_query, chunk_id) {
        return None;
    }

    let chunk_entity_id = {
        let chunk_entity = match chunk_registry.get_loaded_chunk_entity(chunk_id) {
            Some(chunk_entity) => chunk_entity,
            None => panic!("Chunk entity '{:?}' is not loaded!", chunk_id)
        };

        let chunk_entity_id = match entity_registry.get_loaded_entity_id(&chunk_entity) {
            Some(chunk_entity_id) => chunk_entity_id,
            None => panic!("Entity ID associated with chunk entity '{:?}' is not loaded!", chunk_entity)
        };

        chunk_entity_id
    };

    let unload_chunk_request = ChunkRequest {
        chunk_request_id,
        chunk_id,
        chunk_entity_id,
    };

    chunk_registry.start_saving_chunk(chunk_id);
    chunk_request_registry.load_chunk_request(chunk_request_id, unload_chunk_request);
    save_chunk_event_writer.send(SaveChunk(unload_chunk_request));

    Some(chunk_request_id)
}
