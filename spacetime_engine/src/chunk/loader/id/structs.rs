use bevy::prelude::*;
use std::ops;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Reflect)]
pub struct ChunkLoaderID(pub u64);

impl From<u64> for ChunkLoaderID {
    fn from(chunk_loader_id: u64) -> Self {
        ChunkLoaderID(chunk_loader_id)
    }
}

impl From<ChunkLoaderID> for u64 {
    fn from(chunk_loader_id: ChunkLoaderID) -> Self {
        chunk_loader_id.0
    }
}

impl ops::Add<u64> for ChunkLoaderID {
    type Output = ChunkLoaderID;

    fn add(self, rhs: u64) -> Self::Output {
        ChunkLoaderID(self.0 + rhs)
    }
}

impl ops::AddAssign<u64> for ChunkLoaderID {
    fn add_assign(&mut self, rhs: u64) {
        self.0 += rhs;
    }
}

impl ops::Sub<u64> for ChunkLoaderID {
    type Output = ChunkLoaderID;

    fn sub(self, rhs: u64) -> Self::Output {
        ChunkLoaderID(self.0 - rhs)
    }
}

impl ops::SubAssign<u64> for ChunkLoaderID {
    fn sub_assign(&mut self, rhs: u64) {
        self.0 -= rhs;
    }
}

impl ops::Mul<u64> for ChunkLoaderID {
    type Output = ChunkLoaderID;

    fn mul(self, rhs: u64) -> Self::Output {
        ChunkLoaderID(self.0 * rhs)
    }
}

impl ops::MulAssign<u64> for ChunkLoaderID {
    fn mul_assign(&mut self, rhs: u64) {
        self.0 *= rhs;
    }
}

impl ops::Div<u64> for ChunkLoaderID {
    type Output = ChunkLoaderID;

    fn div(self, rhs: u64) -> Self::Output {
        ChunkLoaderID(self.0 / rhs)
    }
}

impl ops::DivAssign<u64> for ChunkLoaderID {
    fn div_assign(&mut self, rhs: u64) {
        self.0 /= rhs;
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Reflect)]
pub struct ChunkLoaderEventID(pub u64);

impl From<u64> for ChunkLoaderEventID {
    fn from(chunk_loader_event_id: u64) -> Self {
        ChunkLoaderEventID(chunk_loader_event_id)
    }
}

impl From<ChunkLoaderEventID> for u64 {
    fn from(chunk_loader_event_id: ChunkLoaderEventID) -> Self {
        chunk_loader_event_id.0
    }
}

impl ops::Add<u64> for ChunkLoaderEventID {
    type Output = ChunkLoaderEventID;

    fn add(self, rhs: u64) -> Self::Output {
        ChunkLoaderEventID(self.0 + rhs)
    }
}

impl ops::AddAssign<u64> for ChunkLoaderEventID {
    fn add_assign(&mut self, rhs: u64) {
        self.0 += rhs;
    }
}

impl ops::Sub<u64> for ChunkLoaderEventID {
    type Output = ChunkLoaderEventID;

    fn sub(self, rhs: u64) -> Self::Output {
        ChunkLoaderEventID(self.0 - rhs)
    }
}

impl ops::SubAssign<u64> for ChunkLoaderEventID {
    fn sub_assign(&mut self, rhs: u64) {
        self.0 -= rhs;
    }
}

impl ops::Mul<u64> for ChunkLoaderEventID {
    type Output = ChunkLoaderEventID;

    fn mul(self, rhs: u64) -> Self::Output {
        ChunkLoaderEventID(self.0 * rhs)
    }
}

impl ops::MulAssign<u64> for ChunkLoaderEventID {
    fn mul_assign(&mut self, rhs: u64) {
        self.0 *= rhs;
    }
}

impl ops::Div<u64> for ChunkLoaderEventID {
    type Output = ChunkLoaderEventID;

    fn div(self, rhs: u64) -> Self::Output {
        ChunkLoaderEventID(self.0 / rhs)
    }
}

impl ops::DivAssign<u64> for ChunkLoaderEventID {
    fn div_assign(&mut self, rhs: u64) {
        self.0 /= rhs;
    }
}