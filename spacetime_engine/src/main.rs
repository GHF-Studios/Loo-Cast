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
        info!("Initializing spacetime engine main module...");

        match self.manager_state {
            ManagerState::Created => {},
            ManagerState::Initialized => {
                return Err(ManagerInitializeError::ManagerAlreadyInitialized);
            },
            ManagerState::Finalized => {
                return Err(ManagerInitializeError::ManagerAlreadyFinalized);
            },
        }

        debug!("Locking spacetime engine module manager mutexes...");

        let kernel_manager = KERNEL_MANAGER.clone();
        let mut kernel_manager = match kernel_manager.lock() {
            Ok(kernel_manager) => {
                trace!("Locked kernel manager mutex.");
                kernel_manager
            }
            Err(err) => {
                panic!("Failed to lock kernel manager mutex! Error: {:?}", err);
            }
        };
        let system_manager = SYSTEM_MANAGER.clone();
        let mut system_manager = match system_manager.lock() {
            Ok(system_manager) => {
                trace!("Locked system manager mutex.");
                system_manager
            }
            Err(err) => {
                panic!("Failed to lock system manager mutex! Error: {:?}", err);
            }
        };

        debug!("Locked spacetime engine module manager mutexes.");

        info!("Initializing spacetime engine modules...");

        match kernel_manager.initialize() {
            Ok(_) => {
                debug!("Initialized kernel main module.");
                drop(kernel_manager);
            }
            Err(err) => {
                panic!("Failed to initialize kernel main module! Error: {:?}", err);
            }
        };
        match system_manager.initialize() {
            Ok(_) => {
                debug!("Initialized system main module.");
                drop(system_manager);
            }
            Err(err) => {
                panic!("Failed to initialize system main module! Error: {:?}", err);
            }
        };

        info!("Initialized spacetime engine modules.");

        self.manager_state = ManagerState::Initialized;

        info!("Initialized spacetime engine main module.");

        Ok(())
    }

    fn finalize(&mut self) -> Result<(), ManagerFinalizeError> {
        info!("Finalizing spacetime engine main module...");

        match self.manager_state {
            ManagerState::Created => {
                return Err(ManagerFinalizeError::ManagerNotInitialized);
            },
            ManagerState::Initialized => {},
            ManagerState::Finalized => {
                return Err(ManagerFinalizeError::ManagerAlreadyFinalized);
            },
        }

        debug!("Locking spacetime engine module manager mutexes...");

        let system_manager = SYSTEM_MANAGER.clone();
        let mut system_manager = match system_manager.lock() {
            Ok(system_manager) => {
                trace!("Locked system manager mutex.");
                system_manager
            }
            Err(err) => {
                panic!("Failed to lock system manager mutex! Error: {:?}", err);
            }
        };
        let kernel_manager = KERNEL_MANAGER.clone();
        let mut kernel_manager = match kernel_manager.lock() {
            Ok(kernel_manager) => {
                trace!("Locked kernel manager mutex.");
                kernel_manager
            }
            Err(err) => {
                panic!("Failed to lock kernel manager mutex! Error: {:?}", err);
            }
        };

        debug!("Locked spacetime engine module manager mutexes.");

        info!("Finalizing spacetime engine modules...");

        match system_manager.finalize() {
            Ok(_) => {
                debug!("Finalized system main module.");
                drop(system_manager);
            }
            Err(err) => {
                panic!("Failed to finalize system! Error: {:?}", err);
            }
        };
        match kernel_manager.finalize() {
            Ok(_) => {
                debug!("Finalized kernel main module.");
                drop(kernel_manager);
            }
            Err(err) => {
                panic!("Failed to finalize kernel! Error: {:?}", err);
            }
        };

        info!("Finalized spacetime engine modules.");

        self.manager_state = ManagerState::Finalized;

        info!("Finalized spacetime engine main module.");

        Ok(())
    }

    fn get_manager_state(&self) -> &ManagerState {
        &self.manager_state
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
    info!("Starting spacetime engine...");

    trace!("Locking spacetime engine main module manager mutex...");

    let main_manager = MAIN_MANAGER.clone();
    let mut main_manager = match main_manager.lock() {
        Ok(main_manager) => {
            trace!("Locked spacetime engine main module manager mutex.");
            main_manager
        }
        Err(err) => {
            panic!("Failed to lock spacetime engine main module manager mutex! Error: {:?}", err);
        }
    };

    debug!("Initializing spacetime engine main module...");

    match main_manager.initialize() {
        Ok(_) => {
            debug!("Initialized spacetime engine main module.");
            drop(main_manager);
        }
        Err(err) => {
            panic!("Failed to initialize spacetime engine main module! Error: {:?}", err);
        }
    };

    info!("Started spacetime engine.");
}

fn spacetime_engine_shutdown(mut exit_events: EventReader<AppExit>) {
    for _ in exit_events.iter() {
        info!("Shutting down spacetime engine...");

        trace!("Locking spacetime engine main module manager mutex...");

        let main_manager = MAIN_MANAGER.clone();
        let mut main_manager = match main_manager.lock() {
            Ok(main_manager) => {
                trace!("Locked spacetime engine main module manager mutex.");
                main_manager
            }
            Err(err) => {
                panic!("Failed to lock spacetime engine main module manager mutex! Error: {:?}", err);
            }
        };

        debug!("Finalizing spacetime engine main module...");

        match main_manager.finalize() {
            Ok(_) => {
                debug!("Finalized spacetime engine main module.");
                drop(main_manager);
            }
            Err(err) => {
                panic!("Failed to finalize spacetime engine main module! Error: {:?}", err);
            }
        };

        info!("Shut down spacetime engine.");
    }
}
