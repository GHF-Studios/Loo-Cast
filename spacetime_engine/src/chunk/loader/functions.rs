use bevy::prelude::*;
use crate::chunk::actor::position::structs::ChunkActorPosition;
use crate::chunk::events::*;
use crate::chunk::id::structs::ChunkID;
use crate::chunk::position::structs::ChunkPosition;
use crate::chunk::ChunkRegistry;
use crate::math::structs::I16Vec2;

pub(in crate::chunk::loader) fn detect_chunks(
    chunk_loader_transform: &Transform,
    chunk_loader_load_radius: u16,
) -> Vec<ChunkID> {
    let chunk_loader_chunk_actor_position: ChunkActorPosition = chunk_loader_transform.translation.into();
    let current_chunk_position: ChunkPosition = chunk_loader_chunk_actor_position.into();
    let load_radius = chunk_loader_load_radius as i16;
    
    let mut detected_chunk_ids = Vec::new();
    for x_offset in -load_radius..=load_radius {
        for y_offset in -load_radius..=load_radius {
            let chunk_position = current_chunk_position + ChunkPosition(I16Vec2(x_offset, y_offset));
            let chunk_id: ChunkID = chunk_position.into();

            detected_chunk_ids.push(chunk_id);
        }
    }

    detected_chunk_ids
}

pub(in crate::chunk::loader) fn categorize_chunks(
    detected_chunk_ids: Vec<ChunkID>,
    chunk_registry: &Res<ChunkRegistry>,
) -> (Vec<ChunkID>, Vec<ChunkID>, Vec<ChunkID>) {
    let mut old_chunks: Vec<ChunkID> = Vec::new();
    let mut unchanged_chunks: Vec<ChunkID> = Vec::new();
    let mut new_chunks: Vec<ChunkID> = Vec::new();

    for loaded_chunk_id in chunk_registry.loaded_chunk_ids() {
        if !detected_chunk_ids.contains(&loaded_chunk_id) {
            old_chunks.push(loaded_chunk_id);
        }
    }

    for detected_chunk_id in detected_chunk_ids {
        if chunk_registry.is_chunk_loaded(detected_chunk_id) {
            unchanged_chunks.push(detected_chunk_id);
        } else {
            new_chunks.push(detected_chunk_id);
        }
    }

    (old_chunks, unchanged_chunks, new_chunks)
}

pub(in crate::chunk::loader) fn start_chunks(
    mut create_chunk_event_writer: EventWriter<CreateChunk>,
    mut load_chunk_event_writer: EventWriter<LoadChunk>,
    chunk_registry: &Res<ChunkRegistry>,
    detected_chunk_ids: &Vec<ChunkID>,
) {
    for detected_chunk_id in detected_chunk_ids {
        let detected_chunk_id = *detected_chunk_id;

        if chunk_registry.is_chunk_registered(detected_chunk_id) {
            load_chunk_event_writer.send(LoadChunk { chunk_id: detected_chunk_id });
        } else {
            create_chunk_event_writer.send(CreateChunk { chunk_id: detected_chunk_id });
        }
    }
}

pub(in crate::chunk::loader) fn update_chunks(
    old_chunk_ids: Vec<ChunkID>,
    new_chunk_ids: Vec<ChunkID>,
    chunk_registry: &Res<ChunkRegistry>,
    mut create_chunk_event_writer: EventWriter<CreateChunk>,
    mut load_chunk_event_writer: EventWriter<LoadChunk>,
    mut unload_chunk_event_writer: EventWriter<UnloadChunk>,
) {
    for old_chunk_id in old_chunk_ids {
        unload_chunk_event_writer.send(UnloadChunk { chunk_id: old_chunk_id });
    }

    for new_chunk_id in new_chunk_ids.iter() {
        let new_chunk_id = *new_chunk_id;
        
        if chunk_registry.is_chunk_registered(new_chunk_id) {
            load_chunk_event_writer.send(LoadChunk { chunk_id: new_chunk_id });
        } else {
            create_chunk_event_writer.send(CreateChunk { chunk_id: new_chunk_id });
        }
    }
}