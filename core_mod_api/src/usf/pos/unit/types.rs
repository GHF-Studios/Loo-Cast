use crate::bevy::prelude::{IVec3, Reflect, Vec3};

use crate::usf::math::digit_stack::{DigitStackOverflow, normalize_balanced_digits_checked, normalize_balanced_digits_strict, normalize_balanced_digits_wrap};
use crate::usf::pos::grid::types::GridVec;
use crate::usf::pos::types::{GridXyz, LocalCell3};
use crate::usf::scale::{DynScale, Scale};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitVecMathError {
    OverflowX(DigitStackOverflow),
    OverflowY(DigitStackOverflow),
    OverflowZ(DigitStackOverflow),
    NonFiniteUnitOffset,
}

#[derive(Clone, Copy)]
enum OverflowMode {
    Wrap,
    Checked,
    Strict,
}

fn normalize_component(mode: OverflowMode, digits: &mut [i32], map_error: fn(DigitStackOverflow) -> UnitVecMathError) -> Result<(), UnitVecMathError> {
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

fn normalize_ivec3_digit_stack(mode: OverflowMode, stack: &mut [IVec3], initial_carry: IVec3) -> Result<(), UnitVecMathError> {
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

    normalize_component(mode, &mut x_digits, UnitVecMathError::OverflowX)?;
    normalize_component(mode, &mut y_digits, UnitVecMathError::OverflowY)?;
    normalize_component(mode, &mut z_digits, UnitVecMathError::OverflowZ)?;

    for (idx, value) in stack.iter_mut().enumerate() {
        *value = IVec3::new(x_digits[idx], y_digits[idx], z_digits[idx]);
    }

    Ok(())
}

#[derive(Default)]
pub struct UnitVecBuilder {
    chain: Vec<GridXyz>,
}

impl UnitVecBuilder {
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

    pub fn finish(self, unit_xyz: (f32, f32, f32)) -> UnitVec {
        UnitVec::try_from((self.chain, Vec3::new(unit_xyz.0, unit_xyz.1, unit_xyz.2))).unwrap()
    }
}

#[derive(Default, Clone, PartialEq, Reflect)]
pub struct UnitVec {
    pub(crate) grid_offset: GridVec, // Recursive chunk position
    pub(crate) unit_offset: Vec3,    // Bevy units inside the chunk (e.g., [-500.0..500.0])
}
impl UnitVec {
    pub fn build() -> UnitVecBuilder {
        UnitVecBuilder::default()
    }

    fn validate_unit_offset(unit_offset: &Vec3) {
        if unit_offset.x < -500.0 {
            panic!("X = {} is too small. Range is (-500.0..500.0)", unit_offset.x);
        }
        if unit_offset.x > 500.0 {
            panic!("X = {} is too large. Range is (-500.0..500.0)", unit_offset.x);
        }
        if unit_offset.y < -500.0 {
            panic!("Y = {} is too small. Range is (-500.0..500.0)", unit_offset.y);
        }
        if unit_offset.y > 500.0 {
            panic!("Y = {} is too large. Range is (-500.0..500.0)", unit_offset.y);
        }
        if unit_offset.z < -500.0 {
            panic!("Z = {} is too small. Range is (-500.0..500.0)", unit_offset.z);
        }
        if unit_offset.z > 500.0 {
            panic!("Z = {} is too large. Range is (-500.0..500.0)", unit_offset.z);
        }
    }

    /// Create a new UnitVec from grid and unit offsets.
    pub fn new(grid_offset: GridVec, unit_offset: Vec3) -> Self {
        let mut my_self = Self { grid_offset, unit_offset };
        my_self.normalize();
        my_self
    }

    /// Create a new UnitVec from a grid offset only, with unit offset (0.0, 0.0).
    pub fn new_grid(grid_offset: GridVec) -> Self {
        let unit_offset = Vec3::ZERO;
        Self { grid_offset, unit_offset }
    }

    fn apply_grid_leaf_delta_mode(grid_offset: GridVec, delta: IVec3, mode: OverflowMode) -> Result<GridVec, UnitVecMathError> {
        let mut stack: Vec<IVec3> = grid_offset.to_raw_vec_3d().iter().map(|xyz| xyz.as_ivec3()).collect();
        let leaf = stack.len() - 1;
        stack[leaf] += delta;
        normalize_ivec3_digit_stack(mode, &mut stack, IVec3::ZERO)?;

        let normalized: Vec<GridXyz> = stack.iter().map(|xyz| GridXyz::new_local(xyz.x, xyz.y, xyz.z)).collect();
        Ok(GridVec::try_from(normalized).expect("Normalized grid digits must always be valid"))
    }

    #[inline]
    fn wrap_unit_component(value: f32) -> (f32, i32) {
        let wrapped = ((value + 500.0).rem_euclid(1000.0)) - 500.0;
        let carry = ((value - wrapped) / 1000.0).round() as i32;
        (wrapped, carry)
    }

    fn add_mode(self, rhs: Vec3, mode: OverflowMode) -> Result<Self, UnitVecMathError> {
        let raw = self.unit_offset + rhs;
        if !raw.x.is_finite() || !raw.y.is_finite() || !raw.z.is_finite() {
            return Err(UnitVecMathError::NonFiniteUnitOffset);
        }

        let (wrapped_x, carry_x) = Self::wrap_unit_component(raw.x);
        let (wrapped_y, carry_y) = Self::wrap_unit_component(raw.y);
        let (wrapped_z, carry_z) = Self::wrap_unit_component(raw.z);

        let grid_offset = Self::apply_grid_leaf_delta_mode(self.grid_offset, IVec3::new(carry_x, carry_y, carry_z), mode)?;
        Ok(Self {
            grid_offset,
            unit_offset: Vec3::new(wrapped_x, wrapped_y, wrapped_z),
        })
    }

    pub fn add_wrap(self, rhs: Vec3) -> Self {
        self.add_mode(rhs, OverflowMode::Wrap).expect("Wrap mode cannot fail")
    }

    pub fn add_checked(self, rhs: Vec3) -> Result<Self, UnitVecMathError> {
        self.add_mode(rhs, OverflowMode::Checked)
    }

    pub fn add_strict(self, rhs: Vec3) -> Self {
        self.add_mode(rhs, OverflowMode::Strict)
            .expect("Strict mode should panic before returning error")
    }

    pub fn sub_wrap(self, rhs: Vec3) -> Self {
        self.add_wrap(-rhs)
    }

    pub fn sub_checked(self, rhs: Vec3) -> Result<Self, UnitVecMathError> {
        self.add_checked(-rhs)
    }

    pub fn sub_strict(self, rhs: Vec3) -> Self {
        self.add_strict(-rhs)
    }

    pub fn normalize(&mut self) {
        // Normalize X
        while self.unit_offset.x < -500.0 {
            self.unit_offset.x += 1000.0;
            self.grid_offset.xyz.x -= 1;
        }
        while self.unit_offset.x >= 500.0 {
            self.unit_offset.x -= 1000.0;
            self.grid_offset.xyz.x += 1;
        }

        // Normalize Y
        while self.unit_offset.y < -500.0 {
            self.unit_offset.y += 1000.0;
            self.grid_offset.xyz.y -= 1;
        }
        while self.unit_offset.y >= 500.0 {
            self.unit_offset.y -= 1000.0;
            self.grid_offset.xyz.y += 1;
        }

        // Normalize Z
        while self.unit_offset.z < -500.0 {
            self.unit_offset.z += 1000.0;
            self.grid_offset.xyz.z -= 1;
        }
        while self.unit_offset.z >= 500.0 {
            self.unit_offset.z -= 1000.0;
            self.grid_offset.xyz.z += 1;
        }

        // Normalize GridVec
        self.grid_offset.normalize();
    }

    pub fn zoom_in_multi(&mut self, target_scale: Scale) -> Result<(), &'static str> {
        if target_scale >= self.grid_offset.scale {
            return Err("Target scale must be smaller than current scale");
        }

        // === Phase 1: Build stack of deltas ===
        let mut unit_offset = self.unit_offset;
        let mut stack: Vec<(Scale, IVec3)> = Vec::new();
        let mut scale = self.grid_offset.scale;

        while scale > target_scale {
            let next_scale = scale.down().unwrap();
            let scale_diff = (scale as i8).abs_diff(next_scale as i8);
            let unit_factor = 10f32.powi(scale_diff as i32);
            let unit_center = 500.0 / unit_factor;
            let unit_size = 1000.0 / unit_factor;

            // Compute delta for this scale step
            let grid_delta = IVec3::new(
                ((unit_offset.x + unit_center) / unit_size).floor() as i32,
                ((unit_offset.y + unit_center) / unit_size).floor() as i32,
                ((unit_offset.z + unit_center) / unit_size).floor() as i32,
            );

            // Update unit_offset for next iteration
            unit_offset = (unit_offset - grid_delta.as_vec3() * unit_size) * 10.0;

            // Push the computed delta into our stack
            stack.push((next_scale, grid_delta));

            scale = next_scale;
        }

        // === Phase 2: Normalize the grid deltas (bottom-up) ===
        let mut carry = IVec3::ZERO;
        for i in (0..stack.len()).rev() {
            let (_scale, xyz) = stack[i];

            let new_xyz = xyz + carry;
            let wrapped_x = ((new_xyz.x + 5).rem_euclid(10)) - 5;
            let wrapped_y = ((new_xyz.y + 5).rem_euclid(10)) - 5;
            let wrapped_z = ((new_xyz.z + 5).rem_euclid(10)) - 5;

            let carry_x = (new_xyz.x - wrapped_x).div_euclid(10);
            let carry_y = (new_xyz.y - wrapped_y).div_euclid(10);
            let carry_z = (new_xyz.z - wrapped_z).div_euclid(10);

            stack[i].1 = IVec3::new(wrapped_x, wrapped_y, wrapped_z);
            carry = IVec3::new(carry_x, carry_y, carry_z);
        }

        // === Phase 3: Apply final carry to current grid_offset ===
        let mut new_grid = self.grid_offset.clone();
        new_grid += carry;

        // === Phase 4: Build new GridVec tree ===
        for (_scale, xyz) in stack {
            let next = GridVec::new(new_grid, GridXyz::new_local(xyz.x, xyz.y, xyz.z));
            new_grid = next;
        }

        // === Phase 5: Normalize final unit_offset ===
        let wrapped_x = ((unit_offset.x + 500.0).rem_euclid(1000.0)) - 500.0;
        let wrapped_y = ((unit_offset.y + 500.0).rem_euclid(1000.0)) - 500.0;
        let wrapped_z = ((unit_offset.z + 500.0).rem_euclid(1000.0)) - 500.0;

        // === Phase 6: Final assignment ===
        self.grid_offset = new_grid;
        self.unit_offset = Vec3::new(wrapped_x, wrapped_y, wrapped_z);
        Self::validate_unit_offset(&self.unit_offset);

        Ok(())
    }

    pub fn zoom_in(&mut self) {
        if let Some(target) = self.grid_offset.scale.down() {
            let _ = self.zoom_in_multi(target);
        }
    }

    pub fn zoom_out(&mut self) {
        let child = self.grid_offset.clone();
        let parent = match &child.parent {
            Some(p) => p.clone(),
            None => return,
        };

        let chunk_size = 1000.0;
        let child_factor = 10.0;
        let child_size = chunk_size / child_factor;

        // Step 1: Get child's origin in parent space
        let child_origin = Vec3::new(
            child.xyz.x as f32 * child_size,
            child.xyz.y as f32 * child_size,
            child.xyz.z as f32 * child_size,
        );

        // Step 2: Shift up into parent space, then rescale
        let offset_in_parent = child_origin + (self.unit_offset / child_factor);

        // Step 3: Update context
        self.grid_offset = (*parent).clone();
        self.unit_offset = offset_in_parent;
        Self::validate_unit_offset(&self.unit_offset);
    }
}
impl std::fmt::Debug for UnitVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}: {})", self.grid_offset, self.unit_offset)
    }
}
impl std::ops::Add<IVec3> for UnitVec {
    type Output = Self;

