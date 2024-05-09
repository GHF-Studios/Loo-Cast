extern crate spacetime_engine;

use std::collections::HashMap;
use std::collections::HashSet;
use std::ops;
use bevy::ecs::system::SystemState;
use bevy::scene::ron;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy::scene::serde::{SceneDeserializer, SceneSerializer};
use bevy_rapier2d::prelude::*;
use spacetime_engine::SpacetimeEnginePlugins;


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect)]
struct ChunkCoordinate(I16Vec2);

impl From<I16Vec2> for ChunkCoordinate {
    fn from(i16_vec2: I16Vec2) -> Self {
        ChunkCoordinate(i16_vec2)
    }
}

impl From<ChunkActorCoordinate> for ChunkCoordinate {
    fn from(chunk_actor_coordinate: ChunkActorCoordinate) -> Self {
        let x = ((chunk_actor_coordinate.0.x + CHUNK_SIZE as f32 / 2.0) / CHUNK_SIZE as f32).floor() as i16;
        let y = ((chunk_actor_coordinate.0.y + CHUNK_SIZE as f32 / 2.0) / CHUNK_SIZE as f32).floor() as i16;
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect)]
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

#[derive(Clone, Copy, Debug, PartialEq, Reflect)]
struct ChunkActorCoordinate(Vec3);

impl From<Vec2> for ChunkActorCoordinate {
    fn from(vec2: Vec2) -> Self {
        ChunkActorCoordinate(Vec3::new(vec2.x, vec2.y, 0.0))
    }
}

impl From<Vec3> for ChunkActorCoordinate {
    fn from(vec3: Vec3) -> Self {
        ChunkActorCoordinate(vec3)
    }
}

impl From<ChunkCoordinate> for ChunkActorCoordinate {
    fn from(chunk_coordinate: ChunkCoordinate) -> Self {
        let x = chunk_coordinate.0.0 as f32 * CHUNK_SIZE as f32;
        let y = chunk_coordinate.0.1 as f32 * CHUNK_SIZE as f32;
        ChunkActorCoordinate(Vec3::new(x, y, CHUNK_Z_INDEX))
    }
}

impl ops::Add<ChunkActorCoordinate> for ChunkActorCoordinate {
    type Output = ChunkActorCoordinate;

    fn add(self, other: ChunkActorCoordinate) -> ChunkActorCoordinate {
        ChunkActorCoordinate(self.0 + other.0)
    }
}

impl ops::Sub<ChunkActorCoordinate> for ChunkActorCoordinate {
    type Output = ChunkActorCoordinate;

    fn sub(self, other: ChunkActorCoordinate) -> ChunkActorCoordinate {
        ChunkActorCoordinate(self.0 - other.0)
    }
}

impl ops::Mul<f32> for ChunkActorCoordinate {
    type Output = ChunkActorCoordinate;

    fn mul(self, scalar: f32) -> ChunkActorCoordinate {
        ChunkActorCoordinate(self.0 * scalar)
    }
}

impl ops::Div<f32> for ChunkActorCoordinate {
    type Output = ChunkActorCoordinate;

    fn div(self, scalar: f32) -> ChunkActorCoordinate {
        ChunkActorCoordinate(self.0 / scalar)
    }
}

impl ChunkActorCoordinate {
    fn new(x: f32, y: f32) -> Self {
        ChunkActorCoordinate(Vec3::new(x, y, CHUNK_Z_INDEX))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect)]
struct ChunkActorID(u64);

impl From<u64> for ChunkActorID {
    fn from(id: u64) -> Self {
        ChunkActorID(id)
    }
}

impl From<ChunkActorID> for u64 {
    fn from(chunk_actor_id: ChunkActorID) -> Self {
        chunk_actor_id.0
    }
}



