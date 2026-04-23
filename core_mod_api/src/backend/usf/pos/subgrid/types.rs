use crate::bevy::prelude::{IVec3, Reflect, Vec3};

use crate::usf::math::digit_stack::{DigitStackOverflow, normalize_balanced_digits_checked, normalize_balanced_digits_strict, normalize_balanced_digits_wrap};
use crate::usf::pos::grid::types::GridVec;
use crate::usf::pos::types::{GridXyz, LocalCell3, SubgridXyz};
use crate::usf::pos::unit::types::UnitVec;
use crate::usf::scale::{DynScale, Scale};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubgridVecMathError {
    OverflowX(DigitStackOverflow),
    OverflowY(DigitStackOverflow),
    OverflowZ(DigitStackOverflow),
    CannotZoomOutBeyondRoot,
}

#[derive(Clone, Copy)]
enum OverflowMode {
    Wrap,
    Checked,
    Strict,
}

fn normalize_component(mode: OverflowMode, digits: &mut [i32], map_error: fn(DigitStackOverflow) -> SubgridVecMathError) -> Result<(), SubgridVecMathError> {
    match mode {
        OverflowMode::Wrap => {
            let _ = normalize_balanced_digits_wrap(digits);
            Ok(())
        }
        OverflowMode::Checked => normalize_balanced_digits_checked(digits).map_err(map_error),
        OverflowMode::Strict => {
            normalize_balanced_digits_strict(digits);
            Ok(())
        }
    }
}

fn normalize_ivec3_digit_stack(mode: OverflowMode, stack: &mut [IVec3], initial_carry: IVec3) -> Result<(), SubgridVecMathError> {
    if stack.is_empty() {
        return Ok(());
    }

    let mut x_digits: Vec<i32> = stack.iter().map(|value| value.x).collect();
    let mut y_digits: Vec<i32> = stack.iter().map(|value| value.y).collect();
    let mut z_digits: Vec<i32> = stack.iter().map(|value| value.z).collect();

    let leaf = stack.len() - 1;
    x_digits[leaf] += initial_carry.x;
    y_digits[leaf] += initial_carry.y;
    z_digits[leaf] += initial_carry.z;

    normalize_component(mode, &mut x_digits, SubgridVecMathError::OverflowX)?;
    normalize_component(mode, &mut y_digits, SubgridVecMathError::OverflowY)?;
    normalize_component(mode, &mut z_digits, SubgridVecMathError::OverflowZ)?;

    for (idx, value) in stack.iter_mut().enumerate() {
        *value = IVec3::new(x_digits[idx], y_digits[idx], z_digits[idx]);
    }

    Ok(())
}

#[derive(Default)]
pub struct SubgridVecBuilder {
    chain: Vec<GridXyz>,
}

impl SubgridVecBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(mut self, next: impl Into<LocalCell3>) -> Self {
        self.chain.push(GridXyz::from_local_cell3(next.into()));
        self
    }

    pub fn push_many<I, C>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = C>,
        C: Into<LocalCell3>,
    {
        self.chain.extend(items.into_iter().map(|xyz| GridXyz::from_local_cell3(xyz.into())));
        self
    }

    pub fn repeat(mut self, xyz: impl Into<LocalCell3>, count: usize) -> Self {
        let xyz = GridXyz::from_local_cell3(xyz.into());
        self.chain.extend(std::iter::repeat_n(xyz, count));
        self
    }

    pub fn reverse(mut self) -> Self {
        self.chain.reverse();
        self
    }

    pub fn finish(self, subgrid_xyz: impl Into<LocalCell3>) -> SubgridVec {
        SubgridVec::try_from((self.chain, SubgridXyz::from_local_cell3(subgrid_xyz.into()))).unwrap()
    }
}

#[derive(Default, Clone, PartialEq, Reflect)]
pub struct SubgridVec {
    pub(in super::super) grid_offset: GridVec,
    pub(in super::super) subgrid_offset: SubgridXyz,
}
impl SubgridVec {
    pub fn build() -> SubgridVecBuilder {
        SubgridVecBuilder::new()
    }

    fn validate_grid_offset(grid_offset: &GridVec) {
        if grid_offset.scale == Scale::MIN {
            panic!("SubgridVec must be based on a scale no smaller than MIN+1, so there is room to represent the subgrid level as a virtual GridVec leaf");
        }
    }

    pub fn new(grid_offset: GridVec, subgrid_offset: SubgridXyz) -> Self {
        Self::validate_grid_offset(&grid_offset);
        subgrid_offset.assert_local();
        Self { grid_offset, subgrid_offset }
    }

