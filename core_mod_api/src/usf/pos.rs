use bevy::prelude::{IVec2, Vec3};
use std::sync::Arc;

use super::scale::{Scale, DynScale};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GridPos {
    parent: Option<Arc<GridPos>>,
    scale: Scale,
    xy: IVec2,
}
impl GridPos {
    fn validate_xy(xy: &IVec2) {
        if xy.x < -5 { panic!("X coordinate {} is too small. Range is (-5..5)", xy.x); }
        if xy.x >= 5 { panic!("X coordinate {} is too large. Range is (-5..5)", xy.x); }
        if xy.y < -5 { panic!("Y coordinate {} is too small. Range is (-5..5)", xy.y); }
        if xy.y >= 5 { panic!("Y coordinate {} is too large. Range is (-5..5)", xy.y); }
    }

    /// Create a GridPos with random (yet valid) coordinates, from the root, down to and including the specified scale, with the same random coords at each scale.
    pub fn new_random_homo(scale: Scale) -> Self {
        todo!()
    }
    
    /// Create a GridPos with random (yet valid) coordinates, from the root, down to and including the specified scale, with different random coords at each scale.
    pub fn new_random_hetero(scale: Scale) -> Self {
        todo!()
    }

    /// Create a GridPos at the absolute root (Scale::MAX) with no parent.
    pub fn new_root(xy: IVec2) -> Self {
        Self::validate_xy(&xy);
        Self { parent: None, scale: Scale::MAX, xy }
    }