#[derive(Resource)]
struct ChunkManager {
    registered_chunks: HashSet<ChunkID>,
    loaded_chunks: HashMap<ChunkID, Entity>,
    serialized_chunks: HashMap<ChunkID, String>,
    creating_chunks: HashSet<ChunkID>,
    destroying_chunks: HashSet<ChunkID>,
    loading_chunks: HashSet<ChunkID>,
    unloading_chunks: HashSet<ChunkID>,
    registered_chunk_actors: HashSet<ChunkActorID>,
    loaded_chunk_actors: HashMap<ChunkActorID, Entity>,
    creating_chunk_actors: HashSet<ChunkActorID>,
    destroying_chunk_actors: HashSet<ChunkActorID>,
    loading_chunk_actors: HashSet<ChunkActorID>,
    unloading_chunk_actors: HashSet<ChunkActorID>,
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

#[derive(Component, Reflect)]
struct Chunk {
    id: ChunkID,
    chunk_actors: Vec<ChunkActorID>,
}

#[derive(Component, Reflect)]
struct ChunkActor {
    id: ChunkActorID,
    current_chunk: ChunkID,
}

#[derive(Component, Reflect)]
struct ChunkLoader {
    load_radius: u16,
    current_chunk_ids: Vec<ChunkID>,
}



#[derive(Clone, Event)]
struct CreateChunk(ChunkID);

#[derive(Clone, Event)]
struct DestroyChunk(ChunkID);

#[derive(Clone, Event)]
struct LoadChunk(ChunkID);

#[derive(Clone, Event)]
struct UnloadChunk(ChunkID);

#[derive(Clone, Event)]
struct CreateChunkActor(ChunkActorID);

#[derive(Clone, Event)]
struct DestroyChunkActor(ChunkActorID);

#[derive(Clone, Event)]
struct LoadChunkActor(ChunkActorID);

#[derive(Clone, Event)]
struct UnloadChunkActor(ChunkActorID);

// TODO: Fix chunk loading by implementing serializable proxies for all necessary rapier components (necessary as of now)
// TODO: Implement sub-chunking/fields
// TODO: Implement gravity via sub-chunking/fields
// TODO: Implement electromagnetism via sub-chunking/fields
// TODO: Implement planets via gravity
// TODO: Implement magnets via electromagnetism
// TODO: Implement stars via gravity and electromagnetism

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(SpacetimeEnginePlugins)
        .run();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_event::<CreateChunk>()
        .add_event::<DestroyChunk>()
        .add_event::<LoadChunk>()
        .add_event::<UnloadChunk>()
        .add_event::<CreateChunkActor>()
        .add_event::<DestroyChunkActor>()
        .add_event::<LoadChunkActor>()
        .add_event::<UnloadChunkActor>()
        .add_systems(Startup, main_setup_system)
        .add_systems(Update, chunk_loader_change_radius_system)
        .add_systems(Update, chunk_loader_system)
        .add_systems(Update, handle_create_chunk_events_system)
        .add_systems(Update, handle_destroy_chunk_events_system)
        .add_systems(Update, handle_load_chunk_events_system)
        .add_systems(Update, handle_unload_chunk_events_system)
        .add_systems(Update, chunk_actor_system)
        .run();
}

fn main_setup_system(mut commands: Commands, mut rapier_configuration: ResMut<RapierConfiguration>) {
    // Rapier Configuration
    rapier_configuration.gravity = Vec2::new(0.0, 0.0);
    
    // Chunk manager
    commands.insert_resource(ChunkManager {
        registered_chunks: HashSet::new(),
        loaded_chunks: HashMap::new(),
        serialized_chunks: HashMap::new(),
        creating_chunks: HashSet::new(),
        destroying_chunks: HashSet::new(),
        loading_chunks: HashSet::new(),
        unloading_chunks: HashSet::new(),
        registered_chunk_actors: HashSet::new(),
        loaded_chunk_actors: HashMap::new(),
        creating_chunk_actors: HashSet::new(),
        destroying_chunk_actors: HashSet::new(),
        loading_chunk_actors: HashSet::new(),
        unloading_chunk_actors: HashSet::new(),
        current_chunk_actor_id: ChunkActorID(0),
        recycled_chunk_actor_ids: Vec::new(),
    });

    // Camera entity
    let _camera_entity = commands.spawn(Camera2dBundle::default())
    .insert(TranslationLerpFollower { target: player_entity, smoothness: 0.1 })
    .id();
}

