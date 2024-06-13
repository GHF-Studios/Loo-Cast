use bevy::prelude::*;
use crate::chunk::events::*;
use crate::chunk::resources::*;
use crate::chunk::loader::components::*;
use crate::chunk::loader::events::*;
use crate::chunk::functions;	

pub(in crate) fn start(
    create_chunk_event_writer: EventWriter<CreateChunkEntity>,
    load_chunk_event_writer: EventWriter<LoadChunkEntity>,
    mut started_chunk_loader_event_writer: EventWriter<StartChunkLoaderResult>,
    mut chunk_loader_query: Query<(&Transform, &mut ChunkLoader), Added<ChunkLoader>>,
    chunk_registry: Res<ChunkRegistry>,
) {
    let (chunk_loader_transform, mut chunk_loader) = chunk_loader_query.single_mut();
    let chunk_loader_id = chunk_loader.id();
    let chunk_loader_load_radius = chunk_loader.load_radius();
    
    let detected_chunk_ids = functions::detect_chunks(chunk_loader_transform, chunk_loader_load_radius);

    functions::start_chunks(
        create_chunk_event_writer, 
        load_chunk_event_writer, 
        &chunk_registry, 
        &detected_chunk_ids
    );

    *chunk_loader.current_chunk_ids_mut() = detected_chunk_ids;

    started_chunk_loader_event_writer.send(StartChunkLoaderResult::Success { chunk_loader_id });
}

pub(in crate) fn update(
    create_chunk_event_writer: EventWriter<CreateChunkEntity>,
    load_chunk_event_writer: EventWriter<LoadChunkEntity>,
    unload_chunk_event_writer: EventWriter<UnloadChunkEntity>,
    mut chunk_loader_query: Query<(&Transform, &mut ChunkLoader)>,
    chunk_registry: Res<ChunkRegistry>,
) {
    let (chunk_loader_transform, mut chunk_loader) = chunk_loader_query.single_mut();
    let chunk_loader_load_radius = chunk_loader.load_radius();

    let detected_chunk_ids = functions::detect_chunks(chunk_loader_transform, chunk_loader_load_radius);

    let (
        old_chunk_ids, 
        unchanged_chunk_ids, 
        new_chunk_ids
    ) = functions::categorize_chunks(detected_chunk_ids, &chunk_registry);

    functions::update_chunks(
        old_chunk_ids, 
        new_chunk_ids.clone(), 
        &chunk_registry, 
        create_chunk_event_writer, 
        load_chunk_event_writer, 
        unload_chunk_event_writer
    );

    *chunk_loader.current_chunk_ids_mut() = vec![unchanged_chunk_ids, new_chunk_ids].concat();
}

// TODO: Implement
pub(super) fn handle_create_chunk_loader_entity_events() {}

// TODO: Implement
pub(super) fn handle_destroy_chunk_loader_entity_events() {}

// TODO: Implement
pub(super) fn handle_upgrade_to_chunk_loader_entity_events() {}

// TODO: Implement
pub(super) fn process_create_chunk_loader_entity_requests() {}

// TODO: Implement
pub(super) fn process_upgrade_to_chunk_loader_requests() {}
