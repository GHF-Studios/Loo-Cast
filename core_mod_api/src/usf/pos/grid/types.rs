use bevy::prelude::*;
use std::sync::Arc;

use crate::usf::scale::Scale;
use crate::usf::pos::unit::types::UnitVec;

pub struct GridVecBuilder {
    chain: Vec<IVec2>,
}

impl GridVecBuilder {
    pub fn new() -> Self {
        Self {
            chain: vec![],
        }
    }

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

    pub fn finish(self) -> GridVec {
        GridVec::try_from(self.chain).unwrap()
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Reflect)]
pub struct GridVec {
    pub(in crate) parent: Option<Arc<GridVec>>,
    pub(in crate) scale: Scale,
    pub(in crate) xy: IVec2,
}
impl GridVec {
    pub fn build() -> GridVecBuilder {
        GridVecBuilder::new()
    }

    #[track_caller]
    fn validate_xy(xy: &IVec2) {
        if xy.x < -5 { panic!("X coordinate {} is too small. Range is (-5..5)", xy.x); }
        if xy.x >= 5 { panic!("X coordinate {} is too large. Range is (-5..5)", xy.x); }
        if xy.y < -5 { panic!("Y coordinate {} is too small. Range is (-5..5)", xy.y); }
        if xy.y >= 5 { panic!("Y coordinate {} is too large. Range is (-5..5)", xy.y); }
    }

    #[track_caller]
    fn try_validate_xy(xy: &IVec2) -> Result<(), String> {
        if xy.x < -5 { return Err(format!("X coordinate {} is too small. Range is (-5..5)", xy.x)) }
        if xy.x >= 5 { return Err(format!("X coordinate {} is too large. Range is (-5..5)", xy.x)) }
        if xy.y < -5 { return Err(format!("Y coordinate {} is too small. Range is (-5..5)", xy.y)) }
        if xy.y >= 5 { return Err(format!("Y coordinate {} is too large. Range is (-5..5)", xy.y)) }
        
        Ok(())
    }

    /// Create a GridVec with random (yet valid) coordinates, from the root, down to and including the specified scale, with the same random coords at each scale.
    #[track_caller]
    pub fn new_random_homo(_scale: Scale) -> Self {
        todo!()
    }
    
    /// Create a GridVec with random (yet valid) coordinates, from the root, down to and including the specified scale, with different random coords at each scale.
    #[track_caller]
    pub fn new_random_hetero(_scale: Scale) -> Self {
        todo!()
    }

    /// Create a GridVec at the absolute root (Scale::MAX) with no parent.
    #[track_caller]
    pub fn new_root(xy: IVec2) -> Self {
        Self::validate_xy(&xy);
        let mut my_self = Self { parent: None, scale: Scale::MAX, xy };
        my_self.normalize();
        my_self
    }

    /// Create a GridVec at the absolute root (Scale::MAX) with no parent.
    #[track_caller]
    pub fn new_root_unchecked(xy: IVec2) -> Self {
        let mut my_self = Self { parent: None, scale: Scale::MAX, xy };
        my_self.normalize();
        my_self
    }

    /// Create a GridVec with the specified parent and xy. The parent can be thought of as a stack onto which we push another level.
    #[track_caller]
    pub fn new(parent: GridVec, xy: IVec2) -> Self {
        Self::validate_xy(&xy);
        if parent.scale == Scale::MIN {
            panic!("Cannot create a child GridVec from a parent at Scale::MIN, as there is no smaller scale.");
        }
        let scale = parent.scale.zoomed_in();
        let parent = Some(Arc::new(parent));

        let mut my_self = Self { parent, scale, xy };
        my_self.normalize();
        my_self
    }

    /// Create a GridVec with the specified parent and xy. The parent can be thought of as a stack onto which we push another level.
    #[track_caller]
    pub fn new_unchecked(parent: GridVec, xy: IVec2) -> Self {
        let scale = parent.scale.zoomed_in();
        let parent = Some(Arc::new(parent));

        let mut my_self = Self { parent, scale, xy };
        my_self.normalize();
        my_self
    }

