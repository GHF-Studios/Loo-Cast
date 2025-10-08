use bevy::prelude::{Vec2, Vec3};

use crate::utils::types::I128Vec2;

use super::scale::{Scale, DynScale};

const GRID_SIZE: i128 = 1000_i128;

/// A position, in the grid at a specific scale, relative to a parent GridOffset (or None if at MAX scale)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GridOffset {
    parent: Option<Box<GridOffset>>,
    scale: Scale,
    xy: I128Vec2,
}
impl GridOffset {
    /// Create a new GridOffset at the origin (0,0) of the given scale
    /// # Arguments
    /// * `parent` - The parent GridOffset, or None if this is the root (MAX scale)
    /// * `scale` - The scale of this GridOffset
    /// # Panics
    /// * If the parent scale is not greater than the child scale
    /// * If the scale is not MAX and parent is None
    /// # Returns
    /// A new GridOffset at the given `scalescale_origin`'s true `grid_origin` at (0,0)
    pub fn new_origin(parent: Option<Box<GridOffset>>, scale: Scale) -> Self {
        let parent = match parent {
            Some(parent) => {
                assert!(parent.scale > scale, "The parent's scale must be greater than child's scale");
                Some(parent)
            },
            None => {
                assert!(scale == Scale::MAX, "Root GridPos must have MAX scale");
                None
            }
        };

        Self { parent, scale, xy: I128Vec2::ZERO }
    }

    /// Create a new GridOffset at the given coordinates of the given scale
    /// # Arguments
    /// * `parent` - The parent GridOffset, or None if this is the root (MAX scale)
    /// * `scale` - The scale of this GridOffset
    /// * `xy` - The coordinates of this GridOffset
    /// # Panics
    /// * If the parent scale is not greater than the child scale
    /// * If the scale is not MAX and parent is None
    /// * If the coordinates are out of bounds for the given scale
    /// # Returns
    /// A new GridOffset at the given `xy`, from the given `scale`'s true `grid_origin` at (0,0)
    pub fn new(parent: Option<Box<GridOffset>>, scale: Scale, xy: I128Vec2) -> Self {
        let parent = match parent {
            Some(parent) => {
                assert!(parent.scale > scale, "Parent scale must be greater than child scale");
                Some(parent)
            },
            None => {
                assert!(scale == Scale::MAX, "Root GridPos must have MAX scale");
                None
            }
        };
        let scale_extent = GRID_SIZE * 10_i128.pow(scale.index_from_top() as u32);
        let half_scale_extent = scale_extent / 2;

        if xy.x < -half_scale_extent { panic!("X coordinate {} is too small for scale {:?}. Range is -{} to {}", xy.x, scale, half_scale_extent, half_scale_extent); }
        if xy.x > half_scale_extent { panic!("X coordinate {} is too large for scale {:?}. Range is -{} to {}", xy.x, scale, half_scale_extent, half_scale_extent); }
        if xy.y < -half_scale_extent { panic!("Y coordinate {} is too small for scale {:?}. Range is -{} to {}", xy.y, scale, half_scale_extent, half_scale_extent); }
        if xy.y > half_scale_extent { panic!("Y coordinate {} is too large for scale {:?}. Range is -{} to {}", xy.y, scale, half_scale_extent, half_scale_extent); }

        Self { parent, scale, xy }
    }
}
impl std::ops::Add<I128Vec2> for GridOffset {
    type Output = Self;

    fn add(mut self, rhs: I128Vec2) -> Self::Output {
        self.xy += rhs;
        self
    }
}
impl std::ops::AddAssign<I128Vec2> for GridOffset {
    fn add_assign(&mut self, rhs: I128Vec2) {
        self.xy += rhs;
    }
}
impl std::ops::Sub<I128Vec2> for GridOffset {
    type Output = Self;