    fn add(mut self, rhs: IVec3) -> Self::Output {
        self.grid_offset += rhs;
        self
    }
}
impl std::ops::AddAssign<IVec3> for UnitVec {
    fn add_assign(&mut self, rhs: IVec3) {
        self.grid_offset += rhs;
    }
}
impl std::ops::Sub<IVec3> for UnitVec {
    type Output = Self;

    fn sub(mut self, rhs: IVec3) -> Self::Output {
        self.grid_offset -= rhs;
        self
    }
}
impl std::ops::SubAssign<IVec3> for UnitVec {
    fn sub_assign(&mut self, rhs: IVec3) {
        self.grid_offset -= rhs;
    }
}
impl std::ops::Add<Vec3> for UnitVec {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        self.add_wrap(rhs)
    }
}
impl std::ops::AddAssign<Vec3> for UnitVec {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = self.clone().add_wrap(rhs);
    }
}
impl std::ops::Sub<Vec3> for UnitVec {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self::Output {
        self.sub_wrap(rhs)
    }
}
impl std::ops::SubAssign<Vec3> for UnitVec {
    fn sub_assign(&mut self, rhs: Vec3) {
        *self = self.clone().sub_wrap(rhs);
    }
}
impl std::ops::Add<UnitVec> for UnitVec {
    type Output = Self;

