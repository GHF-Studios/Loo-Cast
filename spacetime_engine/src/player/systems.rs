use crate::chunk::id::structs::ChunkID;
use crate::chunk::resources::ChunkRegistry;
use crate::entity::resources::*;
use crate::chunk::loader::components::*;
use crate::chunk::actor::components::*;
use bevy::prelude::*;
use crate::physics::components::*;
use super::constants::PLAYER_Z_INDEX;

pub(in crate) fn startup(
    mut commands: Commands, 
    mut player_startup_event_writer: EventWriter<super::events::Startup>,
    mut chunk_registry: ResMut<ChunkRegistry>,
    mut entity_registry: ResMut<EntityRegistry>
) {
    let player_chunk_actor_id = chunk_registry.register_chunk_actor();

    let player_entity = commands
        .spawn(super::components::Player)
        .insert(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.1, 0.1, 1.0),
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, PLAYER_Z_INDEX)),
            ..default()
        })
        .insert(ProxyRigidBody::Dynamic)
        .insert(ProxyCollider::Circle { radius: 15.0 })
        .insert(ProxyVelocity::linear(Vec2::new(0.0, 0.0)))
        .insert(ChunkLoader { load_radius: 1, current_chunk_ids: Vec::new() })
        .insert(ChunkActor { id: player_chunk_actor_id, current_chunk: ChunkID::default()})
        .id();

    chunk_registry.load_chunk_actor(player_chunk_actor_id, player_entity);

    let player_entity_id = entity_registry.register_entity();

    entity_registry.load_entity(player_entity_id, player_entity);

    player_startup_event_writer.send(super::events::Startup { player_entity_id });
}