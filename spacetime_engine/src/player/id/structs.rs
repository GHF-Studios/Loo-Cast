use bevy::reflect::Reflect;
use std::ops;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Reflect)]
pub struct PlayerID(pub u64);

impl From<u64> for PlayerID {
    fn from(id: u64) -> Self {
        PlayerID(id)
    }
}

impl From<PlayerID> for u64 {
    fn from(id: PlayerID) -> Self {
        id.0
    }
}

impl ops::Add<u64> for PlayerID {
    type Output = PlayerID;

    fn add(self, rhs: u64) -> Self::Output {
        PlayerID(self.0 + rhs)
    }
}

impl ops::AddAssign<u64> for PlayerID {
    fn add_assign(&mut self, rhs: u64) {
        self.0 += rhs;
    }
}

impl ops::Sub<u64> for PlayerID {
    type Output = PlayerID;

    fn sub(self, rhs: u64) -> Self::Output {
        PlayerID(self.0 - rhs)
    }
}

impl ops::SubAssign<u64> for PlayerID {
    fn sub_assign(&mut self, rhs: u64) {
        self.0 -= rhs;
    }
}

impl ops::Mul<u64> for PlayerID {
    type Output = PlayerID;

    fn mul(self, rhs: u64) -> Self::Output {
        PlayerID(self.0 * rhs)
    }
}

impl ops::MulAssign<u64> for PlayerID {
    fn mul_assign(&mut self, rhs: u64) {
        self.0 *= rhs;
    }
}

impl ops::Div<u64> for PlayerID {
    type Output = PlayerID;

    fn div(self, rhs: u64) -> Self::Output {
        PlayerID(self.0 / rhs)
    }
}

impl ops::DivAssign<u64> for PlayerID {
    fn div_assign(&mut self, rhs: u64) {
        self.0 /= rhs;
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Reflect)]
pub struct PlayerEventID(pub u64);

impl From<u64> for PlayerEventID {
    fn from(id: u64) -> Self {
        PlayerEventID(id)
    }
}

impl From<PlayerEventID> for u64 {
    fn from(id: PlayerEventID) -> Self {
        id.0
    }
}

impl ops::Add<u64> for PlayerEventID {
    type Output = PlayerEventID;

    fn add(self, rhs: u64) -> Self::Output {
        PlayerEventID(self.0 + rhs)
    }
}

impl ops::AddAssign<u64> for PlayerEventID {
    fn add_assign(&mut self, rhs: u64) {
        self.0 += rhs;
    }
}

impl ops::Sub<u64> for PlayerEventID {
    type Output = PlayerEventID;

    fn sub(self, rhs: u64) -> Self::Output {
        PlayerEventID(self.0 - rhs)
    }
}

impl ops::SubAssign<u64> for PlayerEventID {
    fn sub_assign(&mut self, rhs: u64) {
        self.0 -= rhs;
    }
}

impl ops::Mul<u64> for PlayerEventID {
    type Output = PlayerEventID;

    fn mul(self, rhs: u64) -> Self::Output {
        PlayerEventID(self.0 * rhs)
    }
}

impl ops::MulAssign<u64> for PlayerEventID {
    fn mul_assign(&mut self, rhs: u64) {
        self.0 *= rhs;
    }
}

impl ops::Div<u64> for PlayerEventID {
    type Output = PlayerEventID;

    fn div(self, rhs: u64) -> Self::Output {
        PlayerEventID(self.0 / rhs)
    }
}

impl ops::DivAssign<u64> for PlayerEventID {
    fn div_assign(&mut self, rhs: u64) {
        self.0 /= rhs;
    }
}