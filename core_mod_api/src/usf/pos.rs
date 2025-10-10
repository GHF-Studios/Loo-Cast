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
        if xy.x < 0 { panic!("X coordinate {} is too small. Range is (0..10)", xy.x); }
        if xy.x >= 10 { panic!("X coordinate {} is too large. Range is (0..10)", xy.x); }
        if xy.y < 0 { panic!("Y coordinate {} is too small. Range is (0..10)", xy.y); }
        if xy.y >= 10 { panic!("Y coordinate {} is too large. Range is (0..10)", xy.y); }
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

#[derive(Debug, Clone, PartialEq)]
pub struct SubgridPos {
    grid_offset: GridPos,
    subgrid_offset: IVec2,
}
impl SubgridPos {
    fn validate_subgrid_offset(subgrid_offset: &IVec2) {
        if subgrid_offset.x < 0 { panic!("X coordinate {} is too small. Range is (0..10)", subgrid_offset.x); }
        if subgrid_offset.x >= 10 { panic!("X coordinate {} is too large. Range is (0..10)", subgrid_offset.x); }
        if subgrid_offset.y < 0 { panic!("Y coordinate {} is too small. Range is (0..10)", subgrid_offset.y); }
        if subgrid_offset.y >= 10 { panic!("Y coordinate {} is too large. Range is (0..10)", subgrid_offset.y); }
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
    grid_offset: GridPos,
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
