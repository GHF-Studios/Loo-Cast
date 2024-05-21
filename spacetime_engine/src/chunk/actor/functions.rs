use bevy::ecs::system::SystemState;
use bevy::prelude::*;

use crate::chunk::position::structs::ChunkPosition;
use crate::{chunk::actor::components::*, entity::resources::EntityRegistry};
use crate::chunk::id::structs::*;
use crate::chunk::actor::id::structs::*;
use crate::chunk::ChunkRegistry;
use crate::physics::components::*;
use super::position::structs::ChunkActorPosition;
use super::{constants::*, ChunkActorRegistry};

pub(in crate::chunk::actor) fn OLD_new_chunk_actor_entity(
    commands: &mut Commands,
    hit_world_position: Vec2,
    hit_chunk_id: ChunkID,
    new_chunk_actor_id: ChunkActorID,
) -> Entity {
    let half_prop_size = CHUNK_ACTOR_SIZE / 2.0;

    let new_chunk_actor_entity = commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.5, 0.5, 1.0),
            custom_size: Some(Vec2::splat(CHUNK_ACTOR_SIZE)),
            ..default()
        },
        transform: Transform::from_translation(hit_world_position.extend(CHUNK_ACTOR_Z_INDEX)),
        ..default()
    })
    .insert(ProxyRigidBody::Dynamic)
    .insert(ProxyCollider::Square { half_length: half_prop_size })
    .insert(ProxyVelocity::linear(Vec2 { x: 0.0, y: 0.0 }))
    .insert(ChunkActor::new(new_chunk_actor_id, hit_chunk_id))
    .id();

    new_chunk_actor_entity
}

pub(in crate::chunk::actor) fn new_chunk_actor_entity(
    world: &mut World,
    registry_parameters: &mut SystemState<(
        ResMut<ChunkRegistry>,
        ResMut<ChunkActorRegistry>,
        ResMut<EntityRegistry>,
    )>,
    new_chunk_actor_world_position: Vec2
) {
    let world_position = new_chunk_actor_world_position.extend(CHUNK_ACTOR_Z_INDEX);
    let chunk_actor_position: ChunkActorPosition = world_position.into();
    let chunk_position: ChunkPosition = chunk_actor_position.into();
}