    /// Create a GridVec with all ancestors up, from the specified scale to the root at Scale::MAX, pre-filled with IVec2::ZERO, except for the leaf at the specified scale, which is set to the specified xy.
    #[track_caller]
    pub fn new_at_scale(scale: Scale, xy: IVec2) -> Self {
        Self::validate_xy(&xy);
        if scale == Scale::MAX {
            return Self::new_root(xy);
        }

        let mut current = Self::new_root(IVec2::ZERO);
        let mut current_scale = Scale::MAX;

        while current_scale > scale {
            current_scale = current_scale.zoomed_in();
            current = Self::new(current, IVec2::ZERO);
        }

        let mut my_self = Self { parent: current.parent, scale, xy };
        my_self.normalize();
        my_self
    }

    /// Create a GridVec with all ancestors, from the specified scale up to the root at Scale::MAX, pre-filled with the specified xy.
    #[track_caller]
    pub fn new_splat(scale: Scale, xy: IVec2) -> Self {
        Self::validate_xy(&xy);
        if scale == Scale::MAX {
            return Self::new_root(xy);
        }

        let mut current = Self::new_root(xy);
        let mut current_scale = Scale::MAX;

        while current_scale > scale {
            current_scale = current_scale.zoomed_in();
            current = Self::new(current, xy);
        }

        let mut my_self = Self { parent: current.parent, scale, xy };
        my_self.normalize();
        my_self
    }

    #[track_caller]
    pub fn is_zero(&self) -> bool {
        let mut cursor = self;

        while cursor.scale < Scale::MAX {
            if cursor.xy != IVec2::ZERO {
                return false;
            }

            cursor = cursor.parent.as_ref().unwrap().as_ref();
        }

        if cursor.xy != IVec2::ZERO {
            return false;
        }

        true
    }

    /// - Assumes that the given `scale` is greater than or equal to that of `self`.
    /// - Assumes that the parent of `grid_diff` is the same as `self`'s parent.
    #[track_caller]
    pub fn from_native_logical(origin: Self, (native_logical_pos, scale): (Vec2, Scale)) -> Self {
        assert!(scale >= origin.scale);
        let unit_offset_x = native_logical_pos.x / 1000.0;
        let unit_offset_y = native_logical_pos.y / 1000.0;

        let diff_x = unit_offset_x.round() as i32;
        let diff_y = unit_offset_y.round() as i32;

        let diff = GridVec {
            parent: origin.parent.clone(),
            scale,
            xy: IVec2::new(diff_x, diff_y),
        };

        origin + diff
    }

    /// - Assumes that the given `scale` is greater than or equal to that of `self`.
    /// - Assumes that the parent of `grid_diff` is the same as `self`'s parent.
    #[track_caller]
    pub fn from_native_visual(origin: Self, native_visual_pos: Vec2, scale: Scale) -> Self {
        assert!(scale >= origin.scale);
        let scale_diff = scale as i8 - origin.scale as i8;
        let scale_factor = 10.0_f32.powi(scale_diff as i32);
        let native_unit = 1000.0 / scale_factor;

        let unit_offset_x = native_visual_pos.x / native_unit;
        let unit_offset_y = native_visual_pos.y / native_unit;

        let diff_x = unit_offset_x.round() as i32;
        let diff_y = unit_offset_y.round() as i32;

        let diff = GridVec {
            parent: origin.parent.clone(),
            scale,
            xy: IVec2::new(diff_x, diff_y),
        };

        origin + diff
    }

