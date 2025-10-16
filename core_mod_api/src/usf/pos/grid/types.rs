use bevy::prelude::{IVec2, Vec3};
use std::{marker::PhantomData, sync::Arc};

use crate::usf::scale::Scale;
use crate::utils::logic_safety::{LogicSafety, Checked, Unchecked};
use crate::usf::pos::unit::types::UnitPos;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GridPos<LS: LogicSafety = Checked> {
    pub(in super::super) parent: Option<Arc<GridPos>>,
    pub(in super::super) scale: Scale,
    pub(in super::super) xy: IVec2,
    pub(in super::super) phantom_safety: PhantomData<LS>,
}
impl GridPos {
    fn validate_xy(xy: &IVec2) {
        if xy.x < -5 { panic!("X coordinate {} is too small. Range is (-5..5)", xy.x); }
        if xy.x >= 5 { panic!("X coordinate {} is too large. Range is (-5..5)", xy.x); }
        if xy.y < -5 { panic!("Y coordinate {} is too small. Range is (-5..5)", xy.y); }
        if xy.y >= 5 { panic!("Y coordinate {} is too large. Range is (-5..5)", xy.y); }
    }

    /// Create a GridPos with random (yet valid) coordinates, from the root, down to and including the specified scale, with the same random coords at each scale.
    pub fn new_random_homo(_scale: Scale) -> Self {
        todo!()
    }
    
    /// Create a GridPos with random (yet valid) coordinates, from the root, down to and including the specified scale, with different random coords at each scale.
    pub fn new_random_hetero(_scale: Scale) -> Self {
        todo!()
    }

    /// Create a GridPos at the absolute root (Scale::MAX) with no parent.
    pub fn new_root(xy: IVec2) -> Self {
        Self::validate_xy(&xy);
        Self { parent: None, scale: Scale::MAX, xy, phantom_safety: PhantomData }
    }

    /// Create a GridPos at the absolute root (Scale::MAX) with no parent.
    pub fn new_root_unchecked(xy: IVec2) -> GridPos<Unchecked> {
        GridPos::<Unchecked> { parent: None, scale: Scale::MAX, xy, phantom_safety: PhantomData }
    }

    /// Create a GridPos with the specified parent and xy. The parent can be thought of as a stack onto which we push another level.
    pub fn new(parent: GridPos, xy: IVec2) -> Self {
        Self::validate_xy(&xy);
        if parent.scale == Scale::MIN {
            panic!("Cannot create a child GridPos from a parent at Scale::MIN, as there is no smaller scale.");
        }
        let scale = parent.scale.zoomed_in();
        let parent = Some(Arc::new(parent));

        Self { parent, scale, xy, phantom_safety: PhantomData }
    }

    /// Create a GridPos with the specified parent and xy. The parent can be thought of as a stack onto which we push another level.
    pub fn new_unchecked(parent: GridPos, xy: IVec2) -> GridPos<Unchecked> {
        let scale = parent.scale.zoomed_in();
        let parent = Some(Arc::new(parent));

        GridPos::<Unchecked> { parent, scale, xy, phantom_safety: PhantomData }
    }

    /// Create a GridPos with all ancestors up, from the specified scale to the root at Scale::MAX, pre-filled with IVec2::ZERO, except for the leaf at the specified scale, which is set to the specified xy.
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

        Self { parent: current.parent, scale, xy, phantom_safety: PhantomData }
    }

    /// Create a GridPos with all ancestors, from the specified scale up to the root at Scale::MAX, pre-filled with the specified xy.
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

        Self { parent: current.parent, scale, xy, phantom_safety: PhantomData }
    }

    pub fn zoom_out(&mut self) {
        let mut unit_pos = UnitPos {
            grid_offset: self.clone(),
            unit_offset: Vec3::ZERO,
        };
        unit_pos.zoom_out();
        self.parent = unit_pos.grid_offset.parent;
        self.scale = unit_pos.grid_offset.scale;
        self.xy = unit_pos.grid_offset.xy;
    }
    
    pub fn query_grid_radius(&self, radius: u32) -> Vec<IVec2> {
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

                raw_offsets.push(offset1);
                raw_offsets.push(offset2);
            }
            for dy in -y..=y {
                let offset1 = IVec2::new(dy, x);
                let offset2 = IVec2::new(dy, -x);

                raw_offsets.push(offset1);
                raw_offsets.push(offset2);
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
impl From<GridPos<Unchecked>> for GridPos<Checked> {
    fn from(value: GridPos<Unchecked>) -> Self {
        GridPos::<Checked> { parent: value.parent, scale: value.scale, xy: value.xy, phantom_safety: PhantomData }
    }
}
impl Default for GridPos<Unchecked> {
    fn default() -> Self {
        GridPos::<Unchecked> { parent: None, scale: Scale::default(), xy: IVec2::ZERO, phantom_safety: PhantomData }
    }
}
impl std::fmt::Debug for GridPos {
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
impl std::ops::Add<IVec2> for GridPos {
    type Output = Self;

    fn add(mut self, rhs: IVec2) -> Self::Output {
        self.xy += rhs;
        Self::validate_xy(&self.xy);
        self
    }
}
impl std::ops::AddAssign<IVec2> for GridPos {
    fn add_assign(&mut self, rhs: IVec2) {
        self.xy += rhs;
        Self::validate_xy(&self.xy);
    }
}
impl std::ops::Sub<IVec2> for GridPos {
    type Output = Self;

    fn sub(mut self, rhs: IVec2) -> Self::Output {
        self.xy -= rhs;
        Self::validate_xy(&self.xy);
        self
    }
}
impl std::ops::SubAssign<IVec2> for GridPos {
    fn sub_assign(&mut self, rhs: IVec2) {
        self.xy -= rhs;
        Self::validate_xy(&self.xy);
    }
}
impl std::ops::Add<GridPos> for GridPos {
    type Output = GridPos;

    fn add(self, rhs: GridPos) -> Self::Output {
        // === Phase 1: Collect full stack from root to leaf ===
        fn stack_up(mut cursor: &GridPos) -> Vec<(Scale, IVec2)> {
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

        // === Phase 4: Build final GridPos tree ===
        let mut result: Option<GridPos> = None;
        for (_scale, xy) in raw_stack {
            result = Some(match result {
                Some(parent) => GridPos::new(parent, xy),
                None => GridPos::new_root(xy),
            });
        }

        result.expect("GridPos addition should yield a result")
    }
}
impl std::ops::AddAssign<GridPos> for GridPos {
    fn add_assign(&mut self, rhs: GridPos) {
        *self = self.clone() + rhs;
    }
}
impl std::ops::Sub<GridPos> for GridPos {
    type Output = Self;

    fn sub(self, rhs: GridPos) -> Self::Output {
        // === Phase 1: Collect full stack from root to leaf ===
        fn stack_up(mut cursor: &GridPos) -> Vec<(Scale, IVec2)> {
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

        // === Phase 4: Build final GridPos tree ===
        let mut result: Option<GridPos> = None;
        for (_scale, xy) in raw_stack {
            result = Some(match result {
                Some(parent) => GridPos::new(parent, xy),
                None => GridPos::new_root(xy),
            });
        }

        result.expect("GridPos subtraction should yield a result")
    }
}
impl std::ops::SubAssign<GridPos> for GridPos {
    fn sub_assign(&mut self, rhs: GridPos) {
        *self = self.clone() - rhs;
    }
}
