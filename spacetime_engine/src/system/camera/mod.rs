// Modules

// Local imports

// Internal imports
use crate::kernel::math::*;
use crate::kernel::manager::*;
use crate::system::game::SimulationState;
use crate::system::player::*;
use crate::system::AppState;

// External imports
use bevy::prelude::*;
use lazy_static::*;
use std::sync::{Arc, Mutex};

// Static variables
lazy_static! {
    pub static ref CAMERA_MANAGER: Arc<Mutex<CameraManager>> =
        Arc::new(Mutex::new(CameraManager::new()));
}

// Constant variables
const CAMERA_SPEED: f32 = 10.0;

// Types

// Enums
enum CameraState {
    CameraNotSpawned,
    CameraSpawned,
}

// Structs
pub struct CameraPlugin;

#[derive(Component)]
pub struct MainCamera;

#[derive(Resource)]
pub struct CameraManager {
    manager_state: ManagerState,
    camera_state: CameraState,
}

// Implementations
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            // Startup Systems
            .add_systems(Startup, CameraManager::spawn_camera)
            // Update Systems
            .add_systems(
                Update,
                CameraManager::camera_movement_system
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}

impl Manager for CameraManager {
    fn initialize(&mut self) -> Result<(), ManagerInitializeError> {
        match self.manager_state {
            ManagerState::Created => {}
            ManagerState::Initialized => {
                return Err(ManagerInitializeError::ManagerAlreadyInitialized);
            }
            ManagerState::Finalized => {
                return Err(ManagerInitializeError::ManagerAlreadyFinalized);
            }
        }

        self.manager_state = ManagerState::Initialized;

        Ok(())
    }

    fn finalize(&mut self) -> Result<(), ManagerFinalizeError> {
        match self.manager_state {
            ManagerState::Created => {
                return Err(ManagerFinalizeError::ManagerNotInitialized);
            }
            ManagerState::Initialized => {}
            ManagerState::Finalized => {
                return Err(ManagerFinalizeError::ManagerAlreadyFinalized);
            }
        }

        self.manager_state = ManagerState::Finalized;

        Ok(())
    }

    fn get_state(&self) -> &ManagerState {
        &self.manager_state
    }
}

impl CameraManager {
    fn new() -> CameraManager {
        CameraManager {
            manager_state: ManagerState::Created,
            camera_state: CameraState::CameraNotSpawned,
        }
    }

    fn spawn_camera(mut commands: Commands) {
        let camera_manager = CAMERA_MANAGER.clone();
        let mut camera_manager = match camera_manager.lock() {
            Ok(camera_manager) => {
                trace!("Successfully locked camera manager mutex.");
                camera_manager
            },
            Err(_) => panic!("Failed to lock camera manager mutex!"),
        };

        match camera_manager.camera_state {
            CameraState::CameraNotSpawned => {}
            CameraState::CameraSpawned => {
                error!("Camera already spawned!");
                return;
            }
        }

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
