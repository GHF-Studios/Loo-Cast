use bevy::prelude::*;

use crate::chunk::traits::{Vec2Ext, IVec2Ext};
use crate::chunk::types::{WorldCoord, ChunkCoord, SquaredChunkDist};
use crate::config::statics::CONFIG;
use crate::usf::scale::{Scale, DynScale};

pub(crate) fn chunks_in_radius(coord: ChunkCoord, radius: u32) -> Vec<ChunkCoord> {
    let mut chunks = Vec::new();

    let radius = radius as i32;

    let mut x = 0;
    let mut y = radius;
    let mut d = 1 - radius; // Decision parameter

    while x <= y {
        // Add filled lines between symmetrical points
        for dx in -x..=x {
            chunks.push(IVec2::new(coord.xy.x + dx, coord.xy.y + y).scaled(coord.scale));
            chunks.push(IVec2::new(coord.xy.x + dx, coord.xy.y - y).scaled(coord.scale));
        }
        for dx in -y..=y {
            chunks.push(IVec2::new(coord.xy.x + dx, coord.xy.y + x).scaled(coord.scale));
            chunks.push(IVec2::new(coord.xy.x + dx, coord.xy.y - x).scaled(coord.scale));
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

pub(crate) fn chunk_distance_squared(coord1: ChunkCoord, coord2: ChunkCoord) -> SquaredChunkDist {
    let dx = coord1.xy.x - coord2.xy.x;
    let dy = coord1.xy.y - coord2.xy.y;
    let scale_dist = coord1.scale.scale_factor_exponent() - coord2.scale.scale_factor_exponent();
    let squared_chunk_dist = dx * dx + dy * dy;
    
    SquaredChunkDist { squared_grid_dist: squared_chunk_dist, scale_dist }
}

pub(crate) fn world_pos_to_chunk(position: Vec2, scale: Scale) -> ChunkCoord {
    let chunk_size = CONFIG().get::<u32>("chunk/size") as f32;
    let chunk_x = ((position.x + chunk_size / 2.0) / chunk_size).floor() as i32;
    let chunk_y = ((position.y + chunk_size / 2.0) / chunk_size).floor() as i32;
    IVec2::new(chunk_x, chunk_y).scaled(scale)
}

pub(crate) fn chunk_pos_to_world(grid_coord: ChunkCoord) -> WorldCoord {
    let chunk_size = CONFIG().get::<u32>("chunk/size") as f32;
    let chunk_x = grid_coord.xy.x as f32 * chunk_size;
    let chunk_y = grid_coord.xy.y as f32 * chunk_size;
    Vec2::new(chunk_x, chunk_y).scaled(grid_coord.scale)
}
