// Modules

// Local imports

// Internal imports
use crate::kernel::math::*;
use crate::system::AppState;
use crate::system::game::SimulationState;
use crate::system::player::*;

// External imports
use bevy::prelude::*;

// Static variables

// Constant variables
const CAMERA_SPEED: f32 = 10.0;

// Types

// Enums

// Structs
pub struct CameraPlugin;

#[derive(Component)]
pub struct MainCamera;

#[derive(Resource)]
pub struct CameraManager;

// Implementations
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            // Startup Systems
            .add_systems(Startup, CameraManager::initialize)
            // Update Systems
            .add_systems(
                Update,
                CameraManager::camera_movement_system
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}

impl CameraManager {
    fn initialize(mut commands: Commands) {
        commands.insert_resource(CameraManager {});
        commands.spawn((
            Camera2dBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            MainCamera {},
        ));
    }

    fn camera_movement_system(
        mut camera_query: Query<&mut Transform, With<Camera>>,
        player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
        time: Res<Time>,
    ) {
        let player_transform = player_query.single();
        let mut camera_transform = camera_query.single_mut();

        let interpolation = CAMERA_SPEED * time.delta_seconds();

        let new_x = Math::lerp(
            camera_transform.translation.x,
            player_transform.translation.x,
            interpolation,
        );
        let new_y = Math::lerp(
            camera_transform.translation.y,
            player_transform.translation.y,
            interpolation,
        );

        camera_transform.translation.x = new_x;
        camera_transform.translation.y = new_y;
    }
}

// Module Functions
