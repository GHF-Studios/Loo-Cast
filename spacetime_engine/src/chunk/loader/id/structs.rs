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
pub struct ChunkLoaderRequestID(pub u64);

impl From<u64> for ChunkLoaderRequestID {
    fn from(chunk_loader_request_id: u64) -> Self {
        ChunkLoaderRequestID(chunk_loader_request_id)
    }
}

impl From<ChunkLoaderRequestID> for u64 {
    fn from(chunk_loader_request_id: ChunkLoaderRequestID) -> Self {
        chunk_loader_request_id.0
    }
}

impl ops::Add<u64> for ChunkLoaderRequestID {
    type Output = ChunkLoaderRequestID;

    fn add(self, rhs: u64) -> Self::Output {
        ChunkLoaderRequestID(self.0 + rhs)
    }
}

impl ops::AddAssign<u64> for ChunkLoaderRequestID {
    fn add_assign(&mut self, rhs: u64) {
        self.0 += rhs;
    }
}

impl ops::Sub<u64> for ChunkLoaderRequestID {
    type Output = ChunkLoaderRequestID;

    fn sub(self, rhs: u64) -> Self::Output {
        ChunkLoaderRequestID(self.0 - rhs)
    }
}

impl ops::SubAssign<u64> for ChunkLoaderRequestID {
    fn sub_assign(&mut self, rhs: u64) {
        self.0 -= rhs;
    }
}

impl ops::Mul<u64> for ChunkLoaderRequestID {
    type Output = ChunkLoaderRequestID;

    fn mul(self, rhs: u64) -> Self::Output {
        ChunkLoaderRequestID(self.0 * rhs)
    }
}

impl ops::MulAssign<u64> for ChunkLoaderRequestID {
    fn mul_assign(&mut self, rhs: u64) {
        self.0 *= rhs;
    }
}

impl ops::Div<u64> for ChunkLoaderRequestID {
    type Output = ChunkLoaderRequestID;

    fn div(self, rhs: u64) -> Self::Output {
        ChunkLoaderRequestID(self.0 / rhs)
    }
}

impl ops::DivAssign<u64> for ChunkLoaderRequestID {
    fn div_assign(&mut self, rhs: u64) {
        self.0 /= rhs;
    }
}