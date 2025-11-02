use bevy::prelude::*;

use crate::{chunk_loader::components::ChunkLoader, utils::lifecycle_hook::DropHook};

pub fn run_if_chunk_loader_spawned(chunk_loader_query: Query<&ChunkLoader, Without<DropHook<ChunkLoader>>>) -> bool {
    chunk_loader_query.single().is_ok()
}
