use bevy::prelude::*;

use super::constants::CHUNK_SIZE;

pub(in crate) fn calculate_chunks_in_radius(position: Vec2, radius: u32) -> Vec<(i32, i32)> {
    let (center_chunk_x, center_chunk_y) = grid_position(position);

    let mut chunks = Vec::new();
    let radius = radius as i32; // Convert to signed for easier arithmetic

    for dx in -radius..=radius {
        for dy in -radius..=radius {
            // Check if the chunk is within the circular radius in chunk space
            if dx * dx + dy * dy <= radius * radius {
                chunks.push((center_chunk_x + dx, center_chunk_y + dy));
            }
        }
    }

    chunks
}


pub(in crate) fn grid_position(position: Vec2) -> (i32, i32) {
    let chunk_x = (position.x / CHUNK_SIZE).floor() as i32;
    let chunk_y = (position.y / CHUNK_SIZE).floor() as i32;
    (chunk_x, chunk_y)
}