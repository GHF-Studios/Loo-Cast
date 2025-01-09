use bevy::prelude::*;

use crate::chunk::{enums::ChunkAction, resources::{ChunkActionBuffer, ChunkManager}};

pub(in crate) fn load_chunk(
    chunk_manager: &ChunkManager,
    chunk_action_buffer: &mut ChunkActionBuffer,
    chunk_coord: (i32, i32),
    chunk_owner: Option<Entity>
) {
    let is_loaded = chunk_manager.loaded_chunks.get(&chunk_coord).is_some();
    let is_owned = chunk_manager.owned_chunks.get(&chunk_coord).is_some();
    let is_spawning = chunk_action_buffer.is_spawning(&chunk_coord);
    let is_despawning = chunk_action_buffer.is_despawning(&chunk_coord);
    let is_transfering_ownership = chunk_action_buffer.is_transfering_ownership(&chunk_coord);

    if !is_loaded {
        if !is_spawning && !is_despawning && !is_transfering_ownership { 
            chunk_action_buffer.0.insert(chunk_coord, ChunkAction::Spawn { coord: chunk_coord, owner: chunk_owner });
        }
    } else if !is_owned && !is_despawning && !is_transfering_ownership && chunk_owner.is_some() {
        chunk_action_buffer.0.insert(chunk_coord, ChunkAction::TransferOwnership { coord: chunk_coord, new_owner: chunk_owner.unwrap() });
    }
}

pub(in crate) fn unload_chunk(
    chunk_manager: &ChunkManager,
    chunk_action_buffer: &mut ChunkActionBuffer,
    chunk_coord: (i32, i32)
) {
    let loaded = chunk_manager.is_loaded(&chunk_coord);
    let despawning = chunk_action_buffer.is_despawning(&chunk_coord);

    if loaded && !despawning {
        chunk_action_buffer.0.insert(chunk_coord, ChunkAction::Despawn { coord: chunk_coord });
    }
}