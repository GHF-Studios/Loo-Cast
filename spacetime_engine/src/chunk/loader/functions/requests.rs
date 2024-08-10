use bevy::prelude::*;

use crate::{chunk::loader::{id::structs::{ChunkLoaderID, ChunkLoaderRequestID}, structs::ChunkLoaderRequest, ChunkLoaderRegistry, ChunkLoaderRequestRegistry, DowngradeFromChunkLoader, UpgradeToChunkLoader}, entity::{id::structs::EntityID, resources::EntityRegistry}};

pub fn can_request_upgrade_to_chunk_loader(
    chunk_loader_registry: &mut ChunkLoaderRegistry,
    entity_registry: &mut EntityRegistry,
    chunk_loader_id: ChunkLoaderID,
    entity_id: EntityID,
) -> bool {
    let mut result = true;

    if !entity_registry.is_entity_registered(entity_id) { result = false; }
    if !entity_registry.is_entity_loaded(entity_id) { result = false; }

    if entity_registry.is_entity_creating(entity_id) { result = false; }
    if entity_registry.is_entity_destroying(entity_id) { result = false; }
    if entity_registry.is_entity_loading(entity_id) { result = false; }
    if entity_registry.is_entity_saving(entity_id) { result = false; }

    if chunk_loader_registry.is_chunk_loader_registered(chunk_loader_id) { result = false; }
    if chunk_loader_registry.is_chunk_loader_loaded(chunk_loader_id) { result = false; }

    if chunk_loader_registry.is_chunk_loader_upgrading_to(chunk_loader_id) { result = false; }
    if chunk_loader_registry.is_chunk_loader_downgrading_from(chunk_loader_id) { result = false; }

    result
}

pub fn can_request_downgrade_from_chunk_loader(
    chunk_loader_registry: &mut ChunkLoaderRegistry,
    entity_registry: &mut EntityRegistry,
    chunk_loader_id: ChunkLoaderID,
) -> bool {
    let mut result = true;

    let chunk_loader_entity_id = {
        let chunk_loader_entity = match chunk_loader_registry.get_loaded_chunk_loader(chunk_loader_id) {
            Some(chunk_loader_entity) => chunk_loader_entity,
            None => { return false; }
        };

        match entity_registry.get_loaded_entity_id(&chunk_loader_entity) {
            Some(chunk_loader_entity_id) => chunk_loader_entity_id,
            None => { return false; }
        }
    };

    if !entity_registry.is_entity_registered(chunk_loader_entity_id) { result = false; }
    if !entity_registry.is_entity_loaded(chunk_loader_entity_id) { result = false; }

    if entity_registry.is_entity_creating(chunk_loader_entity_id) { result = false; }
    if entity_registry.is_entity_destroying(chunk_loader_entity_id) { result = false; }
    if entity_registry.is_entity_loading(chunk_loader_entity_id) { result = false; }
    if entity_registry.is_entity_saving(chunk_loader_entity_id) { result = false; }

    if !chunk_loader_registry.is_chunk_loader_registered(chunk_loader_id) { result = false; }
    if !chunk_loader_registry.is_chunk_loader_loaded(chunk_loader_id) { result = false; }

    if chunk_loader_registry.is_chunk_loader_upgrading_to(chunk_loader_id) { result = false; }
    if chunk_loader_registry.is_chunk_loader_downgrading_from(chunk_loader_id) { result = false; }

    result
}

pub fn request_upgrade_to_chunk_loader(
    upgrade_to_chunk_loader_event_writer: &mut EventWriter<UpgradeToChunkLoader>,
    chunk_loader_registry: &mut ChunkLoaderRegistry,
    chunk_loader_request_registry: &mut ChunkLoaderRequestRegistry,
    entity_registry: &mut EntityRegistry,
    chunk_loader_entity_id: EntityID,
) -> Option<(ChunkLoaderID, ChunkLoaderRequestID)> {
    let chunk_loader_request_id = chunk_loader_request_registry.get_unused_chunk_loader_request_id();
    let chunk_loader_id = chunk_loader_registry.register_chunk_loader();

    if !can_request_upgrade_to_chunk_loader(chunk_loader_registry, entity_registry, chunk_loader_id, chunk_loader_entity_id) {
        return None;
    }

    let upgrade_to_chunk_loader_request = ChunkLoaderRequest {
        chunk_loader_request_id,
        chunk_loader_id,
        chunk_loader_entity_id,
    };

    chunk_loader_registry.start_upgrading_to_chunk_loader(chunk_loader_id);
    chunk_loader_request_registry.register_chunk_loader_request(upgrade_to_chunk_loader_request);
    chunk_loader_request_registry.load_chunk_loader_request(chunk_loader_request_id, upgrade_to_chunk_loader_request);
    upgrade_to_chunk_loader_event_writer.send(UpgradeToChunkLoader(upgrade_to_chunk_loader_request));

    Some((chunk_loader_id, chunk_loader_request_id))
}

pub fn request_downgrade_from_chunk_loader(
    downgrade_from_chunk_loader_event_writer: &mut EventWriter<DowngradeFromChunkLoader>,
    chunk_loader_registry: &mut ChunkLoaderRegistry,
    chunk_loader_request_registry: &mut ChunkLoaderRequestRegistry,
    entity_registry: &mut EntityRegistry,
    chunk_loader_id: ChunkLoaderID,
) -> Option<ChunkLoaderRequestID> {
    let chunk_loader_request_id = chunk_loader_request_registry.get_unused_chunk_loader_request_id();

    if !can_request_downgrade_from_chunk_loader(chunk_loader_registry, entity_registry, chunk_loader_id) {
        return None;
    }

    let chunk_loader_entity_id = {
        let chunk_loader_entity = match chunk_loader_registry.get_loaded_chunk_loader(chunk_loader_id) {
            Some(chunk_loader_entity) => chunk_loader_entity,
            None => { return None; }
        };

        match entity_registry.get_loaded_entity_id(&chunk_loader_entity) {
            Some(chunk_loader_entity_id) => chunk_loader_entity_id,
            None => { return None; }
        }
    };

    let downgrade_from_chunk_loader_request = ChunkLoaderRequest {
        chunk_loader_request_id,
        chunk_loader_id,
        chunk_loader_entity_id,
    };

    chunk_loader_registry.start_downgrading_from_chunk_loader(chunk_loader_id);
    chunk_loader_request_registry.register_chunk_loader_request(downgrade_from_chunk_loader_request);
    chunk_loader_request_registry.load_chunk_loader_request(chunk_loader_request_id, downgrade_from_chunk_loader_request);
    downgrade_from_chunk_loader_event_writer.send(DowngradeFromChunkLoader(downgrade_from_chunk_loader_request));

    Some(chunk_loader_request_id)
}