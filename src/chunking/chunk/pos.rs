// Modules


// Local imports


// Internal imports
use crate::chunking::chunk::*;
use crate::chunking::entity::pos::*;

// External imports


// Static variables


// Constant variables


// Types


// Enums


// Structs
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ChunkPos {
    parent_pos: Option<ChunkPos>,
    local_pos: LocalChunkPos,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct LocalChunkPos {
    x: i8,
    y: i8
}

// Implementations
impl ChunkPos {
    pub fn new(parent_pos: Option<ChunkPos>, local_pos: LocalChunkPos) -> Self {
        ChunkPos {
            parent_pos,
            local_pos,
        }
    }

    pub fn get_parent_pos(&self) -> Option<ChunkPos> {
        self.parent_pos
    }

    pub fn get_local_pos(&self) -> LocalChunkPos {
        self.local_pos
    }

    pub fn set_parent_pos(&mut self, parent_pos: Option<ChunkPos>) {
        self.parent_pos = parent_pos;
    }

    pub fn set_local_pos(&mut self, local_pos: LocalChunkPos) {
        self.local_pos = local_pos;
    }
}

impl From<LocalEntityPos> for LocalChunkPos {
    fn from(local_entity_pos: LocalEntityPos) -> LocalChunkPos {
        let half_chunk = (CHUNK_SIZE as f32) / 2.0;
        LocalChunkPos {
            x: ((local_entity_pos.x + half_chunk) / CHUNK_SIZE as f32).floor() as i32,
            y: ((local_entity_pos.y + half_chunk) / CHUNK_SIZE as f32).floor() as i32,
        }
    }
}

impl LocalChunkPos {
    pub fn new(x: i8, y: i8) -> Self {
        LocalChunkPos {
            x,
            y,
        }
    }

    pub fn get_x(&self) -> i8 {
        self.x
    }

    pub fn get_y(&self) -> i8 {
        self.y
    }

    pub fn set_x(&mut self, x: i8) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: i8) {
        self.y = y;
    }
}

// Module Functions