    /// Create a GridPos with the specified parent and xy. The parent can be thought of as a stack onto which we push another level.
    pub fn new(parent: GridPos, xy: IVec2) -> Self {
        Self::validate_xy(&xy);
        if parent.scale == Scale::MIN {
            panic!("Cannot create a child GridPos from a parent at Scale::MIN, as there is no smaller scale.");
        }
        let scale = parent.scale.zoomed_in();
        let parent = Some(Arc::new(parent));

        Self { parent, scale, xy }
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

        Self { parent: current.parent, scale, xy }
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

        Self { parent: current.parent, scale, xy }
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
        for (scale, xy) in raw_stack {
            result = Some(GridPos {
                parent: result.map(Arc::new),
                scale,
                xy,
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
        for (scale, xy) in raw_stack {
            result = Some(GridPos {
                parent: result.map(Arc::new),
                scale,
                xy,
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

#[derive(Debug, Clone, PartialEq)]
pub struct SubgridPos {
    grid_offset: GridPos,
    subgrid_offset: IVec2,
}
impl SubgridPos {
    fn validate_grid_offset(grid_offset: &GridPos) {
        if grid_offset.scale == Scale::MIN {
            panic!("SubgridPos must be based on a scale no smaller than MIN+1, so there is room to represent the subgrid level as a virtual GridPos leaf");
        }
    }

    fn validate_subgrid_offset(subgrid_offset: &IVec2) {
        if subgrid_offset.x < -5 { panic!("X coordinate {} is too small. Range is (-5..5)", subgrid_offset.x); }
        if subgrid_offset.x >= 5 { panic!("X coordinate {} is too large. Range is (-5..5)", subgrid_offset.x); }
        if subgrid_offset.y < -5 { panic!("Y coordinate {} is too small. Range is (-5..5)", subgrid_offset.y); }
        if subgrid_offset.y >= 5 { panic!("Y coordinate {} is too large. Range is (-5..5)", subgrid_offset.y); }
    }

    pub fn new(grid_offset: GridPos, subgrid_offset: IVec2) -> Self {
        Self::validate_grid_offset(&grid_offset);
        Self::validate_subgrid_offset(&subgrid_offset);
        Self { grid_offset, subgrid_offset }
    }

    pub fn zoom_out(&mut self) {
        if self.grid_offset.parent.as_ref().unwrap().parent.is_none() {
            panic!("Cannot zoom out SubgridPos beyond the root GridPos");
        }

        let grid_pos = GridPos {
            parent: Some(Arc::new(self.grid_offset.clone())),
            scale: self.grid_offset.scale.zoomed_in(),
            xy: self.subgrid_offset,
        };

        let mut unit_pos = UnitPos {
            grid_offset: grid_pos,
            unit_offset: Vec3::ZERO,
        };

        unit_pos.zoom_out();

        self.grid_offset = (*unit_pos.grid_offset.parent.unwrap()).clone();
        self.subgrid_offset = unit_pos.grid_offset.xy;
    }
}
impl std::ops::Add<IVec2> for SubgridPos {
    type Output = Self;

    fn add(mut self, rhs: IVec2) -> Self::Output {
        self.subgrid_offset += rhs;
        Self::validate_subgrid_offset(&self.subgrid_offset);
        self
    }
}
impl std::ops::AddAssign<IVec2> for SubgridPos {
    fn add_assign(&mut self, rhs: IVec2) {
        self.subgrid_offset += rhs;
        Self::validate_subgrid_offset(&self.subgrid_offset);
    }
}
impl std::ops::Sub<IVec2> for SubgridPos {
    type Output = Self;

    fn sub(mut self, rhs: IVec2) -> Self::Output {
        self.subgrid_offset -= rhs;
        Self::validate_subgrid_offset(&self.subgrid_offset);
        self
    }
}
impl std::ops::SubAssign<IVec2> for SubgridPos {
    fn sub_assign(&mut self, rhs: IVec2) {
        self.subgrid_offset -= rhs;
        Self::validate_subgrid_offset(&self.subgrid_offset);
    }
}
impl std::ops::Add<SubgridPos> for SubgridPos {
    type Output = Self;

    fn add(self, rhs: SubgridPos) -> Self::Output {
        // === Phase 1: Build extended GridPos stacks from root to leaf ===
        fn build_stack(subgrid: &SubgridPos) -> Vec<(Scale, IVec2)> {
            let mut stack = Vec::new();
            let mut cursor = &subgrid.grid_offset;
            loop {
                stack.push((cursor.scale, cursor.xy));
                if let Some(p) = &cursor.parent {
                    cursor = p;
                } else {
                    break;
                }
            }
            stack.reverse();

            // Append the phantom subgrid level (one scale down)
            let subgrid_scale = subgrid.grid_offset.scale.down().expect("No lower scale for subgrid");
            stack.push((subgrid_scale, subgrid.subgrid_offset));

            stack
        }

        let mut a_stack = build_stack(&self);
        let mut b_stack = build_stack(&rhs);
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

        // === Phase 2: Raw sum top-down ===
        let mut raw_stack = Vec::with_capacity(max_depth);
        for i in 0..max_depth {
            let scale = a_stack[i].0;
            let sum = a_stack[i].1 + b_stack[i].1;
            raw_stack.push((scale, sum));
        }

        // === Phase 3: Normalize with wrapping + carry ===
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

        // === Phase 4: Build GridPos tree and extract SubgridPos ===
        let mut result: Option<GridPos> = None;
        for (scale, xy) in raw_stack {
            result = Some(GridPos {
                parent: result.map(Arc::new),
                scale,
                xy,
            });
        }

        let final_leaf = result.unwrap();
        let subgrid_offset = final_leaf.xy;
        let grid_offset = (*final_leaf.parent.unwrap()).clone();

        SubgridPos { grid_offset, subgrid_offset }
    }
}
impl std::ops::AddAssign<SubgridPos> for SubgridPos {
    fn add_assign(&mut self, rhs: SubgridPos) {
        *self = self.clone() + rhs;
    }
}
impl std::ops::Sub<SubgridPos> for SubgridPos {
    type Output = Self;

    // TODO: Impl
    fn sub(self, rhs: SubgridPos) -> Self::Output {
        // === Phase 1: Build extended GridPos stacks from root to leaf ===
        fn build_stack(subgrid: &SubgridPos) -> Vec<(Scale, IVec2)> {
            let mut stack = Vec::new();
            let mut cursor = &subgrid.grid_offset;
            loop {
                stack.push((cursor.scale, cursor.xy));
                if let Some(p) = &cursor.parent {
                    cursor = p;
                } else {
                    break;
                }
            }
            stack.reverse();

            // Append the phantom subgrid level (one scale down)
            let subgrid_scale = subgrid.grid_offset.scale.down().expect("No lower scale for subgrid");
            stack.push((subgrid_scale, subgrid.subgrid_offset));

            stack
        }

        let mut a_stack = build_stack(&self);
        let mut b_stack = build_stack(&rhs);
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

        // === Phase 2: Raw diff top-down ===
        let mut raw_stack = Vec::with_capacity(max_depth);
        for i in 0..max_depth {
            let scale = a_stack[i].0;
            let diff = a_stack[i].1 - b_stack[i].1;
            raw_stack.push((scale, diff));
        }

        // === Phase 3: Normalize with wrapping + carry ===
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

        // === Phase 4: Build GridPos tree and extract SubgridPos ===
        let mut result: Option<GridPos> = None;
        for (scale, xy) in raw_stack {
            result = Some(GridPos {
                parent: result.map(Arc::new),
                scale,
                xy,
            });
        }

        let final_leaf = result.unwrap();
        let subgrid_offset = final_leaf.xy;
        let grid_offset = (*final_leaf.parent.unwrap()).clone();

        SubgridPos { grid_offset, subgrid_offset }
    }
}
impl std::ops::SubAssign<SubgridPos> for SubgridPos {
    fn sub_assign(&mut self, rhs: SubgridPos) {
        *self = self.clone() - rhs;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnitPos {
    grid_offset: GridPos, // Recursive chunk position
    unit_offset: Vec3, // Bevy units inside the chunk (e.g., [-500.0..500.0])
}
impl UnitPos {
    fn validate_unit_offset(unit_offset: &Vec3) {
        if unit_offset.x < -500.0 { panic!("X = {} is too small. Range is (-500.0..500.0)", unit_offset.x); }
        if unit_offset.x > 500.0 { panic!("X = {} is too large. Range is (-500.0..500.0)", unit_offset.x); }
        if unit_offset.y < -500.0 { panic!("Y = {} is too small. Range is (-500.0..500.0)", unit_offset.y); }
        if unit_offset.y > 500.0 { panic!("Y = {} is too large. Range is (-500.0..500.0)", unit_offset.y); }
    }

    pub fn new(grid_offset: GridPos, unit_offset: Vec3) -> Self {
        Self::validate_unit_offset(&unit_offset);
        Self { grid_offset, unit_offset }
    }

    pub fn zoom_in(&mut self) {
        let parent = self.grid_offset.clone();
        let Some(child_scale) = parent.scale.down() else { return };
        
        let chunk_size = 1000.0;
        let child_factor = 10.0;
        let child_size = chunk_size / child_factor; // = 100.0
        
        // Step 1: Determine which subchunk we're in
        let scaled = self.unit_offset.truncate() / child_size;
        let child_xy = scaled.floor().as_ivec2();
        
        // Step 2: Get the origin of that child chunk in current space
        let child_origin = child_xy.as_vec2() * child_size;
        
        // Step 3: Recompute offset relative to new subchunk center
        let local_offset = self.unit_offset.truncate() - child_origin;
        
        // Step 4: Update context
        self.grid_offset = GridPos {
            parent: Some(Arc::new(parent)),
            scale: child_scale,
            xy: child_xy,
        };
        self.unit_offset = Vec3::new(local_offset.x, local_offset.y, self.unit_offset.z - 10.0);
        Self::validate_unit_offset(&self.unit_offset);
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

        // Step 1: Get child's center in parent chunk space
        let child_origin = child.xy.as_vec2() * child_size;

        // Step 2: Shift up into parent space
        let offset_in_parent = self.unit_offset.truncate() + child_origin;

        // Step 3: Update context
        self.grid_offset = (*parent).clone();
        self.unit_offset = Vec3::new(offset_in_parent.x, offset_in_parent.y, self.unit_offset.z + 10.0);
        Self::validate_unit_offset(&self.unit_offset);
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

    // TODO: Impl properly
    fn add(self, rhs: UnitPos) -> Self::Output {
        const MAX_DEPTH_DIFF: u8 = 4;

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

        if (self.grid_offset.scale as i8 - rhs.grid_offset.scale as i8).abs() > MAX_DEPTH_DIFF as i8 {
            panic!("Cannot add UnitPos with grid offsets differing in scale by more than {} levels", MAX_DEPTH_DIFF);
        }

        let mut a_stack = stack_up(&self.grid_offset);
        let mut b_stack = stack_up(&rhs.grid_offset);

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

        // === Phase 1: Accumulate raw sums top-down ===
        let mut raw_stack = Vec::with_capacity(max_depth);
        for i in 0..max_depth {
            let scale = a_stack[i].0; // should match in both stacks
            let sum = a_stack[i].1 + b_stack[i].1;
            raw_stack.push((scale, sum));
        }

        // === Phase 2: Rescale and add unit offsets ===
        let unit_offset_sum = match self.grid_offset.scale.cmp(&rhs.grid_offset.scale) {
            std::cmp::Ordering::Equal => self.unit_offset + rhs.unit_offset,
            std::cmp::Ordering::Greater => {
                // self is deeper → scale *up* self
                let factor = 10.0_f32.powi((self.grid_offset.scale as i8 - rhs.grid_offset.scale as i8) as i32);
                rhs.unit_offset + (self.unit_offset * factor)
            },
            std::cmp::Ordering::Less => {
                // rhs is deeper → scale *up* rhs
                let factor = 10.0_f32.powi((rhs.grid_offset.scale as i8 - self.grid_offset.scale as i8) as i32);
                self.unit_offset + (rhs.unit_offset * factor)
            },
        };

        // === Phase 3: Extract unit_carry from summed unit_offset ===
        let wrapped_x = ((unit_offset_sum.x + 500.0).rem_euclid(1000.0)) - 500.0;
        let wrapped_y = ((unit_offset_sum.y + 500.0).rem_euclid(1000.0)) - 500.0;
        let carry_x = ((unit_offset_sum.x - wrapped_x) / 1000.0).floor() as i32;
        let carry_y = ((unit_offset_sum.y - wrapped_y) / 1000.0).floor() as i32;

        let unit_offset = Vec3::new(wrapped_x, wrapped_y, unit_offset_sum.z);
        Self::validate_unit_offset(&unit_offset); // Remove if no longer needed
        let mut carry = IVec2::new(carry_x, carry_y);

        // === Phase 4: Normalize bottom-up with wrapping + carry + unit_carry ===
        for i in (0..raw_stack.len()).rev() {
            let (_scale, sum) = raw_stack[i];
            let wrapped_x = ((sum.x + carry.x + 5).rem_euclid(10)) - 5;
            let wrapped_y = ((sum.y + carry.y + 5).rem_euclid(10)) - 5;
            let carry_x = (sum.x + carry.x - wrapped_x).div_euclid(10);
            let carry_y = (sum.y + carry.y - wrapped_y).div_euclid(10);

            raw_stack[i].1 = IVec2::new(wrapped_x, wrapped_y);
            carry = IVec2::new(carry_x, carry_y);
        }

        // === Phase 5: Build final GridPos tree ===
        let mut result: Option<GridPos> = None;
        for (scale, xy) in raw_stack {
            result = Some(GridPos {
                parent: result.map(Arc::new),
                scale,
                xy,
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

    fn sub(self, rhs: UnitPos) -> Self::Output {
        todo!()
    }
}
impl std::ops::SubAssign<UnitPos> for UnitPos {
    fn sub_assign(&mut self, rhs: UnitPos) {
        *self = self.clone() - rhs;
    }
}

#[test]
fn grid_pos_add_test_1() {
    let a = GridPos::new_root(IVec2::new(4, 4));
    let b = GridPos::new_root(IVec2::new(3, 3));
    let c = a + b;
    assert_eq!(c, GridPos::new_root(IVec2::new(-3, -3)));
}

#[test]
fn grid_pos_add_test_2() {
    let scale = Scale::MAX.zoomed_in();
    let a = GridPos::new_at_scale(scale, IVec2::new(4, 4));
    let b = GridPos::new_at_scale(scale, IVec2::new(3, 3));
    let c = a + b;
    let expected = GridPos::new_root(IVec2::new(1, 1));
    let expected = GridPos::new(expected, IVec2::new(-3, -3));
    assert_eq!(c, expected);
}

#[test]
fn grid_pos_add_test_3() {
    let a = GridPos::new_splat(Scale::MIN, IVec2::new(4, 4));
    let b = GridPos::new_at_scale(Scale::MIN, IVec2::new(1, 1));
    let c = a + b;
    let expected = GridPos::new_splat(Scale::MIN, IVec2::new(-5, -5));
    assert_eq!(c, expected);
}

#[test]
fn grid_pos_sub_test_1() {
    let a = GridPos::new_root(IVec2::new(3, 3));
    let b = GridPos::new_root(IVec2::new(4, 4));
    let c = a - b;
    assert_eq!(c, GridPos::new_root(IVec2::new(-1, -1)));
}

#[test]
fn grid_pos_sub_test_2() {
    let scale = Scale::MAX.zoomed_in();
    let a = GridPos::new_at_scale(scale, IVec2::new(-5, -5));
    let b = GridPos::new_at_scale(scale, IVec2::new(3, 3));
    let c = a - b;
    let expected = GridPos::new_root(IVec2::new(-1, -1));
    let expected = GridPos::new(expected, IVec2::new(2, 2));
    assert_eq!(c, expected);
}

#[test]
fn grid_pos_sub_test_3() {
    let a = GridPos::new_splat(Scale::MIN, IVec2::new(-5, -5));
    let b = GridPos::new_at_scale(Scale::MIN, IVec2::new(1, 1));
    let c = a - b;
    let expected = GridPos::new_splat(Scale::MIN, IVec2::new(4, 4));
    assert_eq!(c, expected);
}

#[test]
fn grid_pos_zoom_out_test_1() {
    let a = GridPos::new_root(IVec2::new(0, 0));
    let mut a = GridPos::new(a, IVec2::new(4, 4));
    a.zoom_out();
    let expected = GridPos::new_root(IVec2::new(0, 0));
    assert_eq!(a, expected);
}

#[test]
fn grid_pos_zoom_out_test_2() {
    let a = GridPos::new_root(IVec2::new(0, 0));
    let a = GridPos::new(a, IVec2::new(4, 4));
    let mut a = GridPos::new(a, IVec2::new(3, 3));
    a.zoom_out();
    let expected = GridPos::new_root(IVec2::new(0, 0));
    let expected = GridPos::new(expected, IVec2::new(4, 4));
    assert_eq!(a, expected);
}

#[test]
fn grid_pos_zoom_out_test_3() {
    let a = GridPos::new_root(IVec2::new(0, 0));
    let a = GridPos::new(a, IVec2::new(4, 4));
    let a = GridPos::new(a, IVec2::new(3, 3));
    let mut a = GridPos::new(a, IVec2::new(2, 2));
    a.zoom_out();
    let expected = GridPos::new_root(IVec2::new(0, 0));
    let expected = GridPos::new(expected, IVec2::new(4, 4));
    let expected = GridPos::new(expected, IVec2::new(3, 3));
    assert_eq!(a, expected);
}

#[test]
fn subgrid_pos_add_test_1() {
    let a = GridPos::new_root(IVec2::new(0, 0));
    let a = SubgridPos::new(a, IVec2::new(4, 4));
    let b = GridPos::new_root(IVec2::new(0, 0));
    let b = SubgridPos::new(b, IVec2::new(3, 3));
    let c = a + b;
    let expected_grid = GridPos::new_root(IVec2::new(1, 1));
    let expected = SubgridPos::new(expected_grid, IVec2::new(-3, -3));
    assert_eq!(c, expected);
}

#[test]
fn subgrid_pos_add_test_2() {
    let a = GridPos::new_root(IVec2::new(0, 0));
    let a = GridPos::new(a, IVec2::new(4, 4));
    let a = GridPos::new(a, IVec2::new(3, 3));
    let a = SubgridPos::new(a, IVec2::new(2, 2));
    let b = GridPos::new_root(IVec2::new(0, 0));
    let b = SubgridPos::new(b, IVec2::new(1, 1));
    let c = a + b;
    let expected_grid = GridPos::new_root(IVec2::new(1, 1));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(-5, -5));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(3, 3));
    let expected = SubgridPos::new(expected_grid, IVec2::new(2, 2));
    assert_eq!(c, expected);
}

#[test]
fn subgrid_pos_add_test_3() {
    let a = GridPos::new_root(IVec2::new(0, 0));
    let a = SubgridPos::new(a, IVec2::new(1, 1));
    let b = GridPos::new_root(IVec2::new(0, 0));
    let b = GridPos::new(b, IVec2::new(4, 4));
    let b = GridPos::new(b, IVec2::new(3, 3));
    let b = SubgridPos::new(b, IVec2::new(2, 2));
    let c = a + b;
    let expected_grid = GridPos::new_root(IVec2::new(1, 1));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(-5, -5));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(3, 3));
    let expected = SubgridPos::new(expected_grid, IVec2::new(2, 2));
    assert_eq!(c, expected);
}

#[test]
fn subgrid_pos_sub_test_1() {
    let a = GridPos::new_root(IVec2::new(0, 0));
    let a = SubgridPos::new(a, IVec2::new(-3, -3));
    let b = GridPos::new_root(IVec2::new(0, 0));
    let b = SubgridPos::new(b, IVec2::new(4, 4));
    let c = a - b;
    let expected_grid = GridPos::new_root(IVec2::new(-1, -1));
    let expected = SubgridPos::new(expected_grid, IVec2::new(3, 3));
    assert_eq!(c, expected);
}

#[test]
fn subgrid_pos_sub_test_2() {
    let a = GridPos::new_root(IVec2::new(0, 0));
    let a = GridPos::new(a, IVec2::new(4, 4));
    let a = GridPos::new(a, IVec2::new(3, 3));
    let a = SubgridPos::new(a, IVec2::new(2, 2));
    let b = GridPos::new_root(IVec2::new(0, 0));
    let b = SubgridPos::new(b, IVec2::new(1, 1));
    let c = a - b;
    let expected_grid = GridPos::new_root(IVec2::new(0, 0));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(3, 3));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(3, 3));
    let expected = SubgridPos::new(expected_grid, IVec2::new(2, 2));
    assert_eq!(c, expected);
}

#[test]
fn subgrid_pos_sub_test_3() {
    let a = GridPos::new_root(IVec2::new(0, 0));
    let a = SubgridPos::new(a, IVec2::new(1, 1));
    let b = GridPos::new_root(IVec2::new(0, 0));
    let b = GridPos::new(b, IVec2::new(4, 4));
    let b = GridPos::new(b, IVec2::new(3, 3));
    let b = SubgridPos::new(b, IVec2::new(2, 2));
    let c = a - b;
    let expected_grid = GridPos::new_root(IVec2::new(0, 0));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(-3, -3));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(-3, -3));
    let expected = SubgridPos::new(expected_grid, IVec2::new(-2, -2));
    assert_eq!(c, expected);
}

#[test]
fn subgrid_pos_zoom_out_test_1() {
    let a = GridPos::new_root(IVec2::new(0, 0));
    let a = GridPos::new(a, IVec2::new(4, 4));
    let a = GridPos::new(a, IVec2::new(3, 3));
    let a = SubgridPos::new(a, IVec2::new(2, 2));
    let mut a = a;
    a.zoom_out();
    let expected_grid = GridPos::new_root(IVec2::new(0, 0));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(4, 4));
    let expected = SubgridPos::new(expected_grid, IVec2::new(3, 3));
    assert_eq!(a, expected);
}

#[test]
fn subgrid_pos_zoom_out_test_2() {
    let a = GridPos::new_root(IVec2::new(0, 0));
    let a = GridPos::new(a, IVec2::new(4, 4));
    let a = GridPos::new(a, IVec2::new(3, 3));
    let a = GridPos::new(a, IVec2::new(2, 2));
    let a = SubgridPos::new(a, IVec2::new(1, 1));
    let mut a = a;
    a.zoom_out();
    let expected_grid = GridPos::new_root(IVec2::new(0, 0));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(4, 4));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(3, 3));
    let expected = SubgridPos::new(expected_grid, IVec2::new(2, 2));
    assert_eq!(a, expected);
}

#[test]
fn subgrid_pos_zoom_out_test_3() {
    let a = GridPos::new_root(IVec2::new(0, 0));
    let a = GridPos::new(a, IVec2::new(4, 4));
    let a = GridPos::new(a, IVec2::new(3, 3));
    let a = GridPos::new(a, IVec2::new(2, 2));
    let a = GridPos::new(a, IVec2::new(1, 1));
    let a = SubgridPos::new(a, IVec2::new(0, 0));
    let mut a = a;
    a.zoom_out();
    let expected_grid = GridPos::new_root(IVec2::new(0, 0));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(4, 4));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(3, 3));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(2, 2));
    let expected = SubgridPos::new(expected_grid, IVec2::new(1, 1));
    assert_eq!(a, expected);
}

#[test]
fn unit_pos_add_test_1() {
    let a_grid = GridPos::new_root(IVec2::new(0, 0));
    let a = UnitPos::new(a_grid, Vec3::new(0.0, 0.0, 0.0));
    let b_grid = GridPos::new_root(IVec2::new(0, 0));
    let b_grid = GridPos::new(b_grid, IVec2::new(0, 0));
    let b = UnitPos::new(b_grid, Vec3::new(200.0, 200.0, 0.0));
    let c = a + b;
    let expected_grid = GridPos::new_root(IVec2::new(0, 0));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(0, 0));
    let expected = UnitPos::new(expected_grid, Vec3::new(200.0, 200.0, 0.0));
    assert_eq!(c, expected);
}

