use bevy::{math::I64Vec2, prelude::*};

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct I128Vec2 {
    pub x: i128,
    pub y: i128,
}
impl I128Vec2 {
    pub const ZERO: Self = Self { x: 0, y: 0 };
    pub const ONE: Self = Self { x: 1, y: 1 };
    pub const UNIT_X: Self = Self { x: 1, y: 0 };
    pub const UNIT_Y: Self = Self { x: 0, y: 1 };
    
    pub fn new(x: i128, y: i128) -> Self {
        Self { x, y }
    }

    pub fn distance_squared(&self, rhs: &Self) -> i128 {
        let dx = self.x - rhs.x;
        let dy = self.y - rhs.y;
        dx * dx + dy * dy
    }
}
impl std::fmt::Debug for I128Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "I128Vec2 {{ x: {}, y: {} }}", self.x, self.y)
    }
}
impl std::ops::Add for I128Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl std::ops::AddAssign for I128Vec2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}
impl std::ops::Sub for I128Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl std::ops::SubAssign for I128Vec2 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}
impl From<IVec2> for I128Vec2 {
    fn from(v: IVec2) -> Self {
        Self { x: v.x as i128, y: v.y as i128 }
    }
}
impl From<I64Vec2> for I128Vec2 {
    fn from(v: I64Vec2) -> Self {
        Self { x: v.x as i128, y: v.y as i128 }
    }
}
impl TryFrom<I128Vec2> for IVec2 {
    type Error = &'static str;

    fn try_from(value: I128Vec2) -> Result<Self, Self::Error> {
        if value.x < i32::MIN as i128 || value.x > i32::MAX as i128 || value.y < i32::MIN as i128 || value.y > i32::MAX as i128 {
            Err("I128Vec2 value out of range for IVec2")
        } else {
            Ok(IVec2 { x: value.x as i32, y: value.y as i32 })
        }
    }
}
impl TryFrom<I128Vec2> for I64Vec2 {
    type Error = &'static str;

    fn try_from(value: I128Vec2) -> Result<Self, Self::Error> {
        if value.x < i64::MIN as i128 || value.x > i64::MAX as i128 || value.y < i64::MIN as i128 || value.y > i64::MAX as i128 {
            Err("I128Vec2 value out of range for I64Vec2")
        } else {
            Ok(I64Vec2 { x: value.x as i64, y: value.y as i64 })
        }
    }
}