fn chunk_loader_change_radius_system(
    mut chunk_loader_query: Query<(&mut ChunkLoader, &Player)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for (mut chunk_loader, _) in chunk_loader_query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::KeyQ) {
            chunk_loader.load_radius = (chunk_loader.load_radius as i16 - 1).max(0) as u16;
        }
        if keyboard_input.just_pressed(KeyCode::KeyE) {
            chunk_loader.load_radius = (chunk_loader.load_radius as i16 + 1) as u16;
        }
    }
}

fn chunk_loader_system(
    mut create_chunk_event_writer: EventWriter<CreateChunk>,
    mut destroy_chunk_event_writer: EventWriter<DestroyChunk>,
    mut load_chunk_event_writer: EventWriter<LoadChunk>,
    mut unload_chunk_event_writer: EventWriter<UnloadChunk>,
    mut chunk_loader_query: Query<(&Transform, &mut ChunkLoader)>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    let (chunk_loader_transform, mut chunk_loader) = chunk_loader_query.single_mut();
    let chunk_loader_chunk_actor_coordinate: ChunkActorCoordinate = chunk_loader_transform.translation.into();
    let current_chunk_coordinate: ChunkCoordinate = chunk_loader_chunk_actor_coordinate.into();
    let load_radius = chunk_loader.load_radius as i16;
    
    // Detect chunks around the player
    let mut detected_chunk_coordinates = Vec::new();
    for x_offset in -load_radius..=load_radius {
        for y_offset in -load_radius..=load_radius {
            detected_chunk_coordinates.push(current_chunk_coordinate + ChunkCoordinate::new(x_offset, y_offset));
        }
    }

    // Categorize the detected chunks
    let mut old_chunk_ids: Vec<ChunkID> = Vec::new();
    let mut unchanged_chunk_ids: Vec<ChunkID> = Vec::new();
    let mut new_chunk_ids: Vec<ChunkID> = Vec::new();
    for (loaded_chunk_id, _) in chunk_manager.loaded_chunks.iter() {
        let loaded_chunk_coordinate: ChunkCoordinate = loaded_chunk_id.0;

        if !detected_chunk_coordinates.contains(&loaded_chunk_coordinate) {
            old_chunk_ids.push(*loaded_chunk_id);
        }
    }
    for detected_chunk_coordinate in detected_chunk_coordinates {
        let detected_chunk_id: ChunkID = detected_chunk_coordinate.into();
        if chunk_manager.loaded_chunks.contains_key(&detected_chunk_id) {
            unchanged_chunk_ids.push(detected_chunk_id);
        } else {
            new_chunk_ids.push(detected_chunk_id);
        }
    }

    // Handle old chunks
    for old_chunk_id in old_chunk_ids {
        if !chunk_manager.unloading_chunks.contains(&old_chunk_id) {
            chunk_manager.unloading_chunks.insert(old_chunk_id);
            unload_chunk_event_writer.send(UnloadChunk(old_chunk_id));
        }
    }

    // Handle new chunks
    for new_chunk_id in new_chunk_ids.clone() {
        if chunk_manager.registered_chunks.contains(&new_chunk_id) {
            if !chunk_manager.loading_chunks.contains(&new_chunk_id) {
                chunk_manager.loading_chunks.insert(new_chunk_id);
                load_chunk_event_writer.send(LoadChunk(new_chunk_id));
            }
        } else if !chunk_manager.creating_chunks.contains(&new_chunk_id) {
            chunk_manager.creating_chunks.insert(new_chunk_id);
            create_chunk_event_writer.send(CreateChunk(new_chunk_id));
        }
    }

    // Update the current chunk IDs
    chunk_loader.current_chunk_ids = unchanged_chunk_ids;
    chunk_loader.current_chunk_ids.append(&mut new_chunk_ids);
}

