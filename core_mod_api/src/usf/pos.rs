use bevy::prelude::{IVec2, Vec3};
use std::sync::Arc;

use super::scale::{Scale, DynScale};

const GRID_SIZE: i32 = 1000;
const HALF_GRID_SIZE: i32 = 500;
const HALF_GRID_SIZE_F32: f32 = 500.0;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GridPos {
    parent: Option<Arc<GridPos>>,
    scale: Scale,
    xy: IVec2,
}
impl GridPos {
    pub fn new_root(xy: IVec2) -> Self {
        if xy.x < -HALF_GRID_SIZE { panic!("X coordinate {} is too small. Range is -{} to {}", xy.x, HALF_GRID_SIZE, HALF_GRID_SIZE); }
        if xy.x > HALF_GRID_SIZE { panic!("X coordinate {} is too large. Range is -{} to {}", xy.x, HALF_GRID_SIZE, HALF_GRID_SIZE); }
        if xy.y < -HALF_GRID_SIZE { panic!("Y coordinate {} is too small. Range is -{} to {}", xy.y, HALF_GRID_SIZE, HALF_GRID_SIZE); }
        if xy.y > HALF_GRID_SIZE { panic!("Y coordinate {} is too large. Range is -{} to {}", xy.y, HALF_GRID_SIZE, HALF_GRID_SIZE); }

        Self { parent: None, scale: Scale::MAX, xy }
    }

    pub fn new(parent: GridPos, xy: IVec2) -> Self {
        if xy.x < -HALF_GRID_SIZE { panic!("X coordinate {} is too small. Range is -{} to {}", xy.x, HALF_GRID_SIZE, HALF_GRID_SIZE); }
        if xy.x > HALF_GRID_SIZE { panic!("X coordinate {} is too large. Range is -{} to {}", xy.x, HALF_GRID_SIZE, HALF_GRID_SIZE); }
        if xy.y < -HALF_GRID_SIZE { panic!("Y coordinate {} is too small. Range is -{} to {}", xy.y, HALF_GRID_SIZE, HALF_GRID_SIZE); }
        if xy.y > HALF_GRID_SIZE { panic!("Y coordinate {} is too large. Range is -{} to {}", xy.y, HALF_GRID_SIZE, HALF_GRID_SIZE); }

        let scale = parent.scale.zoomed_in();
        let parent = Some(Arc::new(parent));

        Self { parent, scale, xy }
    }

    pub fn zoom_in(&mut self) {
        todo!()
    }

    pub fn zoom_out(&mut self) {
        todo!()
    }
    
    pub fn query_radius(&self, radius: u32) -> Vec<IVec2> {
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

    /// Convert this GridPos to a SubgridPos with zero subgrid offset
    pub fn to_subgrid(&self) -> SubgridPos {
        SubgridPos { grid_offset: self.clone(), subgrid_offset: Vec3::ZERO }
    }
}
impl std::ops::Add<IVec2> for GridPos {
    type Output = Self;

    fn add(mut self, rhs: IVec2) -> Self::Output {
        self.xy += rhs;
        self
    }
}
impl std::ops::AddAssign<IVec2> for GridPos {
    fn add_assign(&mut self, rhs: IVec2) {
        self.xy += rhs;
    }
}
impl std::ops::Sub<IVec2> for GridPos {
    type Output = Self;

    fn sub(mut self, rhs: IVec2) -> Self::Output {
        self.xy -= rhs;
        self
    }
}
impl std::ops::SubAssign<IVec2> for GridPos {
    fn sub_assign(&mut self, rhs: IVec2) {
        self.xy -= rhs;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SubgridPos {
    grid_offset: GridPos,
    subgrid_offset: Vec3,
}
impl SubgridPos {
    pub fn new(grid_offset: GridPos, subgrid_offset: Vec3) -> Self {
        if subgrid_offset.x < -HALF_GRID_SIZE_F32 { panic!("X coordinate {} is too small. Range is -{} to {}", subgrid_offset.x, HALF_GRID_SIZE_F32, HALF_GRID_SIZE_F32); }
        if subgrid_offset.x > HALF_GRID_SIZE_F32 { panic!("X coordinate {} is too large. Range is -{} to {}", subgrid_offset.x, HALF_GRID_SIZE_F32, HALF_GRID_SIZE_F32); }
        if subgrid_offset.y < -HALF_GRID_SIZE_F32 { panic!("Y coordinate {} is too small. Range is -{} to {}", subgrid_offset.y, HALF_GRID_SIZE_F32, HALF_GRID_SIZE_F32); }
        if subgrid_offset.y > HALF_GRID_SIZE_F32 { panic!("Y coordinate {} is too large. Range is -{} to {}", subgrid_offset.y, HALF_GRID_SIZE_F32, HALF_GRID_SIZE_F32); }

        Self { grid_offset, subgrid_offset }
    }

    pub fn zoom_in(&mut self) {
        todo!()
    }

    pub fn zoom_out(&mut self) {
        todo!()
    }
}
impl std::ops::Add<Vec3> for SubgridPos {
    type Output = Self;

    fn add(mut self, rhs: Vec3) -> Self::Output {
        self.subgrid_offset += rhs;
        self
    }
}
impl std::ops::AddAssign<Vec3> for SubgridPos {
    fn add_assign(&mut self, rhs: Vec3) {
        self.subgrid_offset += rhs;
    }
}
impl std::ops::Sub<Vec3> for SubgridPos {
    type Output = Self;

    fn sub(mut self, rhs: Vec3) -> Self::Output {
        self.subgrid_offset -= rhs;
        self
    }
}
impl std::ops::SubAssign<Vec3> for SubgridPos {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.subgrid_offset -= rhs;
    }
}