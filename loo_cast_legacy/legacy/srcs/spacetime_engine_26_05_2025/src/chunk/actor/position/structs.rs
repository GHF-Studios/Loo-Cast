use bevy::prelude::*;
use std::ops;
use crate::chunk::constants::*;
use crate::chunk::position::structs::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Reflect)]
pub struct ChunkActorPosition(pub Vec3);

impl From<Vec2> for ChunkActorPosition {
    fn from(vec2: Vec2) -> Self {
        ChunkActorPosition(Vec3::new(vec2.x, vec2.y, 0.0))
    }
}

impl From<Vec3> for ChunkActorPosition {
    fn from(vec3: Vec3) -> Self {
        ChunkActorPosition(vec3)
    }
}

impl From<ChunkPosition> for ChunkActorPosition {
    fn from(chunk_position: ChunkPosition) -> Self {
        let x = chunk_position.0.0 as f32 * CHUNK_SIZE as f32;
        let y = chunk_position.0.1 as f32 * CHUNK_SIZE as f32;
        ChunkActorPosition(Vec3::new(x, y, CHUNK_Z_INDEX))
    }
}

impl ops::Add<ChunkActorPosition> for ChunkActorPosition {
    type Output = ChunkActorPosition;

    fn add(self, other: ChunkActorPosition) -> ChunkActorPosition {
        ChunkActorPosition(self.0 + other.0)
    }
}

impl ops::AddAssign<ChunkActorPosition> for ChunkActorPosition {
    fn add_assign(&mut self, other: ChunkActorPosition) {
        self.0 += other.0;
    }
}

impl ops::Sub<ChunkActorPosition> for ChunkActorPosition {
    type Output = ChunkActorPosition;

    fn sub(self, other: ChunkActorPosition) -> ChunkActorPosition {
        ChunkActorPosition(self.0 - other.0)
    }
}

impl ops::SubAssign<ChunkActorPosition> for ChunkActorPosition {
    fn sub_assign(&mut self, other: ChunkActorPosition) {
        self.0 -= other.0;
    }
}

impl ops::Mul<f32> for ChunkActorPosition {
    type Output = ChunkActorPosition;

    fn mul(self, scalar: f32) -> ChunkActorPosition {
        ChunkActorPosition(self.0 * scalar)
    }
}

impl ops::MulAssign<f32> for ChunkActorPosition {
    fn mul_assign(&mut self, scalar: f32) {
        self.0 *= scalar;
    }
}

impl ops::Div<f32> for ChunkActorPosition {
    type Output = ChunkActorPosition;

    fn div(self, scalar: f32) -> ChunkActorPosition {
        ChunkActorPosition(self.0 / scalar)
    }
}

impl ops::DivAssign<f32> for ChunkActorPosition {
    fn div_assign(&mut self, scalar: f32) {
        self.0 /= scalar;
    }
}

impl ChunkActorPosition {
    fn new(x: f32, y: f32) -> Self {
        ChunkActorPosition(Vec3::new(x, y, CHUNK_Z_INDEX))
    }
}
