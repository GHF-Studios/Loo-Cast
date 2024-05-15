use bevy::prelude::*;

use crate::chunk::actor::components::*;
use crate::chunk::id::structs::*;
use crate::chunk::actor::id::structs::*;
use crate::physics::components::*;
use super::constants::*;

pub(in crate) fn new_chunk_actor_entity(
    commands: &mut Commands,
    hit_world_pos: Vec2,
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
        transform: Transform::from_translation(hit_world_pos.extend(CHUNK_ACTOR_Z_INDEX)),
        ..default()
    })
    .insert(ProxyRigidBody::Dynamic)
    .insert(ProxyCollider::Square { half_length: half_prop_size })
    .insert(ProxyVelocity::linear(Vec2 { x: 0.0, y: 0.0 }))
    .insert(ChunkActor::new(new_chunk_actor_id, hit_chunk_id))
    .id();

    new_chunk_actor_entity
}