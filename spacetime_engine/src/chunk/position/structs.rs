use bevy::prelude::*;
use std::ops;
use crate::math::structs::*;
use crate::chunk::constants::*;
use crate::chunk::id::structs::ChunkID;
use crate::chunk::actor::position::structs::*;

/// This represents a position in chunk space.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Reflect)]
pub struct ChunkPosition(pub I16Vec2);

impl From<I16Vec2> for ChunkPosition {
    fn from(i16_vec2: I16Vec2) -> Self {
        ChunkPosition(i16_vec2)
    }
}

impl From<ChunkActorPosition> for ChunkPosition {
    fn from(chunk_actor_position: ChunkActorPosition) -> Self {
        let x = ((chunk_actor_position.0.x + CHUNK_SIZE as f32 / 2.0) / CHUNK_SIZE as f32).floor() as i16;
        let y = ((chunk_actor_position.0.y + CHUNK_SIZE as f32 / 2.0) / CHUNK_SIZE as f32).floor() as i16;
        ChunkPosition(I16Vec2(x, y))
    }
}

impl From<ChunkID> for ChunkPosition {
    fn from(id: ChunkID) -> Self {
        id.0
    }
}

impl ops::Add<ChunkPosition> for ChunkPosition {
    type Output = ChunkPosition;

    fn add(self, other: ChunkPosition) -> ChunkPosition {
        ChunkPosition(self.0 + other.0)
    }
}

impl ops::Sub<ChunkPosition> for ChunkPosition {
    type Output = ChunkPosition;

    fn sub(self, other: ChunkPosition) -> ChunkPosition {
        ChunkPosition(self.0 - other.0)
    }
}

impl ops::Mul<i16> for ChunkPosition {
    type Output = ChunkPosition;

    fn mul(self, scalar: i16) -> ChunkPosition {
        ChunkPosition(self.0 * scalar)
    }
}

impl ops::Div<i16> for ChunkPosition {
    type Output = ChunkPosition;

    fn div(self, scalar: i16) -> ChunkPosition {
        ChunkPosition(self.0 / scalar)
    }
}