use std::collections::HashMap;

use bevy::{math::I16Vec2, prelude::*};
use bevy_rapier2d::prelude::*;

#[derive(Resource)]
struct ChunkManager {
    chunk_size: i16,
    registered_chunks: Vec<I16Vec2>,
    loaded_chunks: HashMap<I16Vec2, Entity>,
}

#[derive(Component)]
struct ChunkLoader {
    load_radius: i16,
    current_chunk_ids: Vec<I16Vec2>,
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
        .add_systems(Startup, main_setup_system)
        .add_systems(Update, player_movement_system)
        .add_systems(Update, chunk_loader_system)
        .add_systems(Update, follower_system)
        .run();
}

fn main_setup_system(mut commands: Commands) {
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
    .insert(ChunkLoader { load_radius: 4, current_chunk_ids: Vec::new() })
    .id();
    
    // Chunk manager
    commands.insert_resource(ChunkManager { chunk_size: 64, registered_chunks: Vec::new(), loaded_chunks: HashMap::new() });

    // Camera that follows the player
    let _camera_entity = commands.spawn(Camera2dBundle::default())
    .insert(Follower { target: player_entity, smoothness: 0.1 })
    .id();
}

fn player_movement_system(
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

fn chunk_loader_system(
    mut commands: Commands,
    mut chunk_loader_query: Query<(&Transform, &mut ChunkLoader)>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    let (chunk_loader_transform, mut chunk_loader) = chunk_loader_query.single_mut();
    let chunk_loader_translation = chunk_loader_transform.translation.truncate();
    let chunk_size = chunk_manager.chunk_size as f32;
    let current_chunk_id = chunk_loader_translation / chunk_size;
    let current_chunk_id = I16Vec2::new(current_chunk_id.x.floor() as i16, current_chunk_id.y.floor() as i16);
    let load_radius = chunk_loader.load_radius;
    
    // Detect chunks around the player
    let mut detected_chunk_ids: Vec<I16Vec2> = Vec::new();
    for x_offset in -load_radius..=load_radius {
        for y_offset in -load_radius..=load_radius {
            detected_chunk_ids.push(current_chunk_id + I16Vec2::new(x_offset, y_offset));
        }
    }

    // Categorize the detected chunks
    let mut old_chunk_ids: Vec<I16Vec2> = Vec::new(); // Chunks which are active, but have not been detected
    let mut unchanged_chunk_ids: Vec<I16Vec2> = Vec::new(); // Chunks which are active and have been detected
    let mut new_chunk_ids: Vec<I16Vec2> = Vec::new(); // Chunks which are not active but have been detected
    for (loaded_chunk_id, _) in chunk_manager.loaded_chunks.iter() {
        if !detected_chunk_ids.contains(loaded_chunk_id) {
            old_chunk_ids.push(*loaded_chunk_id);
        }
    }
    for detected_chunk_id in detected_chunk_ids {
        if chunk_manager.loaded_chunks.contains_key(&detected_chunk_id) {
            unchanged_chunk_ids.push(detected_chunk_id);
        } else {
            new_chunk_ids.push(detected_chunk_id);
        }
    }

    // Handle old chunks
    for old_chunk_id in old_chunk_ids {
        // "Unload" the chunk
        // TODO: Implement actual chunk unloading
        if let Some(loaded_chunk_entity) = chunk_manager.loaded_chunks.remove(&old_chunk_id) {
            commands.entity(loaded_chunk_entity).despawn_recursive();
        }
    }

    // Handle new chunks
    for new_chunk_id in new_chunk_ids.clone() {
        if chunk_manager.registered_chunks.contains(&new_chunk_id) {
            // "Load" the chunk
            // TODO: Implement actual chunk loading
            let new_chunk_entity = new_chunk_entity(&mut commands, &mut chunk_manager, new_chunk_id);

            chunk_manager.loaded_chunks.insert(new_chunk_id, new_chunk_entity);
        } else {
            // Create a new chunk
            let new_chunk_entity = new_chunk_entity(&mut commands, &mut chunk_manager, new_chunk_id);

            chunk_manager.registered_chunks.push(new_chunk_id);
            chunk_manager.loaded_chunks.insert(new_chunk_id, new_chunk_entity);
        }
    }

    // Update the current chunk IDs
    chunk_loader.current_chunk_ids = unchanged_chunk_ids;
    chunk_loader.current_chunk_ids.append(&mut new_chunk_ids);
}

fn new_chunk_entity(commands: &mut Commands, chunk_manager: &mut ResMut<ChunkManager>, chunk_id: I16Vec2) -> Entity {
    let chunk_size = chunk_manager.chunk_size as f32;
    let chunk_translation = Vec3::new(
        (chunk_id.x as f32 * chunk_size) + chunk_size / 2.0, 
        (chunk_id.y as f32 * chunk_size) + chunk_size / 2.0, 
        -1.0);

    let chunk_color = if (chunk_id.x + chunk_id.y) % 2 == 0 {
        Color::rgb(0.25, 0.25, 0.25)
    } else {
        Color::rgb(0.75, 0.75, 0.75)
    };

    let chunk_entity = commands.spawn((
        Chunk { id: chunk_id },
        SpriteBundle {
            sprite: Sprite {
                color: chunk_color,
                custom_size: Some(Vec2::new(chunk_size, chunk_size)),
                ..default()
            },
            transform: Transform::from_translation(chunk_translation),
            ..default()
        },
    )).id();

    chunk_entity
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