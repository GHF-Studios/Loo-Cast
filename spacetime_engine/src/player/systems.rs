use bevy::prelude::*;
use bevy_rapier2d::{dynamics::{RigidBody, Velocity}, geometry::Collider};

use super::components::Player;

pub(in crate) fn startup(mut commands: Commands) {
    commands
        .spawn(Player)
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
        .insert(ChunkLoader { load_radius: 1, current_chunk_ids: Vec::new() });
}