    /// - Assumes that `self`'s scale is greater than or equal to that of `origin`.
    /// - Assumes that the parent of `self` is the same as `origin`'s parent.
    #[track_caller]
    pub fn to_native_logical(self, origin: Self) -> Vec2 {
        assert!(self.scale >= origin.scale);
        let _scale_diff = self.scale as i8 - origin.scale as i8;
        let self_unit = UnitVec {
            grid_offset: self.clone(),
            unit_offset: Vec3::ZERO,
        };
        let origin_unit = UnitVec {
            grid_offset: origin.clone(),
            unit_offset: Vec3::ZERO,
        };
        let diff_unit = self_unit - origin_unit;
        assert!(diff_unit.grid_offset.parent.as_ref().map(|p| p.is_zero()).unwrap_or(true));
        
        let native_x = diff_unit.grid_offset.xy.x as f32 * 1000.0;
        let native_y = diff_unit.grid_offset.xy.y as f32 * 1000.0;

        Vec2::new(native_x, native_y)
    }

    /// - Assumes that `self`'s scale is greater than or equal to that of `origin`.
    /// - Assumes that the parent of `self` is the same as `origin`'s parent.
    #[track_caller]
    pub fn to_native_visual(self, origin: Self) -> (Vec2, f32) {
        assert!(self.scale >= origin.scale);
        let scale_diff = self.scale as i8 - origin.scale as i8;
        let self_unit = UnitVec {
            grid_offset: self.clone(),
            unit_offset: Vec3::ZERO,
        };
        let origin_unit = UnitVec {
            grid_offset: origin.clone(),
            unit_offset: Vec3::ZERO,
        };
        let diff_unit = self_unit - origin_unit;
        assert!(diff_unit.grid_offset.parent.as_ref().map(|p| p.is_zero()).unwrap_or(true));

        let scale = 10.0_f32.powi(scale_diff as i32);
        let native_unit = 1000.0 / scale;

        let native_x = diff_unit.grid_offset.xy.x as f32 * native_unit;
        let native_y = diff_unit.grid_offset.xy.y as f32 * native_unit;

        (Vec2::new(native_x, native_y), scale)
    }

    // TODO: REFACTOR: PERF: This is much less performant than it could be;
    //      it just abuses the fact that Add(and Sub) internally perform a wrap, by just doing +/- "zero", or rather a default that equates in a no-op.
    //      In short: It's a fast, dirty, and lazy solution
    /// Recursively normalize the GridVec to ensure all coordinates are within valid ranges.
    #[track_caller]
    pub fn normalize(&mut self) {
        let zero = GridVec::default();
        let normalized = self.clone() + zero;
        self.parent = normalized.parent;
        self.scale = normalized.scale;
        self.xy = normalized.xy;
    }

    #[track_caller]
    pub fn zoom_out(&mut self) {
        let mut unit_extent = UnitVec {
            grid_offset: self.clone(),
            unit_offset: Vec3::ZERO,
        };
        unit_extent.zoom_out();
        self.parent = unit_extent.grid_offset.parent;
        self.scale = unit_extent.grid_offset.scale;
        self.xy = unit_extent.grid_offset.xy;
    }
    
    #[track_caller]
    pub fn query_grid_radius(&self, radius: u32) -> Vec<GridVec> {
        let mut raw_offsets = Vec::new();

        let radius = radius as i32;

        let mut x = 0;
        let mut y = radius;
        let mut d = 1 - radius; // Decision parameter

        while x <= y {
            // Add filled lines between symmetrical points
            for dx in -x..=x {
                let offset1 = IVec2::new(dx, y);
                let offset2 = IVec2::new(dx, -y);

                raw_offsets.push(GridVec {
                    parent: self.parent.clone(),
                    scale: self.scale,
                    xy: offset1
                });
                raw_offsets.push(GridVec {
                    parent: self.parent.clone(),
                    scale: self.scale,
                    xy: offset2
                });
            }
            for dy in -y..=y {
                let offset1 = IVec2::new(dy, x);
                let offset2 = IVec2::new(dy, -x);

                raw_offsets.push(GridVec {
                    parent: self.parent.clone(),
                    scale: self.scale,
                    xy: offset1
                });
                raw_offsets.push(GridVec {
                    parent: self.parent.clone(),
                    scale: self.scale,
                    xy: offset2
                });
            }

            if d < 0 {
                // Midpoint is inside the circle
                d += 2 * x + 3;
            } else {
                // Midpoint is outside the circle
                d += 2 * (x - y) + 5;
                y -= 1;
            }
            x += 1;
        }

        raw_offsets
    }
}
impl Default for GridVec {
    fn default() -> Self {
        GridVec { parent: None, scale: Scale::MAX, xy: IVec2::ZERO }
    }
}
impl std::fmt::Debug for GridVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = String::new();
        let mut cursor = self;
        loop {
            let suffix = if msg.is_empty() { String::new() } else { format!(", {}", msg) };
            msg = format!("({}, {}){}", cursor.xy.x, cursor.xy.y, suffix);
            if let Some(p) = &cursor.parent {
                cursor = p;
            } else {
                break;
            }
        }

        write!(f, "[{}] @ scale {}", msg, self.scale as i8)
    }
}
impl std::ops::Add<IVec2> for GridVec {
    type Output = Self;

