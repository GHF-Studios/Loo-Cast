use bevy::prelude::*;

use crate::{chunk::{loader::{components::ChunkLoader, id::structs::ChunkLoaderID, ChunkLoaderRegistry}, ChunkRegistry}, entity::{id::structs::EntityID, resources::EntityRegistry}};

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
    if chunk_loader_registry.is_chunk_loader_loading(chunk_loader_id) { result = false; }
    if chunk_loader_registry.is_chunk_loader_saving(chunk_loader_id) { result = false; }

    result
}

pub fn can_request_downgrade_from_chunk_loader(
    chunk_registry: &mut ChunkRegistry,
    chunk_loader_registry: &mut ChunkLoaderRegistry,
    entity_registry: &mut EntityRegistry,
    chunk_loader_query: &Query<&ChunkLoader>,
    chunk_loader_id: ChunkLoaderID,
) -> bool {
    let mut result = true;

    let (entity_id, entity_reference) = {
        let entity_reference = match chunk_loader_registry.get_loaded_chunk_loader_entity(chunk_loader_id) {
            Some(chunk_loader_entity) => chunk_loader_entity,
            None => panic!("Chunk loader entity '{:?}' is not loaded!", chunk_loader_id)
        };

        match entity_registry.get_loaded_entity_id(&entity_reference) {
            Some(entity_id) => (entity_id, entity_reference),
            None => panic!("Entity reference '{:?}' is not loaded!", entity_reference)
        }
    };

    let activated_chunks = {
        let chunk_loader = match chunk_loader_query.get(entity_reference) {
            Ok(chunk_loader) => chunk_loader,
            Err(_) => panic!("Entity '{:?}' has no ChunkLoader!", entity_id)
        };

        chunk_loader.current_chunk_ids()
    };

    if activated_chunks.len() > 0 { result = false; }

    if !entity_registry.is_entity_registered(entity_id) { result = false; }
    if !entity_registry.is_entity_loaded(entity_id) { result = false; }

    if entity_registry.is_entity_creating(entity_id) { result = false; }
    if entity_registry.is_entity_destroying(entity_id) { result = false; }
    if entity_registry.is_entity_loading(entity_id) { result = false; }
    if entity_registry.is_entity_saving(entity_id) { result = false; }

    if !chunk_loader_registry.is_chunk_loader_registered(chunk_loader_id) { result = false; }
    if !chunk_loader_registry.is_chunk_loader_loaded(chunk_loader_id) { result = false; }

    if chunk_loader_registry.is_chunk_loader_upgrading_to(chunk_loader_id) { result = false; }
    if chunk_loader_registry.is_chunk_loader_downgrading_from(chunk_loader_id) { result = false; }
    if chunk_loader_registry.is_chunk_loader_loading(chunk_loader_id) { result = false; }
    if chunk_loader_registry.is_chunk_loader_saving(chunk_loader_id) { result = false; }

    result
}
