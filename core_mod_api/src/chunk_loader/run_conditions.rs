use bevy::prelude::*;

use crate::chunk_loader::components::ChunkLoader;

pub fn run_if_chunk_loader_spawned(chunk_loader_query: Query<&ChunkLoader>) -> bool {
    chunk_loader_query.iter().count() == 1
}
