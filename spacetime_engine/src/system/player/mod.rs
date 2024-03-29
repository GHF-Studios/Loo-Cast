// Modules

// Local imports

// Internal imports
use crate::system::camera::MainCamera;
use crate::system::game::SimulationState;
use crate::system::universe::chunk::id::*;
use crate::system::universe::chunk::pos::*;
use crate::system::universe::entity::data::*;
use crate::system::universe::entity::metadata::*;
use crate::system::universe::entity::pos::*;
use crate::system::universe::entity::*;
use crate::system::universe::global::*;
use crate::system::universe::local::*;
use crate::system::universe::*;
use crate::system::AppState;

// External imports
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;

// Static variables

// Constant variables
pub const ACCELERATION: f32 = 1000.0;
pub const MAX_SPEED: f32 = 200.0;
pub const LINEAR_DAMPING: f32 = 5.0;
pub const SPRINT_MULTIPLIER: f32 = 5.0;

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
            .add_systems(
                Update,
                (
                    PlayerManager::handle_initialize_player,
                    PlayerManager::handle_terminate_player,
                )
                    .run_if(in_state(AppState::Game)),
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
        if initialize_player_event_reader.iter().next().is_some() {
            commands.insert_resource(PlayerManager {});
            commands.spawn((
                Player {},
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(64.0, 64.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    texture: asset_server.load("loo_cast_base_mod/resources/sprites/circle.png"),
                    ..default()
                },
                RigidBody::Dynamic,
                Collider::ball(32.0),
                Velocity {
                    linvel: Vec2::splat(0.0),
                    angvel: 0.0,
                },
                LockedAxes::ROTATION_LOCKED,
                Damping {
                    linear_damping: LINEAR_DAMPING,
                    angular_damping: 0.0,
                },
            ));
            let _ = universe_manager.register_local_universe(LocalUniverse::default());
        }
    }

    fn handle_terminate_player(
        mut commands: Commands,
        mut terminate_player_event_reader: EventReader<TerminatePlayer>,
        player_query: Query<Entity, With<Player>>,
    ) {
        if terminate_player_event_reader.iter().next().is_some() {
            commands.remove_resource::<PlayerManager>();
            if let Ok(player_entity) = player_query.get_single() {
                commands.entity(player_entity).despawn();
            }
        }
    }

    fn player_movement_system(
        keyboard_input: Res<Input<KeyCode>>,
        mut player_velocity_query: Query<&mut Velocity, With<Player>>,
        time: Res<Time>,
    ) {
        if let Ok(mut player_velocity) = player_velocity_query.get_single_mut() {
            let mut direction = Vec2::ZERO;

            if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
                direction += Vec2::new(-1.0, 0.0);
            }
            if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
                direction += Vec2::new(1.0, 0.0);
            }
            if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
                direction += Vec2::new(0.0, 1.0);
            }
            if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
                direction += Vec2::new(0.0, -1.0);
            }

            if direction.length() > 0.0 {
                let sprinting = keyboard_input.pressed(KeyCode::ShiftLeft);

                direction = direction.normalize();

                if sprinting {
                    player_velocity.linvel +=
                        direction * ACCELERATION * SPRINT_MULTIPLIER * time.delta_seconds();
                    if player_velocity.linvel.length() > MAX_SPEED * SPRINT_MULTIPLIER {
                        player_velocity.linvel =
                            player_velocity.linvel.normalize() * MAX_SPEED * SPRINT_MULTIPLIER;
                    }
                } else {
                    player_velocity.linvel += direction * ACCELERATION * time.delta_seconds();
                    if player_velocity.linvel.length() > MAX_SPEED {
                        player_velocity.linvel = player_velocity.linvel.normalize() * MAX_SPEED;
                    }
                }
            }
        }
    }

    fn player_god_system(
        main_camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
        primary_window_query: Query<&Window, With<PrimaryWindow>>,
        mouse_button_input: Res<Input<MouseButton>>,
        universe_manager: ResMut<UniverseManager>,
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

        // TODO: Reimplement chunk operations using the new system
        /*
        if mouse_button_input.just_pressed(MouseButton::Left) {
            let local_entity_pos = LocalEntityPos::from(world_position);
            let absolute_local_parent_chunk_pos = AbsoluteLocalChunkPos::from(local_entity_pos);
            let absolute_local_parent_chunk_pos_base10x10: (u8, u8) = absolute_local_parent_chunk_pos.into();
            let local_chunk_id_base10x10 = match LocalChunkIDBase10x10::new_from_tuple(absolute_local_parent_chunk_pos_base10x10) {
                Ok(local_chunk_id_base10x10) => local_chunk_id_base10x10,
                Err(_) => return,
            };
            let local_chunk_id = LocalChunkID::new_from_base10x10(local_chunk_id_base10x10);
            let parent_chunk_id = match ChunkID::try_from(local_chunk_id) {
                Ok(parent_chunk_id) => parent_chunk_id,
                Err(_) => return,
            };

            let parent_chunk_mutex = match global_universe.get_registered_chunk(&parent_chunk_id) {
                Some(parent_chunk_mutex) => parent_chunk_mutex,
                None => return,
            };
            let mut parent_chunk = match parent_chunk_mutex.lock() {
                Ok(parent_chunk) => parent_chunk,
                Err(_) => return,
            };

            let entity_id = match GlobalUniverse::generate_entity_id(&mut parent_chunk) {
                Ok(entity_id) => entity_id,
                Err(_) => return,
            };

            drop(parent_chunk);

            let entity_metadata = EntityMetadata::new(parent_chunk_mutex.clone());
            let entity_data = EntityData::new();

            let _ = global_universe.send_entity_operation_request(EntityOperationRequest::new(vec![
                EntityOperation::Register {
                    id: entity_id.clone(),
                    success_callback: Box::new(|_| {}),
                    failure_callback: Box::new(|err| {
                        println!("Failed to register entity: {:?}", err);
                    })
                },
                EntityOperation::LoadMetadata {
                    id: entity_id.clone(),
                    metadata: entity_metadata,
                    success_callback: Box::new(|_| {}),
                    failure_callback: Box::new(|err| {
                        println!("Failed to load entity metadata: {:?}", err);
                    })
                },
                EntityOperation::LoadData {
                    id: entity_id.clone(),
                    data: entity_data,
                    success_callback: Box::new(|_| {}),
                    failure_callback: Box::new(|err| {
                        println!("Failed to load entity data: {:?}", err);
                    })
                },
                EntityOperation::Spawn {
                    id: entity_id.clone(),
                    success_callback: Box::new(|_| {}),
                    failure_callback: Box::new(|err| {
                        println!("Failed to spawn entity: {:?}", err);
                    })
                },
                EntityOperation::Command {
                    id: entity_id,
                    entity_commands: Box::new(move |mut entity_commands| {
                        entity_commands.insert((
                            SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::new(64.0, 64.0)),
                                    ..default()
                                },
                                transform: Transform::from_xyz(world_position.x, world_position.y, 0.0),
                                ..default()
                            },
                            RigidBody::Dynamic,
                            Collider::cuboid(32.0, 32.0),
                            Velocity {
                                linvel: Vec2::splat(0.0),
                                angvel: 0.0,
                            },
                            LockedAxes::ROTATION_LOCKED,
                            Damping { linear_damping: LINEAR_DAMPING, angular_damping: 0.0 }
                        ));
                    }),
                    success_callback: Box::new(|_| {}),
                    failure_callback: Box::new(|err| {
                        println!("Failed to command entity: {:?}", err);
                    })
                },
            ]));
        }
        */
    }
}

// Module Functions
