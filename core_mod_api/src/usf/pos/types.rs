use crate::bevy::prelude::*;
use std::fmt::{Display, Formatter};

pub const LOCAL_CELL_MIN: i8 = -5;
pub const LOCAL_CELL_MAX_EXCLUSIVE: i8 = 5;
pub const LOCAL_CELL_MAX_INCLUSIVE: i8 = LOCAL_CELL_MAX_EXCLUSIVE - 1;

#[derive(Default, Clone, Copy, Reflect, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LocalCellCoord(i8);
impl LocalCellCoord {
    pub const ZERO: Self = Self(0);

    #[track_caller]
    pub fn new_local(value: i32) -> Self {
        let coord = Self::new_raw(value as i8);
        coord.assert_local();
        coord
    }

    pub const fn new_raw(value: i8) -> Self {
        Self(value)
    }

    pub fn is_local(self) -> bool {
        in_local_range(self.0 as i32)
    }

    pub fn try_assert_local(self) -> Result<(), String> {
        if !in_local_range(self.0 as i32) {
            return Err(format!(
                "Coordinate {} is outside [{LOCAL_CELL_MIN}..{LOCAL_CELL_MAX_EXCLUSIVE})",
                self.0
            ));
        }
        Ok(())
    }

    #[track_caller]
    pub fn assert_local(self) {
        if let Err(error) = self.try_assert_local() {
            panic!("{error}");
        }
    }

    pub fn as_i8(self) -> i8 {
        self.0
    }

    pub fn as_i32(self) -> i32 {
        self.0 as i32
    }
}
impl Display for LocalCellCoord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl TryFrom<i32> for LocalCellCoord {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let coord = Self::new_raw(value as i8);
        coord.try_assert_local()?;
        Ok(coord)
    }
}
impl TryFrom<i8> for LocalCellCoord {
    type Error = String;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let coord = Self::new_raw(value);
        coord.try_assert_local()?;
        Ok(coord)
    }
}
impl From<LocalCellCoord> for i8 {
    fn from(value: LocalCellCoord) -> Self {
        value.as_i8()
    }
}
impl From<LocalCellCoord> for i32 {
    fn from(value: LocalCellCoord) -> Self {
        value.as_i32()
    }
}

#[derive(Default, Clone, Copy, Reflect, Debug, PartialEq, Eq, Hash)]
pub struct LocalCell3 {
    pub x: LocalCellCoord,
    pub y: LocalCellCoord,
    pub z: LocalCellCoord,
}
impl LocalCell3 {
    pub const ZERO: Self = Self {
        x: LocalCellCoord::ZERO,
        y: LocalCellCoord::ZERO,
        z: LocalCellCoord::ZERO,
    };

    #[track_caller]
    pub fn new_local(x: i32, y: i32, z: i32) -> Self {
        Self {
            x: LocalCellCoord::new_local(x),
            y: LocalCellCoord::new_local(y),
            z: LocalCellCoord::new_local(z),
        }
    }

    pub const fn new_raw(x: LocalCellCoord, y: LocalCellCoord, z: LocalCellCoord) -> Self {
        Self { x, y, z }
    }

    pub fn is_local(self) -> bool {
        self.x.is_local() && self.y.is_local() && self.z.is_local()
    }

    pub fn try_assert_local(self) -> Result<(), String> {
        self.x.try_assert_local()?;
        self.y.try_assert_local()?;
        self.z.try_assert_local()?;
        Ok(())
    }

    #[track_caller]
    pub fn assert_local(self) {
        if let Err(error) = self.try_assert_local() {
            panic!("{error}");
        }
    }

