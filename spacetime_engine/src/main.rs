use std::collections::HashMap;

use bevy::{math::I16Vec2, prelude::*};
use bevy_rapier2d::prelude::*;

#[derive(Resource)]
struct ChunkManager {
    chunk_size: i16,
    chunks: HashMap<I16Vec2, Entity>,
}

#[derive(Component)]
struct ChunkLoader {
    load_radius: i16,
}

#[derive(Component)]
struct Chunk {
    id: I16Vec2,  // Identifier for the chunk based on its position
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Follower {
    target: Entity,
    smoothness: f32, // Higher values mean slower following (less smooth)
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_systems(Startup, setup)
        //.add_systems(Startup, setup_grid_background)
        .add_systems(Update, player_movement)
        .add_systems(Update, chunk_loading)
        .add_systems(Update, follower_system)
        .run();
}

fn setup(mut commands: Commands) {
    // Player entity
    let player_entity = commands.spawn(Player)
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
    .insert(ChunkLoader { load_radius: 4 })
    .id();
    
    // Chunk manager
    commands.insert_resource(ChunkManager { chunks: HashMap::new(), chunk_size: 1024 });

    // Camera that follows the player
    let _camera_entity = commands.spawn(Camera2dBundle::default())
    .insert(Follower { target: player_entity, smoothness: 0.1 })
    .id();
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &Player)>,
) {
    let mut player_velocity = Vec2::new(0.0, 0.0);
    let speed = 1000.0;

    if keyboard_input.pressed(KeyCode::KeyW) {
        player_velocity.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        player_velocity.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        player_velocity.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        player_velocity.x += 1.0;
    }

    for (mut velocity, _) in query.iter_mut() {
        velocity.linvel = player_velocity.normalize_or_zero() * speed;
    }
}

fn chunk_loading(
    mut commands: Commands,
    chunk_loader_query: Query<(&Transform, &ChunkLoader)>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    let (chunk_loader_transform, chunk_loader) = chunk_loader_query.single();

    let chunk_loader_translation = chunk_loader_transform.translation.truncate();
    let chunk_size = chunk_manager.chunk_size as f32;
    let current_chunk_id = chunk_loader_translation / chunk_size;
    let current_chunk_id = I16Vec2::new(current_chunk_id.x.floor() as i16, current_chunk_id.y.floor() as i16);
    let load_radius = chunk_loader.load_radius;
    
    // Load and unload chunks
    for x_offset in -load_radius..=load_radius {
        for y_offset in -load_radius..=load_radius {
            let check_chunk_id = current_chunk_id + I16Vec2::new(x_offset, y_offset);
            let check_chunk_translation = Vec3::new(
                (check_chunk_id.x as f32 * chunk_size) + chunk_size / 2.0, 
                (check_chunk_id.y as f32 * chunk_size) + chunk_size / 2.0, 
                -1.0);

            chunk_manager.chunks.entry(check_chunk_id).or_insert_with(|| {
                let chunk_color = if (check_chunk_id.x + check_chunk_id.y) % 2 == 0 {
                    Color::rgb(0.25, 0.25, 0.25)
                } else {
                    Color::rgb(0.75, 0.75, 0.75)
                };

                let check_chunk_entity = commands.spawn((
                    Chunk { id: check_chunk_id },
                    SpriteBundle {
                        sprite: Sprite {
                            color: chunk_color,
                            custom_size: Some(Vec2::new(chunk_size, chunk_size)),
                            ..default()
                        },
                        transform: Transform::from_translation(check_chunk_translation),
                        ..default()
                    },
                )).id();

                check_chunk_entity
            });
        }
    }

    // Optionally, implement a routine to unload chunks outside of a specific range
}

fn setup_grid_background(mut commands: Commands) {
    let grid_size = 5000.0; // Extent of the grid from the center
    let line_interval = 100.0; // Spacing between grid lines

    for x in (-grid_size as i32..=grid_size as i32).step_by(line_interval as usize) {
        // Vertical lines
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(2.0, 2.0 * grid_size)),
                ..default()
            },
            transform: Transform::from_xyz(x as f32, 0.0, 1.0),
            ..default()
        });
        // Horizontal lines
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(2.0 * grid_size, 2.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, x as f32, 1.0),
            ..default()
        });
    }
}

fn follower_system(
    mut follower_query: Query<(&mut Transform, &Follower)>,
    target_query: Query<&Transform, Without<Follower>>
) {
    for (mut transform, follower) in follower_query.iter_mut() {
        if let Ok(target_transform) = target_query.get(follower.target) {
            let target_position = target_transform.translation;
            transform.translation = transform.translation.lerp(target_position, 1.0 - follower.smoothness);
        }
    }
}