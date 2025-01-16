use bevy::prelude::*;
use crate::chunk::enums::ChunkActionPriority;
use crate::config::statics::CONFIG;

use super::components::ChunkComponent;
use super::enums::ChunkAction;
use super::functions::{chunk_pos_to_world, despawn_chunk, spawn_chunk, transfer_chunk_ownership, world_pos_to_chunk};
use super::resources::ChunkRenderHandles;
use super::{ChunkActionBuffer, ChunkManager};

pub(in crate) fn startup_chunk_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let quad_handle = meshes.add(Mesh::from(Rectangle::new(1.0, 1.0)));
    let light_material_handle = materials.add(ColorMaterial::from_color(Color::srgb(0.75, 0.75, 0.75)));
    let dark_material_handle = materials.add(ColorMaterial::from_color(Color::srgb(0.25, 0.25, 0.25)));

    commands.insert_resource(ChunkRenderHandles {
        quad_handle,
        light_material_handle,
        dark_material_handle
    });
}

pub(in crate) fn update_chunk_system(
    chunk_query: Query<(Entity, &Transform, &ChunkComponent)>,
) {
    for (_, transform, chunk) in chunk_query.iter() {
        let world_pos = transform.translation.truncate();
        let chunk_pos = world_pos_to_chunk(world_pos);

        assert_eq!(
            chunk.coord, 
            chunk_pos, 
            "Attempted to move chunk entity"
        );
        assert_eq!(
            chunk_pos_to_world(chunk.coord),
            world_pos,
            "Attempted to move chunk entity"
        );
    }
}

pub(in crate) fn process_chunk_actions(
    mut commands: Commands,
    mut chunk_query: Query<(Entity, &mut ChunkComponent)>,
    mut chunk_manager: ResMut<ChunkManager>,
    mut chunk_action_buffer: ResMut<ChunkActionBuffer>,
    chunk_render_handles: Res<ChunkRenderHandles>,
) {
    let max_actions = CONFIG.get::<u32>("chunk/max_actions_per_update") as usize;

    let chunk_actions = chunk_action_buffer
        .iter()
        .map(|(key, action)| (*key, action.clone()))
        .collect::<Vec<_>>();
    let total_actions = chunk_actions.len();

    if total_actions == 0 {
        return;
    }

    for (total_actions_processed, (chunk_coord, chunk_action)) in chunk_actions.into_iter().enumerate() {
        if chunk_action.get_priority() != ChunkActionPriority::Realtime && total_actions_processed >= max_actions {
            break;
        }

        match chunk_action {
            ChunkAction::Spawn { coord, owner, .. } => {
                let quad_handle = chunk_render_handles.quad_handle.clone();
                let material_handle = if (coord.0 + coord.1) % 2 == 0 {
                    chunk_render_handles.light_material_handle.clone()
                } else {
                    chunk_render_handles.dark_material_handle.clone()
                };

                if let Err(err) =
                    spawn_chunk(
                        &mut commands, 
                        &mut chunk_manager, 
                        &mut chunk_action_buffer, 
                        coord, 
                        owner,
                        quad_handle,
                        material_handle
                    )
                {
                    panic!("Failed to spawn chunk '{:?}': {:?}", coord, err);
                }
            }
            ChunkAction::Despawn { coord, .. } => {
                if let Err(err) = despawn_chunk(
                    &mut commands,
                    &mut chunk_manager,
                    &mut chunk_action_buffer,
                    &mut chunk_query,
                    coord,
                ) {
                    panic!("Failed to despawn chunk '{:?}': {:?}", coord, err);
                }
            }
            ChunkAction::TransferOwnership { coord, new_owner, .. } => {
                if let Err(err) = transfer_chunk_ownership(
                    &mut chunk_manager,
                    &mut chunk_action_buffer,
                    &mut chunk_query,
                    coord,
                    new_owner,
                ) {
                    panic!("Failed to transfer ownership of chunk '{:?}': {:?}", coord, err);
                }
            }
        }

        chunk_action_buffer.remove_action(&chunk_coord);
    }
}