use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::geometry::Collider;
use crate::chunk::actor::constants::*;
use crate::chunk::actor::events::*;
use crate::chunk::actor::resources::*;
use crate::chunk::components::Chunk;
use crate::chunk::resources::*;
use crate::chunk::id::structs::*;
use crate::chunk::position::structs::*;
use crate::chunk::actor::position::structs::*;
use crate::chunk::actor::components::*;

pub(in crate) fn update(
    mut create_chunk_actor_entity_event_writer: EventWriter<CreateChunkActorEntity>,
    mut destroy_chunk_actor_entity_event_writer: EventWriter<DestroyChunkActorEntity>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    chunk_actor_query: Query<(&Transform, &ChunkActor), With<Collider>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut chunk_actor_event_registry: ResMut<ChunkActorEventRegistry>,
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
        let chunk_actor_event_id = chunk_actor_event_registry.get_unused_chunk_actor_event_id();
        
        create_chunk_actor_entity_event_writer.send(CreateChunkActorEntity {
            chunk_actor_event_id,
            chunk_id: hit_chunk_id,
            world_position: hit_world_position,
        });

        // TODO: somehow add secondary components to the chunk actor entity
    } else if mouse_button_input.just_pressed(MouseButton::Left) {
        for (chunk_actor_transform, chunk_actor) in chunk_actor_query.iter() {
            let chunk_actor_position = chunk_actor_transform.translation.truncate();

            if hit_chunk_id != chunk_actor.current_chunk() {
                continue;
            }

            if (chunk_actor_position - hit_world_position).abs().max_element() >= CHUNK_ACTOR_SIZE / 2.0 {
                continue;
            }

            let chunk_actor_event_id = chunk_actor_event_registry.get_unused_chunk_actor_event_id();

            destroy_chunk_actor_entity_event_writer.send(DestroyChunkActorEntity {
                chunk_actor_event_id,
                chunk_actor_id: chunk_actor.id(),
            });
        }
    }
}