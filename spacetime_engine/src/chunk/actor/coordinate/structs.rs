use bevy::prelude::*;
use std::ops;
use crate::chunk::constants::*;
use crate::chunk::coordinate::structs::*;

/// This represents a position in regular bevy world space.
#[derive(Clone, Copy, Debug, Default, PartialEq, Reflect)]
pub struct ChunkActorCoordinate(pub Vec3);

impl From<Vec2> for ChunkActorCoordinate {
    fn from(vec2: Vec2) -> Self {
        ChunkActorCoordinate(Vec3::new(vec2.x, vec2.y, 0.0))
    }
}

impl From<Vec3> for ChunkActorCoordinate {
    fn from(vec3: Vec3) -> Self {
        ChunkActorCoordinate(vec3)
    }
}

impl From<ChunkCoordinate> for ChunkActorCoordinate {
    fn from(chunk_coordinate: ChunkCoordinate) -> Self {
        let x = chunk_coordinate.0.0 as f32 * CHUNK_SIZE as f32;
        let y = chunk_coordinate.0.1 as f32 * CHUNK_SIZE as f32;
        ChunkActorCoordinate(Vec3::new(x, y, CHUNK_Z_INDEX))
    }
}

impl ops::Add<ChunkActorCoordinate> for ChunkActorCoordinate {
    type Output = ChunkActorCoordinate;

    fn add(self, other: ChunkActorCoordinate) -> ChunkActorCoordinate {
        ChunkActorCoordinate(self.0 + other.0)
    }
}

impl ops::Sub<ChunkActorCoordinate> for ChunkActorCoordinate {
    type Output = ChunkActorCoordinate;

    fn sub(self, other: ChunkActorCoordinate) -> ChunkActorCoordinate {
        ChunkActorCoordinate(self.0 - other.0)
    }
}

impl ops::Mul<f32> for ChunkActorCoordinate {
    type Output = ChunkActorCoordinate;

    fn mul(self, scalar: f32) -> ChunkActorCoordinate {
        ChunkActorCoordinate(self.0 * scalar)
    }
}

impl ops::Div<f32> for ChunkActorCoordinate {
    type Output = ChunkActorCoordinate;

    fn div(self, scalar: f32) -> ChunkActorCoordinate {
        ChunkActorCoordinate(self.0 / scalar)
    }
}

impl ChunkActorCoordinate {
    fn new(x: f32, y: f32) -> Self {
        ChunkActorCoordinate(Vec3::new(x, y, CHUNK_Z_INDEX))
    }
}
