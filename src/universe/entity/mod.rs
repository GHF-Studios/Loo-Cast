// Modules

// Local imports

// Internal imports
use crate::universe::chunk::*;

// External imports
use bevy::prelude::*;

// Static variables

// Constant variables

// Types

// Enums

// Structs
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct EntityPos {
    pub x: f32,
    pub y: f32,
}

// Implementations
impl From<ChunkPos> for EntityPos {
    fn from(chunk_pos: ChunkPos) -> Self {
        EntityPos {
            x: chunk_pos.x as f32 * CHUNK_SIZE as f32,
            y: chunk_pos.y as f32 * CHUNK_SIZE as f32,
        }
    }
}

impl From<Vec2> for EntityPos {
    fn from(vec2: Vec2) -> Self {
        EntityPos {
            x: vec2.x,
            y: vec2.y,
        }
    }
}

impl From<Vec3> for EntityPos {
    fn from(vec3: Vec3) -> Self {
        EntityPos {
            x: vec3.x,
            y: vec3.y,
        }
    }
}

impl Into<Vec2> for EntityPos {
    fn into(self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

impl Into<Vec3> for EntityPos {
    fn into(self) -> Vec3 {
        Vec3::new(self.x, self.y, 0.0)
    }
}

// Module Functions
