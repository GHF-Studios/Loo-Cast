use crate::system::math::*;
use bevy::prelude::*;

const MAIN_CAMERA_SPEED: f32 = 10.0;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum MainCameraState {
    #[default]
    NotSpawned,
    Spawned,
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct MainCameraTarget;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            // States
            .add_state::<MainCameraState>()
            // Startup Systems
            .add_systems(PreStartup, CameraManager::pre_startup)
            .add_systems(Startup, CameraManager::startup)
            // Update Systems
            .add_systems(Update, CameraManager::main_camera_movement.run_if(in_state(MainCameraState::Spawned)));
    }
}

#[derive(Resource, Default)]
pub struct CameraManager {
}

impl CameraManager {
    fn pre_startup(mut commands: Commands) {
        info!("Pre-Starting camera Manager...");

        commands.insert_resource(CameraManager::default());

        info!("Pre-Started camera Manager.");
    }

    fn startup(
        commands: Commands, 
        current_main_camera_state: Res<State<MainCameraState>>, 
        next_main_camera_state: ResMut<NextState<MainCameraState>>,
    ) {
        info!("Starting camera Manager...");

        CameraManager::spawn_main_camera(commands, current_main_camera_state, next_main_camera_state);

        info!("Started camera Manager.");
    }

    fn spawn_main_camera(
        mut commands: Commands,
        current_main_camera_state: Res<State<MainCameraState>>, 
        mut next_main_camera_state: ResMut<NextState<MainCameraState>>,
    ) {
        match current_main_camera_state.get() {
            MainCameraState::NotSpawned => {
                commands.spawn((
                    Camera2dBundle {
                        transform: Transform::from_xyz(0.0, 0.0, 0.0),
                        ..default()
                    },
                    MainCamera {},
                ));

                next_main_camera_state.set(MainCameraState::Spawned);
            }
            MainCameraState::Spawned => {
                error!("Main camera already spawned!");
            }
        }
    }

    #[allow(clippy::type_complexity)]
    fn main_camera_movement(
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

        let interpolation = MAIN_CAMERA_SPEED * time.delta_seconds();

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
