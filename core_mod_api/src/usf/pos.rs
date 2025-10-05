use std::collections::HashMap;

use bevy::prelude::*;

use crate::utils::types::I128Vec2;

use super::scale::{Scale, DynScale};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GridPos {
    parent: Option<Box<GridPos>>,
    scale: Scale,
    xy: I128Vec2,
}
impl GridPos {
    pub fn new(parent: Option<GridPos>, scale: Scale, xy: I128Vec2) -> Self {
        const GRID_SIZE: i128 = 1000_i128;

        let parent = match parent {
            Some(parent) => {
                assert!(parent.scale > scale, "Parent scale must be greater than child scale");
                Some(Box::new(parent))
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
pub struct ChunkPos {
    scale_origin: GridPos,
    offset: GridPos,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FloatPos {
    chunk_origin: ChunkPos,
    offset: Vec3,
}
impl FloatPos {
    fn new(chunk_origin: ChunkPos, offset: Vec3) -> Self {
        Self { chunk_origin, offset }
    }

    pub fn from_raw(scale_origin: GridPos, raw_offset: Vec3) -> Self {
        
    }
}