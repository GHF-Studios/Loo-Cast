// Modules

// Local imports

// Internal imports
use crate::camera::MainCamera;
use crate::game::SimulationState;
use crate::universe::chunk::id::*;
use crate::universe::chunk::pos::*;
use crate::universe::entity::*;
use crate::universe::entity::data::*;
use crate::universe::entity::metadata::*;
use crate::universe::entity::pos::*;
use crate::universe::global::*;
use crate::universe::local::*;
use crate::universe::local::id::*;
use crate::universe::*;
use crate::AppState;

// External imports
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

// Static variables

// Constant variables
pub const PLAYER_SPEED: f32 = 500.0;

// Types

// Enums

// Structs
pub struct PlayerPlugin;

#[derive(Event)]
pub struct InitializePlayer;

#[derive(Event)]
pub struct TerminatePlayer;

#[derive(Component)]
pub struct Player {}

#[derive(Resource)]
pub struct PlayerManager;

// Implementations
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<InitializePlayer>()
            .add_event::<TerminatePlayer>()
            // Update Systems
            .add_systems(Update, 
                (
                PlayerManager::handle_initialize_player,
                PlayerManager::handle_terminate_player
                ).run_if(in_state(AppState::Game))
            )
            .add_systems(
                Update,
                (
                    PlayerManager::player_movement_system,
                    PlayerManager::player_god_system,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}

impl PlayerManager {
    fn handle_initialize_player(
        mut commands: Commands,
        mut initialize_player_event_reader: EventReader<InitializePlayer>,
        asset_server: Res<AssetServer>,
        mut universe_manager: ResMut<UniverseManager>,
    ) {
        if let Some(_) = initialize_player_event_reader.iter().next() {
            commands.insert_resource(PlayerManager {});
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
            ));
            universe_manager.register_local_universe(LocalUniverse::default());
        }
    }

    fn handle_terminate_player(
        mut commands: Commands, 
        mut terminate_player_event_reader: EventReader<TerminatePlayer>,
        player_query: Query<Entity, With<Player>>
    ) {
        if let Some(_) = terminate_player_event_reader.iter().next() {
            commands.remove_resource::<PlayerManager>();
            if let Ok(player_entity) = player_query.get_single() {
                commands.entity(player_entity).despawn();
            }
        }
    }

    fn player_movement_system(
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
        }
    }

    fn player_god_system(
        mut commands: Commands,
        main_camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
        primary_window_query: Query<&Window, With<PrimaryWindow>>,
        mouse_button_input: Res<Input<MouseButton>>,
        mut universe_manager: ResMut<UniverseManager>,
    ) {
        let (camera, camera_transform) = main_camera_query.single();

        let primary_window = primary_window_query.single();

        let global_universe = match universe_manager.get_global_universe() {
            Some(global_universe) => global_universe,
            None => return,
        };
        let mut global_universe = match global_universe.lock() {
            Ok(global_universe) => global_universe,
            Err(_) => return,
        };

        let world_position = match primary_window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            Some(world_position) => world_position,
            None => return,
        };

        if mouse_button_input.just_pressed(MouseButton::Left) {
            let local_entity_pos = LocalEntityPos::from(world_position);
            let local_chunk_pos = LocalChunkPos::from(local_entity_pos.clone());
            let local_chunk_pos_base10x10: (u8, u8) = local_chunk_pos.into();
            let chunk_id = match ChunkID::try_from(local_chunk_pos_base10x10) {
                Ok(chunk_id) => chunk_id,
                Err(_) => return,
            };

            let chunk_mutex = match global_universe.get_registered_chunk(&chunk_id) {
                Ok(chunk_mutex) => chunk_mutex,
                Err(_) => return,
            };
            let chunk_mutex = match chunk_mutex {
                Some(chunk_mutex) => chunk_mutex,
                None => return,
            };
            let mut chunk = match chunk_mutex.lock() {
                Ok(chunk) => chunk,
                Err(_) => return,
            };

            let entity_id = match GlobalUniverse::generate_entity_id(&mut chunk) {
                Ok(entity_id) => entity_id,
                Err(_) => return,
            };

            drop(chunk);

            let local_universe = match universe_manager.get_local_universe(LocalUniverseID::default()) {
                Some(local_universe) => local_universe,
                None => return,
            };

            let entity_metadata = match EntityMetadata::new(local_universe, chunk_mutex.clone(), local_entity_pos) {
                Ok(entity_metadata) => entity_metadata,
                Err(_) => return,
            };

            let entity_data = EntityData::new();

            let _ = global_universe.send_entity_operation_request(EntityOperationRequest::new(vec![
                EntityOperation::Register { 
                    id: entity_id.clone(), 
                    success_callback: Box::new(|_| {}), 
                    failure_callback: Box::new(|_, _| {})
                },
                EntityOperation::LoadMetadata { 
                    id: entity_id.clone(), 
                    metadata: entity_metadata, 
                    success_callback: Box::new(|_| {}), 
                    failure_callback: Box::new(|_, _, _| {}) 
                },
                EntityOperation::LoadData { 
                    id: entity_id.clone(), 
                    data: entity_data, 
                    success_callback: Box::new(|_| {}), 
                    failure_callback: Box::new(|_, _, _| {}) 
                },
                EntityOperation::Spawn { 
                    id: entity_id, 
                    success_callback: Box::new(|_| {}), 
                    failure_callback: Box::new(|_, _| {}) 
                },
            ]));
        }
    }
}

// Module Functions
