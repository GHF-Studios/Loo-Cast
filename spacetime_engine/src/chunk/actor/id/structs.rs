use bevy::prelude::*;
use std::ops;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Reflect)]
pub struct ChunkActorID(pub u64);

impl From<u64> for ChunkActorID {
    fn from(chunk_actor_id: u64) -> Self {
        ChunkActorID(chunk_actor_id)
    }
}

impl From<ChunkActorID> for u64 {
    fn from(chunk_actor_id: ChunkActorID) -> Self {
        chunk_actor_id.0
    }
}

impl ops::Add<u64> for ChunkActorID {
    type Output = ChunkActorID;

    fn add(self, rhs: u64) -> Self::Output {
        ChunkActorID(self.0 + rhs)
    }
}

impl ops::AddAssign<u64> for ChunkActorID {
    fn add_assign(&mut self, rhs: u64) {
        self.0 += rhs;
    }
}

impl ops::Sub<u64> for ChunkActorID {
    type Output = ChunkActorID;

    fn sub(self, rhs: u64) -> Self::Output {
        ChunkActorID(self.0 - rhs)
    }
}

impl ops::SubAssign<u64> for ChunkActorID {
    fn sub_assign(&mut self, rhs: u64) {
        self.0 -= rhs;
    }
}

impl ops::Mul<u64> for ChunkActorID {
    type Output = ChunkActorID;

    fn mul(self, rhs: u64) -> Self::Output {
        ChunkActorID(self.0 * rhs)
    }
}

impl ops::MulAssign<u64> for ChunkActorID {
    fn mul_assign(&mut self, rhs: u64) {
        self.0 *= rhs;
    }
}

impl ops::Div<u64> for ChunkActorID {
    type Output = ChunkActorID;

    fn div(self, rhs: u64) -> Self::Output {
        ChunkActorID(self.0 / rhs)
    }
}

impl ops::DivAssign<u64> for ChunkActorID {
    fn div_assign(&mut self, rhs: u64) {
        self.0 /= rhs;
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Reflect)]
pub struct ChunkActorEventID(pub u64);

impl From<u64> for ChunkActorEventID {
    fn from(chunk_actor_event_id: u64) -> Self {
        ChunkActorEventID(chunk_actor_event_id)
    }
}

impl From<ChunkActorEventID> for u64 {
    fn from(chunk_actor_event_id: ChunkActorEventID) -> Self {
        chunk_actor_event_id.0
    }
}

impl ops::Add<u64> for ChunkActorEventID {
    type Output = ChunkActorEventID;

    fn add(self, rhs: u64) -> Self::Output {
        ChunkActorEventID(self.0 + rhs)
    }
}

impl ops::AddAssign<u64> for ChunkActorEventID {
    fn add_assign(&mut self, rhs: u64) {
        self.0 += rhs;
    }
}

impl ops::Sub<u64> for ChunkActorEventID {
    type Output = ChunkActorEventID;

    fn sub(self, rhs: u64) -> Self::Output {
        ChunkActorEventID(self.0 - rhs)
    }
}

impl ops::SubAssign<u64> for ChunkActorEventID {
    fn sub_assign(&mut self, rhs: u64) {
        self.0 -= rhs;
    }
}

impl ops::Mul<u64> for ChunkActorEventID {
    type Output = ChunkActorEventID;

    fn mul(self, rhs: u64) -> Self::Output {
        ChunkActorEventID(self.0 * rhs)
    }
}

impl ops::MulAssign<u64> for ChunkActorEventID {
    fn mul_assign(&mut self, rhs: u64) {
        self.0 *= rhs;
    }
}

impl ops::Div<u64> for ChunkActorEventID {
    type Output = ChunkActorEventID;

    fn div(self, rhs: u64) -> Self::Output {
        ChunkActorEventID(self.0 / rhs)
    }
}

impl ops::DivAssign<u64> for ChunkActorEventID {
    fn div_assign(&mut self, rhs: u64) {
        self.0 /= rhs;
    }
}