    fn sub(mut self, rhs: I128Vec2) -> Self::Output {
        self.xy -= rhs;
        self
    }
}
impl std::ops::SubAssign<I128Vec2> for GridOffset {
    fn sub_assign(&mut self, rhs: I128Vec2) {
        self.xy -= rhs;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GridPos {
    scale: Scale,
    grid_offset: GridOffset,
}
impl GridPos {
    /// Create a new GridPos from a scale origin and a grid offset
    /// # Arguments
    /// * `scale` - The currently active scale that this position is relative to
    /// * `grid_offset` - The grid offset from the `scale`'s true `grid_origin` at (0,0)
    /// # Returns
    /// A new GridPos, at the given `grid_offset`, from the given `scale`'s true `grid_origin` at (0,0)
    pub fn new_origin(scale: Scale, grid_offset: GridOffset) -> Self {
        Self { scale, grid_offset }
    }

    /// Create a new GridPos from a scale origin, a grid offset, and a subgrid offset within that grid
    /// # Arguments
    /// * `scale` - The currently active scale that this position is relative to
    /// * `grid_offset` - The grid offset from the `scale`'s true `grid_origin` at (0,0)
    /// * `subgrid_offset` - The subgrid offset within the grid chunk described by `grid_offset` (in world units)
    /// # Panics
    /// * If the difference in scale factor exponent between the `scale` and the `grid_offset`'s `scale` is greater than +/- `Scale::MAX_DIFF_SCALE_EXP`
    /// # Returns
    /// A new GridPos, at the given `subgrid_offset`, from the given `grid_offset`, from the given `scale`'s true `grid_origin` at (0,0)
    pub fn new(scale: Scale, grid_offset: GridOffset, subgrid_offset: Vec2) -> Self {
        let diff_scale_factor = grid_offset.scale.difference_scale_factor(&scale);
        let grid_size = diff_scale_factor * GRID_SIZE as f64;
        let half_grid_size = grid_size / 2.0;
        let grid_offset_x = ((subgrid_offset.x as f64 + half_grid_size) / grid_size).floor() as i128;
        let grid_offset_y = ((subgrid_offset.y as f64 + half_grid_size) / grid_size).floor() as i128;
        let grid_offset_xy = I128Vec2::new(grid_offset_x, grid_offset_y);
        let grid_offset = GridOffset::new(grid_offset.parent, grid_offset.scale, grid_offset_xy);

        Self { scale, grid_offset }
    }
    
    /// Get all grid positions in a radius around this position
    /// # Returns
    /// A vector of I128Vec2 raw `grid_offset.xy`s from this position
    pub fn query_radius(&self, radius: u32) -> Vec<I128Vec2> {
        let mut chunks = Vec::new();

        let radius = radius as i128;

        let mut x = 0;
        let mut y = radius;
        let mut d = 1 - radius; // Decision parameter

        while x <= y {
            // Add filled lines between symmetrical points
            for dx in -x..=x {
                let offset1 = I128Vec2::new(dx, y);
                let offset2 = I128Vec2::new(dx, -y);

                chunks.push(offset1);
                chunks.push(offset2);
            }
            for dy in -y..=y {
                let offset1 = I128Vec2::new(dy, x);
                let offset2 = I128Vec2::new(dy, -x);

                chunks.push(offset1);
                chunks.push(offset2);
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

        chunks
    }

    /// Convert this GridPos to a SubgridPos with zero subgrid offset
    pub fn to_subgrid(&self) -> SubgridPos {
        SubgridPos { grid_origin: self.clone(), subgrid_offset: Vec3::ZERO }
    }
}
impl std::ops::Add<I128Vec2> for GridPos {
    type Output = Self;

    fn add(mut self, rhs: I128Vec2) -> Self::Output {
        self.grid_offset.xy += rhs;
        self
    }
}
impl std::ops::AddAssign<I128Vec2> for GridPos {
    fn add_assign(&mut self, rhs: I128Vec2) {
        self.grid_offset.xy += rhs;
    }
}
impl std::ops::Sub<I128Vec2> for GridPos {
    type Output = Self;

    fn sub(mut self, rhs: I128Vec2) -> Self::Output {
        self.grid_offset.xy -= rhs;
        self
    }
}
impl std::ops::SubAssign<I128Vec2> for GridPos {
    fn sub_assign(&mut self, rhs: I128Vec2) {
        self.grid_offset.xy -= rhs;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SubgridPos {
    grid_origin: GridPos,
    subgrid_offset: Vec3,
}
impl SubgridPos {
    /// Create a new SubgridPos from a grid offset
    /// # Returns
    /// A new SubgridPos, at the `subgrid_origin` (0,0,0) of the given `grid_offset`
    pub fn new_origin(grid_offset: GridOffset) -> Self {
        let grid_origin = GridPos::new(grid_offset.scale, grid_offset, Vec2::ZERO);

        Self { grid_origin, subgrid_offset: Vec3::ZERO }
    }

    /// Create a new SubgridPos from a grid offset, and a subgrid offset
    /// # Returns
    /// A new SubgridPos, at the given `subgrid_offset` from the given `grid_offset`
    pub fn new(grid_offset: GridOffset, subgrid_offset: Vec3) -> Self {
        let grid_origin = GridPos::new(grid_offset.scale, grid_offset, subgrid_offset.truncate());

        Self { grid_origin, subgrid_offset }
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