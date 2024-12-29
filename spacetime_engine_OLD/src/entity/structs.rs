use bevy::prelude::*;
use std::ops;
use crate::chunk::constants::*;
use crate::chunk::structs::*;

#[derive(Deref, DerefMut, Clone, Copy, Debug, Default, Reflect)]
pub struct EntityPosition(pub Vec2);
impl From<Vec2> for EntityPosition {
    fn from(vec2: Vec2) -> Self {
        EntityPosition(Vec2::new(vec2.x, vec2.y))
    }
}
impl From<Vec3> for EntityPosition {
    fn from(vec3: Vec3) -> Self {
        EntityPosition(Vec2::new(vec3.x, vec3.y))
    }
}
impl From<ChunkPosition> for EntityPosition {
    fn from(chunk_position: ChunkPosition) -> Self {
        let x = chunk_position.0.0 as f32 * CHUNK_SIZE as f32;
        let y = chunk_position.0.1 as f32 * CHUNK_SIZE as f32;
        EntityPosition(Vec2::new(x, y))
    }
}
impl ops::Add<EntityPosition> for EntityPosition {
    type Output = EntityPosition;

    fn add(self, other: EntityPosition) -> EntityPosition {
        EntityPosition(self.0 + other.0)
    }
}
impl ops::AddAssign<EntityPosition> for EntityPosition {
    fn add_assign(&mut self, other: EntityPosition) {
        self.0 += other.0;
    }
}
impl ops::Sub<EntityPosition> for EntityPosition {
    type Output = EntityPosition;

    fn sub(self, other: EntityPosition) -> EntityPosition {
        EntityPosition(self.0 - other.0)
    }
}
impl ops::SubAssign<EntityPosition> for EntityPosition {
    fn sub_assign(&mut self, other: EntityPosition) {
        self.0 -= other.0;
    }
}
impl ops::Mul<f32> for EntityPosition {
    type Output = EntityPosition;

    fn mul(self, scalar: f32) -> EntityPosition {
        EntityPosition(self.0 * scalar)
    }
}
impl ops::MulAssign<f32> for EntityPosition {
    fn mul_assign(&mut self, scalar: f32) {
        self.0 *= scalar;
    }
}
impl ops::Div<f32> for EntityPosition {
    type Output = EntityPosition;

    fn div(self, scalar: f32) -> EntityPosition {
        EntityPosition(self.0 / scalar)
    }
}
impl ops::DivAssign<f32> for EntityPosition {
    fn div_assign(&mut self, scalar: f32) {
        self.0 /= scalar;
    }
}