#[test]
fn unit_pos_add_test_2() {
    let a_grid = GridPos::new_root(IVec2::new(0, 0));
    let a = UnitPos::new(a_grid, Vec3::new(400.0, 400.0, 0.0));
    let b_grid = GridPos::new_root(IVec2::new(0, 0));
    let b = UnitPos::new(b_grid, Vec3::new(200.0, 200.0, 0.0));
    let c = a + b;
    let expected_grid = GridPos::new_root(IVec2::new(1, 1));
    let expected = UnitPos::new(expected_grid, Vec3::new(-400.0, -400.0, 0.0));
    assert_eq!(c, expected);
}

#[test]
fn unit_pos_add_test_3() {
    let a_grid = GridPos::new_root(IVec2::new(1, 1));
    let a = UnitPos::new(a_grid, Vec3::new(437.0, 437.0, 0.0));
    let b_grid = GridPos::new_root(IVec2::new(1, 1));
    let b_grid = GridPos::new(b_grid, IVec2::new(1, 1));
    let b_grid = GridPos::new(b_grid, IVec2::new(1, 1));
    let b = UnitPos::new(b_grid, Vec3::new(200.0, 200.0, 0.0));
    let c = a + b;
    let expected_grid = GridPos::new_root(IVec2::new(3, 3));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(-4, -4));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(-5, -5));
    let expected = UnitPos::new(expected_grid, Vec3::new(-100.0, -100.0, 0.0));
    assert_eq!(c, expected);
}

