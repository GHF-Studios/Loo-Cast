use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::geometry::Collider;
use crate::chunk::actor::constants::*;
use crate::chunk::actor::events::*;
use crate::chunk::actor::resources::*;
use crate::chunk::id::structs::*;
use crate::chunk::position::structs::*;
use crate::chunk::actor::position::structs::*;
use crate::chunk::actor::components::*;
use crate::entity::resources::EntityRegistry;
use crate::physics::components::*;
use crate::player::components::Player;

pub(in crate) fn update_phase1(
    mut create_chunk_actor_entity_event_writer: EventWriter<CreateChunkActorEntity>,
    mut destroy_chunk_actor_entity_event_writer: EventWriter<DestroyChunkActorEntity>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    chunk_actor_query: Query<(&Transform, &ChunkActor), With<Collider>>,
    mut player_query: Query<&mut Player>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut chunk_actor_request_registry: ResMut<ChunkActorRequestRegistry>,
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
        let mut player = match player_query.get_single_mut() {
            Ok(player) => player,
            Err(_) => {
                panic!("Player entity does not exist or there is more than one player entity.");
            }
        };

        let chunk_actor_request_id = chunk_actor_request_registry.get_unused_chunk_actor_request_id();

        player.create_chunk_actor_request_ids.push(chunk_actor_request_id);

        create_chunk_actor_entity_event_writer.send(CreateChunkActorEntity {
            chunk_actor_request_id,
            world_position: hit_world_position,
        });
    } else if mouse_button_input.just_pressed(MouseButton::Left) {
        for (chunk_actor_transform, chunk_actor) in chunk_actor_query.iter() {
            let chunk_actor_position = chunk_actor_transform.translation.truncate();

            if hit_chunk_id != chunk_actor.current_chunk() {
                continue;
            }

            if (chunk_actor_position - hit_world_position).abs().max_element() >= CHUNK_ACTOR_SIZE / 2.0 {
                continue;
            }

            let chunk_actor_request_id = chunk_actor_request_registry.get_unused_chunk_actor_request_id();

            destroy_chunk_actor_entity_event_writer.send(DestroyChunkActorEntity {
                chunk_actor_request_id,
                chunk_actor_id: chunk_actor.id(),
            });
        }
    }
}

pub(in crate) fn update_phase2(
    mut commands: Commands,
    mut created_chunk_actor_entity_event_reader: EventReader<CreatedChunkActorEntity>,
    chunk_actor_query: Query<&Transform, With<ChunkActor>>,
    mut player_query: Query<&mut Player>,
    entity_registry: Res<EntityRegistry>,
) {
    let mut created_chunk_actor_entity_events = Vec::new();
    for created_chunk_actor_entity_event in created_chunk_actor_entity_event_reader.read() {
        created_chunk_actor_entity_events.push(created_chunk_actor_entity_event);
    }

    for created_chunk_actor_entity_event in created_chunk_actor_entity_events {
        let (chunk_actor_request_id, chunk_actor_entity_id) = match created_chunk_actor_entity_event {
            CreatedChunkActorEntity::Success { 
                chunk_actor_request_id, 
                chunk_actor_entity_id, 
                ..
            } => (chunk_actor_request_id, chunk_actor_entity_id),
            CreatedChunkActorEntity::Failure { .. } => {
                // TODO: Make this better
                panic!("Something is wrong, I can feel it");
            }
        };

        let mut player = match player_query.get_single_mut() {
            Ok(player) => player,
            Err(_) => {
                panic!("Player entity does not exist or there is more than one player entity.");
            }
        };

        if !player.create_chunk_actor_request_ids.contains(chunk_actor_request_id) {
            continue;
        } else {
            player.create_chunk_actor_request_ids.retain(|&id| id != *chunk_actor_request_id);
        }

        let chunk_actor_entity_reference = match entity_registry.get_loaded_entity_reference(chunk_actor_entity_id) {
            Some(chunk_actor_entity_reference) => chunk_actor_entity_reference,
            None => {
                panic!("Chunk Actor Entity '{:?}' does not exist!", chunk_actor_entity_id);
            }
        };

        let chunk_actor_transform = match chunk_actor_query.get(chunk_actor_entity_reference) {
            Ok(chunk_actor_transform) => chunk_actor_transform,
            Err(_) => {
                panic!("Chunk Actor Entity '{:?}' does not exist!", chunk_actor_entity_id);
            }
        };

        commands
            .entity(chunk_actor_entity_reference)
            .insert(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1.0, 0.1, 0.1),
                    custom_size: Some(Vec2::splat(CHUNK_ACTOR_SIZE)),
                    ..default()
                },
                transform: *chunk_actor_transform,
                ..default()
            })
            .insert(ProxyRigidBody::Dynamic)
            .insert(ProxyCollider::Square { half_length: CHUNK_ACTOR_SIZE / 2.0})
            .insert(ProxyVelocity::linear(Vec2 { x: 0.0, y: 0.0 }));

        continue;
    }
}

