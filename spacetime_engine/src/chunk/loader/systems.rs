use bevy::prelude::*;
use crate::chunk::events::*;
use crate::chunk::resources::*;
use crate::chunk::loader::components::*;
use crate::chunk::loader::events::*;
use crate::chunk::loader::functions;	

// TODO: ???????
// Maybe we should control startup not via the startup schedule but via some startup event or whatever
// See player/functions.rs
pub(in crate) fn startup(
    create_chunk_event_writer: EventWriter<CreateChunk>,
    load_chunk_event_writer: EventWriter<LoadChunk>,
    mut started_chunk_loader_event_writer: EventWriter<StartedChunkLoader>,
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

    started_chunk_loader_event_writer.send(StartedChunkLoader { chunk_loader_id });
}

pub(in crate) fn update(
    create_chunk_event_writer: EventWriter<CreateChunk>,
    load_chunk_event_writer: EventWriter<LoadChunk>,
    unload_chunk_event_writer: EventWriter<UnloadChunk>,
    mut updated_chunk_loader_event_writer: EventWriter<UpdatedChunkLoader>,
    mut chunk_loader_query: Query<(&Transform, &mut ChunkLoader)>,
    chunk_registry: Res<ChunkRegistry>,
) {
    let (chunk_loader_transform, mut chunk_loader) = chunk_loader_query.single_mut();
    let chunk_loader_id = chunk_loader.id();
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

    updated_chunk_loader_event_writer.send(UpdatedChunkLoader { chunk_loader_id });
}