use crate::entity::resources::*;
use crate::chunk::loader::components::*;
use bevy::prelude::*;
use bevy_rapier2d::{dynamics::{RigidBody, Velocity}, geometry::Collider};

pub(in crate) fn startup(
    mut commands: Commands, 
    mut player_startup_event_writer: EventWriter<super::events::Startup>,
    mut entity_manager: ResMut<EntityManager>
) {
    let player_entity = commands
        .spawn(super::components::Player)
        .insert(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.1, 0.1, 1.0),
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(15.0))
        .insert(Velocity::linear(Vec2::new(0.0, 0.0)))
        .insert(ChunkLoader { load_radius: 1, current_chunk_ids: Vec::new() })
        .id();

    let player_entity_id = entity_manager.register_entity();
    entity_manager.load_entity(player_entity_id, player_entity);

    player_startup_event_writer.send(super::events::Startup { player_entity_id });
}