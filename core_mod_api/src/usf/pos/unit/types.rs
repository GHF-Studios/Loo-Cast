use crate::bevy::prelude::{IVec2, Reflect, Vec2, Vec3};

use crate::usf::pos::grid::types::GridVec;
use crate::usf::scale::{DynScale, Scale};

#[derive(Default)]
pub struct UnitVecBuilder {
    chain: Vec<IVec2>,
}

impl UnitVecBuilder {
    pub fn push(mut self, next: (i32, i32)) -> Self {
        let next = IVec2::new(next.0, next.1);
        self.chain.push(next);
        self
    }

    pub fn push_many<I: IntoIterator<Item = (i32, i32)>>(mut self, items: I) -> Self {
        self.chain.extend(items.into_iter().map(|xy| IVec2::new(xy.0, xy.1)));
        self
    }

    pub fn repeat(mut self, xy: (i32, i32), count: usize) -> Self {
        self.chain.extend(std::iter::repeat_n(IVec2::new(xy.0, xy.1), count));
        self
    }

    pub fn reverse(mut self) -> Self {
        self.chain.reverse();
        self
    }

    pub fn finish(self, unit_xy: (f32, f32)) -> UnitVec {
        UnitVec::try_from((self.chain, Vec2::new(unit_xy.0, unit_xy.1))).unwrap()
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
    }

    /// Create a new UnitVec from grid and unit offsets.
    pub fn new(grid_offset: GridVec, unit_offset: Vec2) -> Self {
        let unit_offset = unit_offset.extend(grid_offset.scale.compute_z());
        let mut my_self = Self { grid_offset, unit_offset };
        my_self.normalize();
        my_self
    }

    /// Create a new UnitVec from a grid offset only, with unit offset (0.0, 0.0).
    pub fn new_grid(grid_offset: GridVec) -> Self {
        let unit_offset = Vec2::ZERO.extend(grid_offset.scale.compute_z());
        Self { grid_offset, unit_offset }
    }

    pub fn normalize(&mut self) {
        // Normalize X
        while self.unit_offset.x < -500.0 {
            self.unit_offset.x += 1000.0;
            self.grid_offset.xy.x -= 1;
        }
        while self.unit_offset.x >= 500.0 {
            self.unit_offset.x -= 1000.0;
            self.grid_offset.xy.x += 1;
        }

        // Normalize Y
        while self.unit_offset.y < -500.0 {
            self.unit_offset.y += 1000.0;
            self.grid_offset.xy.y -= 1;
        }
        while self.unit_offset.y >= 500.0 {
            self.unit_offset.y -= 1000.0;
            self.grid_offset.xy.y += 1;
        }

        // Normalize GridVec
        self.grid_offset.normalize();
    }

    // /// Returns the total Bevy-space offset between two same-layer siblings.
    // /// It's like std::ops::Sub<UnitVec>, but can be used to return a Vec2 representing the native logical offset in Bevy space
    // /// This assumes that we store and update an `origin: GridVec` somewhere so that we can convert between Bevy space and UnitVec space (which is just a translation, no fancy transformation needed)
    // /// - Fails if `grid_offset.parent` differs, or if scale differs.
    // pub fn native_logical_offset(from: &UnitVec, to: &UnitVec) -> Option<Vec2> {
    //     // Scale must match
    //     if from.grid_offset.scale != to.grid_offset.scale {
    //         return None;
    //     }
    //
    //     // Parents must be equal (value-wise)
    //     let same_parent = match (&from.grid_offset.parent, &to.grid_offset.parent) {
    //         (Some(a), Some(b)) => **a == **b,
    //         (None, None) => true,
    //         _ => false,
    //     };
    //     if !same_parent {
    //         return None;
    //     }
    //
    //     // Compute offset
    //     let chunk_delta = (to.grid_offset.xy - from.grid_offset.xy).as_vec2() * 1000.0;
    //     let unit_delta = to.unit_offset.truncate() - from.unit_offset.truncate();
    //
    //     Some(chunk_delta + unit_delta)
    // }

    pub fn zoom_in_multi(&mut self, target_scale: Scale) -> Result<(), &'static str> {
        if target_scale >= self.grid_offset.scale {
            return Err("Target scale must be smaller than current scale");
        }

        // === Phase 1: Build stack of deltas ===
        let mut unit_offset = self.unit_offset.truncate();
        let mut stack: Vec<(Scale, IVec2)> = Vec::new();
        let mut scale = self.grid_offset.scale;

        while scale > target_scale {
            let next_scale = scale.down().unwrap();
            let scale_diff = (scale as i8).abs_diff(next_scale as i8);
            let unit_factor = 10f32.powi(scale_diff as i32);
            let unit_center = 500.0 / unit_factor;
            let unit_size = 1000.0 / unit_factor;

            // Compute delta for this scale step
            let grid_delta = IVec2::new(
                ((unit_offset.x + unit_center) / unit_size).floor() as i32,
                ((unit_offset.y + unit_center) / unit_size).floor() as i32,
            );

            // Update unit_offset for next iteration
            unit_offset = (unit_offset - Vec2::new(grid_delta.x as f32 * unit_size, grid_delta.y as f32 * unit_size)) * 10.0;

            // Push the computed delta into our stack
            stack.push((next_scale, grid_delta));

            scale = next_scale;
        }

        // === Phase 2: Normalize the grid deltas (bottom-up) ===
        let mut carry = IVec2::ZERO;
        for i in (0..stack.len()).rev() {
            let (_scale, xy) = stack[i];

            let new_xy = xy + carry;
            let wrapped_x = ((new_xy.x + 5).rem_euclid(10)) - 5;
            let wrapped_y = ((new_xy.y + 5).rem_euclid(10)) - 5;

            let carry_x = (new_xy.x - wrapped_x).div_euclid(10);
            let carry_y = (new_xy.y - wrapped_y).div_euclid(10);

            stack[i].1 = IVec2::new(wrapped_x, wrapped_y);
            carry = IVec2::new(carry_x, carry_y);
        }