#[test]
fn unit_pos_add_test_4() {
    // Three-level deep grid
    let grid = GridPos::new_root(IVec2::new(1, 1));
    let grid = GridPos::new(grid, IVec2::new(1, 1));
    let grid = GridPos::new(grid, IVec2::new(1, 1));

    // Two UnitPos, their Vec3 adds up to cause a wrap on one level
    let a = UnitPos::new(grid.clone(), Vec3::new(499.99, 499.99, 0.0));
    let b = UnitPos::new(grid.clone(), Vec3::new(0.02, 0.02, 0.0));

    let c = a + b;

    // Carry of (1,1) applied to the lowest grid level
    let expected_grid = GridPos::new_root(IVec2::new(2, 2));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(2, 2));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(3, 3));

    let expected = UnitPos::new(expected_grid, Vec3::new(-499.99, -499.99, 0.0));

    assert_eq!(c, expected);
}

/*

// TODO: Impl
#[test]
fn unit_pos_sub_test_1() {
    todo!()
}

// TODO: Impl
#[test]
fn unit_pos_sub_test_2() {
    todo!()
}

// TODO: Impl
#[test]
fn unit_pos_sub_test_3() {
    todo!()
}

// TODO: Impl
#[test]
fn unit_pos_zoom_in_test_1() {
    todo!()
}

// TODO: Impl
#[test]
fn unit_pos_zoom_in_test_2() {
    todo!()
}

// TODO: Impl
#[test]
fn unit_pos_zoom_in_test_3() {
    todo!()
}

// TODO: Impl
#[test]
fn unit_pos_zoom_out_test_1() {
    todo!()
}

// TODO: Impl
#[test]
fn unit_pos_zoom_out_test_2() {
    todo!()
}

// TODO: Impl
#[test]
fn unit_pos_zoom_out_test_3() {
    todo!()
}

*/