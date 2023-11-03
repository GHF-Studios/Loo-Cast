// External imports
use bevy::prelude::*;

// Structs
#[derive(Component)]
pub struct Chunk {
    pub local_chunk_pos: ChunkPos,
}

pub struct ChunkPos {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct ChunkViewer {
    pub current_viewed_chunks: Vec<ChunkPos>,
    pub previous_viewed_chunks: Vec<ChunkPos>,
}