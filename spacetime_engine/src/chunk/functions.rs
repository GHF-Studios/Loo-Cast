use std::{collections::HashSet, sync::MutexGuard};

use bevy::prelude::*;

use crate::chunk::{bundles::ChunkBundle, components::ChunkComponent, constants::HALF_CHUNK_SIZE};

use super::constants::{CHUNK_SIZE, DEFAULT_CHUNK_Z};

pub(in crate) fn calculate_chunks_in_radius(position: Vec2, radius: u32) -> Vec<(i32, i32)> {
    let (center_chunk_x, center_chunk_y) = world_pos_to_chunk(position);

    let mut chunks = Vec::new();
    let radius = radius as i32; // Convert radius to signed integer for chunk-space logic
    let half_chunk = CHUNK_SIZE / 2.0; // Half the chunk size for adjusting the radius check

    for dx in -radius..=radius {
        for dy in -radius..=radius {
            // Calculate the world position of the current chunk's center
            let chunk_center_x = (center_chunk_x + dx) as f32 * CHUNK_SIZE;
            let chunk_center_y = (center_chunk_y + dy) as f32 * CHUNK_SIZE;

            // Adjust the radius comparison to include chunks partially in the range
            let adjusted_distance_x = chunk_center_x - position.x;
            let adjusted_distance_y = chunk_center_y - position.y;

            // Include chunks partially in range by extending the radius
            let distance_squared = (adjusted_distance_x).powi(2) + (adjusted_distance_y).powi(2);
            if distance_squared <= (radius as f32 * CHUNK_SIZE + half_chunk).powi(2) {
                chunks.push((center_chunk_x + dx, center_chunk_y + dy));
            }
        }
    }

    chunks
}


pub(in crate) fn world_pos_to_chunk(position: Vec2) -> (i32, i32) {
    let chunk_x = ((position.x + CHUNK_SIZE / 2.0) / CHUNK_SIZE).floor() as i32;
    let chunk_y = ((position.y + CHUNK_SIZE / 2.0) / CHUNK_SIZE).floor() as i32;
    (chunk_x, chunk_y)
}

pub(in crate) fn chunk_pos_to_world(grid_coord: (i32, i32)) -> Vec2 {
    let chunk_x = grid_coord.0 as f32 * CHUNK_SIZE;
    let chunk_y = grid_coord.1 as f32 * CHUNK_SIZE;
    Vec2::new(chunk_x, chunk_y)
}

pub(in crate) fn spawn_chunk(commands: &mut Commands, mut requested_chunk_additions: MutexGuard<HashSet<(i32, i32)>>, chunk_coord: (i32, i32), chunk_owner: Entity) {
    if requested_chunk_additions.contains(&chunk_coord) {
        return;
    }

    requested_chunk_additions.insert(chunk_coord);
    commands.spawn(ChunkBundle {
        chunk: ChunkComponent {
            coord: chunk_coord,
            owner: Some(chunk_owner)
        },
        sprite_bundle: SpriteBundle {
            sprite: Sprite {
                color: if (chunk_coord.0 + chunk_coord.1) % 2 == 0 {
                    Color::srgb(0.75, 0.75, 0.75)
                } else {
                    Color::srgb(0.25, 0.25, 0.25)
                },
                rect: Some(Rect::new(-HALF_CHUNK_SIZE, -HALF_CHUNK_SIZE, HALF_CHUNK_SIZE, HALF_CHUNK_SIZE)),
                ..Default::default()
            },
            transform: Transform {
                translation: chunk_pos_to_world(chunk_coord).extend(DEFAULT_CHUNK_Z),
                ..Default::default()
            },
            ..Default::default()
        },
    });
}

pub(in crate) fn despawn_chunk(commands: &mut Commands, mut requested_chunk_removals: MutexGuard<HashSet<(i32, i32)>>, chunk_coord: (i32, i32), chunk_entity: Entity) {
    if requested_chunk_removals.contains(&chunk_coord) {
        return;
    }

    requested_chunk_removals.insert(chunk_coord);
    commands.entity(chunk_entity).despawn_recursive();
}