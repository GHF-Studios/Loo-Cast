use bevy::prelude::*;
use crate::{chunk::{actor::{id::structs::{ChunkActorID, ChunkActorRequestID}, position::structs::ChunkActorPosition, structs::ChunkActorRequest, ChunkActorRegistry, ChunkActorRequestRegistry, DowngradeFromChunkActor, UpgradeToChunkActor}, id::structs::ChunkID, position::structs::ChunkPosition, ChunkRegistry}, entity::{id::structs::EntityID, resources::EntityRegistry}};
use super::checks::*;

pub fn request_upgrade_to_chunk_actor(
    upgrade_to_chunk_actor_event_writer: &mut EventWriter<UpgradeToChunkActor>,
    chunk_registry: &mut ChunkRegistry,
    chunk_actor_registry: &mut ChunkActorRegistry,
    chunk_actor_request_registry: &mut ChunkActorRequestRegistry,
    entity_registry: &mut EntityRegistry,
    transform_query: &Query<&Transform>,
    chunk_actor_entity_id: EntityID,
) -> Option<(ChunkActorID, ChunkActorRequestID)> {
    let start_chunk_id = {
        let chunk_actor_entity = match entity_registry.get_loaded_entity_reference(&chunk_actor_entity_id) {
            Some(chunk_actor_entity) => chunk_actor_entity,
            None => {
                panic!("Entity reference '{:?}' is not loaded!", chunk_actor_entity_id);
            }
        };

        let transform = match transform_query.get(chunk_actor_entity) {
            Ok(transform) => transform,
            Err(_) => {
                panic!("Entity '{:?}' has no Transform!", chunk_actor_entity_id);
            }
        };

        let position = transform.translation;
        let chunk_actor_pos: ChunkActorPosition = position.into();
        let chunk_pos: ChunkPosition = chunk_actor_pos.into();
        let chunk_id: ChunkID = chunk_pos.into();

        chunk_id
    };

    let chunk_actor_request_id = chunk_actor_request_registry.register_chunk_actor_request();
    let chunk_actor_id = chunk_actor_registry.register_chunk_actor();

    if !can_request_upgrade_to_chunk_actor(chunk_registry, chunk_actor_registry, entity_registry, start_chunk_id, chunk_actor_id, chunk_actor_entity_id) {
        return None;
    }

    let upgrade_to_chunk_actor_request = ChunkActorRequest {
        chunk_actor_request_id,
        chunk_actor_id,
        chunk_actor_entity_id,
    };

    chunk_actor_registry.start_upgrading_to_chunk_actor(chunk_actor_id);
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
    let chunk_actor_request_id = chunk_actor_request_registry.register_chunk_actor_request();

    if !can_request_downgrade_from_chunk_actor(chunk_actor_registry, entity_registry, chunk_actor_id) {
        return None;
    }

    let chunk_actor_entity_id = {
        let chunk_actor_entity = match chunk_actor_registry.get_loaded_chunk_actor_entity(chunk_actor_id) {
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
    chunk_actor_request_registry.load_chunk_actor_request(chunk_actor_request_id, downgrade_from_chunk_actor_request);
    downgrade_from_chunk_actor_event_writer.send(DowngradeFromChunkActor(downgrade_from_chunk_actor_request));

    Some(chunk_actor_request_id)
}