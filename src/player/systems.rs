use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::player::components::Player;
use crate::universe::components::UniverseObserver;

pub const PLAYER_SPEED: f32 = 500.0;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(64.0, 64.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            texture: asset_server.load("sprites/circle.png"),
            ..default()
        },
        Player {},
        UniverseObserver::new(2)
    ));
}

pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).despawn();
    }
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();

        let global_pos: ((i16, i16), (f32, f32)) = f32_to_i16_chunk_position_with_local_offset(transform.translation.x, transform.translation.y, crate::universe::CHUNK_SIZE as f32, 0, 0);

        println!("Scene Position: {}, {} | Chunk position: {:?}", transform.translation.x, transform.translation.y, global_pos);
    }
}

fn f32_to_i16_chunk_position_with_local_offset(
    x: f32, y: f32, 
    chunk_size: f32, 
    global_origin_x: i16, global_origin_y: i16
) -> ((i16, i16), (f32, f32)) {
    let intermediate_x = x / chunk_size;
    let intermediate_y = y / chunk_size;

    let normalized_x = intermediate_x.floor() as i16;
    let normalized_y = intermediate_y.floor() as i16;

    let final_chunk_x = normalized_x + global_origin_x;
    let final_chunk_y = normalized_y + global_origin_y;

    let local_offset_x = x % chunk_size;
    let local_offset_y = y % chunk_size;

    ((final_chunk_x, final_chunk_y), (local_offset_x, local_offset_y))
}
