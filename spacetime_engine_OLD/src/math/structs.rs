use std::ops;
use bevy::reflect::Reflect;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Reflect)]
pub struct I16Vec2(pub i16, pub i16);

impl From<(i16, i16)> for I16Vec2 {
    fn from((x, y): (i16, i16)) -> Self {
        I16Vec2(x, y)
    }
}

impl From<I16Vec2> for (i16, i16) {
    fn from(i16_vec2: I16Vec2) -> Self {
        (i16_vec2.0, i16_vec2.1)
    }
}

impl ops::Add<I16Vec2> for I16Vec2 {
    type Output = I16Vec2;

    fn add(self, other: I16Vec2) -> I16Vec2 {
        I16Vec2(self.0 + other.0, self.1 + other.1)
    }
}

impl ops::AddAssign<I16Vec2> for I16Vec2 {
    fn add_assign(&mut self, other: I16Vec2) {
        self.0 += other.0;
        self.1 += other.1;
    }
}

impl ops::Sub<I16Vec2> for I16Vec2 {
    type Output = I16Vec2;

    fn sub(self, other: I16Vec2) -> I16Vec2 {
        I16Vec2(self.0 - other.0, self.1 - other.1)
    }
}

impl ops::SubAssign<I16Vec2> for I16Vec2 {
    fn sub_assign(&mut self, other: I16Vec2) {
        self.0 -= other.0;
        self.1 -= other.1;
    }
}

impl ops::Mul<i16> for I16Vec2 {
    type Output = I16Vec2;

    fn mul(self, scalar: i16) -> I16Vec2 {
        I16Vec2(self.0 * scalar, self.1 * scalar)
    }
}

impl ops::MulAssign<i16> for I16Vec2 {
    fn mul_assign(&mut self, scalar: i16) {
        self.0 *= scalar;
        self.1 *= scalar;
    }
}

impl ops::Div<i16> for I16Vec2 {
    type Output = I16Vec2;

    fn div(self, scalar: i16) -> I16Vec2 {
        I16Vec2(self.0 / scalar, self.1 / scalar)
    }
}

impl ops::DivAssign<i16> for I16Vec2 {
    fn div_assign(&mut self, scalar: i16) {
        self.0 /= scalar;
        self.1 /= scalar;
    }
}