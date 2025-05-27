use bevy::prelude::*;

use crate::config::statics::CONFIG;

pub(crate) fn calculate_chunks_in_radius(position: Vec2, radius: u32) -> Vec<(i32, i32)> {
    let (center_chunk_x, center_chunk_y) = world_pos_to_chunk(position);
    let mut chunks = Vec::new();

    let radius = radius as i32;

    let mut x = 0;
    let mut y = radius;
    let mut d = 1 - radius; // Decision parameter

    while x <= y {
        // Add filled lines between symmetrical points
        for dx in -x..=x {
            chunks.push((center_chunk_x + dx, center_chunk_y + y));
            chunks.push((center_chunk_x + dx, center_chunk_y - y));
        }
        for dx in -y..=y {
            chunks.push((center_chunk_x + dx, center_chunk_y + x));
            chunks.push((center_chunk_x + dx, center_chunk_y - x));
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

pub(crate) fn calculate_chunk_distance_from_owner(coord1: &(i32, i32), coord2: &(i32, i32)) -> u32 {
    let dx = coord1.0 - coord2.0;
    let dy = coord1.1 - coord2.1;
    (dx * dx + dy * dy).try_into().unwrap()
}

pub(crate) fn world_pos_to_chunk(position: Vec2) -> (i32, i32) {
    let chunk_size = CONFIG.get::<f32>("chunk/size");
    let chunk_x = ((position.x + chunk_size / 2.0) / chunk_size).floor() as i32;
    let chunk_y = ((position.y + chunk_size / 2.0) / chunk_size).floor() as i32;
    (chunk_x, chunk_y)
}

pub(crate) fn chunk_pos_to_world(grid_coord: (i32, i32)) -> Vec2 {
    let chunk_size = CONFIG.get::<f32>("chunk/size");
    let chunk_x = grid_coord.0 as f32 * chunk_size;
    let chunk_y = grid_coord.1 as f32 * chunk_size;
    Vec2::new(chunk_x, chunk_y)
}
