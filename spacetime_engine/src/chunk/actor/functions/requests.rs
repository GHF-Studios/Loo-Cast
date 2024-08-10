use bevy::prelude::*;
use crate::{chunk::actor::{id::structs::{ChunkActorID, ChunkActorRequestID}, structs::ChunkActorRequest, ChunkActorRegistry, ChunkActorRequestRegistry, DowngradeFromChunkActor, UpgradeToChunkActor}, entity::{id::structs::EntityID, resources::EntityRegistry}};

pub fn can_request_upgrade_to_chunk_actor(
    chunk_actor_registry: &mut ChunkActorRegistry,
    entity_registry: &mut EntityRegistry,
    chunk_actor_id: ChunkActorID,
    entity_id: EntityID,
) -> bool {
    let mut result = true;

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

    result
}

pub fn can_request_downgrade_from_chunk_actor(
    chunk_actor_registry: &mut ChunkActorRegistry,
    entity_registry: &mut EntityRegistry,
    chunk_actor_id: ChunkActorID,
) -> bool {
    let mut result = true;

    let chunk_actor_entity_id = {
        let chunk_actor_entity = match chunk_actor_registry.get_loaded_chunk_actor(chunk_actor_id) {
            Some(chunk_actor_entity) => chunk_actor_entity,
            None => { return false; }
        };

        match entity_registry.get_loaded_entity_id(&chunk_actor_entity) {
            Some(chunk_actor_entity_id) => chunk_actor_entity_id,
            None => { return false; }
        }
    };

    if !entity_registry.is_entity_registered(chunk_actor_entity_id) { result = false; }
    if !entity_registry.is_entity_loaded(chunk_actor_entity_id) { result = false; }

    if entity_registry.is_entity_creating(chunk_actor_entity_id) { result = false; }
    if entity_registry.is_entity_destroying(chunk_actor_entity_id) { result = false; }
    if entity_registry.is_entity_loading(chunk_actor_entity_id) { result = false; }
    if entity_registry.is_entity_saving(chunk_actor_entity_id) { result = false; }

    if !chunk_actor_registry.is_chunk_actor_registered(chunk_actor_id) { result = false; }
    if !chunk_actor_registry.is_chunk_actor_loaded(chunk_actor_id) { result = false; }

    if chunk_actor_registry.is_chunk_actor_upgrading_to(chunk_actor_id) { result = false; }
    if chunk_actor_registry.is_chunk_actor_downgrading_from(chunk_actor_id) { result = false; }
}

pub fn request_upgrade_to_chunk_actor(
    upgrade_to_chunk_actor_event_writer: &mut EventWriter<UpgradeToChunkActor>,
    chunk_actor_registry: &mut ChunkActorRegistry,
    chunk_actor_request_registry: &mut ChunkActorRequestRegistry,
    entity_registry: &mut EntityRegistry,
    chunk_actor_entity_id: EntityID,
) -> Option<(ChunkActorID, ChunkActorRequestID)> {
    let chunk_actor_request_id = chunk_actor_request_registry.get_unused_chunk_actor_request_id();
    let chunk_actor_id = chunk_actor_registry.register_chunk_actor();

    if !can_request_upgrade_to_chunk_actor(chunk_actor_registry, entity_registry, chunk_actor_id, chunk_actor_entity_id) {
        return None;
    }

    let upgrade_to_chunk_actor_request = ChunkActorRequest {
        chunk_actor_request_id,
        chunk_actor_id,
        chunk_actor_entity_id,
    };

    chunk_actor_registry.start_upgrading_to_chunk_actor(chunk_actor_id);
    chunk_actor_request_registry.register_chunk_actor_request(chunk_actor_request_id);
    chunk_actor_request_registry.load_chunk_actor_request(chunk_actor_request_id, upgrade_to_chunk_actor_request);
    upgrade_to_chunk_actor_event_writer.send(UpgradeToChunkActor(upgrade_to_chunk_actor_request));

    Some((chunk_actor_id, chunk_actor_request_id))
}

pub fn request_downgrade_from_chunk_actor(
    downgrade_from_chunk_actor_event_writer: &mut EventWriter<DowngradeFromChunkActor>,
    chunk_actor_registry: &mut ChunkActorRegistry,
    chunk_actor_request_registry: &mut ChunkActorRequestRegistry,
    entity_registry: &mut EntityRegistry,
    chunk_actor_id: ChunkActorID,
) -> Option<ChunkActorRequestID> {
    let chunk_actor_request_id = chunk_actor_request_registry.get_unused_chunk_actor_request_id();

    if !can_request_downgrade_from_chunk_actor(chunk_actor_registry, entity_registry, chunk_actor_id) {
        return None;
    }

    let chunk_actor_entity_id = {
        let chunk_actor_entity = match chunk_actor_registry.get_loaded_chunk_actor(chunk_actor_id) {
            Some(chunk_actor_entity) => chunk_actor_entity,
            None => panic!("Chunk actor entity '{:?}' is not loaded!", chunk_actor_id)
        };

        match entity_registry.get_loaded_entity_id(&chunk_actor_entity) {
            Some(chunk_actor_entity_id) => chunk_actor_entity_id,
            None => panic!("Entity ID associated with chunk actor entity '{:?}' is not loaded!", chunk_actor_entity)
        }
    };

    let downgrade_from_chunk_actor_request = ChunkActorRequest {
        chunk_actor_request_id,
        chunk_actor_id,
        chunk_actor_entity_id,
    };

    chunk_actor_registry.start_downgrading_from_chunk_actor(chunk_actor_id);
    chunk_actor_request_registry.register_chunk_actor_request(chunk_actor_request_id);
    chunk_actor_request_registry.load_chunk_actor_request(chunk_actor_request_id, downgrade_from_chunk_actor_request);
    downgrade_from_chunk_actor_event_writer.send(DowngradeFromChunkActor(downgrade_from_chunk_actor_request));

    Some(chunk_actor_request_id)
}