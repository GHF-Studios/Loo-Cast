use crate::bevy::prelude::*;
use std::fmt::{Display, Formatter};

const LOCAL_MIN: i32 = -5;
const LOCAL_MAX_EXCLUSIVE: i32 = 5;

#[derive(Default, Clone, Copy, Reflect, Debug, PartialEq, Eq, Hash)]
pub struct GridXyz {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
impl GridXyz {
    pub const ZERO: Self = Self { x: 0, y: 0, z: 0 };

    #[track_caller]
    pub fn new_local(x: i32, y: i32, z: i32) -> Self {
        let xyz = Self { x, y, z };
        xyz.assert_local();
        xyz
    }

    pub const fn new_raw(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn is_local(self) -> bool {
        in_local_range(self.x) && in_local_range(self.y) && in_local_range(self.z)
    }

    pub fn try_assert_local(self) -> Result<(), String> {
        if !in_local_range(self.x) {
            return Err(format!("X coordinate {} is outside [{LOCAL_MIN}..{LOCAL_MAX_EXCLUSIVE})", self.x));
        }
        if !in_local_range(self.y) {
            return Err(format!("Y coordinate {} is outside [{LOCAL_MIN}..{LOCAL_MAX_EXCLUSIVE})", self.y));
        }
        if !in_local_range(self.z) {
            return Err(format!("Z coordinate {} is outside [{LOCAL_MIN}..{LOCAL_MAX_EXCLUSIVE})", self.z));
        }
        Ok(())
    }

    #[track_caller]
    pub fn assert_local(self) {
        if let Err(error) = self.try_assert_local() {
            panic!("{error}");
        }
    }

    pub fn as_ivec3(self) -> IVec3 {
        IVec3::new(self.x, self.y, self.z)
    }
}
impl Display for GridXyz {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
impl From<(i32, i32, i32)> for GridXyz {
    #[track_caller]
    fn from(value: (i32, i32, i32)) -> Self {
        Self::new_local(value.0, value.1, value.2)
    }
}
impl From<[i32; 3]> for GridXyz {
    #[track_caller]
    fn from(value: [i32; 3]) -> Self {
        Self::new_local(value[0], value[1], value[2])
    }
}
impl From<GridXyz> for IVec3 {
    fn from(value: GridXyz) -> Self {
        value.as_ivec3()
    }
}
impl TryFrom<IVec3> for GridXyz {
    type Error = String;

    fn try_from(value: IVec3) -> Result<Self, Self::Error> {
        let xyz = Self::new_raw(value.x, value.y, value.z);
        xyz.try_assert_local()?;
        Ok(xyz)
    }
}
impl std::ops::Add<IVec3> for GridXyz {
    type Output = Self;

    fn add(self, rhs: IVec3) -> Self::Output {
        Self::new_raw(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
impl std::ops::AddAssign<IVec3> for GridXyz {
    fn add_assign(&mut self, rhs: IVec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl std::ops::Sub<IVec3> for GridXyz {
    type Output = Self;

    fn sub(self, rhs: IVec3) -> Self::Output {
        Self::new_raw(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
impl std::ops::SubAssign<IVec3> for GridXyz {
    fn sub_assign(&mut self, rhs: IVec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

#[derive(Default, Clone, Copy, Reflect, Debug, PartialEq, Eq, Hash)]
pub struct SubgridXyz {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
impl SubgridXyz {
    pub const ZERO: Self = Self { x: 0, y: 0, z: 0 };

    #[track_caller]
    pub fn new_local(x: i32, y: i32, z: i32) -> Self {
        let xyz = Self { x, y, z };
        xyz.assert_local();
        xyz
    }

    pub const fn new_raw(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn is_local(self) -> bool {
        in_local_range(self.x) && in_local_range(self.y) && in_local_range(self.z)
    }

    pub fn try_assert_local(self) -> Result<(), String> {
        if !in_local_range(self.x) {
            return Err(format!("X coordinate {} is outside [{LOCAL_MIN}..{LOCAL_MAX_EXCLUSIVE})", self.x));
        }
        if !in_local_range(self.y) {
            return Err(format!("Y coordinate {} is outside [{LOCAL_MIN}..{LOCAL_MAX_EXCLUSIVE})", self.y));
        }
        if !in_local_range(self.z) {
            return Err(format!("Z coordinate {} is outside [{LOCAL_MIN}..{LOCAL_MAX_EXCLUSIVE})", self.z));
        }
        Ok(())
    }

    #[track_caller]
    pub fn assert_local(self) {
        if let Err(error) = self.try_assert_local() {
            panic!("{error}");
        }
    }

    pub fn as_ivec3(self) -> IVec3 {
        IVec3::new(self.x, self.y, self.z)
    }
}
impl Display for SubgridXyz {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
impl From<(i32, i32, i32)> for SubgridXyz {
    #[track_caller]
    fn from(value: (i32, i32, i32)) -> Self {
        Self::new_local(value.0, value.1, value.2)
    }
}
impl From<[i32; 3]> for SubgridXyz {
    #[track_caller]
    fn from(value: [i32; 3]) -> Self {
        Self::new_local(value[0], value[1], value[2])
    }
}
impl From<SubgridXyz> for IVec3 {
    fn from(value: SubgridXyz) -> Self {
        value.as_ivec3()
    }
}
impl TryFrom<IVec3> for SubgridXyz {
    type Error = String;

    fn try_from(value: IVec3) -> Result<Self, Self::Error> {
        let xyz = Self::new_raw(value.x, value.y, value.z);
        xyz.try_assert_local()?;
        Ok(xyz)
    }
}
impl From<GridXyz> for SubgridXyz {
    fn from(value: GridXyz) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}
impl From<SubgridXyz> for GridXyz {
    fn from(value: SubgridXyz) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}
impl std::ops::Add<IVec3> for SubgridXyz {
    type Output = Self;

    fn add(self, rhs: IVec3) -> Self::Output {
        Self::new_raw(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
impl std::ops::AddAssign<IVec3> for SubgridXyz {
    fn add_assign(&mut self, rhs: IVec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl std::ops::Sub<IVec3> for SubgridXyz {
    type Output = Self;

    fn sub(self, rhs: IVec3) -> Self::Output {
        Self::new_raw(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
impl std::ops::SubAssign<IVec3> for SubgridXyz {
    fn sub_assign(&mut self, rhs: IVec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

#[inline]
fn in_local_range(value: i32) -> bool {
    (LOCAL_MIN..LOCAL_MAX_EXCLUSIVE).contains(&value)
}
