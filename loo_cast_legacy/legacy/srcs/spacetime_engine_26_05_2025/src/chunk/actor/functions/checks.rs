use crate::{chunk::{actor::{id::structs::ChunkActorID, ChunkActorRegistry}, id::structs::ChunkID, ChunkRegistry}, entity::{id::structs::EntityID, resources::EntityRegistry}};

pub fn can_request_upgrade_to_chunk_actor(
    chunk_registry: &mut ChunkRegistry,
    chunk_actor_registry: &mut ChunkActorRegistry,
    entity_registry: &mut EntityRegistry,
    start_chunk_id: ChunkID,
    chunk_actor_id: ChunkActorID,
    entity_id: EntityID,
) -> bool {
    let mut result = true;

    if !chunk_registry.is_chunk_registered(start_chunk_id) { result = false; }
    if !chunk_registry.is_chunk_loaded(start_chunk_id) { result = false; }

    if chunk_registry.is_chunk_upgrading_to(start_chunk_id) { result = false; }
    if chunk_registry.is_chunk_downgrading_from(start_chunk_id) { result = false; }
    if chunk_registry.is_chunk_loading(start_chunk_id) { result = false; }
    if chunk_registry.is_chunk_saving(start_chunk_id) { result = false; }

    if !entity_registry.is_entity_registered(entity_id) { result = false; }
    if !entity_registry.is_entity_loaded(entity_id) { result = false; }

    if entity_registry.is_entity_creating(entity_id) { result = false; }
    if entity_registry.is_entity_destroying(entity_id) { result = false; }
    if entity_registry.is_entity_loading(entity_id) { result = false; }
    if entity_registry.is_entity_saving(entity_id) { result = false; }

    if chunk_actor_registry.is_chunk_actor_registered(chunk_actor_id) { result = false; }
    if chunk_actor_registry.is_chunk_actor_loaded(chunk_actor_id) { result = false; }

    if chunk_actor_registry.is_chunk_actor_upgrading_to(chunk_actor_id) { result = false; }
    if chunk_actor_registry.is_chunk_actor_downgrading_from(chunk_actor_id) { result = false; }
    if chunk_actor_registry.is_chunk_actor_loading(chunk_actor_id) { result = false; }
    if chunk_actor_registry.is_chunk_actor_saving(chunk_actor_id) { result = false; }

    result
}

pub fn can_request_downgrade_from_chunk_actor(
    chunk_actor_registry: &mut ChunkActorRegistry,
    entity_registry: &mut EntityRegistry,
    chunk_actor_id: ChunkActorID,
) -> bool {
    let mut result = true;

    let entity_id = {
        let entity_reference = match chunk_actor_registry.get_loaded_chunk_actor_entity(chunk_actor_id) {
            Some(chunk_actor_entity) => chunk_actor_entity,
            None => panic!("Chunk actor entity '{:?}' is not loaded!", chunk_actor_id)
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

    if !chunk_actor_registry.is_chunk_actor_registered(chunk_actor_id) { result = false; }
    if !chunk_actor_registry.is_chunk_actor_loaded(chunk_actor_id) { result = false; }

    if chunk_actor_registry.is_chunk_actor_upgrading_to(chunk_actor_id) { result = false; }
    if chunk_actor_registry.is_chunk_actor_downgrading_from(chunk_actor_id) { result = false; }
    if chunk_actor_registry.is_chunk_actor_loading(chunk_actor_id) { result = false; }
    if chunk_actor_registry.is_chunk_actor_saving(chunk_actor_id) { result = false; }

    result
}