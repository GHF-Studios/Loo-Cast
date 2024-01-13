// Modules

// Local imports

// Internal imports
use crate::system::universe::chunk::pos::*;
use crate::system::universe::chunk::*;

// External imports
use bevy::prelude::*;

// Static variables

// Constant variables

// Types

// Enums

// Structs
#[derive(Clone, PartialEq, Debug)]
#[derive(Default)]
pub struct EntityPos {
    parent_chunk_pos: ChunkPos,
    local_pos: LocalEntityPos,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct LocalEntityPos {
    pub x: f32,
    pub y: f32,
}

// Implementations


impl EntityPos {
    pub fn new(parent_chunk_pos: ChunkPos, local_pos: LocalEntityPos) -> Self {
        EntityPos {
            parent_chunk_pos,
            local_pos,
        }
    }

    pub fn get_parent_chunk_pos(&self) -> &ChunkPos {
        &self.parent_chunk_pos
    }

    pub fn get_local_pos(&self) -> &LocalEntityPos {
        &self.local_pos
    }

    pub fn set_parent_chunk_pos(&mut self, parent_chunk_pos: ChunkPos) {
        self.parent_chunk_pos = parent_chunk_pos;
    }

    pub fn set_local_pos(&mut self, local_pos: LocalEntityPos) {
        self.local_pos = local_pos;
    }
}

impl From<AbsoluteLocalChunkPos> for LocalEntityPos {
    fn from(absolute_local_chunk_pos: AbsoluteLocalChunkPos) -> Self {
        LocalEntityPos {
            x: absolute_local_chunk_pos.x as f32 * CHUNK_SIZE as f32,
            y: absolute_local_chunk_pos.y as f32 * CHUNK_SIZE as f32,
        }
    }
}

impl From<ApparentLocalChunkPos> for LocalEntityPos {
    fn from(apparent_local_chunk_pos: ApparentLocalChunkPos) -> Self {
        LocalEntityPos {
            x: apparent_local_chunk_pos.x as f32 * CHUNK_SIZE as f32,
            y: apparent_local_chunk_pos.y as f32 * CHUNK_SIZE as f32,
        }
    }
}

impl From<Vec2> for LocalEntityPos {
    fn from(vec2: Vec2) -> Self {
        LocalEntityPos {
            x: vec2.x,
            y: vec2.y,
        }
    }
}

impl From<Vec3> for LocalEntityPos {
    fn from(vec3: Vec3) -> Self {
        LocalEntityPos {
            x: vec3.x,
            y: vec3.y,
        }
    }
}

impl From<LocalEntityPos> for Vec2 {
    fn from(val: LocalEntityPos) -> Self {
        Vec2::new(val.x, val.y)
    }
}

impl From<LocalEntityPos> for Vec3 {
    fn from(val: LocalEntityPos) -> Self {
        Vec3::new(val.x, val.y, 0.0)
    }
}

impl Default for LocalEntityPos {
    fn default() -> Self {
        LocalEntityPos { x: 0.0, y: 0.0 }
    }
}

impl LocalEntityPos {
    pub fn new(x: f32, y: f32) -> Self {
        LocalEntityPos { x, y }
    }
}

// Module Functions
