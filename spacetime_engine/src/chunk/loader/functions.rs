use bevy::prelude::*;
use crate::{chunk::{self, id::structs::ChunkID, ChunkRegistry, ChunkRequestRegistry, LoadChunk, SaveChunk}, entity::{self, events::CreateEntity, resources::{EntityRegistry, EntityRequestRegistry}}};

use super::{components::ChunkLoader, constants::*, id::structs::*};

pub(super) fn new_chunk_loader_entity(
    world: &mut World, 
    chunk_loader_id: ChunkLoaderID,
    world_position: Vec2,
) -> Entity {
    world.spawn((
        Transform::from_translation(Vec3::new(world_position.x, world_position.y, 0.0)),
        ChunkLoader::new(chunk_loader_id, CHUNK_LOADER_LOAD_RADIUS),
    )).id()
}

pub(super) fn promote_chunk_loader_entity(
    world: &mut World, 
    chunk_loader_id: ChunkLoaderID, 
    target_entity_reference: Entity,
) -> Result<Entity, Entity> {
    
    let mut ineligible_entity_query_0: QueryState<Entity, Without<Transform>> = world.query_filtered::<Entity, Without<Transform>>();
    let mut ineligible_entity_query_1 = world.query_filtered::<Entity, With<ChunkLoader>>();
    let mut eligible_entity_query = world.query_filtered::<Entity, (With<Transform>, Without<ChunkLoader>)>();

    if ineligible_entity_query_0.get(world, target_entity_reference).is_ok() {
        error!("Entity '{:?}' does not have a Transform component!", target_entity_reference);

        return Err(target_entity_reference);
    };

    if ineligible_entity_query_1.get(world, target_entity_reference).is_ok() {
        error!("Entity '{:?}' already has a ChunkLoader component!", target_entity_reference);

        return Err(target_entity_reference);
    };

    if let Ok(eligible_entity) = eligible_entity_query.get_mut(world, target_entity_reference) {
        Ok(world.entity_mut(eligible_entity.clone()).insert(ChunkLoader::new(chunk_loader_id, CHUNK_LOADER_LOAD_RADIUS)).id())
    } else {
        error!("Entity does not exist or does not have a Transform component.");

        Err(target_entity_reference)
    }
}

