use std::{collections::HashSet, sync::MutexGuard};
use bevy::prelude::*;

use crate::chunk::{bundles::ChunkBundle, components::ChunkComponent, constants::HALF_CHUNK_SIZE};

use super::{constants::{CHUNK_SIZE, DEFAULT_CHUNK_Z}, errors::{DespawnError, SpawnError}, resources::ChunkRetryQueue};

pub(in crate) fn calculate_chunks_in_radius(position: Vec2, radius: u32) -> Vec<(i32, i32)> {
    let (center_chunk_x, center_chunk_y) = world_pos_to_chunk(position);
    let mut chunks = Vec::new();

    let radius = radius as i32; // Convert to signed integer

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

pub(in crate) fn spawn_chunk(
    commands: &mut Commands, 
    mut requested_chunk_additions: MutexGuard<HashSet<(i32, i32)>>, 
    requested_chunk_removals: MutexGuard<HashSet<(i32, i32)>>, 
    chunk_coord: (i32, i32), 
    chunk_owner: Entity
) -> Result<(), SpawnError> {
    if requested_chunk_removals.contains(&chunk_coord) {
        return Err(SpawnError::AlreadyBeingDespawned { chunk_coord });
    }

    if requested_chunk_additions.contains(&chunk_coord) {
        return Err(SpawnError::AlreadyBeingSpawned { chunk_coord });
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

    Ok(())
}

pub(in crate) fn despawn_chunk(
    commands: &mut Commands, 
    requested_chunk_additions: MutexGuard<HashSet<(i32, i32)>>, 
    mut requested_chunk_removals: MutexGuard<HashSet<(i32, i32)>>, 
    chunk_coord: (i32, i32), 
    chunk_entity: Entity
) -> Result<(), DespawnError> {
    if requested_chunk_additions.contains(&chunk_coord) {
        return Err(DespawnError::StillBeingSpawned { chunk_coord });
    }

    if requested_chunk_removals.contains(&chunk_coord) {
        return Err(DespawnError::AlreadyBeingDespawned { chunk_coord });
    }

    requested_chunk_removals.insert(chunk_coord);
    commands.entity(chunk_entity).despawn_recursive();

    Ok(())
}