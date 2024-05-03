use std::collections::HashMap;
use std::ops;
use bevy::{math::I16Vec2, prelude::*};
use bevy_rapier2d::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct ChunkCoordinate(I16Vec2);

impl From<I16Vec2> for ChunkCoordinate {
    fn from(i16_vec2: I16Vec2) -> Self {
        ChunkCoordinate(i16_vec2)
    }
}

impl Into<I16Vec2> for ChunkCoordinate {
    fn into(self) -> I16Vec2 {
        self.0
    }
}

impl From<EntityCoordinate> for ChunkCoordinate {
    fn from(entity_coordinate: EntityCoordinate) -> Self {
        let x = (entity_coordinate.0.x / CHUNK_SIZE as f32).floor() as i16;
        let y = (entity_coordinate.0.y / CHUNK_SIZE as f32).floor() as i16;
        ChunkCoordinate(I16Vec2::new(x, y))
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

impl Into<ChunkCoordinate> for ChunkID {
    fn into(self) -> ChunkCoordinate {
        self.0
    }
}

impl ChunkID {
    fn new(x: i16, y: i16) -> Self {
        ChunkID(ChunkCoordinate::new(x, y))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct EntityCoordinate(Vec3);

impl From<Vec3> for EntityCoordinate {
    fn from(vec3: Vec3) -> Self {
        EntityCoordinate(vec3)
    }
}

impl Into<Vec3> for EntityCoordinate {
    fn into(self) -> Vec3 {
        self.0
    }
}

impl From<ChunkCoordinate> for EntityCoordinate {
    fn from(chunk_coordinate: ChunkCoordinate) -> Self {
        let x = chunk_coordinate.0.x as f32 * CHUNK_SIZE as f32;
        let y = chunk_coordinate.0.y as f32 * CHUNK_SIZE as f32;
        EntityCoordinate(Vec3::new(x, y, CHUNK_Z_INDEX))
    }
}

impl ops::Add<EntityCoordinate> for EntityCoordinate {
    type Output = EntityCoordinate;

    fn add(self, other: EntityCoordinate) -> EntityCoordinate {
        EntityCoordinate(self.0 + other.0)
    }
}

impl ops::Sub<EntityCoordinate> for EntityCoordinate {
    type Output = EntityCoordinate;

    fn sub(self, other: EntityCoordinate) -> EntityCoordinate {
        EntityCoordinate(self.0 - other.0)
    }
}

impl ops::Mul<f32> for EntityCoordinate {
    type Output = EntityCoordinate;

    fn mul(self, scalar: f32) -> EntityCoordinate {
        EntityCoordinate(self.0 * scalar)
    }
}

impl ops::Div<f32> for EntityCoordinate {
    type Output = EntityCoordinate;

    fn div(self, scalar: f32) -> EntityCoordinate {
        EntityCoordinate(self.0 / scalar)
    }
}

impl EntityCoordinate {
    fn new(x: f32, y: f32) -> Self {
        EntityCoordinate(Vec3::new(x, y, CHUNK_Z_INDEX))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct EntityID(u64);

const CHUNK_SIZE: u16 = 128;
const CHUNK_Z_INDEX: f32 = -1.0;

#[derive(Resource)]
struct UniverseManager {
    registered_chunks: Vec<ChunkID>,
    loaded_chunks: HashMap<ChunkID, Entity>,
    current_entity_id: EntityID,
    recycled_entity_ids: Vec<EntityID>,
}

impl UniverseManager {
    fn get_unused_entity_id(&mut self) -> EntityID {
        if let Some(recycled_entity_id) = self.recycled_entity_ids.pop() {
            recycled_entity_id
        } else {
            let new_entity_id = self.current_entity_id;
            self.current_entity_id = EntityID(new_entity_id.0 + 1);

            new_entity_id
        }
    }

    fn recycle_entity_id(&mut self, entity_id: EntityID) {
        self.recycled_entity_ids.push(entity_id);
    }
}

#[derive(Component)]
struct ChunkLoader {
    load_radius: u16,
    current_chunk_ids: Vec<ChunkID>,
}

#[derive(Component)]
struct Chunk {
    id: ChunkID,
    tracked_entities: Vec<EntityID>,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct TrackedEntity {
    id: EntityID,
    current_chunk: Option<ChunkID>,
}

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
    
    // Universe manager
    commands.insert_resource(UniverseManager { 
        registered_chunks: Vec::new(), 
        loaded_chunks: HashMap::new(), 
        current_entity_id: EntityID(0), 
        recycled_entity_ids: Vec::new()
    });

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
    mut universe_manager: ResMut<UniverseManager>,
) {
    let (chunk_loader_transform, mut chunk_loader) = chunk_loader_query.single_mut();
    let chunk_loader_entity_coordinate: EntityCoordinate = chunk_loader_transform.translation.into();
    let current_chunk_coordinate: ChunkCoordinate = chunk_loader_entity_coordinate.into();
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
    for (loaded_chunk_id, _) in universe_manager.loaded_chunks.iter() {
        let loaded_chunk_coordinate: ChunkCoordinate = loaded_chunk_id.clone().into();

        if !detected_chunk_coordinates.contains(&loaded_chunk_coordinate) {
            old_chunk_ids.push(*loaded_chunk_id);
        }
    }
    for detected_chunk_coordinate in detected_chunk_coordinates {
        let detected_chunk_id: ChunkID = detected_chunk_coordinate.clone().into();
        if universe_manager.loaded_chunks.contains_key(&detected_chunk_id) {
            unchanged_chunk_ids.push(detected_chunk_id);
        } else {
            new_chunk_ids.push(detected_chunk_id);
        }
    }

    // Handle old chunks
    for old_chunk_id in old_chunk_ids {
        // "Unload" the chunk
        // TODO: Implement actual chunk unloading and entity serialization
        if let Some(loaded_chunk_entity) = universe_manager.loaded_chunks.remove(&old_chunk_id) {
            commands.entity(loaded_chunk_entity).despawn_recursive();
        }
    }

    // Handle new chunks
    for new_chunk_id in new_chunk_ids.clone() {
        if universe_manager.registered_chunks.contains(&new_chunk_id) {
            // "Load" the chunk
            // TODO: Implement actual chunk loading and entity deserialization
            let new_chunk_entity = new_chunk_entity(&mut commands, new_chunk_id);

            universe_manager.loaded_chunks.insert(new_chunk_id, new_chunk_entity);
        } else {
            // Create a new chunk
            let new_chunk_entity = new_chunk_entity(&mut commands, new_chunk_id);

            universe_manager.registered_chunks.push(new_chunk_id);
            universe_manager.loaded_chunks.insert(new_chunk_id, new_chunk_entity);
        }
    }

    // Update the current chunk IDs
    chunk_loader.current_chunk_ids = unchanged_chunk_ids;
    chunk_loader.current_chunk_ids.append(&mut new_chunk_ids);
}

fn new_chunk_entity(commands: &mut Commands, chunk_id: ChunkID) -> Entity {
    let chunk_coordinate: ChunkCoordinate = chunk_id.into();
    let chunk_entity_coordinate: EntityCoordinate = chunk_coordinate.into();

    let chunk_color = if (chunk_coordinate.0.x + chunk_coordinate.0.y) % 2 == 0 {
        Color::rgb(0.25, 0.25, 0.25)
    } else {
        Color::rgb(0.75, 0.75, 0.75)
    };

    let chunk_entity = commands.spawn((
        Chunk { id: chunk_id, tracked_entities: Vec::new()},
        SpriteBundle {
            sprite: Sprite {
                color: chunk_color,
                custom_size: Some(Vec2::new(CHUNK_SIZE as f32, CHUNK_SIZE as f32)),
                ..default()
            },
            transform: Transform::from_translation(chunk_entity_coordinate.0),
            ..default()
        },
    )).id();

    chunk_entity
}

fn tracked_entity_system(
    mut commands: Commands,
    mut tracked_entity_query: Query<(Entity, &mut TrackedEntity, &Transform)>,
    mut chunk_query: Query<(&mut Chunk)>,
    universe_manager: Res<UniverseManager>,
) {
    // If a tracked entity has no current chunk and no chunk is found, do nothing
    // If a tracked entity has no current chunk and a chunk is found, add the entity to the chunk, and set the current chunk
    // If a tracked entity has a current chunk and that same chunk is found, do nothing
    // If a tracked entity has a current chunk and a different chunk is found, remove the entity from the current chunk, add it to the new chunk, and update the current chunk
    // If a tracked entity has a current chunk and no chunk is found, remove the entity from the current chunk and destroy the entity recursively
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