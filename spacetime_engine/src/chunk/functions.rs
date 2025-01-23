use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{chunk::components::ChunkComponent, chunk_loader::components::ChunkLoaderComponent, config::statics::CONFIG};

use super::{enums::ChunkAction, errors::{DespawnError, SpawnError, TransferOwnershipError}, resources::ChunkRenderHandles, ChunkActionBuffer, ChunkManager};

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

pub(in crate) fn calculate_chunk_distance_from_owner(coord1: &(i32, i32), coord2: &(i32, i32)) -> u32 {
    let dx = coord1.0 - coord2.0;
    let dy = coord1.1 - coord2.1;
    (dx * dx + dy * dy).try_into().unwrap()
}

pub(in crate) fn world_pos_to_chunk(position: Vec2) -> (i32, i32) {
    let chunk_size = CONFIG.get::<f32>("chunk/size");
    let chunk_x = ((position.x + chunk_size / 2.0) / chunk_size).floor() as i32;
    let chunk_y = ((position.y + chunk_size / 2.0) / chunk_size).floor() as i32;
    (chunk_x, chunk_y)
}

pub(in crate) fn chunk_pos_to_world(grid_coord: (i32, i32)) -> Vec2 {
    let chunk_size = CONFIG.get::<f32>("chunk/size");
    let chunk_x = grid_coord.0 as f32 * chunk_size;
    let chunk_y = grid_coord.1 as f32 * chunk_size;
    Vec2::new(chunk_x, chunk_y)
}

pub(in crate) fn process_chunk_action(
    action: ChunkAction,
    commands: &mut Commands,
    chunk_query: &mut Query<(Entity, &mut ChunkComponent)>,
    chunk_loader_query: &Query<Entity, With<ChunkLoaderComponent>>,
    chunk_manager: &mut ChunkManager,
    chunk_action_buffer: &mut ChunkActionBuffer,
    chunk_render_handles: &ChunkRenderHandles,
) {
    match action {
        ChunkAction::Spawn { coord, new_owner: owner, .. } => {
            let quad_handle = chunk_render_handles.quad_handle.clone();
            let material_handle = if (coord.0 + coord.1) % 2 == 0 {
                chunk_render_handles.light_material_handle.clone()
            } else {
                chunk_render_handles.dark_material_handle.clone()
            };

            if let Some(owner) = owner {
                if !chunk_loader_query.contains(owner) {
                    return;
                }
            }

            if let Err(err) = spawn_chunk(
                commands,
                chunk_manager,
                chunk_action_buffer,
                coord,
                owner,
                quad_handle,
                material_handle,
            ) {
                panic!("Failed to spawn chunk '{:?}': {:?}", coord, err);
            }
        }
        ChunkAction::Despawn { coord, .. } => {
            if let Err(err) = despawn_chunk(commands, chunk_manager, chunk_action_buffer, chunk_query, coord) {
                panic!("Failed to despawn chunk '{:?}': {:?}", coord, err);
            }
        }
        ChunkAction::TransferOwnership { coord, new_owner, .. } => {
            if let Err(err) = transfer_chunk_ownership(chunk_manager, chunk_action_buffer, chunk_query, coord, new_owner)
            {
                panic!("Failed to transfer ownership of chunk '{:?}': {:?}", coord, err);
            }
        }
    }
}

pub(in crate) fn spawn_chunk(
    commands: &mut Commands,
    chunk_manager: &mut ChunkManager,
    chunk_action_buffer: &mut ChunkActionBuffer,
    chunk_coord: (i32, i32),
    chunk_owner: Option<Entity>,
    quad_handle: Handle<Mesh>,
    material_handle: Handle<ColorMaterial>,
) -> Result<(), SpawnError> {
    let (is_loaded, _) = chunk_manager.get_states(&chunk_coord);
    if is_loaded {
        return Err(SpawnError::AlreadySpawned { chunk_coord });
    }
    
    let (is_spawning, is_despawning, is_transfering_ownership) = chunk_action_buffer.get_action_states(&chunk_coord);
    if !is_spawning {
        return Err(SpawnError::NotSpawning { chunk_coord });
    }
    if is_despawning {
        return Err(SpawnError::AlreadyBeingDespawned { chunk_coord });
    }
    if is_transfering_ownership {
        return Err(SpawnError::AlreadyTransferingOwnership { chunk_coord });
    }

    let half_chunk_size = CONFIG.get::<f32>("chunk/size") / 2.0;
    let default_chunk_z = CONFIG.get::<f32>("chunk/default_z") / 2.0;

    let chunk_transform = Transform {
        translation: chunk_pos_to_world(chunk_coord).extend(default_chunk_z),
        scale: Vec3::new(half_chunk_size * 2.0, half_chunk_size * 2.0, 1.0),
        ..Default::default()
    };

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: quad_handle.clone().into(),
            material: material_handle.clone(),
            transform: chunk_transform,
            ..Default::default()
        },
        ChunkComponent {
            coord: chunk_coord,
            owner: chunk_owner,
        },
    ));

    chunk_manager.loaded_chunks.insert(chunk_coord);
    if let Some(chunk_owner) = chunk_owner {
        chunk_manager.owned_chunks.insert(chunk_coord, chunk_owner);
    }
    chunk_action_buffer.remove_action(&chunk_coord);

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

    match chunk_query.iter().find(|(_, chunk)| chunk.coord == chunk_coord) {
        Some((chunk_entity, _)) => {
            commands.entity(chunk_entity).despawn_recursive();
            
            chunk_manager.loaded_chunks.remove(&chunk_coord);
            chunk_manager.owned_chunks.remove(&chunk_coord);
            chunk_action_buffer.remove_action(&chunk_coord);

            Ok(())
        },
        None => {
            chunk_manager.loaded_chunks.remove(&chunk_coord);
            chunk_manager.owned_chunks.remove(&chunk_coord);
            chunk_action_buffer.remove_action(&chunk_coord);

            Ok(())
        }
    }
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
        .unwrap_or_else(|| unreachable!("Failed to transfer ownership of chunk '{:?}': it is already despawned according to the Chunk Query", chunk_coord));
    
    if chunk.owner.is_some() {
        chunk_manager.owned_chunks.remove(&chunk_coord);
    }
    chunk.owner = Some(new_chunk_owner);
    chunk_manager.owned_chunks.insert(chunk_coord, new_chunk_owner);
    chunk_action_buffer.remove_action(&chunk_coord);

    Ok(())
}
