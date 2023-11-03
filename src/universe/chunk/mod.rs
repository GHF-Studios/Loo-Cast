// Modules


// Local imports


// Internal imports


// External imports
use bevy::prelude::*;

// Static variables


// Constant variables


// Types


// Enums


// Structs
pub struct ChunkPos {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Chunk {
    pub local_chunk_pos: ChunkPos,
}

#[derive(Component)]
pub struct ChunkViewer {
    pub current_viewed_chunks: Vec<ChunkPos>,
    pub previous_viewed_chunks: Vec<ChunkPos>,
}

// Implementations
impl Chunk {
    fn render_system(
        mut gizmos: Gizmos, 
        chunk_query: Query<&Chunk>,
        player_query: Query<&Transform, With<Player>>,
    ) {

    }
}

impl ChunkViewer {
    
}

// Module Functions