fn handle_create_chunk_events_system(
    mut commands: Commands,
    mut create_chunk_event_reader: EventReader<CreateChunk>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    for create_chunk_event in create_chunk_event_reader.read() {
        let new_chunk_entity = new_chunk_entity(&mut commands, create_chunk_event.0);

        chunk_manager.registered_chunks.insert(create_chunk_event.0);
        chunk_manager.loaded_chunks.insert(create_chunk_event.0, new_chunk_entity);
    }
}

fn handle_destroy_chunk_events_system(
    mut commands: Commands,
    mut destroy_chunk_event_reader: EventReader<DestroyChunk>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    for destroy_chunk_event in destroy_chunk_event_reader.read() {
        chunk_manager.registered_chunks.remove(&destroy_chunk_event.0);

        if let Some(loaded_chunk_entity) = chunk_manager.loaded_chunks.remove(&destroy_chunk_event.0) {
            commands.entity(loaded_chunk_entity).despawn_recursive();
        }
    }
}

fn handle_load_chunk_events_system(
    world: &mut World,
    params: &mut SystemState<(
        EventReader<LoadChunk>,
        ResMut<ChunkManager>,
    )>
) {
    let (mut load_chunk_event_reader, _) = params.get_mut(world);
    let mut load_chunk_events: Vec<LoadChunk> = Vec::new();
    for load_chunk_event in load_chunk_event_reader.read() {
        load_chunk_events.push(load_chunk_event.clone());
    }

    for load_chunk_event in load_chunk_events {
        let (_, mut chunk_manager) = params.get_mut(world);
        let serialized = chunk_manager.serialized_chunks.remove(&load_chunk_event.0).unwrap();

        let dyn_scene = {
            let type_registry_rwlock = &world.resource::<AppTypeRegistry>().0.read();
        
            let deserializer = SceneDeserializer {
                type_registry: type_registry_rwlock,
            };
        
            let mut ron_deserializer = ron::de::Deserializer::from_str(&serialized).unwrap();

            use serde::de::DeserializeSeed;
        
            deserializer.deserialize(&mut ron_deserializer).unwrap()
        };

        dyn_scene
            .write_to_world(world, &mut default())
            .unwrap();

        println!("# of loading entities: {}", dyn_scene.entities.len());

        for entity in &dyn_scene.entities {
            println!("Deserialized Entity: {:?}", entity.entity);
    
            for component in &entity.components {
                let component_reflect: &dyn Reflect = component.as_reflect();
                let type_name = component_reflect.type_id();
                
                println!("Component Type: {:?}", type_name);
            }
        }

        let chunk_entity = dyn_scene.entities.last().unwrap().entity;

        println!("Chunk Entity: {:?}", chunk_entity);

        let (_, mut chunk_manager) = params.get_mut(world);
        chunk_manager.loading_chunks.remove(&load_chunk_event.0);
        chunk_manager.registered_chunks.insert(load_chunk_event.0);
        chunk_manager.loaded_chunks.insert(load_chunk_event.0, chunk_entity);
    }
}

