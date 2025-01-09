use bevy::prelude::*;

use crate::chunk::{bundles::ChunkBundle, components::ChunkComponent, constants::HALF_CHUNK_SIZE};

use super::{constants::{CHUNK_SIZE, DEFAULT_CHUNK_Z}, errors::{DespawnError, SpawnError, TransferOwnershipError}, ChunkActionBuffer, ChunkManager};

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
    chunk_manager: &mut ChunkManager,
    chunk_action_buffer: &mut ChunkActionBuffer,
    chunk_coord: (i32, i32),
    chunk_owner: Option<Entity>,
) -> Result<(), SpawnError> {
    let (is_loaded, _) = chunk_manager.get_states(&chunk_coord);
    if is_loaded {
        return Err(SpawnError::AlreadySpawned { chunk_coord });
    }
    
    let (is_spawning, is_despawning, is_transfering_ownership) = chunk_action_buffer.get_action_states(&chunk_coord) ;
    if !is_spawning {
        return Err(SpawnError::NotSpawning { chunk_coord });
    }
    if is_despawning {
        return Err(SpawnError::AlreadyBeingDespawned { chunk_coord });
    }
    if is_transfering_ownership {
        return Err(SpawnError::AlreadyTransferingOwnership { chunk_coord });
    }

    commands.spawn(ChunkBundle {
        chunk: ChunkComponent {
            coord: chunk_coord,
            owner: chunk_owner
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
    
    chunk_manager.loaded_chunks.insert(chunk_coord);
    if let Some(chunk_owner) = chunk_owner {
        chunk_manager.owned_chunks.insert(chunk_coord, chunk_owner);
    }

    chunk_action_buffer.0.remove(&chunk_coord);
    
    Ok(())
}

pub(in crate) fn despawn_chunk(
    commands: &mut Commands,
    chunk_manager: &mut ChunkManager,
    chunk_action_buffer: &mut ChunkActionBuffer,
    chunk_query: &mut Query<(Entity, &mut ChunkComponent)>,
    chunk_coord: (i32, i32),
) -> Result<(), DespawnError> {
    let (is_loaded, _) = chunk_manager.get_states(&chunk_coord);
    if !is_loaded {
        return Err(DespawnError::AlreadyDespawned { chunk_coord });
    }

    let (is_spawning, is_despawning, is_transfering_ownership) = chunk_action_buffer.get_action_states(&chunk_coord) ;
    if is_spawning {
        return Err(DespawnError::AlreadyBeingSpawned { chunk_coord });
    }
    if !is_despawning {
        return Err(DespawnError::NotDespawning { chunk_coord });
    }
    if is_transfering_ownership {
        return Err(DespawnError::AlreadyTransferingOwnership { chunk_coord });
    }

    let (chunk_entity, chunk) = chunk_query
        .iter()
        .find(|(_, chunk)| chunk.coord == chunk_coord)
        .expect(format!("Failed to despawn chunk {:?}: Chunk Query did not include it", chunk_coord).as_str());
    
    commands.entity(chunk_entity).despawn_recursive();
    
    chunk_manager.loaded_chunks.insert(chunk_coord);
    if let Some(chunk_owner) = chunk.owner {
        chunk_manager.owned_chunks.insert(chunk_coord, chunk_owner);
    }
    
    chunk_action_buffer.0.remove(&chunk_coord);
    
    Ok(())
}

pub(in crate) fn transfer_chunk_ownership(
    chunk_manager: &mut ChunkManager,
    chunk_action_buffer: &mut ChunkActionBuffer,
    chunk_query: &mut Query<(Entity, &mut ChunkComponent)>,
    chunk_coord: (i32, i32),
    new_chunk_owner: Entity
) -> Result<(), TransferOwnershipError> {
    let (is_loaded, _) = chunk_manager.get_states(&chunk_coord);
    if !is_loaded {
        return Err(TransferOwnershipError::AlreadyDespawned { chunk_coord });
    }
    
    let (is_spawning, is_despawning, is_transfering_ownership) = chunk_action_buffer.get_action_states(&chunk_coord) ;
    if is_spawning {
        return Err(TransferOwnershipError::AlreadyBeingSpawned { chunk_coord });
    }
    if is_despawning {
        return Err(TransferOwnershipError::AlreadyBeingDespawned { chunk_coord });
    }
    if !is_transfering_ownership {
        return Err(TransferOwnershipError::NotTransferingOwnership { chunk_coord });
    }
    
    let (_, mut chunk) = chunk_query
        .iter_mut()
        .find(|(_, chunk)| chunk.coord == chunk_coord)
        .expect(format!("Failed to transfer ownership of chunk {:?}: Chunk Query did not include it", chunk_coord).as_str());
    
    chunk.owner = Some(new_chunk_owner);
    
    chunk_manager.loaded_chunks.insert(chunk_coord);
    chunk_manager.owned_chunks.insert(chunk_coord, new_chunk_owner);
    
    chunk_action_buffer.0.remove(&chunk_coord);
    
    Ok(())
}