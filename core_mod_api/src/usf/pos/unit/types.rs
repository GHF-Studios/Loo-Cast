use bevy::prelude::{IVec2, Vec2, Vec3};

use crate::usf::scale::{Scale, DynScale};
use crate::utils::logic_safety::Checked;
use crate::usf::pos::grid::types::GridPos;

pub struct UnitPosBuilder {
    chain: Vec<IVec2>,
}

impl UnitPosBuilder {
    pub fn from_root(root: IVec2) -> Self {
        Self {
            chain: vec![root],
        }
    }

    pub fn push(mut self, next: IVec2) -> Self {
        self.chain.push(next);
        self
    }

    pub fn push_many<I: IntoIterator<Item = IVec2>>(mut self, items: I) -> Self {
        self.chain.extend(items);
        self
    }

    pub fn repeat(mut self, xy: IVec2, count: usize) -> Self {
        self.chain.extend(std::iter::repeat_n(xy, count));
        self
    }

    pub fn finish(self, unit_offset: Vec2) -> UnitPos {
        UnitPos::try_from((self.chain, unit_offset)).unwrap()
    }
}

#[derive(Clone, PartialEq)]
pub struct UnitPos {
    pub(in super::super) grid_offset: GridPos, // Recursive chunk position
    pub(in super::super) unit_offset: Vec3, // Bevy units inside the chunk (e.g., [-500.0..500.0])
}
impl UnitPos {
    pub fn build(root: IVec2) -> UnitPosBuilder {
        UnitPosBuilder::from_root(root)
    }

    fn validate_unit_offset(unit_offset: &Vec3) {
        if unit_offset.x < -500.0 { panic!("X = {} is too small. Range is (-500.0..500.0)", unit_offset.x); }
        if unit_offset.x > 500.0 { panic!("X = {} is too large. Range is (-500.0..500.0)", unit_offset.x); }
        if unit_offset.y < -500.0 { panic!("Y = {} is too small. Range is (-500.0..500.0)", unit_offset.y); }
        if unit_offset.y > 500.0 { panic!("Y = {} is too large. Range is (-500.0..500.0)", unit_offset.y); }
    }

    /// Compute the one and only valid Z coordinate for any given scale level.
    /// - Scale::MIN corresponds to Z = -10.0
    /// - Each subsequent smaller scale decreases Z by an additional 10.0 units.
    #[inline]
    fn compute_z(scale: Scale) -> f32 {
        scale.to_index_from_bottom() as f32 * -10.0
    }

    pub fn new(grid_offset: GridPos, unit_offset: Vec2) -> Self {
        let unit_offset = unit_offset.extend(Self::compute_z(grid_offset.scale));
        Self::validate_unit_offset(&unit_offset);
        Self { grid_offset, unit_offset }
    }

    pub fn new_unchecked(grid_offset: GridPos, unit_offset: Vec2) -> Self {
        let unit_offset = unit_offset.extend(Self::compute_z(grid_offset.scale));
        Self { grid_offset, unit_offset }
    }
    
    fn zoom_in_multi(&mut self, target_scale: Scale) -> Result<(), &'static str> {
        if target_scale >= self.grid_offset.scale {
            return Err("Target scale must be smaller than current scale")
        }

        let cursor = &mut self.grid_offset;
        let mut unit_offset = self.unit_offset.truncate();
        let mut placeholder_grid_pos: Option<GridPos<Checked>> = Some(GridPos::default().into());

        while cursor.scale > target_scale {
            let scale_factor = 10.0;

            // Push unit_offset into grid
            let grid_delta = IVec2::new(
                (unit_offset.x / 1000.0).floor() as i32,
                (unit_offset.y / 1000.0).floor() as i32,
            );

            unit_offset -= Vec2::new(
                grid_delta.x as f32 * 1000.0,
                grid_delta.y as f32 * 1000.0,
            );

            // Multiply unit_offset by 10 to rebase to finer scale
            unit_offset *= scale_factor;

            // Step down
            unsafe {
                let placeholder = std::mem::take(&mut placeholder_grid_pos).unwrap_unchecked();
                let parent = std::mem::replace(cursor, placeholder);
                placeholder_grid_pos = Some(std::mem::replace(cursor, GridPos::new(
                    parent,
                    grid_delta,
                )));
            }
        }

        self.unit_offset = unit_offset.extend(Self::compute_z(cursor.scale));
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
        let child_origin = child.xy.as_vec2() * child_size;

        // Step 2: Shift up into parent space, then rescale
        let offset_in_parent = (self.unit_offset.truncate() + child_origin) / child_factor;

        // Step 3: Update context
        self.grid_offset = (*parent).clone();
        self.unit_offset = Vec3::new(offset_in_parent.x, offset_in_parent.y, Self::compute_z(parent.scale));
        Self::validate_unit_offset(&self.unit_offset);
    }
}
impl std::fmt::Debug for UnitPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}: {})", self.grid_offset, self.unit_offset)
    }
}
impl std::ops::Add<IVec2> for UnitPos {
    type Output = Self;

