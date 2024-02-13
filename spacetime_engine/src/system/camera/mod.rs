// Modules

// Local imports

// Internal imports
use crate::system::math::*;

// External imports
use bevy::prelude::*;

// Static variables

// Constant variables
const CAMERA_SPEED: f32 = 10.0;

// Types

// Enums
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum MainCameraState {
    #[default]
    NotSpawned,
    Spawned,
}

// Structs
pub struct CameraPlugin;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct MainCameraTarget;

#[derive(Resource, Default)]
pub struct CameraManager {
    main_camera_state: MainCameraState,
}

// Implementations
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            // Startup Systems
            .add_systems(PreStartup, CameraManager::pre_startup)
            .add_systems(Startup, CameraManager::startup)
            // Update Systems
            .add_systems(Update, CameraManager::handle_main_camera_movement);
    }
}

impl CameraManager {
    fn pre_startup(mut commands: Commands) {
        info!("Pre-Starting camera Manager...");

        commands.insert_resource(CameraManager::default());

        info!("Pre-Started camera Manager.");
    }

    fn startup(commands: Commands, mut camera_manager: ResMut<CameraManager>) {
        info!("Starting camera Manager...");

        camera_manager.spawn_main_camera(commands);

        info!("Started camera Manager.");
    }

    fn post_startup(mut commands: Commands) {
        info!("Post-Starting camera Manager...");

        info!("Post-Started camera Manager.");
    }

    fn shutdown() {
        info!("Shutting down camera Manager...");

        info!("Shut down camera Manager.");
    }

    fn spawn_main_camera(&mut self, mut commands: Commands) {
        match self.main_camera_state {
            MainCameraState::NotSpawned => {
                commands.spawn((
                    Camera2dBundle {
                        transform: Transform::from_xyz(0.0, 0.0, 0.0),
                        ..default()
                    },
                    MainCamera {},
                ));

                self.main_camera_state = MainCameraState::Spawned;
            }
            MainCameraState::Spawned => {
                error!("Main camera already spawned!");

                return;
            }
        }
    }

    fn despawn_main_camera(&mut self, mut commands: Commands) {
        match self.main_camera_state {
            MainCameraState::NotSpawned => {
                error!("Main camera already despawned!");

                return;
            }
            MainCameraState::Spawned => {
                self.main_camera_state = MainCameraState::Spawned;
            }
        }
    }

    fn handle_main_camera_movement(
        mut main_camera_query: Query<
            &mut Transform,
            (With<Camera>, With<MainCamera>, Without<MainCameraTarget>),
        >,
        main_camera_target_query: Query<
            &Transform,
            (With<MainCameraTarget>, Without<Camera>, Without<MainCamera>),
        >,
        time: Res<Time>,
    ) {
        let player_transform = match main_camera_target_query.get_single() {
            Ok(single) => single,
            Err(_) => return,
        };

        let mut main_camera_transform = match main_camera_query.get_single_mut() {
            Ok(single_mut) => single_mut,
            Err(_) => return,
        };

        let interpolation = CAMERA_SPEED * time.delta_seconds();

        let new_x = Math::lerp(
            main_camera_transform.translation.x,
            player_transform.translation.x,
            interpolation,
        );
        let new_y = Math::lerp(
            main_camera_transform.translation.y,
            player_transform.translation.y,
            interpolation,
        );

        main_camera_transform.translation.x = new_x;
        main_camera_transform.translation.y = new_y;
    }
}

// Module Functions