    fn add_mode(self, rhs: IVec3, mode: OverflowMode) -> Result<Self, SubgridVecMathError> {
        let mut stack: Vec<IVec3> = self.grid_offset.to_raw_vec_3d().iter().map(|xyz| xyz.as_ivec3()).collect();
        stack.push(self.subgrid_offset.as_ivec3());

        let leaf = stack.len() - 1;
        stack[leaf] += rhs;
        normalize_ivec3_digit_stack(mode, &mut stack, IVec3::ZERO)?;

        let normalized_subgrid = stack.pop().expect("Subgrid stack must have a leaf");
        let normalized_grid: Vec<GridXyz> = stack.iter().map(|xyz| GridXyz::new_local(xyz.x, xyz.y, xyz.z)).collect();

        let grid_offset = GridVec::try_from(normalized_grid).expect("Normalized grid digits must always be valid");
        let subgrid_offset = SubgridXyz::new_local(normalized_subgrid.x, normalized_subgrid.y, normalized_subgrid.z);
        Ok(SubgridVec::new(grid_offset, subgrid_offset))
    }

    pub fn add_wrap(self, rhs: IVec3) -> Self {
        self.add_mode(rhs, OverflowMode::Wrap).expect("Wrap mode cannot fail")
    }

    pub fn add_checked(self, rhs: IVec3) -> Result<Self, SubgridVecMathError> {
        self.add_mode(rhs, OverflowMode::Checked)
    }

    pub fn add_strict(self, rhs: IVec3) -> Self {
        self.add_mode(rhs, OverflowMode::Strict)
            .expect("Strict mode should panic before returning error")
    }

    pub fn sub_wrap(self, rhs: IVec3) -> Self {
        self.add_wrap(-rhs)
    }

    pub fn sub_checked(self, rhs: IVec3) -> Result<Self, SubgridVecMathError> {
        self.add_checked(-rhs)
    }

    pub fn sub_strict(self, rhs: IVec3) -> Self {
        self.add_strict(-rhs)
    }

    pub fn try_zoom_out(&mut self) -> Result<(), SubgridVecMathError> {
        let Some(parent) = self.grid_offset.parent.as_ref() else {
            return Err(SubgridVecMathError::CannotZoomOutBeyondRoot);
        };
        if parent.parent.is_none() {
            return Err(SubgridVecMathError::CannotZoomOutBeyondRoot);
        }

        let grid_extent = GridVec::new(
            self.grid_offset.clone(),
            GridXyz::new_local(self.subgrid_offset.x, self.subgrid_offset.y, self.subgrid_offset.z),
        );

        let mut unit_extent = UnitVec {
            grid_offset: grid_extent,
            unit_offset: Vec3::ZERO,
        };

        unit_extent.zoom_out();

        let Some(new_parent) = unit_extent.grid_offset.parent else {
            return Err(SubgridVecMathError::CannotZoomOutBeyondRoot);
        };
        self.grid_offset = (*new_parent).clone();
        self.subgrid_offset = SubgridXyz::new_local(unit_extent.grid_offset.xyz.x, unit_extent.grid_offset.xyz.y, unit_extent.grid_offset.xyz.z);
        Ok(())
    }

    pub fn zoom_out(&mut self) {
        let _ = self.try_zoom_out();
    }
}
impl std::fmt::Debug for SubgridVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}: {})", self.grid_offset, self.subgrid_offset)
    }
}
impl std::ops::Add<IVec3> for SubgridVec {
    type Output = Self;

    fn add(self, rhs: IVec3) -> Self::Output {
        self.add_wrap(rhs)
    }
}
impl std::ops::AddAssign<IVec3> for SubgridVec {
    fn add_assign(&mut self, rhs: IVec3) {
        *self = self.clone().add_wrap(rhs);
    }
}
impl std::ops::Sub<IVec3> for SubgridVec {
    type Output = Self;

    fn sub(self, rhs: IVec3) -> Self::Output {
        self.sub_wrap(rhs)
    }
}
impl std::ops::SubAssign<IVec3> for SubgridVec {
    fn sub_assign(&mut self, rhs: IVec3) {
        *self = self.clone().sub_wrap(rhs);
    }
}
impl std::ops::Add<SubgridVec> for SubgridVec {
    type Output = Self;