    #[track_caller]
    fn add(mut self, rhs: IVec2) -> Self::Output {
        self.xy += rhs;
        if Self::try_validate_xy(&self.xy).is_err() {
            self.normalize();
        }
        self
    }
}
impl std::ops::AddAssign<IVec2> for GridVec {
    #[track_caller]
    fn add_assign(&mut self, rhs: IVec2) {
        self.xy += rhs;
        if Self::try_validate_xy(&self.xy).is_err() {
            self.normalize();
        }
    }
}
impl std::ops::Sub<IVec2> for GridVec {
    type Output = Self;

    #[track_caller]
    fn sub(mut self, rhs: IVec2) -> Self::Output {
        self.xy -= rhs;
        if Self::try_validate_xy(&self.xy).is_err() {
            self.normalize();
        }
        self
    }
}
impl std::ops::SubAssign<IVec2> for GridVec {
    #[track_caller]
    fn sub_assign(&mut self, rhs: IVec2) {
        self.xy -= rhs;
        if Self::try_validate_xy(&self.xy).is_err() {
            self.normalize();
        }
    }
}
impl std::ops::Add<GridVec> for GridVec {
    type Output = GridVec;

    #[track_caller]
    fn add(self, rhs: GridVec) -> Self::Output {
        // === Phase 1: Collect full stack from root to leaf ===
        fn stack_up(mut cursor: &GridVec) -> Vec<(Scale, IVec2)> {
            let mut stack = Vec::new();
            loop {
                stack.push((cursor.scale, cursor.xy));
                if let Some(p) = &cursor.parent {
                    cursor = p;
                } else {
                    break;
                }
            }
            stack.reverse();
            stack
        }

        let mut a_stack = stack_up(&self);
        let mut b_stack = stack_up(&rhs);

        let max_depth = a_stack.len().max(b_stack.len());

        // Pad shorter stack with (scale, ZERO)
        while a_stack.len() < max_depth {
            let (s, _) = b_stack[a_stack.len()];
            a_stack.push((s, IVec2::ZERO));
        }
        while b_stack.len() < max_depth {
            let (s, _) = a_stack[b_stack.len()];
            b_stack.push((s, IVec2::ZERO));
        }

        // === Phase 2: Accumulate raw sums top-down ===
        let mut raw_stack = Vec::with_capacity(max_depth);
        for i in 0..max_depth {
            let scale = a_stack[i].0; // should match in both stacks
            let sum = a_stack[i].1 + b_stack[i].1;
            raw_stack.push((scale, sum));
        }

        // === Phase 3: Normalize bottom-up with wrapping + carry ===
        let mut carry = IVec2::ZERO;
        for i in (0..raw_stack.len()).rev() {
            let (_scale, sum) = raw_stack[i];
            let wrapped_x = ((sum.x + carry.x + 5).rem_euclid(10)) - 5;
            let wrapped_y = ((sum.y + carry.y + 5).rem_euclid(10)) - 5;
            let carry_x = (sum.x + carry.x - wrapped_x).div_euclid(10);
            let carry_y = (sum.y + carry.y - wrapped_y).div_euclid(10);

            raw_stack[i].1 = IVec2::new(wrapped_x, wrapped_y);
            carry = IVec2::new(carry_x, carry_y);
        }

        // === Phase 4: Build final GridVec tree ===
        let mut result: Option<GridVec> = None;
        for (scale, xy) in raw_stack {
            result = Some(GridVec { parent: result.map(Arc::new), scale, xy })
        }

        result.expect("GridVec addition should yield a result")
    }
}
impl std::ops::AddAssign<GridVec> for GridVec {
    #[track_caller]
    fn add_assign(&mut self, rhs: GridVec) {
        *self = self.clone() + rhs;
    }
}
impl std::ops::Sub<GridVec> for GridVec {
    type Output = Self;

