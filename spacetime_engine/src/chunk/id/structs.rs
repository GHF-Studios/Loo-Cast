use std::ops;
use bevy::prelude::*;
use super::super::position::structs::*;
use crate::math::structs::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Reflect)]
pub struct ChunkID(pub ChunkPosition);

impl From<ChunkPosition> for ChunkID {
    fn from(chunk_position: ChunkPosition) -> Self {
        ChunkID(chunk_position)
    }
}

impl ChunkID {
    fn new(x: i16, y: i16) -> Self {
        ChunkID(ChunkPosition(I16Vec2(x, y)))
    }
}

impl ops::Add<I16Vec2> for ChunkID {
    type Output = ChunkID;

    fn add(self, rhs: I16Vec2) -> Self::Output {
        ChunkID(self.0 + ChunkPosition(rhs))
    }
}

impl ops::AddAssign<I16Vec2> for ChunkID {
    fn add_assign(&mut self, rhs: I16Vec2) {
        self.0 += ChunkPosition(rhs);
    }
}

impl ops::Sub<I16Vec2> for ChunkID {
    type Output = ChunkID;

    fn sub(self, rhs: I16Vec2) -> Self::Output {
        ChunkID(self.0 - ChunkPosition(rhs))
    }
}

impl ops::SubAssign<I16Vec2> for ChunkID {
    fn sub_assign(&mut self, rhs: I16Vec2) {
        self.0 -= ChunkPosition(rhs);
    }
}

impl ops::Mul<i16> for ChunkID {
    type Output = ChunkID;

    fn mul(self, rhs: i16) -> Self::Output {
        ChunkID(self.0 * rhs)
    }
}

impl ops::MulAssign<i16> for ChunkID {
    fn mul_assign(&mut self, rhs: i16) {
        self.0 *= rhs;
    }
}

impl ops::Div<i16> for ChunkID {
    type Output = ChunkID;

    fn div(self, rhs: i16) -> Self::Output {
        ChunkID(self.0 / rhs)
    }
}

impl ops::DivAssign<i16> for ChunkID {
    fn div_assign(&mut self, rhs: i16) {
        self.0 /= rhs;
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Reflect)]
pub struct ChunkEventID(pub u64);

impl From<u64> for ChunkEventID {
    fn from(event_id: u64) -> Self {
        ChunkEventID(event_id)
    }
}

impl From<ChunkEventID> for u64 {
    fn from(chunk_event_id: ChunkEventID) -> Self {
        chunk_event_id.0
    }
}

impl ops::Add<u64> for ChunkEventID {
    type Output = ChunkEventID;

    fn add(self, rhs: u64) -> Self::Output {
        ChunkEventID(self.0 + rhs)
    }
}

impl ops::AddAssign<u64> for ChunkEventID {
    fn add_assign(&mut self, rhs: u64) {
        self.0 += rhs;
    }
}

impl ops::Sub<u64> for ChunkEventID {
    type Output = ChunkEventID;

    fn sub(self, rhs: u64) -> Self::Output {
        ChunkEventID(self.0 - rhs)
    }
}

impl ops::SubAssign<u64> for ChunkEventID {
    fn sub_assign(&mut self, rhs: u64) {
        self.0 -= rhs;
    }
}

impl ops::Mul<u64> for ChunkEventID {
    type Output = ChunkEventID;

    fn mul(self, rhs: u64) -> Self::Output {
        ChunkEventID(self.0 * rhs)
    }
}

impl ops::MulAssign<u64> for ChunkEventID {
    fn mul_assign(&mut self, rhs: u64) {
        self.0 *= rhs;
    }
}

impl ops::Div<u64> for ChunkEventID {
    type Output = ChunkEventID;

    fn div(self, rhs: u64) -> Self::Output {
        ChunkEventID(self.0 / rhs)
    }
}

impl ops::DivAssign<u64> for ChunkEventID {
    fn div_assign(&mut self, rhs: u64) {
        self.0 /= rhs;
    }
}