        // === Phase 3: Apply final carry to current grid_offset ===
        let mut new_grid = self.grid_offset.clone();
        new_grid.xy += carry;

        // === Phase 4: Build new GridVec tree ===
        for (_scale, xy) in stack {
            new_grid = GridVec::new(new_grid, xy);
        }

        // === Phase 5: Normalize final unit_offset ===
        let wrapped_x = ((unit_offset.x + 500.0).rem_euclid(1000.0)) - 500.0;
        let wrapped_y = ((unit_offset.y + 500.0).rem_euclid(1000.0)) - 500.0;

        // === Phase 6: Final assignment ===
        self.grid_offset = new_grid;
        self.unit_offset = Vec2::new(wrapped_x, wrapped_y).extend(self.grid_offset.scale.compute_z());
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
        let offset_in_parent = child_origin + (self.unit_offset.truncate() / child_factor);

        // Step 3: Update context
        self.grid_offset = (*parent).clone();
        self.unit_offset = Vec3::new(offset_in_parent.x, offset_in_parent.y, parent.scale.compute_z());
        Self::validate_unit_offset(&self.unit_offset);
    }
}
impl std::fmt::Debug for UnitVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}: {})", self.grid_offset, self.unit_offset)
    }
}
impl std::ops::Add<IVec2> for UnitVec {
    type Output = Self;

    fn add(mut self, rhs: IVec2) -> Self::Output {
        self.grid_offset += rhs;
        self
    }
}
impl std::ops::AddAssign<IVec2> for UnitVec {
    fn add_assign(&mut self, rhs: IVec2) {
        self.grid_offset += rhs;
    }
}
impl std::ops::Sub<IVec2> for UnitVec {
    type Output = Self;

    fn sub(mut self, rhs: IVec2) -> Self::Output {
        self.grid_offset -= rhs;
        self
    }
}
impl std::ops::SubAssign<IVec2> for UnitVec {
    fn sub_assign(&mut self, rhs: IVec2) {
        self.grid_offset -= rhs;
    }
}
impl std::ops::Add<Vec3> for UnitVec {
    type Output = Self;

    fn add(mut self, rhs: Vec3) -> Self::Output {
        self.unit_offset += rhs;
        Self::validate_unit_offset(&self.unit_offset);
        self
    }
}
impl std::ops::AddAssign<Vec3> for UnitVec {
    fn add_assign(&mut self, rhs: Vec3) {
        self.unit_offset += rhs;
        Self::validate_unit_offset(&self.unit_offset);
    }
}
impl std::ops::Sub<Vec3> for UnitVec {
    type Output = Self;

    fn sub(mut self, rhs: Vec3) -> Self::Output {
        self.unit_offset -= rhs;
        Self::validate_unit_offset(&self.unit_offset);
        self
    }
}
impl std::ops::SubAssign<Vec3> for UnitVec {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.unit_offset -= rhs;
        Self::validate_unit_offset(&self.unit_offset);
    }
}
impl std::ops::Add<UnitVec> for UnitVec {
    type Output = Self;

    fn add(self, rhs: UnitVec) -> Self::Output {
        const MAX_DEPTH_DIFF: u8 = 4;

        fn stack_up(mut cursor: &GridVec) -> Vec<IVec2> {
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
            panic!("Cannot add UnitVec with grid offsets differing in scale by more than {} levels", MAX_DEPTH_DIFF);
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
            }
            std::cmp::Ordering::Less => {
                // rhs is deeper → scale *up* rhs
                let factor = 10.0_f32.powi((rhs.grid_offset.scale as i8 - self.grid_offset.scale as i8) as i32);
                self.unit_offset.truncate() + (rhs.unit_offset.truncate() * factor)
            }
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

        // === Phase 5: Build final GridVec tree ===
        let mut result: Option<GridVec> = None;
        for xy in raw_stack {
            result = Some(match result {
                Some(parent) => GridVec::new(parent, xy),
                None => GridVec::new_root(xy),
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
        const MAX_DEPTH_DIFF: u8 = 4;

        match self.grid_offset.scale.cmp(&rhs.grid_offset.scale) {
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Greater => self.zoom_in_multi(rhs.grid_offset.scale).unwrap(),
            std::cmp::Ordering::Less => rhs.zoom_in_multi(self.grid_offset.scale).unwrap(),
        }

        fn stack_up(mut cursor: &GridVec) -> Vec<IVec2> {
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
            }
            std::cmp::Ordering::Less => {
                // rhs is deeper → scale *up* rhs
                let factor = 10.0_f32.powi((rhs.grid_offset.scale as i8 - self.grid_offset.scale as i8) as i32);
                self.unit_offset.truncate() - (rhs.unit_offset.truncate() * factor)
            }
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

        // === Phase 5: Build final GridVec tree ===
        let mut result: Option<GridVec> = None;
        for xy in raw_stack {
            result = Some(match result {
                Some(parent) => GridVec::new(parent, xy),
                None => GridVec::new_root(xy),
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
impl std::convert::TryFrom<(Vec<IVec2>, Vec2)> for UnitVec {
    type Error = &'static str;

    fn try_from((stack, unit_offset): (Vec<IVec2>, Vec2)) -> Result<Self, Self::Error> {
        let grid_offset = GridVec::try_from(stack)?;
        Ok(UnitVec::new(grid_offset, unit_offset))
    }
}
