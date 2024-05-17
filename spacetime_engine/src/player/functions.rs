use bevy::prelude::*;
use crate::chunk::actor::components::ChunkActor;
use crate::chunk::coordinate::structs::*;
use crate::chunk::actor::coordinate::structs::*;
use crate::chunk::id::structs::*;
use crate::chunk::actor::id::structs::*;
use crate::chunk::loader::components::ChunkLoader;
use crate::physics::components::*;
use super::constants::*;

pub(in crate) fn new_player_entity(
    world: &mut World,
    player_chunk_id: ChunkID,
    player_chunk_actor_id: ChunkActorID,
) -> Entity {
    let player_chunk_coordinate: ChunkCoordinate = player_chunk_id.into();
    let player_chunk_actor_coordinate: ChunkActorCoordinate = player_chunk_coordinate.into();
    let player_world_pos: Vec3 = player_chunk_actor_coordinate.0;

    let player_entity = world
        .spawn(super::components::Player)
        .insert(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.1, 0.1, 1.0),
                custom_size: Some(Vec2::splat(PLAYER_SIZE)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(player_world_pos.x, player_world_pos.y, PLAYER_Z_INDEX)),
            ..default()
        })
        .insert(ProxyRigidBody::Dynamic)
        .insert(ProxyCollider::Circle { radius: 15.0 })
        .insert(ProxyVelocity::linear(Vec2::new(0.0, 0.0)))
        .insert(ChunkLoader { load_radius: 1, current_chunk_ids: Vec::new() })
        .insert(ChunkActor::new(player_chunk_actor_id, player_chunk_id))
        .id();

    player_entity
}