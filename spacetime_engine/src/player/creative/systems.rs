use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::geometry::Collider;
use crate::chunk::actor::constants::*;
use crate::chunk::actor::functions;
use crate::chunk::actor::resources::ChunkActorRegistry;
use crate::chunk::components::Chunk;
use crate::chunk::resources::*;
use crate::chunk::id::structs::*;
use crate::chunk::position::structs::*;
use crate::chunk::actor::position::structs::*;
use crate::chunk::actor::components::*;

pub(in crate) fn update(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    chunk_actor_query: Query<(Entity, &Transform, &ChunkActor), With<Collider>>,
    mut chunk_query: Query<&mut Chunk>,
    chunk_registry: Res<ChunkRegistry>,
    mut chunk_actor_registry: ResMut<ChunkActorRegistry>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    let window = match window_query.get_single() {
        Ok(window) => window,
        Err(_) => {
            return;
        }
    };

    let cursor_position = match window.cursor_position() {
        Some(cursor_position) => cursor_position,
        None => {
            return;
        }
    };
    
    let window_size = Vec2::new(window.width(), window.height());
    let cursor_position_ndc = Vec2::new(
        (cursor_position.x / window_size.x) * 2.0 - 1.0, 
        1.0 - (cursor_position.y / window_size.y) * 2.0
    );

    let (camera, camera_transform) = match camera_query.get_single() {
        Ok((camera, camera_transform)) => (camera, camera_transform),
        Err(_) => {
            return;
        }
    };

    let hit_ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
    let hit_world_position = hit_ndc_to_world.project_point3(cursor_position_ndc.extend(-1.0)).truncate();
    let hit_chunk_chunk_actor_position: ChunkActorPosition = hit_world_position.into();
    let hit_chunk_position: ChunkPosition = hit_chunk_chunk_actor_position.into();
    let hit_chunk_id: ChunkID = hit_chunk_position.into();

    if mouse_button_input.just_pressed(MouseButton::Right) {
        let hit_chunk_entity = match chunk_registry.get_loaded_chunk_entity(hit_chunk_id) {
            Some(chunk_entity) => chunk_entity,
            None => {
                return;
            }
        };

        let mut hit_chunk = match chunk_query.get_mut(hit_chunk_entity) {
            Ok(chunk) => chunk,
            Err(_) => {
                return;
            }
        };

        let new_chunk_actor_id = chunk_actor_registry.register_chunk_actor();
        let new_chunk_actor_entity = functions::new_chunk_actor_entity(&mut commands, new_chunk_actor_id, hit_chunk_id, hit_world_position);
        chunk_actor_registry.load_chunk_actor(new_chunk_actor_id, new_chunk_actor_entity);
        hit_chunk.add_chunk_actor(new_chunk_actor_id);
    } else if mouse_button_input.just_pressed(MouseButton::Left) {
        for (chunk_actor_entity, chunk_actor_transform, chunk_actor) in chunk_actor_query.iter() {
            let chunk_actor_position = chunk_actor_transform.translation.truncate();

            if (chunk_actor_position - hit_world_position).abs().max_element() >= CHUNK_ACTOR_SIZE / 2.0 {
                continue;
            }

            let chunk_actor_id = chunk_actor.id();
            let chunk_id = chunk_actor.current_chunk();

            let chunk_entity = match chunk_registry.get_loaded_chunk_entity(chunk_id) {
                Some(chunk_entity) => chunk_entity,
                None => {
                    continue;
                }
            };

            let mut chunk = match chunk_query.get_mut(chunk_entity) {
                Ok(chunk) => chunk,
                Err(_) => {
                    continue;
                }
            };

            chunk.remove_chunk_actor(chunk_actor.id());
            chunk_actor_registry.unload_chunk_actor(chunk_actor_id);
            commands.entity(chunk_actor_entity).despawn_recursive();
            chunk_actor_registry.unregister_chunk_actor(chunk_actor_id);
        }
    }
}