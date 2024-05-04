use std::collections::HashMap;
use std::ops;
use bevy::{math::I16Vec2, prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct ChunkCoordinate(I16Vec2);

impl From<I16Vec2> for ChunkCoordinate {
    fn from(i16_vec2: I16Vec2) -> Self {
        ChunkCoordinate(i16_vec2)
    }
}

impl From<ObjectCoordinate> for ChunkCoordinate {
    fn from(object_coordinate: ObjectCoordinate) -> Self {
        let x = ((object_coordinate.0.x + CHUNK_SIZE as f32 / 2.0) / CHUNK_SIZE as f32).floor() as i16;
        let y = ((object_coordinate.0.y + CHUNK_SIZE as f32 / 2.0) / CHUNK_SIZE as f32).floor() as i16;
        ChunkCoordinate(I16Vec2::new(x, y))
    }
}

impl From<ChunkID> for ChunkCoordinate {
    fn from(chunk_id: ChunkID) -> Self {
        chunk_id.0
    }
}

impl ops::Add<ChunkCoordinate> for ChunkCoordinate {
    type Output = ChunkCoordinate;

    fn add(self, other: ChunkCoordinate) -> ChunkCoordinate {
        ChunkCoordinate(self.0 + other.0)
    }
}

impl ops::Sub<ChunkCoordinate> for ChunkCoordinate {
    type Output = ChunkCoordinate;

    fn sub(self, other: ChunkCoordinate) -> ChunkCoordinate {
        ChunkCoordinate(self.0 - other.0)
    }
}

impl ops::Mul<i16> for ChunkCoordinate {
    type Output = ChunkCoordinate;

    fn mul(self, scalar: i16) -> ChunkCoordinate {
        ChunkCoordinate(self.0 * scalar)
    }
}

impl ops::Div<i16> for ChunkCoordinate {
    type Output = ChunkCoordinate;

    fn div(self, scalar: i16) -> ChunkCoordinate {
        ChunkCoordinate(self.0 / scalar)
    }
}

impl ChunkCoordinate {
    fn new(x: i16, y: i16) -> Self {
        ChunkCoordinate(I16Vec2::new(x, y))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct ChunkID(ChunkCoordinate);

impl From<ChunkCoordinate> for ChunkID {
    fn from(chunk_coordinate: ChunkCoordinate) -> Self {
        ChunkID(chunk_coordinate)
    }
}

impl ChunkID {
    fn new(x: i16, y: i16) -> Self {
        ChunkID(ChunkCoordinate::new(x, y))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ObjectCoordinate(Vec3);

impl From<Vec2> for ObjectCoordinate {
    fn from(vec2: Vec2) -> Self {
        ObjectCoordinate(Vec3::new(vec2.x, vec2.y, 0.0))
    }
}

impl From<Vec3> for ObjectCoordinate {
    fn from(vec3: Vec3) -> Self {
        ObjectCoordinate(vec3)
    }
}

impl From<ChunkCoordinate> for ObjectCoordinate {
    fn from(chunk_coordinate: ChunkCoordinate) -> Self {
        let x = chunk_coordinate.0.x as f32 * CHUNK_SIZE as f32;
        let y = chunk_coordinate.0.y as f32 * CHUNK_SIZE as f32;
        ObjectCoordinate(Vec3::new(x, y, CHUNK_Z_INDEX))
    }
}

impl ops::Add<ObjectCoordinate> for ObjectCoordinate {
    type Output = ObjectCoordinate;

    fn add(self, other: ObjectCoordinate) -> ObjectCoordinate {
        ObjectCoordinate(self.0 + other.0)
    }
}

impl ops::Sub<ObjectCoordinate> for ObjectCoordinate {
    type Output = ObjectCoordinate;

    fn sub(self, other: ObjectCoordinate) -> ObjectCoordinate {
        ObjectCoordinate(self.0 - other.0)
    }
}

impl ops::Mul<f32> for ObjectCoordinate {
    type Output = ObjectCoordinate;

    fn mul(self, scalar: f32) -> ObjectCoordinate {
        ObjectCoordinate(self.0 * scalar)
    }
}

impl ops::Div<f32> for ObjectCoordinate {
    type Output = ObjectCoordinate;

    fn div(self, scalar: f32) -> ObjectCoordinate {
        ObjectCoordinate(self.0 / scalar)
    }
}

impl ObjectCoordinate {
    fn new(x: f32, y: f32) -> Self {
        ObjectCoordinate(Vec3::new(x, y, CHUNK_Z_INDEX))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct ChunkActorID(u64);

const CHUNK_SIZE: u16 = 128;
const CHUNK_Z_INDEX: f32 = -1.0;

const PLAYER_MOVEMENT_SPEED: f32 = 1000.0;
const PLAYER_CREATIVE_SQUARE_PROP_SIZE: f32 = 50.0;

#[derive(Resource)]
struct ChunkManager {
    registered_chunks: Vec<ChunkID>,
    loaded_chunks: HashMap<ChunkID, Entity>,
    current_chunk_actor_id: ChunkActorID,
    recycled_chunk_actor_ids: Vec<ChunkActorID>,
}

impl ChunkManager {
    fn get_unused_chunk_actor_id(&mut self) -> ChunkActorID {
        if let Some(recycled_chunk_actor_id) = self.recycled_chunk_actor_ids.pop() {
            recycled_chunk_actor_id
        } else {
            let new_chunk_actor_id = self.current_chunk_actor_id;
            self.current_chunk_actor_id = ChunkActorID(new_chunk_actor_id.0 + 1);

            new_chunk_actor_id
        }
    }

    fn recycle_chunk_actor_id(&mut self, chunk_actor_id: ChunkActorID) {
        self.recycled_chunk_actor_ids.push(chunk_actor_id);
    }
}

#[derive(Component)]
struct Chunk {
    id: ChunkID,
    chunk_actors: Vec<ChunkActorID>,
}

#[derive(Component)]
struct ChunkLoader {
    load_radius: u16,
    current_chunk_ids: Vec<ChunkID>,
}

#[derive(Component)]
struct ChunkActor {
    id: ChunkActorID,
    current_chunk: ChunkID,
}

#[derive(Component)]
struct TranslationLerpFollower {
    target: Entity,
    smoothness: f32, // Higher values mean slower following (less smooth)
}

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_systems(Startup, main_setup_system)
        .add_systems(Update, chunk_loader_system)
        .add_systems(Update, chunk_actor_system)
        .add_systems(Update, player_movement_system)
        .add_systems(Update, player_creative_system)
        .add_systems(Update, translation_lerp_follower_system)
        .run();
}

fn main_setup_system(mut commands: Commands, mut rapier_configuration: ResMut<RapierConfiguration>) {
    // Rapier Configuration
    rapier_configuration.gravity = Vec2::new(0.0, 0.0);
    
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
    
    // Universe manager
    commands.insert_resource(ChunkManager { 
        registered_chunks: Vec::new(), 
        loaded_chunks: HashMap::new(), 
        current_chunk_actor_id: ChunkActorID(0), 
        recycled_chunk_actor_ids: Vec::new()
    });

    // Camera that follows the player
    let _camera_entity = commands.spawn(Camera2dBundle::default())
    .insert(TranslationLerpFollower { target: player_entity, smoothness: 0.1 })
    .id();
}

fn new_chunk_entity(commands: &mut Commands, chunk_id: ChunkID) -> Entity {
    let chunk_coordinate: ChunkCoordinate = chunk_id.into();
    let chunk_object_coordinate: ObjectCoordinate = chunk_coordinate.into();

    let chunk_color = if (chunk_coordinate.0.x + chunk_coordinate.0.y) % 2 == 0 {
        Color::rgb(0.25, 0.25, 0.25)
    } else {
        Color::rgb(0.75, 0.75, 0.75)
    };

    let chunk_entity = commands.spawn((
        Chunk { id: chunk_id, chunk_actors: Vec::new()},
        SpriteBundle {
            sprite: Sprite {
                color: chunk_color,
                custom_size: Some(Vec2::new(CHUNK_SIZE as f32, CHUNK_SIZE as f32)),
                ..default()
            },
            transform: Transform::from_translation(chunk_object_coordinate.0),
            ..default()
        },
    )).id();

    chunk_entity
}

fn chunk_loader_system(
    mut commands: Commands,
    mut chunk_loader_query: Query<(&Transform, &mut ChunkLoader)>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    let (chunk_loader_transform, mut chunk_loader) = chunk_loader_query.single_mut();
    let chunk_loader_object_coordinate: ObjectCoordinate = chunk_loader_transform.translation.into();
    let current_chunk_coordinate: ChunkCoordinate = chunk_loader_object_coordinate.into();
    let load_radius = chunk_loader.load_radius as i16;
    
    // Detect chunks around the player
    let mut detected_chunk_coordinates = Vec::new();
    for x_offset in -load_radius..=load_radius {
        for y_offset in -load_radius..=load_radius {
            detected_chunk_coordinates.push(current_chunk_coordinate + ChunkCoordinate::new(x_offset, y_offset));
        }
    }

    // Categorize the detected chunks
    let mut old_chunk_ids: Vec<ChunkID> = Vec::new(); // Chunks which are active, but have not been detected
    let mut unchanged_chunk_ids: Vec<ChunkID> = Vec::new(); // Chunks which are active and have been detected
    let mut new_chunk_ids: Vec<ChunkID> = Vec::new(); // Chunks which are not active but have been detected
    for (loaded_chunk_id, _) in chunk_manager.loaded_chunks.iter() {
        let loaded_chunk_coordinate: ChunkCoordinate = loaded_chunk_id.clone().into();

        if !detected_chunk_coordinates.contains(&loaded_chunk_coordinate) {
            old_chunk_ids.push(*loaded_chunk_id);
        }
    }
    for detected_chunk_coordinate in detected_chunk_coordinates {
        let detected_chunk_id: ChunkID = detected_chunk_coordinate.clone().into();
        if chunk_manager.loaded_chunks.contains_key(&detected_chunk_id) {
            unchanged_chunk_ids.push(detected_chunk_id);
        } else {
            new_chunk_ids.push(detected_chunk_id);
        }
    }

    // Handle old chunks
    for old_chunk_id in old_chunk_ids {
        // "Unload" the chunk
        // TODO: Implement actual chunk unloading and entity serialization
        if let Some(loaded_chunk_entity) = chunk_manager.loaded_chunks.remove(&old_chunk_id) {
            commands.entity(loaded_chunk_entity).despawn_recursive();
        }
    }

    // Handle new chunks
    for new_chunk_id in new_chunk_ids.clone() {
        if chunk_manager.registered_chunks.contains(&new_chunk_id) {
            // "Load" the chunk
            // TODO: Implement actual chunk loading and entity deserialization
            let new_chunk_entity = new_chunk_entity(&mut commands, new_chunk_id);

            chunk_manager.loaded_chunks.insert(new_chunk_id, new_chunk_entity);
        } else {
            // Create a new chunk
            let new_chunk_entity = new_chunk_entity(&mut commands, new_chunk_id);

            chunk_manager.registered_chunks.push(new_chunk_id);
            chunk_manager.loaded_chunks.insert(new_chunk_id, new_chunk_entity);
        }
    }

    // Update the current chunk IDs
    chunk_loader.current_chunk_ids = unchanged_chunk_ids;
    chunk_loader.current_chunk_ids.append(&mut new_chunk_ids);
}

fn chunk_actor_system(
    mut commands: Commands,
    mut chunk_actor_query: Query<(Entity, &Transform, &mut ChunkActor)>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    for (chunk_actor_entity, chunk_actor_transform, mut chunk_actor) in chunk_actor_query.iter_mut() {
        let chunk_object_coordinate: ObjectCoordinate = chunk_actor_transform.translation.into();
        let chunk_coordinate: ChunkCoordinate = chunk_object_coordinate.into();
        let chunk_id: ChunkID = chunk_coordinate.into();

        if !chunk_manager.loaded_chunks.contains_key(&chunk_id) {
            chunk_manager.recycle_chunk_actor_id(chunk_actor.id);
            commands.entity(chunk_actor_entity).despawn_recursive();
            continue;
        }

        if chunk_id != chunk_actor.current_chunk {
            chunk_actor.current_chunk = chunk_id;
        }
    }
}

fn translation_lerp_follower_system(
    mut translation_lerp_follower_query: Query<(&mut Transform, &TranslationLerpFollower)>,
    target_query: Query<&Transform, Without<TranslationLerpFollower>>
) {
    for (mut transform, translation_lerp_follower) in translation_lerp_follower_query.iter_mut() {
        if let Ok(target_transform) = target_query.get(translation_lerp_follower.target) {
            let target_position = target_transform.translation;
            transform.translation = transform.translation.lerp(target_position, 1.0 - translation_lerp_follower.smoothness);
        }
    }
}

fn player_movement_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &Player)>,
) {
    let mut player_velocity = Vec2::new(0.0, 0.0);

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
        velocity.linvel = player_velocity.normalize_or_zero() * PLAYER_MOVEMENT_SPEED;
    }
}

fn player_creative_system(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    collider_query: Query<(Entity, &Collider, &Transform)>,
    mut chunk_manager: ResMut<ChunkManager>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    if let Ok(window) = window_query.get_single() {
        if let Some(cursor_pos) = window.cursor_position() {
            // Adjust for the y-coordinate to correctly map from screen to NDC
            let window_size = Vec2::new(window.width(), window.height());
            let cursor_pos_ndc = Vec2::new(
                (cursor_pos.x / window_size.x) * 2.0 - 1.0, 
                1.0 - (cursor_pos.y / window_size.y) * 2.0
            );

            if let Ok((camera, camera_transform)) = camera_query.get_single() {
                let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
                let world_pos = ndc_to_world.project_point3(cursor_pos_ndc.extend(-1.0)).truncate();
                let chunk_object_coordinate: ObjectCoordinate = world_pos.into();
                let chunk_coordinate: ChunkCoordinate = chunk_object_coordinate.into();
                let chunk_id: ChunkID = chunk_coordinate.into();
                let half_prop_size = PLAYER_CREATIVE_SQUARE_PROP_SIZE / 2.0;

                // Place a new prop on right click
                if mouse_button_input.just_pressed(MouseButton::Right) {
                    commands.spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(0.5, 0.5, 1.0),
                            custom_size: Some(Vec2::splat(PLAYER_CREATIVE_SQUARE_PROP_SIZE)),
                            ..default()
                        },
                        transform: Transform::from_translation(world_pos.extend(0.0)),
                        ..default()
                    })
                    .insert(RigidBody::Dynamic)
                    .insert(Collider::cuboid(half_prop_size, half_prop_size))
                    .insert(ChunkActor { id: chunk_manager.get_unused_chunk_actor_id(), current_chunk: chunk_id });
                }

                // Delete props under the cursor on left click
                if mouse_button_input.just_pressed(MouseButton::Left) {
                    for (entity, _, transform) in collider_query.iter() {
                        let collider_position = transform.translation.truncate();

                        if (collider_position - world_pos).abs().max_element() < PLAYER_CREATIVE_SQUARE_PROP_SIZE / 2.0 {
                            commands.entity(entity).despawn_recursive();
                        }
                    }
                }
            }
        }
    }
}
