use crate::background::components::*;
use crate::background::resources::*;
use crate::player::components::*;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn insert_background_manager(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    commands.insert_resource(BackgroundManager {
        background_origin_x: (window.width() / 2.0) as i32,
        background_origin_y: (window.height() / 2.0) as i32,
    })
}

pub fn remove_background_manager(mut commands: Commands) {
    commands.remove_resource::<BackgroundManager>();
}

pub fn spawn_background(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for x in -1..2 {
        for y in -1..2 {
            let window_width = window.width();
            let window_height = window.height();
            let x = (window_width / 2.0) + (window_width * x as f32);
            let y = (window_height / 2.0) + (window_height * y as f32);

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(window_width, window_height)),
                        ..default()
                    },
                    transform: Transform::from_xyz(x, y, -1.0),
                    texture: asset_server.load("sprites/background.png").into(),
                    ..default()
                },
                Background {
                    background_width: window_width as i32,
                    background_height: window_height as i32,
                    background_chunk_position_x: x as i32,
                    background_chunk_position_y: y as i32,
                },
            ));
        }
    }
}

pub fn despawn_background(
    mut commands: Commands,
    background_query: Query<Entity, With<Background>>,
) {
    if let Ok(background_entity) = background_query.get_single() {
        commands.entity(background_entity).despawn();
    }
}

pub fn move_background(
    mut background_manager: ResMut<BackgroundManager>,
    mut background_transform_query: Query<&mut Transform, With<Background>>,
    player_transform_query: Query<&Transform, (With<Player>, Without<Background>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    if let Ok(player_entity) = player_transform_query.get_single() {
        let difference_x =
            player_entity.translation.x - background_manager.background_origin_x as f32;
        let difference_y =
            player_entity.translation.y - background_manager.background_origin_y as f32;
        let window_width = window.width();
        let window_height = window.height();

        if difference_x.abs() > window_width as f32 {
            if difference_x > 0.0 {
                background_manager.background_origin_x += window_width as i32;

                println!("Shifting background +X");

                for mut background_transform in background_transform_query.iter_mut() {
                    background_transform.translation.x += window_width;
                }
            } else {
                background_manager.background_origin_x -= window_width as i32;

                println!("Shifting background -X");

                for mut background_transform in background_transform_query.iter_mut() {
                    background_transform.translation.x -= window_width;
                }
            }
        }
        if difference_y.abs() > window_height as f32 {
            if difference_y > 0.0 {
                background_manager.background_origin_y += window_height as i32;

                println!("Shifting background in +Y");

                for mut background_transform in background_transform_query.iter_mut() {
                    background_transform.translation.y += window_height;
                }
            } else {
                background_manager.background_origin_y -= window_height as i32;

                println!("Shifting background in -Y");

                for mut background_transform in background_transform_query.iter_mut() {
                    background_transform.translation.y -= window_height;
                }
            }
        }
    }
}
