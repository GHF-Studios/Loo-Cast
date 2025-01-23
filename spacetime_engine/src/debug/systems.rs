use bevy::{prelude::*, window::PrimaryWindow};
use crate::{camera::components::MainCamera, chunk::{components::ChunkComponent, functions::world_pos_to_chunk}, chunk_loader::components::ChunkLoaderComponent};

use super::components::{TestObjectComponent, TestObjectMovement};

pub(in crate) fn test_object_movement_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &TestObjectComponent)>,
) {
    for (mut transform, test_object) in query.iter_mut() {
        match &test_object.movement {
            TestObjectMovement::Static => {}
            TestObjectMovement::Circle { radius, speed } => {
                let time_factor = time.elapsed_seconds() * speed;
                transform.translation.x = radius * time_factor.cos();
                transform.translation.y = radius * time_factor.sin();
            }
            TestObjectMovement::Line { distance, speed } => {
                let time_factor = time.elapsed_seconds() * speed;
                let offset = time_factor.sin() * distance;
                transform.translation.x = offset;
            }
        }
    }
}

pub(in crate) fn chunk_inspection_system(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    chunk_query: Query<&ChunkComponent>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if mouse.just_pressed(MouseButton::Right) {
        let (camera, camera_transform) = camera_query.single();
        let window = window_query.single();

        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            let chunk_coord = world_pos_to_chunk(world_position);
            if let Some(chunk) = chunk_query.iter().find(|chunk| chunk.coord == chunk_coord) {
                debug!("Inspecting chunk: {:?}", chunk);
            }
        }
    }
}

pub(in crate) fn chunk_loader_inspection_system(
    chunk_loader_query: Query<Entity, With<ChunkLoaderComponent>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::KeyL) {
        let mut chunk_loader_entities = vec![];
        for chunk_loader_entity in chunk_loader_query.iter() {
            chunk_loader_entities.push(chunk_loader_entity);
        }

        debug!("Inspecting chunk loaders: {:?}", chunk_loader_entities);
    }
}