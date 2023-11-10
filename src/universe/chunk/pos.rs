// Modules


// Local imports


// Internal imports
use crate::universe::chunk::*;
use crate::universe::entity::pos::*;

// External imports


// Static variables


// Constant variables


// Types


// Enums


// Structs
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ChunkPos {
    parent_pos: Option<Box<ChunkPos>>,
    local_pos: LocalChunkPos,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct LocalChunkPos {
    pub x: i8,
    pub y: i8
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

impl From<(u8, u8)> for LocalChunkPos {
    fn from((x, y): (u8, u8)) -> LocalChunkPos {
        LocalChunkPos {
            x: x as i8,
            y: y as i8,
        }
    }
}

impl Into<(u8, u8)> for LocalChunkPos {
    fn into(self) -> (u8, u8) {
        (self.x as u8, self.y as u8)
    }
}

impl LocalChunkPos {
    pub fn new(x: i8, y: i8) -> Self {
        LocalChunkPos {
            x,
            y,
        }
    }
}

// Module Functions
