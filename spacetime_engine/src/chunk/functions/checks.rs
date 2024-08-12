use crate::chunk::id::structs::ChunkID as ChunkID;
use crate::entity::id::structs::EntityID;
use crate::entity::resources::EntityRegistry;
use crate::chunk::ChunkRegistry;

pub fn can_request_upgrade_to_chunk(
    chunk_registry: &mut ChunkRegistry,
    entity_registry: &mut EntityRegistry,
    chunk_id: ChunkID,
    entity_id: EntityID,
) -> bool {
    let mut result = true;

    if !entity_registry.is_entity_registered(entity_id) { result = false; }
    if !entity_registry.is_entity_loaded(entity_id) { result = false; }

    if entity_registry.is_entity_creating(entity_id) { result = false; }
    if entity_registry.is_entity_destroying(entity_id) { result = false; }
    if entity_registry.is_entity_loading(entity_id) { result = false; }
    if entity_registry.is_entity_saving(entity_id) { result = false; }

    if chunk_registry.is_chunk_registered(chunk_id) { result = false; }
    if chunk_registry.is_chunk_loaded(chunk_id) { result = false; }

    if chunk_registry.is_chunk_upgrading_to(chunk_id) { result = false; }
    if chunk_registry.is_chunk_downgrading_from(chunk_id) { result = false; }
    if chunk_registry.is_chunk_loading(chunk_id) { result = false; }
    if chunk_registry.is_chunk_saving(chunk_id) { result = false; }

    result
}

pub fn can_request_downgrade_from_chunk(
    chunk_registry: &mut ChunkRegistry,
    entity_registry: &mut EntityRegistry,
    chunk_id: ChunkID,
) -> bool {
    let mut result = true;

    let entity_id = {
        let entity_reference = match chunk_registry.get_loaded_chunk_entity(chunk_id) {
            Some(chunk_entity) => chunk_entity,
            None => panic!("Chunk entity '{:?}' is not loaded!", chunk_id)
        };

        match entity_registry.get_loaded_entity_id(&entity_reference) {
            Some(entity_id) => entity_id,
            None => panic!("Entity reference '{:?}' is not loaded!", entity_reference)
        }
    };

    if !entity_registry.is_entity_registered(entity_id) { result = false; }
    if !entity_registry.is_entity_loaded(entity_id) { result = false; }

    if entity_registry.is_entity_creating(entity_id) { result = false; }
    if entity_registry.is_entity_destroying(entity_id) { result = false; }
    if entity_registry.is_entity_loading(entity_id) { result = false; }
    if entity_registry.is_entity_saving(entity_id) { result = false; }

    if !chunk_registry.is_chunk_registered(chunk_id) { result = false; }
    if !chunk_registry.is_chunk_loaded(chunk_id) { result = false; }

    if chunk_registry.is_chunk_upgrading_to(chunk_id) { result = false; }
    if chunk_registry.is_chunk_downgrading_from(chunk_id) { result = false; }
    if chunk_registry.is_chunk_loading(chunk_id) { result = false; }
    if chunk_registry.is_chunk_saving(chunk_id) { result = false; }

    result
}

pub fn can_request_load_chunk(
    chunk_registry: &mut ChunkRegistry,
    chunk_id: ChunkID,
) -> bool {
    let mut result = true;

    if chunk_registry.is_chunk_registered(chunk_id) { result = false; }
    if chunk_registry.is_chunk_loaded(chunk_id) { result = false; }

    if chunk_registry.is_chunk_upgrading_to(chunk_id) { result = false; }
    if chunk_registry.is_chunk_downgrading_from(chunk_id) { result = false; }
    if chunk_registry.is_chunk_loading(chunk_id) { result = false; }
    if chunk_registry.is_chunk_saving(chunk_id) { result = false; }

    result
}

pub fn can_request_save_chunk(
    chunk_registry: &mut ChunkRegistry,
    entity_registry: &mut EntityRegistry,
    chunk_id: ChunkID,
) -> bool {
    let mut result = true;

    let entity_id = {
        let entity_reference = match chunk_registry.get_loaded_chunk_entity(chunk_id) {
            Some(chunk_entity) => chunk_entity,
            None => panic!("Chunk entity '{:?}' is not loaded!", chunk_id)
        };

        match entity_registry.get_loaded_entity_id(&entity_reference) {
            Some(entity_id) => entity_id,
            None => panic!("Entity reference '{:?}' is not loaded!", entity_reference)
        }
    };

    if !entity_registry.is_entity_registered(entity_id) { result = false; }
    if !entity_registry.is_entity_loaded(entity_id) { result = false; }

    if entity_registry.is_entity_creating(entity_id) { result = false; }
    if entity_registry.is_entity_destroying(entity_id) { result = false; }
    if entity_registry.is_entity_loading(entity_id) { result = false; }
    if entity_registry.is_entity_saving(entity_id) { result = false; }

    if !chunk_registry.is_chunk_registered(chunk_id) { result = false; }
    if !chunk_registry.is_chunk_loaded(chunk_id) { result = false; }

    if chunk_registry.is_chunk_upgrading_to(chunk_id) { result = false; }
    if chunk_registry.is_chunk_downgrading_from(chunk_id) { result = false; }
    if chunk_registry.is_chunk_loading(chunk_id) { result = false; }
    if chunk_registry.is_chunk_saving(chunk_id) { result = false; }

    result
}
