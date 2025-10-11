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

    pub fn new_root(xy: IVec2) -> Self {
        Self::validate_xy(&xy);
        Self { parent: None, scale: Scale::MAX, xy }
    }

    pub fn new(parent: GridPos, xy: IVec2) -> Self {
        Self::validate_xy(&xy);
        let scale = parent.scale.zoomed_in();
        let parent = Some(Arc::new(parent));

        Self { parent, scale, xy }
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
        let mut a_stack = vec![self.clone()];
        let mut b_stack = vec![rhs.clone()];
        
        let mut a_cursor = &self;
        while let Some(p) = &a_cursor.parent {
            a_stack.push((**p).clone());
            a_cursor = p;
        }

        let mut b_cursor = &rhs;
        while let Some(p) = &b_cursor.parent {
            b_stack.push((**p).clone());
            b_cursor = p;
        }

        a_stack.reverse();
        b_stack.reverse();

        let mut result_stack = Vec::new();
        let mut carry = IVec2::ZERO;
        let max_depth = a_stack.len().max(b_stack.len());

        for i in 0..max_depth {
            let a = a_stack.get(i).cloned();
            let b = b_stack.get(i).cloned();

            let scale = a.as_ref().or(b.as_ref()).expect("At least one branch must have a scale").scale;
            let a_xy = a.as_ref().map(|g| g.xy).unwrap_or(IVec2::ZERO);
            let b_xy = b.as_ref().map(|g| g.xy).unwrap_or(IVec2::ZERO);

            let sum = a_xy + b_xy + carry;
            let wrapped_x = ((sum.x + 5).rem_euclid(10)) - 5;
            let wrapped_y = ((sum.y + 5).rem_euclid(10)) - 5;
            let carry_x = (sum.x - wrapped_x).div_euclid(10);
            let carry_y = (sum.y - wrapped_y).div_euclid(10);

            carry = IVec2::new(carry_x, carry_y);
            result_stack.push((scale, IVec2::new(wrapped_x, wrapped_y)));
        }

        // Now rebuild GridPos from root down
        let mut result: Option<GridPos> = None;
        for (scale, xy) in result_stack.into_iter() {
            result = Some(GridPos {
                parent: result.map(|p| Arc::new(p)),
                scale,
                xy,
            });
        }

        // Final carry pass upward
        if carry != IVec2::ZERO {
            let mut current = result.as_mut().unwrap();
            loop {
                current.xy += carry;

                if current.xy.x < -5 || current.xy.x >= 5 || current.xy.y < -5 || current.xy.y >= 5 {
                    let wrapped_x = ((current.xy.x + 5).rem_euclid(10)) - 5;
                    let wrapped_y = ((current.xy.y + 5).rem_euclid(10)) - 5;
                    let carry_x = (current.xy.x - wrapped_x).div_euclid(10);
                    let carry_y = (current.xy.y - wrapped_y).div_euclid(10);

                    current.xy = IVec2::new(wrapped_x, wrapped_y);
                    carry = IVec2::new(carry_x, carry_y);

                    if let Some(parent) = current.parent.as_mut() {
                        current = Arc::make_mut(parent);
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        result.expect("Resulting GridPos should not be None")
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
        todo!()
    }
}
impl std::ops::SubAssign<GridPos> for GridPos {
    fn sub_assign(&mut self, rhs: GridPos) {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SubgridPos {
    grid_offset: GridPos,
    subgrid_offset: IVec2,
}
impl SubgridPos {
    fn validate_subgrid_offset(subgrid_offset: &IVec2) {
        if subgrid_offset.x < -5 { panic!("X coordinate {} is too small. Range is (-5..5)", subgrid_offset.x); }
        if subgrid_offset.x >= 5 { panic!("X coordinate {} is too large. Range is (-5..5)", subgrid_offset.x); }
        if subgrid_offset.y < -5 { panic!("Y coordinate {} is too small. Range is (-5..5)", subgrid_offset.y); }
        if subgrid_offset.y >= 5 { panic!("Y coordinate {} is too large. Range is (-5..5)", subgrid_offset.y); }
    }

    pub fn new(grid_offset: GridPos, subgrid_offset: IVec2) -> Self {
        Self::validate_subgrid_offset(&subgrid_offset);
        Self { grid_offset, subgrid_offset }
    }

    pub fn zoom_out(&mut self) {
        let mut unit_pos = UnitPos {
            grid_offset: self.grid_offset.clone(),
            unit_offset: Vec3::ZERO,
        };
        unit_pos.zoom_out();
        self.grid_offset = unit_pos.grid_offset;
        self.subgrid_offset = IVec2::ZERO;
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
