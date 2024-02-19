// Modules

// Local imports

// Internal imports
use crate::system::camera::MainCamera;
use crate::system::game::SimulationState;
use crate::system::universe::*;
use crate::system::universe::commands::*;
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

#[derive(Component)]
pub struct Player {}

#[derive(Resource)]
pub struct PlayerManager;

// Implementations
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Startup Systems
            .add_systems(Startup, PlayerManager::startup)
            // Enter Systems
            .add_systems(OnEnter(AppState::Game), PlayerManager::on_enter_game)
            // Update Systems
            .add_systems(
                Update,
                (
                    PlayerManager::player_movement_system,
                    PlayerManager::player_god_system,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            // Exit Systems
            .add_systems(OnExit(AppState::Game), PlayerManager::on_exit_game);
    }
}

impl PlayerManager {
    fn startup(mut commands: Commands,) {
        commands.insert_resource(PlayerManager {});
    }

    fn on_enter_game(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
    ) {
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
    }

    fn on_exit_game(
        mut commands: Commands,
        player_query: Query<Entity, With<Player>>,
    ) {
        if let Ok(player_entity) = player_query.get_single() {
            commands.entity(player_entity).despawn();
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
        mut universe_commands: ResMut<UniverseCommands>,
    ) {
        let (camera, camera_transform) = main_camera_query.single();

        let primary_window = primary_window_query.single();

        let world_position = match primary_window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            Some(world_position) => world_position,
            None => return,
        };

        if mouse_button_input.just_pressed(MouseButton::Left) {
            // TODO: RECYCLE CODE SNIPPET BELOW
            /*
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
            let mut parent_chunk = match parent_chunk_mutex.clone().lock() {
                Ok(parent_chunk) => parent_chunk,
                Err(_) => return,
            };

            let entity_id = match GlobalUniverse::generate_entity_id(&mut parent_chunk) {
                Ok(entity_id) => entity_id,
                Err(_) => return,
            };

            let local_entity_id = entity_id.get_local_entity_id();

            drop(parent_chunk);
            */

            let parent_chunk_id = chunk_commands.query_chunk_id_at_pos(world_position);

            let local_entity_id = entity_commands.generate_local_entity_id(parent_chunk_id);

            let entity_id = commands::EntityID {
                parent_chunk_id,
                local_entity_id,
            };

            let entity_metadata = entity_commands.generate_entity_metadata(entity_id);

            let entity_data = entity_commands.create_entity_data(entity_id);

            entity_commands.register_entity(entity_id);
            entity_commands.load_entity_metadata(entity_id, entity_metadata);
            entity_commands.load_entity_data(entity_id, entity_data);
            entity_commands.spawn_entity(entity_id);
            entity_commands.command_bevy_entity(entity_id, Box::new(|mut bevy_entity_commands: bevy::ecs::system::EntityCommands| {
                bevy_entity_commands.insert((SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(64.0, 64.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(world_position.x, world_position.y, 0.0),
                    ..default()
                }, RigidBody::Dynamic, Collider::cuboid(32.0, 32.0), Velocity {
                    linvel: Vec2::splat(0.0),
                    angvel: 0.0,
                }, LockedAxes::ROTATION_LOCKED, Damping { linear_damping: LINEAR_DAMPING, angular_damping: 0.0 }));
            }));
        }
    }
}

// Module Functions