    #[track_caller]
    fn sub(self, rhs: GridVec) -> Self::Output {
        // === Phase 1: Collect full stack from root to leaf ===
        fn stack_up(mut cursor: &GridVec) -> Vec<(Scale, IVec2)> {
            let mut stack = Vec::new();
            loop {
                stack.push((cursor.scale, cursor.xy));
                if let Some(p) = &cursor.parent {
                    cursor = p;
                } else {
                    break;
                }
            }
            stack.reverse();
            stack
        }

        let mut a_stack = stack_up(&self);
        let mut b_stack = stack_up(&rhs);

        let max_depth = a_stack.len().max(b_stack.len());

        // Pad shorter stack with (scale, ZERO)
        while a_stack.len() < max_depth {
            let (s, _) = b_stack[a_stack.len()];
            a_stack.push((s, IVec2::ZERO));
        }
        while b_stack.len() < max_depth {
            let (s, _) = a_stack[b_stack.len()];
            b_stack.push((s, IVec2::ZERO));
        }

        // === Phase 2: Accumulate raw diffs top-down ===
        let mut raw_stack = Vec::with_capacity(max_depth);
        for i in 0..max_depth {
            let scale = a_stack[i].0; // should match in both stacks
            let diff = a_stack[i].1 - b_stack[i].1;
            raw_stack.push((scale, diff));
        }

        // === Phase 3: Normalize bottom-up with wrapping + carry ===
        let mut carry = IVec2::ZERO;
        for i in (0..raw_stack.len()).rev() {
            let (_scale, diff) = raw_stack[i];
            let wrapped_x = ((diff.x + carry.x + 5).rem_euclid(10)) - 5;
            let wrapped_y = ((diff.y + carry.y + 5).rem_euclid(10)) - 5;
            let carry_x = (diff.x + carry.x - wrapped_x).div_euclid(10);
            let carry_y = (diff.y + carry.y - wrapped_y).div_euclid(10);

            raw_stack[i].1 = IVec2::new(wrapped_x, wrapped_y);
            carry = IVec2::new(carry_x, carry_y);
        }

        // === Phase 4: Build final GridVec tree ===
        let mut result: Option<GridVec> = None;
        for (scale, xy) in raw_stack {
            result = Some(GridVec { parent: result.map(Arc::new), scale, xy })
        }

        result.expect("GridVec subtraction should yield a result")
    }
}
impl std::ops::SubAssign<GridVec> for GridVec {
    #[track_caller]
    fn sub_assign(&mut self, rhs: GridVec) {
        *self = self.clone() - rhs;
    }
}
impl std::convert::TryFrom<Vec<IVec2>> for GridVec {
    type Error = &'static str;

    fn try_from(stack: Vec<IVec2>) -> Result<Self, Self::Error> {
        if stack.is_empty() {
            return Err("GridVec stack must contain at least one element");
        }

        let mut iter = stack.into_iter();
        let mut current = GridVec::new_root(iter.next().unwrap());

        for xy in iter {
            current = GridVec::new(current, xy);
        }

        Ok(current)
    }
}