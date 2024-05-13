use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::geometry::Collider;
use super::constants::*;
use crate::chunk::actor::events::CreateChunkActor;
use crate::chunk::resources::*;
use crate::chunk::id::structs::*;
use crate::chunk::coordinate::structs::*;
use crate::chunk::actor::coordinate::structs::*;
use crate::chunk::actor::components::*;
use crate::physics::components::*;

pub(in crate) fn update(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    collider_query: Query<(Entity, &Transform), With<Collider>>,
    mut chunk_registry: ResMut<ChunkRegistry>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    if let Ok(window) = window_query.get_single() {
        if let Some(cursor_pos) = window.cursor_position() {
            let window_size = Vec2::new(window.width(), window.height());
            let cursor_pos_ndc = Vec2::new(
                (cursor_pos.x / window_size.x) * 2.0 - 1.0, 
                1.0 - (cursor_pos.y / window_size.y) * 2.0
            );

            if let Ok((camera, camera_transform)) = camera_query.get_single() {
                let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
                let world_pos = ndc_to_world.project_point3(cursor_pos_ndc.extend(-1.0)).truncate();
                let chunk_chunk_actor_coordinate: ChunkActorCoordinate = world_pos.into();
                let chunk_coordinate: ChunkCoordinate = chunk_chunk_actor_coordinate.into();
                let chunk_id: ChunkID = chunk_coordinate.into();
                let half_prop_size = SQUARE_PROP_SIZE / 2.0;

                // Place a new prop on right click
                if mouse_button_input.just_pressed(MouseButton::Right) {
                    // NEW CODE
                    create_chunk_actor_event_writer.send(CreateChunkActor());





                    // OLD CODE
                    let prop_chunk_actor_id = chunk_registry.register_chunk_actor();

                    let prop_entity = commands.spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(0.5, 0.5, 1.0),
                            custom_size: Some(Vec2::splat(SQUARE_PROP_SIZE)),
                            ..default()
                        },
                        transform: Transform::from_translation(world_pos.extend(PROP_Z_INDEX)),
                        ..default()
                    })
                    .insert(ProxyRigidBody::Dynamic)
                    .insert(ProxyCollider::Square { half_length: half_prop_size })
                    .insert(ProxyVelocity::linear(Vec2 { x: 0.0, y: 0.0 }))
                    .insert(ChunkActor::new(prop_chunk_actor_id, chunk_id))
                    .id();

                    chunk_registry.load_chunk_actor(prop_chunk_actor_id, prop_entity);
                }

                // Delete props under the cursor on left click
                if mouse_button_input.just_pressed(MouseButton::Left) {
                    for (collider_entity, collider_transform) in collider_query.iter() {
                        let collider_position = collider_transform.translation.truncate();

                        if (collider_position - world_pos).abs().max_element() < SQUARE_PROP_SIZE / 2.0 {
                            commands.entity(collider_entity).despawn_recursive();
                        }
                    }
                }
            }
        }
    }
}