pub(in crate) fn request_start_chunks(
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

        if !chunk_registry.is_chunk_registered(chunk_id) {
            let (entity_request_id, _) = entity::functions::request_create_entity(
                create_entity_event_writer, 
                entity_registry, 
                entity_request_registry
            );

            chunk_loader.start_preparing_entity_for_chunk_upgrade(chunk_id, entity_request_id);

            continue;
        }

        if chunk_registry.is_chunk_loaded(chunk_id) { continue; }
        if !chunk_loader.can_load_chunk(chunk_id) { continue };
        if !chunk::functions::can_request_load_chunk(chunk_registry, chunk_id) { continue };

        match chunk::functions::request_load_chunk(load_chunk_event_writer, chunk_registry, chunk_request_registry, chunk_id) {
            Some(_) => chunk_loader.start_loading_chunk(chunk_id),
            None => continue
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(in crate) fn request_update_chunks(
    create_entity_event_writer: &mut EventWriter<CreateEntity>,
    load_chunk_event_writer: &mut EventWriter<LoadChunk>,
    save_chunk_event_writer: &mut EventWriter<SaveChunk>,
    chunk_loader: &mut Mut<ChunkLoader>,
    chunk_registry: &mut ResMut<ChunkRegistry>,
    chunk_request_registry: &mut ResMut<ChunkRequestRegistry>,
    entity_registry: &mut EntityRegistry,
    entity_request_registry: &mut EntityRequestRegistry,
    old_chunk_ids: Vec<ChunkID>,
    new_chunk_ids: Vec<ChunkID>,
) {
    for old_chunk_id in old_chunk_ids.iter() {
        debug!("Old chunk '{:?}' detected!", old_chunk_id);
        
        let chunk_id = old_chunk_id;

        if !chunk_registry.is_chunk_registered(*chunk_id) { continue; }
        if !chunk_registry.is_chunk_loaded(*chunk_id) { continue; }
        if !chunk_loader.can_save_chunk(*chunk_id) { continue; }
        if !chunk::functions::can_request_save_chunk(chunk_registry, entity_registry, *chunk_id) { continue; }

        match chunk::functions::request_save_chunk(save_chunk_event_writer, chunk_registry, chunk_request_registry, entity_registry, *chunk_id) {
            Some(_) => chunk_loader.start_saving_chunk(*chunk_id),
            None => continue
        }
    }

    for new_chunk_id in new_chunk_ids.iter() { 
        debug!("New chunk '{:?}' detected!", new_chunk_id);

        let chunk_id = new_chunk_id;

        if !chunk_registry.is_chunk_registered(*chunk_id) {
            let (entity_request_id, _) = entity::functions::request_create_entity(
                create_entity_event_writer, 
                entity_registry, 
                entity_request_registry
            );

            chunk_loader.start_preparing_entity_for_chunk_upgrade(*chunk_id, entity_request_id);

            continue;
        }

        if chunk_registry.is_chunk_loaded(*chunk_id) { continue; }
        if !chunk_loader.can_load_chunk(*chunk_id) { continue };
        if !chunk::functions::can_request_load_chunk(chunk_registry, *chunk_id) { continue };

        match chunk::functions::request_load_chunk(load_chunk_event_writer, chunk_registry, chunk_request_registry, *chunk_id) {
            Some(_) => chunk_loader.start_loading_chunk(*chunk_id),
            None => continue
        }
    }
}

pub(in crate) fn handle_updated_chunks(
    
) {

}

pub(in crate) fn handle_updated_chunks_OLD(
    mut created_chunk_internal_event_reader: EventReader<CreatedChunkEntityInternal>,
    mut loaded_chunk_internal_event_reader: EventReader<LoadedChunkEntityInternal>,
    mut unloaded_chunk_internal_event_reader: EventReader<UnloadedChunkEntityInternal>,
    chunk_loader: &mut Mut<ChunkLoader>,
    chunk_registry: &mut ResMut<ChunkRegistry>,
) {
    let mut created_chunk_internal_events = Vec::new();
    for created_chunk_event in created_chunk_internal_event_reader.read() {
        created_chunk_internal_events.push(created_chunk_event.clone());
    }

    let mut loaded_chunk_internal_events = Vec::new();
    for loaded_chunk_event in loaded_chunk_internal_event_reader.read() {
        loaded_chunk_internal_events.push(loaded_chunk_event.clone());
    }

    let mut unloaded_chunk_internal_events = Vec::new();
    for unloaded_chunk_event in unloaded_chunk_internal_event_reader.read() {
        unloaded_chunk_internal_events.push(unloaded_chunk_event.clone());
    }

    for created_chunk_internal_event in created_chunk_internal_events {
        let chunk_id = match created_chunk_internal_event {
            CreatedChunkEntityInternal::Success { chunk_id, .. } => chunk_id,
            CreatedChunkEntityInternal::Failure { chunk_id, .. } => chunk_id,
        };

        if chunk_loader.currently_upgrading_to_chunks().contains(&chunk_id) {
            chunk_loader.stop_upgrading_to_chunk(chunk_id);
        }

        if chunk_registry.is_chunk_being_created(chunk_id) {
            chunk_registry.stop_creating_chunk(chunk_id);
        }
    }

    for loaded_chunk_internal_event in loaded_chunk_internal_events {
        let chunk_id = match loaded_chunk_internal_event {
            LoadedChunkEntityInternal::Success { chunk_id, .. } => chunk_id,
            LoadedChunkEntityInternal::Failure { chunk_id, .. } => chunk_id,
        };

        if chunk_loader.currently_loading_chunks().contains(&chunk_id) {
            chunk_loader.stop_loading_chunk(chunk_id);
        }

        if chunk_registry.is_chunk_loading(chunk_id) {
            chunk_registry.stop_loading_chunk(chunk_id);
        }
    }

    for unloaded_chunk_internal_event in unloaded_chunk_internal_events {
        let chunk_id = match unloaded_chunk_internal_event {
            UnloadedChunkEntityInternal::Success { chunk_id, .. } => chunk_id,
            UnloadedChunkEntityInternal::Failure { chunk_id, .. } => chunk_id,
        
        };

        if chunk_loader.currently_saving_chunks().contains(&chunk_id) {
            chunk_loader.stop_saving_chunk(chunk_id);
        }

        if chunk_registry.is_chunk_saving(chunk_id) {
            chunk_registry.stop_unloading_chunk(chunk_id);
        }
    }
}