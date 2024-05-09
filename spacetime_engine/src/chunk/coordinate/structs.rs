use bevy::prelude::*;
use std::ops;
use crate::math::structs::*;
use crate::chunk::constants::*;
use crate::chunk::id::structs::ChunkID;
use crate::chunk::actor::coordinate::structs::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Reflect)]
pub struct ChunkCoordinate(pub I16Vec2);

impl From<I16Vec2> for ChunkCoordinate {
    fn from(i16_vec2: I16Vec2) -> Self {
        ChunkCoordinate(i16_vec2)
    }
}

impl From<ChunkActorCoordinate> for ChunkCoordinate {
    fn from(chunk_actor_coordinate: ChunkActorCoordinate) -> Self {
        let x = ((chunk_actor_coordinate.0.x + CHUNK_SIZE as f32 / 2.0) / CHUNK_SIZE as f32).floor() as i16;
        let y = ((chunk_actor_coordinate.0.y + CHUNK_SIZE as f32 / 2.0) / CHUNK_SIZE as f32).floor() as i16;
        ChunkCoordinate(I16Vec2(x, y))
    }
}

impl From<ChunkID> for ChunkCoordinate {
    fn from(id: ChunkID) -> Self {
        id.0
    }
}

impl ops::Add<ChunkCoordinate> for ChunkCoordinate {
    type Output = ChunkCoordinate;

    fn add(self, other: ChunkCoordinate) -> ChunkCoordinate {
        ChunkCoordinate(self.0 + other.0)
    }
}

impl ops::Sub<ChunkCoordinate> for ChunkCoordinate {
    type Output = ChunkCoordinate;

    fn sub(self, other: ChunkCoordinate) -> ChunkCoordinate {
        ChunkCoordinate(self.0 - other.0)
    }
}

impl ops::Mul<i16> for ChunkCoordinate {
    type Output = ChunkCoordinate;

    fn mul(self, scalar: i16) -> ChunkCoordinate {
        ChunkCoordinate(self.0 * scalar)
    }
}

impl ops::Div<i16> for ChunkCoordinate {
    type Output = ChunkCoordinate;

    fn div(self, scalar: i16) -> ChunkCoordinate {
        ChunkCoordinate(self.0 / scalar)
    }
}