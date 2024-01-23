// Crates
extern crate spacetime_engine;

// Modules

// Local imports

// Internal imports
use spacetime_engine::kernel::manager::*;
use spacetime_engine::kernel::*;
use spacetime_engine::system::*;

// External imports
use bevy::log::*;
use bevy::prelude::*;
use bevy::app::AppExit;
use lazy_static::*;
use std::sync::{Arc, Mutex};

// Static variables
lazy_static! {
    pub static ref MAIN_MANAGER: Arc<Mutex<MainManager>> = Arc::new(Mutex::new(MainManager::new()));
}

// Constant variables

// Types

// Enums

// Structs
pub struct MainManager {
    manager_state: ManagerState,
}

// Implementations
impl Manager for MainManager {
    fn initialize(&mut self) -> Result<(), ManagerInitializeError> {
        match self.state {
            ManagerState::Created => {},
            ManagerState::Initialized => {
                return Err(ManagerInitializeError::ManagerAlreadyInitialized);
            },
            ManagerState::Finalized => {
                return Err(ManagerInitializeError::ManagerAlreadyFinalized);
            },
        }

        let kernel_manager = KERNEL_MANAGER.clone();

        let mut kernel_manager = match kernel_manager.lock() {
            Ok(kernel_manager) => {
                trace!("Successfully locked main manager mutex.");
                kernel_manager
            }
            Err(err) => {
                panic!("Failed to lock main manager mutex! Error: {:?}", err);
            }
        };

        match kernel_manager.initialize() {
            Ok(_) => {
                info!("Successfully initialized spacetime engine.");
                drop(kernel_manager);
            }
            Err(err) => {
                panic!("Failed to initialize spacetime engine! Error: {:?}", err);
            }
        };

        self.state = ManagerState::Initialized;

        Ok(())
    }

    fn finalize(&mut self) -> Result<(), ManagerFinalizeError> {
        match self.state {
            ManagerState::Created => {
                return Err(ManagerFinalizeError::ManagerNotInitialized);
            },
            ManagerState::Initialized => {},
            ManagerState::Finalized => {
                return Err(ManagerFinalizeError::ManagerAlreadyFinalized);
            },
        }

        let kernel_manager = KERNEL_MANAGER.clone();

        let mut kernel_manager = match kernel_manager.lock() {
            Ok(kernel_manager) => {
                trace!("Successfully locked main manager mutex.");
                kernel_manager
            }
            Err(err) => {
                panic!("Failed to lock main manager mutex! Error: {:?}", err);
            }
        };

        match kernel_manager.finalize() {
            Ok(_) => {
                info!("Successfully finalized spacetime engine.");
                drop(kernel_manager);
            }
            Err(err) => {
                panic!("Failed to finalize spacetime engine! Error: {:?}", err);
            }
        };

        self.state = ManagerState::Finalized;

        Ok(())
    }

    fn get_manager_state(&self) -> &ManagerState {
        &self.state
    }
}

impl MainManager {
    pub fn new() -> Self {
        Self {
            manager_state: ManagerState::Created,
        }
    }
}

// Module Functions
fn main() {
    App::new()
        // Startup Systems
        .add_systems(PreStartup, spacetime_engine_startup)
        // Update Systems
        .add_systems(Update, spacetime_engine_shutdown)
        // Default Bevy Plugins
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    asset_folder: "mods".to_string(),
                    ..default()
                })
                .set(LogPlugin {
                    level: bevy::log::Level::INFO,
                    ..default()
                }),
        )
        // Plugins
        .add_plugins(SystemPlugins)
        // States
        .add_state::<AppState>()
        // Run
        .run();

}

fn spacetime_engine_startup() {
    info!("Initializing engine...");

    let kernel_manager = KERNEL_MANAGER.clone();
    let mut kernel_manager = match kernel_manager.lock() {
        Ok(kernel_manager) => {
            trace!("Successfully locked kernel manager mutex.");
            kernel_manager
        }
        Err(err) => {
            panic!("Failed to lock kernel manager mutex! Error: {:?}", err);
        }
    };
    let system_manager = SYSTEM_MANAGER.clone();
    let mut system_manager = match system_manager.lock() {
        Ok(system_manager) => {
            trace!("Successfully locked system manager mutex.");
            system_manager
        }
        Err(err) => {
            panic!("Failed to lock system manager mutex! Error: {:?}", err);
        }
    };

    match kernel_manager.initialize() {
        Ok(_) => {
            drop(kernel_manager);
        }
        Err(err) => {
            panic!("Failed to initialize kernel! Error: {:?}", err);
        }
    };
    match system_manager.initialize() {
        Ok(_) => {
            drop(system_manager);
        }
        Err(err) => {
            panic!("Failed to initialize system! Error: {:?}", err);
        }
    };

    info!("Initialized engine.");
}

fn spacetime_engine_shutdown(mut exit_events: EventReader<AppExit>) {
    for _ in exit_events.iter() {
        info!("Finalizing engine...");
        
        let system_manager = SYSTEM_MANAGER.clone();

        let mut system_manager = match system_manager.lock() {
            Ok(system_manager) => {
                trace!("Successfully locked system manager mutex.");
                system_manager
            }
            Err(err) => {
                panic!("Failed to lock system manager mutex! Error: {:?}", err);
            }
        };

        match system_manager.finalize() {
            Ok(_) => {
                drop(system_manager);
            }
            Err(err) => {
                panic!("Failed to finalize system! Error: {:?}", err);
            }
        };

        let kernel_manager = KERNEL_MANAGER.clone();

        let mut kernel_manager = match kernel_manager.lock() {
            Ok(kernel_manager) => {
                trace!("Successfully locked kernel manager mutex.");
                kernel_manager
            }
            Err(err) => {
                panic!("Failed to lock kernel manager mutex! Error: {:?}", err);
            }
        };

        match kernel_manager.finalize() {
            Ok(_) => {
                drop(kernel_manager);
            }
            Err(err) => {
                panic!("Failed to finalize kernel! Error: {:?}", err);
            }
        };

        info!("Finalized engine.");
    }
}
