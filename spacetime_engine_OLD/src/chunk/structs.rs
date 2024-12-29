use bevy::prelude::*;
use std::ops;
use crate::entity::structs::EntityPosition;
use crate::math::structs::*;
use crate::chunk::constants::*;

#[derive(Deref, DerefMut, Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Reflect)]
pub struct ChunkPosition(pub I16Vec2);
impl From<I16Vec2> for ChunkPosition {
    fn from(i16_vec2: I16Vec2) -> Self {
        ChunkPosition(i16_vec2)
    }
}
impl From<EntityPosition> for ChunkPosition {
    fn from(entity_position: EntityPosition) -> Self {
        let x = ((entity_position.0.x + CHUNK_SIZE as f32 / 2.0) / CHUNK_SIZE as f32).floor() as i16;
        let y = ((entity_position.0.y + CHUNK_SIZE as f32 / 2.0) / CHUNK_SIZE as f32).floor() as i16;
        ChunkPosition(I16Vec2(x, y))
    }
}
impl ops::Add<ChunkPosition> for ChunkPosition {
    type Output = ChunkPosition;

    fn add(self, other: ChunkPosition) -> ChunkPosition {
        ChunkPosition(self.0 + other.0)
    }
}
impl ops::AddAssign<ChunkPosition> for ChunkPosition {
    fn add_assign(&mut self, other: ChunkPosition) {
        self.0 += other.0;
    }
}
impl ops::Sub<ChunkPosition> for ChunkPosition {
    type Output = ChunkPosition;

    fn sub(self, other: ChunkPosition) -> ChunkPosition {
        ChunkPosition(self.0 - other.0)
    }
}
impl ops::SubAssign<ChunkPosition> for ChunkPosition {
    fn sub_assign(&mut self, other: ChunkPosition) {
        self.0 -= other.0;
    }
}
impl ops::Mul<i16> for ChunkPosition {
    type Output = ChunkPosition;

    fn mul(self, scalar: i16) -> ChunkPosition {
        ChunkPosition(self.0 * scalar)
    }
}
impl ops::MulAssign<i16> for ChunkPosition {
    fn mul_assign(&mut self, scalar: i16) {
        self.0 *= scalar;
    }
}
impl ops::Div<i16> for ChunkPosition {
    type Output = ChunkPosition;

    fn div(self, scalar: i16) -> ChunkPosition {
        ChunkPosition(self.0 / scalar)
    }
}
impl ops::DivAssign<i16> for ChunkPosition {
    fn div_assign(&mut self, scalar: i16) {
        self.0 /= scalar;
    }
}