    fn add(self, rhs: UnitVec) -> Self::Output {
        const MAX_DEPTH_DIFF: u8 = Scale::SCALE_LEVEL_COUNT - 1;

        fn stack_up(mut cursor: &GridVec) -> Vec<IVec3> {
            let mut stack = Vec::new();
            loop {
                stack.push(cursor.xyz.as_ivec3());
                if let Some(p) = &cursor.parent {
                    cursor = p;
                } else {
                    break;
                }
            }
            stack.reverse();
            stack
        }

        if (self.grid_offset.scale as i8 - rhs.grid_offset.scale as i8).abs() > MAX_DEPTH_DIFF as i8 {
            panic!("Cannot add UnitVec with grid offsets differing in scale by more than {} levels", MAX_DEPTH_DIFF);
        }

        let mut a_stack = stack_up(&self.grid_offset);
        let mut b_stack = stack_up(&rhs.grid_offset);

        let max_depth = a_stack.len().max(b_stack.len());

        // Pad shorter stack with (scale, ZERO)
        while a_stack.len() < max_depth {
            a_stack.push(IVec3::ZERO);
        }
        while b_stack.len() < max_depth {
            b_stack.push(IVec3::ZERO);
        }

        // === Phase 1: Accumulate raw sums top-down ===
        let mut raw_stack = Vec::with_capacity(max_depth);
        for i in 0..max_depth {
            let sum = a_stack[i] + b_stack[i];
            raw_stack.push(sum);
        }

        // === Phase 2: Rescale and add unit offsets ===
        let unit_offset_sum = match self.grid_offset.scale.cmp(&rhs.grid_offset.scale) {
            std::cmp::Ordering::Equal => self.unit_offset + rhs.unit_offset,
            std::cmp::Ordering::Greater => {
                // self is deeper → scale *up* self
                let factor = 10.0_f32.powi((self.grid_offset.scale as i8 - rhs.grid_offset.scale as i8) as i32);
                rhs.unit_offset + (self.unit_offset * factor)
            }
            std::cmp::Ordering::Less => {
                // rhs is deeper → scale *up* rhs
                let factor = 10.0_f32.powi((rhs.grid_offset.scale as i8 - self.grid_offset.scale as i8) as i32);
                self.unit_offset + (rhs.unit_offset * factor)
            }
        };

        // === Phase 3: Extract unit_carry from summed unit_offset ===
        let wrapped_x = ((unit_offset_sum.x + 500.0).rem_euclid(1000.0)) - 500.0;
        let wrapped_y = ((unit_offset_sum.y + 500.0).rem_euclid(1000.0)) - 500.0;
        let wrapped_z = ((unit_offset_sum.z + 500.0).rem_euclid(1000.0)) - 500.0;
        let carry_x = ((unit_offset_sum.x - wrapped_x) / 1000.0).floor() as i32;
        let carry_y = ((unit_offset_sum.y - wrapped_y) / 1000.0).floor() as i32;
        let carry_z = ((unit_offset_sum.z - wrapped_z) / 1000.0).floor() as i32;

        let unit_offset = Vec3::new(wrapped_x, wrapped_y, wrapped_z);
        normalize_ivec3_digit_stack(OverflowMode::Wrap, &mut raw_stack, IVec3::new(carry_x, carry_y, carry_z)).expect("Wrap mode cannot fail");

        // === Phase 5: Build final GridVec tree ===
        let mut result: Option<GridVec> = None;
        for xyz in raw_stack {
            result = Some(match result {
                Some(parent) => GridVec::new(parent, GridXyz::new_local(xyz.x, xyz.y, xyz.z)),
                None => GridVec::new_root(GridXyz::new_local(xyz.x, xyz.y, xyz.z)),
            });
        }

        UnitVec::new(result.unwrap(), unit_offset)
    }
}
impl std::ops::AddAssign<UnitVec> for UnitVec {
    fn add_assign(&mut self, rhs: UnitVec) {
        *self = self.clone() + rhs;
    }
}
impl std::ops::Sub<UnitVec> for UnitVec {
    type Output = Self;

