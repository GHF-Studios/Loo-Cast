use bevy::prelude::*;
use crate::chunk::actor::components::ChunkActor;
use crate::chunk::position::structs::*;
use crate::chunk::actor::position::structs::*;
use crate::chunk::id::structs::*;
use crate::chunk::actor::id::structs::*;
use crate::chunk::loader::components::ChunkLoader;
use crate::physics::components::*;
use super::constants::*;

// TODO: Revamp this completely: Request a new chunk loader entity (which will be available once it's starting chunk has been loaded) 
//       and attach a Player component and any other relevant components to it.
//       This will allow the player to be spawned in a chunk that is already loaded.
// Do this as well for regular chunk actor entities.
// But think about this some more and in regards to chunk actors as opposed to chunk loaders and shit.
pub(in crate) fn new_player_entity(
    world: &mut World,
    player_chunk_id: ChunkID,
    player_chunk_actor_id: ChunkActorID,
) -> Entity {
    let player_chunk_position: ChunkPosition = player_chunk_id.into();
    let player_chunk_actor_position: ChunkActorPosition = player_chunk_position.into();
    let player_world_position: Vec3 = player_chunk_actor_position.0;

    let player_entity = world
        .spawn(super::components::Player)
        .insert(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.1, 0.1, 1.0),
                custom_size: Some(Vec2::splat(PLAYER_SIZE)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(player_world_position.x, player_world_position.y, PLAYER_Z_INDEX)),
            ..default()
        })
        .insert(ProxyRigidBody::Dynamic)
        .insert(ProxyCollider::Circle { radius: 15.0 })
        .insert(ProxyVelocity::linear(Vec2::new(0.0, 0.0)))
        .insert(ChunkLoader::new(chunk_loader_id, 1))
        .insert(ChunkActor::new(player_chunk_actor_id, player_chunk_id))
        .id();

    player_entity
}