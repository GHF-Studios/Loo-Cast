// Modules

// Local imports

// Internal imports
use crate::kernel::manager::*;
use crate::system::game::SimulationState;
use crate::system::player::*;
use crate::system::AppState;

// External imports
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use lazy_static::*;
use std::sync::{Arc, Mutex};

// Static variables
lazy_static! {
    pub static ref BACKGROUND_MANAGER: Arc<Mutex<BackgroundManager>> =
        Arc::new(Mutex::new(BackgroundManager::new()));
}

// Constant variables

// Types

// Enums
enum BackgroundState {
    BackgroundNotSpawned,
    BackgroundSpawned {
        background_origin_x: i32,
        background_origin_y: i32,
    },
}

// Structs
pub struct BackgroundPlugin;

#[derive(Component)]
pub struct Background {
    background_width: i32,
    background_height: i32,
    background_chunk_position_x: i32,
    background_chunk_position_y: i32,
}

pub struct BackgroundManager {
    manager_state: ManagerState,
    background_state: BackgroundState,
}

// Implementations
impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
            // Enter Systems
            .add_systems(OnEnter(AppState::Game), BackgroundManager::spawn_background)
            // Update Systems
            .add_systems(
                Update,
                BackgroundManager::background_movement_system
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            // Exit Systems
            .add_systems(OnExit(AppState::Game), BackgroundManager::despawn_background);
    }
}

impl Manager for BackgroundManager {
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

    fn get_manager_state(&self) -> &ManagerState {
        &self.manager_state
    }
}

impl BackgroundManager {
    fn new() -> BackgroundManager {
        BackgroundManager {
            manager_state: ManagerState::Created,
            background_state: BackgroundState::BackgroundNotSpawned,
        }
    }

    fn spawn_background(
        mut commands: Commands,
        window_query: Query<&Window, With<PrimaryWindow>>,
        asset_server: Res<AssetServer>,
    ) {
        let window = window_query.get_single().unwrap();

        let background_manager = BACKGROUND_MANAGER.clone();
        let mut background_manager = match background_manager.lock() {
            Ok(background_manager) => {
                trace!("Successfully locked background manager mutex.");
                background_manager
            },
            Err(_) => panic!("Failed to lock background manager mutex!"),
        };

        match background_manager.background_state {
            BackgroundState::BackgroundNotSpawned => {
                background_manager.background_state = BackgroundState::BackgroundSpawned {
                    background_origin_x: (window.width() / 2.0) as i32,
                    background_origin_y: (window.height() / 2.0) as i32,
                };
            }
            BackgroundState::BackgroundSpawned { .. } => {
                error!("Background already spawned!");
                return;
            }
        };

        drop(background_manager);

        for x in -1..2 {
            for y in -1..2 {
                let window_width = window.width();
                let window_height = window.height();
                let x = (window_width / 2.0) + (window_width * x as f32);
                let y = (window_height / 2.0) + (window_height * y as f32);

                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(window_width, window_height)),
                            ..default()
                        },
                        transform: Transform::from_xyz(x, y, -1.0),
                        texture: asset_server
                            .load("loo_cast_base_mod/resources/sprites/background.png"),
                        ..default()
                    },
                    Background {
                        background_width: window_width as i32,
                        background_height: window_height as i32,
                        background_chunk_position_x: x as i32,
                        background_chunk_position_y: y as i32,
                    },
                ));
            }
        }
    }

    fn despawn_background(mut commands: Commands, background_query: Query<Entity, With<Background>>) {
        let background_manager = BACKGROUND_MANAGER.clone();
        let mut background_manager = match background_manager.lock() {
            Ok(background_manager) => {
                trace!("Successfully locked background manager mutex.");
                background_manager
            },
            Err(_) => panic!("Failed to lock background manager mutex!"),
        };

        match background_manager.background_state {
            BackgroundState::BackgroundNotSpawned => {
                error!("Background already spawned!");
                return;
            }
            BackgroundState::BackgroundSpawned { .. } => {
                background_manager.background_state = BackgroundState::BackgroundNotSpawned;

                drop(background_manager);
        
                for entity in background_query.iter() {
                    commands.entity(entity).despawn();
                }
            }
        };
    }

    fn background_movement_system(
        mut background_transform_query: Query<&mut Transform, With<Background>>,
        player_transform_query: Query<&Transform, (With<Player>, Without<Background>)>,
        window_query: Query<&Window, With<PrimaryWindow>>,
    ) {
        let window = window_query.get_single().unwrap();

        let background_manager = BACKGROUND_MANAGER.clone();
        let mut background_manager = match background_manager.lock() {
            Ok(background_manager) => {
                trace!("Successfully locked background manager mutex.");
                background_manager
            },
            Err(_) => panic!("Failed to lock background manager mutex!"),
        };

        let (mut background_origin_x, mut background_origin_y) = match background_manager.background_state {
            BackgroundState::BackgroundNotSpawned => {
                error!("Background not spawned!");
                return;
            }
            BackgroundState::BackgroundSpawned { background_origin_x, background_origin_y } => {
                (background_origin_x, background_origin_y)
            }
        };

        if let Ok(player_entity) = player_transform_query.get_single() {
            let difference_x =
                player_entity.translation.x - background_origin_x as f32;
            let difference_y =
                player_entity.translation.y - background_origin_y as f32;
            let window_width = window.width();
            let window_height = window.height();

            if difference_x.abs() > window_width {
                if difference_x > 0.0 {
                    background_origin_x += window_width as i32;

                    trace!("Shifting background towards +X");

                    for mut background_transform in background_transform_query.iter_mut() {
                        background_transform.translation.x += window_width;
                    }
                } else {
                    background_origin_x -= window_width as i32;

                    trace!("Shifting background towards -X");

                    for mut background_transform in background_transform_query.iter_mut() {
                        background_transform.translation.x -= window_width;
                    }
                }
            }
            if difference_y.abs() > window_height {
                if difference_y > 0.0 {
                    background_origin_y += window_height as i32;

                    trace!("Shifting background towards +Y");

                    for mut background_transform in background_transform_query.iter_mut() {
                        background_transform.translation.y += window_height;
                    }
                } else {
                    background_origin_y -= window_height as i32;

                    trace!("Shifting background towards -Y");

                    for mut background_transform in background_transform_query.iter_mut() {
                        background_transform.translation.y -= window_height;
                    }
                }
            }
        }

        background_manager.background_state = BackgroundState::BackgroundSpawned {
            background_origin_x,
            background_origin_y,
        };
    }
}

// Module Functions
