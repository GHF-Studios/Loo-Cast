use bevy::prelude::{Vec2, Vec3};

use crate::utils::types::I128Vec2;

use super::scale::{Scale, DynScale};

const GRID_SIZE: i128 = 1000_i128;
const MAX_SCALE_DISTANCE: i8 = 8;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GridOffset {
    parent: Option<Box<GridOffset>>,
    scale: Scale,
    xy: I128Vec2,
}
impl GridOffset {
    pub fn new_origin(parent: Option<Box<GridOffset>>, scale: Scale) -> Self {
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

        Self { parent, scale, xy: I128Vec2::ZERO }
    }

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GridPos {
    scale_origin: Scale,
    grid_offset: GridOffset,
}
impl GridPos {
    pub fn new_origin(scale_origin: Scale, grid_offset: GridOffset) -> Self {
        Self { scale_origin, grid_offset }
    }

    pub fn new(scale_origin: Scale, grid_offset: GridOffset, subgrid_offset: Vec2) -> Self {
        let delta_scale_factor_exponent = (grid_offset.scale.scale_factor_exponent() - scale_origin.scale_factor_exponent()) - Scale::MAX.scale_factor_exponent();

        assert!(
            delta_scale_factor_exponent <= MAX_SCALE_DISTANCE,
            "Scale difference '{}' between scale_origin and grid_offset is larger than MAX '{}'",
            delta_scale_factor_exponent, MAX_SCALE_DISTANCE
        );

        let delta_scale = Scale::from_scale_factor_exponent(delta_scale_factor_exponent).unwrap();
        let grid_size = delta_scale.scale_factor() * GRID_SIZE as f64;
        let half_grid_size = grid_size / 2.0;
        let grid_offset_x = ((subgrid_offset.x as f64 + half_grid_size) / grid_size).floor() as i128;
        let grid_offset_y = ((subgrid_offset.y as f64 + half_grid_size) / grid_size).floor() as i128;
        let grid_offset_xy = I128Vec2::new(grid_offset_x, grid_offset_y);
        let grid_offset = GridOffset::new(grid_offset.parent, grid_offset.scale, grid_offset_xy);

        Self { scale_origin, grid_offset }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SubgridPos {
    grid_origin: GridPos,
    subgrid_offset: Vec3,
}
impl SubgridPos {
    pub fn new_origin(scale_origin: Scale, grid_offset: GridOffset) -> Self {
        let grid_origin = GridPos::new(scale_origin, grid_offset, Vec2::ZERO);

        Self { grid_origin, subgrid_offset }
    }

    pub fn new(scale_origin: Scale, grid_offset: GridOffset, subgrid_offset: Vec3) -> Self {
        let grid_origin = GridPos::new(scale_origin, grid_offset, subgrid_offset.truncate());

        Self { grid_origin, subgrid_offset }
    }
}