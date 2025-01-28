use bevy::prelude::*;

use crate::chunk_loader::components::ChunkLoaderComponent;

use super::components::ChunkComponent;
use super::functions::{chunk_pos_to_world, process_chunk_action, world_pos_to_chunk};
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

pub(in crate) fn process_chunk_ecs_actions(
    mut commands: Commands,
    mut chunk_query: Query<(Entity, &mut ChunkComponent)>,
    chunk_loader_query: Query<Entity, With<ChunkLoaderComponent>>,
    mut chunk_manager: ResMut<ChunkManager>,
    mut chunk_action_buffer: ResMut<ChunkActionBuffer>,
    chunk_render_handles: Res<ChunkRenderHandles>,
) {
    let mut processed_ecs_actions = vec![];
    let mut to_be_processed = vec![];

    let bucket_iter = chunk_action_buffer.priority_buckets.iter();

    for (_, coords) in bucket_iter {
        for coord in coords.iter().copied() {
            if let Some(action) = chunk_action_buffer.ecs_actions.get(&coord).cloned() {
                to_be_processed.push(action);
                processed_ecs_actions.push(coord);
            }
        }
    }

    for action in to_be_processed {
        process_chunk_action(
            action,
            &mut commands,
            &mut chunk_query,
            &chunk_loader_query,
            &mut chunk_manager,
            &mut chunk_action_buffer,
            &chunk_render_handles,
        );
    }

    chunk_action_buffer.remove_ecs_actions(processed_ecs_actions);
}