    fn sub(mut self, mut rhs: UnitVec) -> Self::Output {
        const MAX_DEPTH_DIFF: u8 = Scale::SCALE_LEVEL_COUNT - 1;

        match self.grid_offset.scale.cmp(&rhs.grid_offset.scale) {
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Greater => self.zoom_in_multi(rhs.grid_offset.scale).unwrap(),
            std::cmp::Ordering::Less => rhs.zoom_in_multi(self.grid_offset.scale).unwrap(),
        }

        fn stack_up(mut cursor: &GridVec) -> Vec<IVec3> {
            let mut stack = Vec::new();
            loop {
                stack.push(cursor.xyz.as_ivec3());
                if let Some(p) = &cursor.parent {
                    cursor = p;
                } else {
                    break;
                }
            }
            stack.reverse();
            stack
        }

        if (self.grid_offset.scale as i8 - rhs.grid_offset.scale as i8).abs() > MAX_DEPTH_DIFF as i8 {
            panic!(
                "Cannot subtract UnitVec with grid offsets differing in scale by more than {} levels",
                MAX_DEPTH_DIFF
            );
        }

        let mut a_stack = stack_up(&self.grid_offset);
        let mut b_stack = stack_up(&rhs.grid_offset);

        let max_depth = a_stack.len().max(b_stack.len());

        // Pad shorter stack with (scale, ZERO)
        while a_stack.len() < max_depth {
            a_stack.push(IVec3::ZERO);
        }
        while b_stack.len() < max_depth {
            b_stack.push(IVec3::ZERO);
        }

        // === Phase 1: Accumulate raw diffs top-down ===
        let mut raw_stack = Vec::with_capacity(max_depth);
        for i in 0..max_depth {
            let diff = a_stack[i] - b_stack[i];
            raw_stack.push(diff);
        }

        // === Phase 2: Rescale and add unit offsets ===
        let unit_offset_diff = match self.grid_offset.scale.cmp(&rhs.grid_offset.scale) {
            std::cmp::Ordering::Equal => self.unit_offset - rhs.unit_offset,
            std::cmp::Ordering::Greater => {
                // self is deeper → scale *up* self
                let factor = 10.0_f32.powi((self.grid_offset.scale as i8 - rhs.grid_offset.scale as i8) as i32);
                (self.unit_offset * factor) - rhs.unit_offset
            }
            std::cmp::Ordering::Less => {
                // rhs is deeper → scale *up* rhs
                let factor = 10.0_f32.powi((rhs.grid_offset.scale as i8 - self.grid_offset.scale as i8) as i32);
                self.unit_offset - (rhs.unit_offset * factor)
            }
        };

        // === Phase 3: Extract unit_carry from summed unit_offset ===
        let wrapped_x = ((unit_offset_diff.x + 500.0).rem_euclid(1000.0)) - 500.0;
        let wrapped_y = ((unit_offset_diff.y + 500.0).rem_euclid(1000.0)) - 500.0;
        let wrapped_z = ((unit_offset_diff.z + 500.0).rem_euclid(1000.0)) - 500.0;
        let carry_x = ((unit_offset_diff.x - wrapped_x) / 1000.0).floor() as i32;
        let carry_y = ((unit_offset_diff.y - wrapped_y) / 1000.0).floor() as i32;
        let carry_z = ((unit_offset_diff.z - wrapped_z) / 1000.0).floor() as i32;

        let unit_offset = Vec3::new(wrapped_x, wrapped_y, wrapped_z);
        normalize_ivec3_digit_stack(OverflowMode::Wrap, &mut raw_stack, IVec3::new(carry_x, carry_y, carry_z)).expect("Wrap mode cannot fail");

        // === Phase 5: Build final GridVec tree ===
        let mut result: Option<GridVec> = None;
        for xyz in raw_stack {
            result = Some(match result {
                Some(parent) => GridVec::new(parent, GridXyz::new_local(xyz.x, xyz.y, xyz.z)),
                None => GridVec::new_root(GridXyz::new_local(xyz.x, xyz.y, xyz.z)),
            });
        }

        UnitVec::new(result.unwrap(), unit_offset)
    }
}
impl std::ops::SubAssign<UnitVec> for UnitVec {
    fn sub_assign(&mut self, rhs: UnitVec) {
        *self = self.clone() - rhs;
    }
}
impl std::convert::TryFrom<(Vec<GridXyz>, Vec3)> for UnitVec {
    type Error = &'static str;

    fn try_from((stack, unit_offset): (Vec<GridXyz>, Vec3)) -> Result<Self, Self::Error> {
        let grid_offset = GridVec::try_from(stack)?;
        Ok(UnitVec::new(grid_offset, unit_offset))
    }
}
impl std::convert::TryFrom<(Vec<IVec3>, Vec3)> for UnitVec {
    type Error = &'static str;

    fn try_from((stack, unit_offset): (Vec<IVec3>, Vec3)) -> Result<Self, Self::Error> {
        let grid_offset = GridVec::try_from(stack)?;
        Ok(UnitVec::new(grid_offset, unit_offset))
    }
}