    fn add(mut self, rhs: IVec2) -> Self::Output {
        self.grid_offset += rhs;
        self
    }
}
impl std::ops::AddAssign<IVec2> for UnitPos {
    fn add_assign(&mut self, rhs: IVec2) {
        self.grid_offset += rhs;
    }
}
impl std::ops::Sub<IVec2> for UnitPos {
    type Output = Self;

    fn sub(mut self, rhs: IVec2) -> Self::Output {
        self.grid_offset -= rhs;
        self
    }
}
impl std::ops::SubAssign<IVec2> for UnitPos {
    fn sub_assign(&mut self, rhs: IVec2) {
        self.grid_offset -= rhs;
    }
}
impl std::ops::Add<Vec3> for UnitPos {
    type Output = Self;

    fn add(mut self, rhs: Vec3) -> Self::Output {
        self.unit_offset += rhs;
        Self::validate_unit_offset(&self.unit_offset);
        self
    }
}
impl std::ops::AddAssign<Vec3> for UnitPos {
    fn add_assign(&mut self, rhs: Vec3) {
        self.unit_offset += rhs;
        Self::validate_unit_offset(&self.unit_offset);
    }
}
impl std::ops::Sub<Vec3> for UnitPos {
    type Output = Self;

    fn sub(mut self, rhs: Vec3) -> Self::Output {
        self.unit_offset -= rhs;
        Self::validate_unit_offset(&self.unit_offset);
        self
    }
}
impl std::ops::SubAssign<Vec3> for UnitPos {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.unit_offset -= rhs;
        Self::validate_unit_offset(&self.unit_offset);
    }
}
impl std::ops::Add<UnitPos> for UnitPos {
    type Output = Self;

    fn add(self, rhs: UnitPos) -> Self::Output {
        const MAX_DEPTH_DIFF: u8 = 4;

        fn stack_up(mut cursor: &GridPos) -> Vec<IVec2> {
            let mut stack = Vec::new();
            loop {
                stack.push(cursor.xy);
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
            panic!("Cannot add UnitPos with grid offsets differing in scale by more than {} levels", MAX_DEPTH_DIFF);
        }

        let mut a_stack = stack_up(&self.grid_offset);
        let mut b_stack = stack_up(&rhs.grid_offset);

        let max_depth = a_stack.len().max(b_stack.len());

        // Pad shorter stack with (scale, ZERO)
        while a_stack.len() < max_depth {
            a_stack.push(IVec2::ZERO);
        }
        while b_stack.len() < max_depth {
            b_stack.push(IVec2::ZERO);
        }

        // === Phase 1: Accumulate raw sums top-down ===
        let mut raw_stack = Vec::with_capacity(max_depth);
        for i in 0..max_depth {
            let sum = a_stack[i] + b_stack[i];
            raw_stack.push(sum);
        }

        // === Phase 2: Rescale and add unit offsets ===
        let unit_offset_sum = match self.grid_offset.scale.cmp(&rhs.grid_offset.scale) {
            std::cmp::Ordering::Equal => self.unit_offset.truncate() + rhs.unit_offset.truncate(),
            std::cmp::Ordering::Greater => {
                // self is deeper → scale *up* self
                let factor = 10.0_f32.powi((self.grid_offset.scale as i8 - rhs.grid_offset.scale as i8) as i32);
                rhs.unit_offset.truncate() + (self.unit_offset.truncate() * factor)
            },
            std::cmp::Ordering::Less => {
                // rhs is deeper → scale *up* rhs
                let factor = 10.0_f32.powi((rhs.grid_offset.scale as i8 - self.grid_offset.scale as i8) as i32);
                self.unit_offset.truncate() + (rhs.unit_offset.truncate() * factor)
            },
        };

        // === Phase 3: Extract unit_carry from summed unit_offset ===
        let wrapped_x = ((unit_offset_sum.x + 500.0).rem_euclid(1000.0)) - 500.0;
        let wrapped_y = ((unit_offset_sum.y + 500.0).rem_euclid(1000.0)) - 500.0;
        let carry_x = ((unit_offset_sum.x - wrapped_x) / 1000.0).floor() as i32;
        let carry_y = ((unit_offset_sum.y - wrapped_y) / 1000.0).floor() as i32;

        let unit_offset = Vec2::new(wrapped_x, wrapped_y);
        let mut carry = IVec2::new(carry_x, carry_y);

        // === Phase 4: Normalize bottom-up with wrapping + carry + unit_carry ===
        for i in (0..raw_stack.len()).rev() {
            let sum = raw_stack[i];
            let wrapped_x = ((sum.x + carry.x + 5).rem_euclid(10)) - 5;
            let wrapped_y = ((sum.y + carry.y + 5).rem_euclid(10)) - 5;
            let carry_x = (sum.x + carry.x - wrapped_x).div_euclid(10);
            let carry_y = (sum.y + carry.y - wrapped_y).div_euclid(10);

            raw_stack[i] = IVec2::new(wrapped_x, wrapped_y);
            carry = IVec2::new(carry_x, carry_y);
        }

        // === Phase 5: Build final GridPos tree ===
        let mut result: Option<GridPos> = None;
        for xy in raw_stack {
            result = Some(match result {
                Some(parent) => GridPos::new(parent, xy),
                None => GridPos::new_root(xy),
            });
        }

        UnitPos::new(result.unwrap(), unit_offset)
    }
}
impl std::ops::AddAssign<UnitPos> for UnitPos {
    fn add_assign(&mut self, rhs: UnitPos) {
        *self = self.clone() + rhs;
    }
}
impl std::ops::Sub<UnitPos> for UnitPos {
    type Output = Self;