    fn add(self, rhs: SubgridVec) -> Self::Output {
        fn build_stack(subgrid: &SubgridVec) -> Vec<(Scale, IVec3)> {
            let mut stack = Vec::new();
            let mut cursor = &subgrid.grid_offset;
            loop {
                stack.push((cursor.scale, cursor.xyz.as_ivec3()));
                if let Some(p) = &cursor.parent {
                    cursor = p;
                } else {
                    break;
                }
            }
            stack.reverse();

            let subgrid_scale = subgrid.grid_offset.scale.down().expect("No lower scale for subgrid");
            stack.push((subgrid_scale, subgrid.subgrid_offset.as_ivec3()));

            stack
        }

        let mut a_stack = build_stack(&self);
        let mut b_stack = build_stack(&rhs);
        let max_depth = a_stack.len().max(b_stack.len());

        while a_stack.len() < max_depth {
            let (s, _) = b_stack[a_stack.len()];
            a_stack.push((s, IVec3::ZERO));
        }
        while b_stack.len() < max_depth {
            let (s, _) = a_stack[b_stack.len()];
            b_stack.push((s, IVec3::ZERO));
        }

        let mut raw_stack = Vec::with_capacity(max_depth);
        for i in 0..max_depth {
            let scale = a_stack[i].0;
            let sum = a_stack[i].1 + b_stack[i].1;
            raw_stack.push((scale, sum));
        }

        let mut normalized: Vec<IVec3> = raw_stack.iter().map(|(_, xyz)| *xyz).collect();
        normalize_ivec3_digit_stack(OverflowMode::Wrap, &mut normalized, IVec3::ZERO).expect("Wrap mode cannot fail");
        for (idx, xyz) in normalized.into_iter().enumerate() {
            raw_stack[idx].1 = xyz;
        }

        let mut result: Option<GridVec> = None;
        for (_scale, xyz) in raw_stack {
            result = Some(match result {
                Some(parent) => GridVec::new(parent, GridXyz::new_local(xyz.x, xyz.y, xyz.z)),
                None => GridVec::new_root(GridXyz::new_local(xyz.x, xyz.y, xyz.z)),
            });
        }

        let final_leaf = result.unwrap();
        let subgrid_offset = SubgridXyz::new_local(final_leaf.xyz.x, final_leaf.xyz.y, final_leaf.xyz.z);
        let grid_offset = (*final_leaf.parent.unwrap()).clone();

        SubgridVec { grid_offset, subgrid_offset }
    }
}
impl std::ops::AddAssign<SubgridVec> for SubgridVec {
    fn add_assign(&mut self, rhs: SubgridVec) {
        *self = self.clone() + rhs;
    }
}
impl std::ops::Sub<SubgridVec> for SubgridVec {
    type Output = Self;

    fn sub(self, rhs: SubgridVec) -> Self::Output {
        fn build_stack(subgrid: &SubgridVec) -> Vec<(Scale, IVec3)> {
            let mut stack = Vec::new();
            let mut cursor = &subgrid.grid_offset;
            loop {
                stack.push((cursor.scale, cursor.xyz.as_ivec3()));
                if let Some(p) = &cursor.parent {
                    cursor = p;
                } else {
                    break;
                }
            }
            stack.reverse();

            let subgrid_scale = subgrid.grid_offset.scale.down().expect("No lower scale for subgrid");
            stack.push((subgrid_scale, subgrid.subgrid_offset.as_ivec3()));

            stack
        }

        let mut a_stack = build_stack(&self);
        let mut b_stack = build_stack(&rhs);
        let max_depth = a_stack.len().max(b_stack.len());

        while a_stack.len() < max_depth {
            let (s, _) = b_stack[a_stack.len()];
            a_stack.push((s, IVec3::ZERO));
        }
        while b_stack.len() < max_depth {
            let (s, _) = a_stack[b_stack.len()];
            b_stack.push((s, IVec3::ZERO));
        }

        let mut raw_stack = Vec::with_capacity(max_depth);
        for i in 0..max_depth {
            let scale = a_stack[i].0;
            let diff = a_stack[i].1 - b_stack[i].1;
            raw_stack.push((scale, diff));
        }

        let mut normalized: Vec<IVec3> = raw_stack.iter().map(|(_, xyz)| *xyz).collect();
        normalize_ivec3_digit_stack(OverflowMode::Wrap, &mut normalized, IVec3::ZERO).expect("Wrap mode cannot fail");
        for (idx, xyz) in normalized.into_iter().enumerate() {
            raw_stack[idx].1 = xyz;
        }

        let mut result: Option<GridVec> = None;
        for (_scale, xyz) in raw_stack {
            result = Some(match result {
                Some(parent) => GridVec::new(parent, GridXyz::new_local(xyz.x, xyz.y, xyz.z)),
                None => GridVec::new_root(GridXyz::new_local(xyz.x, xyz.y, xyz.z)),
            });
        }

        let final_leaf = result.unwrap();
        let subgrid_offset = SubgridXyz::new_local(final_leaf.xyz.x, final_leaf.xyz.y, final_leaf.xyz.z);
        let grid_offset = (*final_leaf.parent.unwrap()).clone();

        SubgridVec { grid_offset, subgrid_offset }
    }
}
impl std::ops::SubAssign<SubgridVec> for SubgridVec {
    fn sub_assign(&mut self, rhs: SubgridVec) {
        *self = self.clone() - rhs;
    }
}
impl std::convert::TryFrom<(Vec<GridXyz>, SubgridXyz)> for SubgridVec {
    type Error = &'static str;

    fn try_from((stack, subgrid_offset): (Vec<GridXyz>, SubgridXyz)) -> Result<Self, Self::Error> {
        let grid_offset = GridVec::try_from(stack)?;
        Ok(SubgridVec::new(grid_offset, subgrid_offset))
    }
}
impl std::convert::TryFrom<(Vec<IVec3>, IVec3)> for SubgridVec {
    type Error = &'static str;

    fn try_from((stack, subgrid_offset): (Vec<IVec3>, IVec3)) -> Result<Self, Self::Error> {
        let grid_offset = GridVec::try_from(stack)?;
        let Ok(local) = SubgridXyz::try_from(subgrid_offset) else {
            return Err("SubgridVec requires local subgrid coordinates in [-5..5)");
        };
        Ok(SubgridVec::new(grid_offset, local))
    }
}