    pub fn as_ivec3(self) -> IVec3 {
        IVec3::new(self.x.as_i32(), self.y.as_i32(), self.z.as_i32())
    }
}
impl Display for LocalCell3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
impl From<LocalCell3> for IVec3 {
    fn from(value: LocalCell3) -> Self {
        value.as_ivec3()
    }
}
impl From<(i32, i32, i32)> for LocalCell3 {
    #[track_caller]
    fn from(value: (i32, i32, i32)) -> Self {
        Self::new_local(value.0, value.1, value.2)
    }
}
impl From<[i32; 3]> for LocalCell3 {
    #[track_caller]
    fn from(value: [i32; 3]) -> Self {
        Self::new_local(value[0], value[1], value[2])
    }
}
impl TryFrom<IVec3> for LocalCell3 {
    type Error = String;

    fn try_from(value: IVec3) -> Result<Self, Self::Error> {
        Ok(Self {
            x: LocalCellCoord::try_from(value.x)?,
            y: LocalCellCoord::try_from(value.y)?,
            z: LocalCellCoord::try_from(value.z)?,
        })
    }
}

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
            return Err(format!(
                "X coordinate {} is outside [{LOCAL_CELL_MIN}..{LOCAL_CELL_MAX_EXCLUSIVE})",
                self.x
            ));
        }
        if !in_local_range(self.y) {
            return Err(format!(
                "Y coordinate {} is outside [{LOCAL_CELL_MIN}..{LOCAL_CELL_MAX_EXCLUSIVE})",
                self.y
            ));
        }
        if !in_local_range(self.z) {
            return Err(format!(
                "Z coordinate {} is outside [{LOCAL_CELL_MIN}..{LOCAL_CELL_MAX_EXCLUSIVE})",
                self.z
            ));
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

    #[track_caller]
    pub fn as_local_cell3(self) -> LocalCell3 {
        LocalCell3::new_local(self.x, self.y, self.z)
    }

    pub fn try_as_local_cell3(self) -> Result<LocalCell3, String> {
        LocalCell3::try_from(self.as_ivec3())
    }

    pub fn from_local_cell3(local: LocalCell3) -> Self {
        Self {
            x: local.x.as_i32(),
            y: local.y.as_i32(),
            z: local.z.as_i32(),
        }
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
impl From<LocalCell3> for GridXyz {
    fn from(value: LocalCell3) -> Self {
        Self::from_local_cell3(value)
    }
}
impl TryFrom<GridXyz> for LocalCell3 {
    type Error = String;

    fn try_from(value: GridXyz) -> Result<Self, Self::Error> {
        value.try_as_local_cell3()
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
            return Err(format!(
                "X coordinate {} is outside [{LOCAL_CELL_MIN}..{LOCAL_CELL_MAX_EXCLUSIVE})",
                self.x
            ));
        }
        if !in_local_range(self.y) {
            return Err(format!(
                "Y coordinate {} is outside [{LOCAL_CELL_MIN}..{LOCAL_CELL_MAX_EXCLUSIVE})",
                self.y
            ));
        }
        if !in_local_range(self.z) {
            return Err(format!(
                "Z coordinate {} is outside [{LOCAL_CELL_MIN}..{LOCAL_CELL_MAX_EXCLUSIVE})",
                self.z
            ));
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

    #[track_caller]
    pub fn as_local_cell3(self) -> LocalCell3 {
        LocalCell3::new_local(self.x, self.y, self.z)
    }

    pub fn try_as_local_cell3(self) -> Result<LocalCell3, String> {
        LocalCell3::try_from(self.as_ivec3())
    }

    pub fn from_local_cell3(local: LocalCell3) -> Self {
        Self {
            x: local.x.as_i32(),
            y: local.y.as_i32(),
            z: local.z.as_i32(),
        }
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
impl From<LocalCell3> for SubgridXyz {
    fn from(value: LocalCell3) -> Self {
        Self::from_local_cell3(value)
    }
}
impl TryFrom<SubgridXyz> for LocalCell3 {
    type Error = String;

    fn try_from(value: SubgridXyz) -> Result<Self, Self::Error> {
        value.try_as_local_cell3()
    }
}

#[inline]
fn in_local_range(value: i32) -> bool {
    (LOCAL_CELL_MIN as i32..LOCAL_CELL_MAX_EXCLUSIVE as i32).contains(&value)
}
