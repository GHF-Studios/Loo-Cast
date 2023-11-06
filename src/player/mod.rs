// Modules

// Local imports

// Internal imports
use crate::universe::chunk::ChunkViewer;
use crate::game::SimulationState;
use crate::AppState;

// External imports
use bevy::prelude::*;

// Static variables

// Constant variables
pub const PLAYER_SPEED: f32 = 500.0;

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
            // Enter Systems
            .add_systems(OnEnter(AppState::Game), PlayerManager::initialize)
            // Update Systems
            .add_systems(
                Update,
                PlayerManager::player_movement_system
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            // Exit Systems
            .add_systems(OnExit(AppState::Game), PlayerManager::terminate);
    }
}

impl PlayerManager {
    fn initialize(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            ChunkViewer::new(),
        ));
    }

    fn terminate(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
        commands.remove_resource::<PlayerManager>();
        if let Ok(player_entity) = player_query.get_single() {
            commands.entity(player_entity).despawn();
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
}

// Module Functions