fn handle_unload_chunk_events_system(
    world: &mut World,
    params: &mut SystemState<(
        EventReader<UnloadChunk>,
        ResMut<ChunkManager>,
    )>
) {
    let (mut unload_chunk_event_reader, _) = params.get_mut(world);
    let mut unload_chunk_events: Vec<UnloadChunk> = Vec::new();
    for unload_chunk_event in unload_chunk_event_reader.read() {
        unload_chunk_events.push(unload_chunk_event.clone());
    }

    for unload_chunk_event in unload_chunk_events {
        let mut chunk_actor_entities = world
            .query::<(Entity, &ChunkActor)>()
            .iter(world)
            .filter(|(_, chunk_actor)| chunk_actor.current_chunk == unload_chunk_event.0)
            .map(|(entity, _)| entity)
            .collect::<Vec<_>>();

        chunk_actor_entities.retain(|entity| {
            match world.get_entity_mut(*entity) {
                Some(_) => {
                    true
                },
                None => {
                    println!("Tried to unload non-existent entity!");
                    
                    false
                },
            }
        });

        let (_, chunk_manager) = params.get_mut(world);

        let chunk_entity = match chunk_manager.loaded_chunks.get(&unload_chunk_event.0) {
            Some(chunk_entity) => *chunk_entity,
            None => continue,
        };

        println!("Checking integrity of chunk entity...");
        match world.get_entity(chunk_entity) {
            Some(_) => {
                println!("Chunk Entity '{:?}' exists!", chunk_entity);
            },
            None => {
                panic!("Chunk Entity '{:?}' does not exist!", chunk_entity);
            },
        }

        chunk_actor_entities.push(chunk_entity);
        let all_entities = chunk_actor_entities;
        
        let mut builder = DynamicSceneBuilder::from_world(world);
        
        builder = builder.extract_entities(all_entities.clone().into_iter());

        let dyn_scene = builder.build();
        let type_registry_arc = &world.resource::<AppTypeRegistry>().0;
        let serializer = SceneSerializer::new(&dyn_scene, type_registry_arc);
        let serialized = ron::to_string(&serializer).unwrap();

        for entity in all_entities {
            world.entity_mut(entity).despawn_recursive();
        }

        let (_, mut chunk_manager) = params.get_mut(world);
        chunk_manager.serialized_chunks.insert(unload_chunk_event.0, serialized);
        chunk_manager.loaded_chunks.remove(&unload_chunk_event.0);
        chunk_manager.unloading_chunks.remove(&unload_chunk_event.0);
    }
}

fn new_chunk_entity(commands: &mut Commands, chunk_id: ChunkID) -> Entity {
    let chunk_coordinate: ChunkCoordinate = chunk_id.into();
    let chunk_chunk_actor_coordinate: ChunkActorCoordinate = chunk_coordinate.into();

    let chunk_color = if (chunk_coordinate.0.0 + chunk_coordinate.0.1) % 2 == 0 {
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
            transform: Transform::from_translation(chunk_chunk_actor_coordinate.0),
            ..default()
        },
    )).id();

    chunk_entity
}

fn chunk_actor_system(
    mut commands: Commands,
    mut chunk_actor_query: Query<(Entity, &Transform, &mut ChunkActor)>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    for (chunk_actor_entity, chunk_actor_transform, mut chunk_actor) in chunk_actor_query.iter_mut() {
        let chunk_chunk_actor_coordinate: ChunkActorCoordinate = chunk_actor_transform.translation.into();
        let chunk_coordinate: ChunkCoordinate = chunk_chunk_actor_coordinate.into();
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

fn handle_create_chunk_actor_events_system(
    mut commands: Commands,
    mut create_chunk_actor_event_reader: EventReader<CreateChunkActor>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    // TODO: Implement
}

fn handle_destroy_chunk_actor_events_system(
    mut commands: Commands,
    mut destroy_chunk_actor_event_reader: EventReader<DestroyChunkActor>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    // TODO: Implement
}

fn handle_load_chunk_actor_events_system(
    mut commands: Commands,
    mut load_chunk_actor_event_reader: EventReader<LoadChunkActor>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    // TODO: Implement
}

fn handle_unload_chunk_actor_events_system(
    mut commands: Commands,
    mut unload_chunk_actor_event_reader: EventReader<UnloadChunkActor>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    // TODO: Implement
}