    fn sub(mut self, mut rhs: UnitPos) -> Self::Output {
        const MAX_DEPTH_DIFF: u8 = 4;

        match self.grid_offset.scale.cmp(&rhs.grid_offset.scale) {
            std::cmp::Ordering::Equal => {},
            std::cmp::Ordering::Greater => self.zoom_in_multi(rhs.grid_offset.scale).unwrap(),
            std::cmp::Ordering::Less => rhs.zoom_in_multi(self.grid_offset.scale).unwrap(),
        }

        fn stack_up(mut cursor: &GridPos) -> Vec<IVec2> {
            let mut stack = Vec::new();
            loop {
                stack.push(cursor.xy);
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
            panic!("Cannot subtract UnitPos with grid offsets differing in scale by more than {} levels", MAX_DEPTH_DIFF);
        }

        let mut a_stack = stack_up(&self.grid_offset);
        let mut b_stack = stack_up(&rhs.grid_offset);

        let max_depth = a_stack.len().max(b_stack.len());

        // Pad shorter stack with (scale, ZERO)
        while a_stack.len() < max_depth {
            a_stack.push(IVec2::ZERO);
        }
        while b_stack.len() < max_depth {
            b_stack.push(IVec2::ZERO);
        }

        // === Phase 1: Accumulate raw diffs top-down ===
        let mut raw_stack = Vec::with_capacity(max_depth);
        for i in 0..max_depth {
            let diff = a_stack[i] - b_stack[i];
            raw_stack.push(diff);
        }

        // === Phase 2: Rescale and add unit offsets ===
        let unit_offset_diff = match self.grid_offset.scale.cmp(&rhs.grid_offset.scale) {
            std::cmp::Ordering::Equal => self.unit_offset.truncate() - rhs.unit_offset.truncate(),
            std::cmp::Ordering::Greater => {
                // self is deeper → scale *up* self
                let factor = 10.0_f32.powi((self.grid_offset.scale as i8 - rhs.grid_offset.scale as i8) as i32);
                (self.unit_offset.truncate() * factor) - rhs.unit_offset.truncate()
            },
            std::cmp::Ordering::Less => {
                // rhs is deeper → scale *up* rhs
                let factor = 10.0_f32.powi((rhs.grid_offset.scale as i8 - self.grid_offset.scale as i8) as i32);
                self.unit_offset.truncate() - (rhs.unit_offset.truncate() * factor)
            },
        };

        // === Phase 3: Extract unit_carry from summed unit_offset ===
        let wrapped_x = ((unit_offset_diff.x + 500.0).rem_euclid(1000.0)) - 500.0;
        let wrapped_y = ((unit_offset_diff.y + 500.0).rem_euclid(1000.0)) - 500.0;
        let carry_x = ((unit_offset_diff.x - wrapped_x) / 1000.0).floor() as i32;
        let carry_y = ((unit_offset_diff.y - wrapped_y) / 1000.0).floor() as i32;

        let unit_offset = Vec2::new(wrapped_x, wrapped_y);
        let mut carry = IVec2::new(carry_x, carry_y);

        // === Phase 4: Normalize bottom-up with wrapping + carry + unit_carry ===
        for i in (0..raw_stack.len()).rev() {
            let diff = raw_stack[i];
            let wrapped_x = ((diff.x + carry.x + 5).rem_euclid(10)) - 5;
            let wrapped_y = ((diff.y + carry.y + 5).rem_euclid(10)) - 5;
            let carry_x = (diff.x + carry.x - wrapped_x).div_euclid(10);
            let carry_y = (diff.y + carry.y - wrapped_y).div_euclid(10);

            raw_stack[i] = IVec2::new(wrapped_x, wrapped_y);
            carry = IVec2::new(carry_x, carry_y);
        }

        // === Phase 5: Build final GridPos tree ===
        let mut result: Option<GridPos> = None;
        for xy in raw_stack {
            result = Some(match result {
                Some(parent) => GridPos::new(parent, xy),
                None => GridPos::new_root(xy),
            });
        }

        UnitPos::new(result.unwrap(), unit_offset)
    }
}
impl std::ops::SubAssign<UnitPos> for UnitPos {
    fn sub_assign(&mut self, rhs: UnitPos) {
        *self = self.clone() - rhs;
    }
}
impl std::convert::TryFrom<(Vec<IVec2>, Vec2)> for UnitPos {
    type Error = &'static str;

    fn try_from((stack, unit_offset): (Vec<IVec2>, Vec2)) -> Result<Self, Self::Error> {
        let grid_offset = GridPos::try_from(stack)?;
        Ok(UnitPos::new(grid_offset, unit_offset))
    }
}
