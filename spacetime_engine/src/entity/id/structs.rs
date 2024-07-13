use bevy::reflect::Reflect;
use std::ops;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Reflect)]
pub struct EntityID(u64);

impl From<u64> for EntityID {
    fn from(id: u64) -> Self {
        EntityID(id)
    }
}

impl From<EntityID> for u64 {
    fn from(id: EntityID) -> Self {
        id.0
    }
}

impl ops::Add<u64> for EntityID {
    type Output = EntityID;

    fn add(self, rhs: u64) -> Self::Output {
        EntityID(self.0 + rhs)
    }
}

impl ops::AddAssign<u64> for EntityID {
    fn add_assign(&mut self, rhs: u64) {
        self.0 += rhs;
    }
}

impl ops::Sub<u64> for EntityID {
    type Output = EntityID;

    fn sub(self, rhs: u64) -> Self::Output {
        EntityID(self.0 - rhs)
    }
}

impl ops::SubAssign<u64> for EntityID {
    fn sub_assign(&mut self, rhs: u64) {
        self.0 -= rhs;
    }
}

impl ops::Mul<u64> for EntityID {
    type Output = EntityID;

    fn mul(self, rhs: u64) -> Self::Output {
        EntityID(self.0 * rhs)
    }
}

impl ops::MulAssign<u64> for EntityID {
    fn mul_assign(&mut self, rhs: u64) {
        self.0 *= rhs;
    }
}

impl ops::Div<u64> for EntityID {
    type Output = EntityID;

    fn div(self, rhs: u64) -> Self::Output {
        EntityID(self.0 / rhs)
    }
}

impl ops::DivAssign<u64> for EntityID {
    fn div_assign(&mut self, rhs: u64) {
        self.0 /= rhs;
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Reflect)]
pub struct EntityRequestID(u64);

impl From<u64> for EntityRequestID {
    fn from(id: u64) -> Self {
        EntityRequestID(id)
    }
}

impl From<EntityRequestID> for u64 {
    fn from(id: EntityRequestID) -> Self {
        id.0
    }
}

impl ops::Add<u64> for EntityRequestID {
    type Output = EntityRequestID;

    fn add(self, rhs: u64) -> Self::Output {
        EntityRequestID(self.0 + rhs)
    }
}

impl ops::AddAssign<u64> for EntityRequestID {
    fn add_assign(&mut self, rhs: u64) {
        self.0 += rhs;
    }
}

impl ops::Sub<u64> for EntityRequestID {
    type Output = EntityRequestID;

    fn sub(self, rhs: u64) -> Self::Output {
        EntityRequestID(self.0 - rhs)
    }
}

impl ops::SubAssign<u64> for EntityRequestID {
    fn sub_assign(&mut self, rhs: u64) {
        self.0 -= rhs;
    }
}

impl ops::Mul<u64> for EntityRequestID {
    type Output = EntityRequestID;

    fn mul(self, rhs: u64) -> Self::Output {
        EntityRequestID(self.0 * rhs)
    }
}

impl ops::MulAssign<u64> for EntityRequestID {
    fn mul_assign(&mut self, rhs: u64) {
        self.0 *= rhs;
    }
}

impl ops::Div<u64> for EntityRequestID {
    type Output = EntityRequestID;

    fn div(self, rhs: u64) -> Self::Output {
        EntityRequestID(self.0 / rhs)
    }
}

impl ops::DivAssign<u64> for EntityRequestID {
    fn div_assign(&mut self, rhs: u64) {
        self.0 /= rhs;
    }
}