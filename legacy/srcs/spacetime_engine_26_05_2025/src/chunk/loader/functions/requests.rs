use bevy::prelude::*;
use crate::{chunk::loader::{components::ChunkLoader, id::structs::{ChunkLoaderID, ChunkLoaderRequestID}, structs::ChunkLoaderRequest, ChunkLoaderRegistry, ChunkLoaderRequestRegistry, DowngradeFromChunkLoader, UpgradeToChunkLoader}, entity::{id::structs::EntityID, resources::EntityRegistry}};
use super::checks::*;

pub fn request_upgrade_to_chunk_loader(
    upgrade_to_chunk_loader_event_writer: &mut EventWriter<UpgradeToChunkLoader>,
    chunk_loader_registry: &mut ChunkLoaderRegistry,
    chunk_loader_request_registry: &mut ChunkLoaderRequestRegistry,
    entity_registry: &mut EntityRegistry,
    chunk_loader_entity_id: EntityID,
) -> Option<(ChunkLoaderID, ChunkLoaderRequestID)> {
    let chunk_loader_request_id = chunk_loader_request_registry.register_chunk_loader_request();
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
    chunk_loader_request_registry.load_chunk_loader_request(chunk_loader_request_id, upgrade_to_chunk_loader_request);
    upgrade_to_chunk_loader_event_writer.send(UpgradeToChunkLoader(upgrade_to_chunk_loader_request));

    Some((chunk_loader_id, chunk_loader_request_id))
}

pub fn request_downgrade_from_chunk_loader(
    downgrade_from_chunk_loader_event_writer: &mut EventWriter<DowngradeFromChunkLoader>,
    chunk_loader_registry: &mut ChunkLoaderRegistry,
    chunk_loader_request_registry: &mut ChunkLoaderRequestRegistry,
    entity_registry: &mut EntityRegistry,
    chunk_loader_query: &Query<&ChunkLoader>,
    chunk_loader_id: ChunkLoaderID,
) -> Option<ChunkLoaderRequestID> {
    let chunk_loader_request_id = chunk_loader_request_registry.register_chunk_loader_request();

    if !can_request_downgrade_from_chunk_loader(chunk_loader_registry, entity_registry, chunk_loader_query, chunk_loader_id) {
        return None;
    }

    let chunk_loader_entity_id = {
        let chunk_loader_entity = match chunk_loader_registry.get_loaded_chunk_loader_entity(chunk_loader_id) {
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
    chunk_loader_request_registry.load_chunk_loader_request(chunk_loader_request_id, downgrade_from_chunk_loader_request);
    downgrade_from_chunk_loader_event_writer.send(DowngradeFromChunkLoader(downgrade_from_chunk_loader_request));

    Some(chunk_loader_request_id)
}
