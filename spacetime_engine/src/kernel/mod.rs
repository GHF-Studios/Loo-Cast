// Modules
pub mod config;
pub mod data;
pub mod debug;
pub mod event;
pub mod manager;
pub mod math;
pub mod plugin;
pub mod resource;

// Internal imports
use manager::*;

// External imports
use lazy_static::*;
use std::sync::{Arc, Mutex};
use bevy::log::*;

// Static variables
lazy_static! {
    pub static ref MAIN_MANAGER: Arc<Mutex<MainManager>> = Arc::new(Mutex::new(MainManager::new()));
}

// Constant variables

// Types

// Enums

// Structs
pub struct MainManager {
    state: ManagerState,
}

// Implementations
impl Manager for MainManager {
    fn initialize(&mut self) -> Result<(), ManagerInitializeError> {
        match self.state {
            ManagerState::Created => {}
            ManagerState::Initialized => {
                return Err(ManagerInitializeError::ManagerAlreadyInitialized);
            }
            ManagerState::Finalized => {
                return Err(ManagerInitializeError::ManagerAlreadyFinalized);
            }
        }

        debug!("Locking kernel manager mutexes...");

        let config_manager = config::CONFIG_MANAGER.clone();
        let mut config_manager = match config_manager.lock() {
            Ok(config_manager) => {
                trace!("Successfully locked config manager mutex.");
                config_manager
            }
            Err(err) => {
                panic!("Failed to lock config manager mutex! Error: {:?}", err);
            }
        };
        let data_manager = data::DATA_MANAGER.clone();
        let mut data_manager = match data_manager.lock() {
            Ok(data_manager) => {
                trace!("Successfully locked data manager mutex.");
                data_manager
            }
            Err(err) => {
                panic!("Failed to lock data manager mutex! Error: {:?}", err);
            }
        };
        let debug_manager = debug::DEBUG_MANAGER.clone();
        let mut debug_manager = match debug_manager.lock() {
            Ok(debug_manager) => {
                trace!("Successfully locked debug manager mutex.");
                debug_manager
            }
            Err(err) => {
                panic!("Failed to lock debug manager mutex! Error: {:?}", err);
            }
        };
        let event_manager = event::EVENT_MANAGER.clone();
        let mut event_manager = match event_manager.lock() {
            Ok(event_manager) => {
                trace!("Successfully locked event manager mutex.");
                event_manager
            }
            Err(err) => {
                panic!("Failed to lock event manager mutex! Error: {:?}", err);
            }
        };
        let plugin_manager = plugin::PLUGIN_MANAGER.clone();
        let mut plugin_manager = match plugin_manager.lock() {
            Ok(plugin_manager) => {
                trace!("Successfully locked plugin manager mutex.");
                plugin_manager
            }
            Err(err) => {
                panic!("Failed to lock plugin manager mutex! Error: {:?}", err);
            }
        };
        let resource_manager = resource::RESOURCE_MANAGER.clone();
        let mut resource_manager = match resource_manager.lock() {
            Ok(resource_manager) => {
                trace!("Successfully locked resource manager mutex.");
                resource_manager
            }
            Err(err) => {
                panic!("Failed to lock resource manager mutex! Error: {:?}", err);
            }
        };

        debug!("Successfully locked kernel manager mutexes.");

        info!("Initializing kernel managers...");

        match config_manager.initialize() {
            Ok(_) => {
                debug!("Successfully initialized config manager.");
            }
            Err(err) => {
                panic!("Failed to initialize config manager! Error: {:?}", err);
            }
        }

        match data_manager.initialize() {
            Ok(_) => {
                debug!("Successfully initialized data manager.");
            }
            Err(err) => {
                panic!("Failed to initialize data manager! Error: {:?}", err);
            }
        }

        match debug_manager.initialize() {
            Ok(_) => {
                debug!("Successfully initialized debug manager.");
            }
            Err(err) => {
                panic!("Failed to initialize debug manager! Error: {:?}", err);
            }
        }

        match event_manager.initialize() {
            Ok(_) => {
                debug!("Successfully initialized event manager.");
            }
            Err(err) => {
                panic!("Failed to initialize event manager! Error: {:?}", err);
            }
        }

        match plugin_manager.initialize() {
            Ok(_) => {
                debug!("Successfully initialized plugin manager.");
            }
            Err(err) => {
                panic!("Failed to initialize plugin manager! Error: {:?}", err);
            }
        }

        match resource_manager.initialize() {
            Ok(_) => {
                debug!("Successfully initialized resource manager.");
            }
            Err(err) => {
                panic!("Failed to initialize resource manager! Error: {:?}", err);
            }
        }

        info!("Successfully initialized kernel managers.");

        // initialize the emergent systems(aka the engine system) such as mod management, USF(massive oversimplification, but baaaaaasically USF = ECS) management, player management, savegame management, camera management, UI Management, etc.

        // initialize the bevy engine, acting as the user interface for the engine, essentially being the first visual indication that the game has started

        self.state = ManagerState::Initialized;

        Ok(())
    }

    fn finalize(&mut self) -> Result<(), ManagerFinalizeError> {
        match self.state {
            ManagerState::Created => {
                return Err(ManagerFinalizeError::ManagerNotInitialized);
            }
            ManagerState::Initialized => {}
            ManagerState::Finalized => {
                return Err(ManagerFinalizeError::ManagerAlreadyFinalized);
            }
        }

        debug!("Locking all kernel manager mutexes...");

        let config_manager = config::CONFIG_MANAGER.clone();
        let mut config_manager = match config_manager.lock() {
            Ok(config_manager) => {
                trace!("Successfully locked config manager mutex.");
                config_manager
            }
            Err(err) => {
                panic!("Failed to lock config manager mutex! Error: {:?}", err);
            }
        };
        let data_manager = data::DATA_MANAGER.clone();
        let mut data_manager = match data_manager.lock() {
            Ok(data_manager) => {
                trace!("Successfully locked data manager mutex.");
                data_manager
            }
            Err(err) => {
                panic!("Failed to lock data manager mutex! Error: {:?}", err);
            }
        };
        let debug_manager = debug::DEBUG_MANAGER.clone();
        let mut debug_manager = match debug_manager.lock() {
            Ok(debug_manager) => {
                trace!("Successfully locked debug manager mutex.");
                debug_manager
            }
            Err(err) => {
                panic!("Failed to lock debug manager mutex! Error: {:?}", err);
            }
        };
        let event_manager = event::EVENT_MANAGER.clone();
        let mut event_manager = match event_manager.lock() {
            Ok(event_manager) => {
                trace!("Successfully locked event manager mutex.");
                event_manager
            }
            Err(err) => {
                panic!("Failed to lock event manager mutex! Error: {:?}", err);
            }
        };
        let plugin_manager = plugin::PLUGIN_MANAGER.clone();
        let mut plugin_manager = match plugin_manager.lock() {
            Ok(plugin_manager) => {
                trace!("Successfully locked plugin manager mutex.");
                plugin_manager
            }
            Err(err) => {
                panic!("Failed to lock plugin manager mutex! Error: {:?}", err);
            }
        };
        let resource_manager = resource::RESOURCE_MANAGER.clone();
        let mut resource_manager = match resource_manager.lock() {
            Ok(resource_manager) => {
                trace!("Successfully locked resource manager mutex.");
                resource_manager
            }
            Err(err) => {
                panic!("Failed to lock resource manager mutex! Error: {:?}", err);
            }
        };

        debug!("Successfully locked kernel manager mutexes.");

        info!("Finalizing kernel managers...");

        match config_manager.finalize() {
            Ok(_) => {
                debug!("Successfully finalized config manager.");
            }
            Err(err) => {
                panic!("Failed to finalize config manager! Error: {:?}", err);
            }
        }

        match data_manager.finalize() {
            Ok(_) => {
                debug!("Successfully finalized data manager.");
            }
            Err(err) => {
                panic!("Failed to finalize data manager! Error: {:?}", err);
            }
        }

        match debug_manager.finalize() {
            Ok(_) => {
                debug!("Successfully finalized debug manager.");
            }
            Err(err) => {
                panic!("Failed to finalize debug manager! Error: {:?}", err);
            }
        }

        match event_manager.finalize() {
            Ok(_) => {
                debug!("Successfully finalized event manager.");
            }
            Err(err) => {
                panic!("Failed to finalize event manager! Error: {:?}", err);
            }
        }

        match plugin_manager.finalize() {
            Ok(_) => {
                debug!("Successfully finalized plugin manager.");
            }
            Err(err) => {
                panic!("Failed to finalize plugin manager! Error: {:?}", err);
            }
        }

        match resource_manager.finalize() {
            Ok(_) => {
                debug!("Successfully finalized resource manager.");
            }
            Err(err) => {
                panic!("Failed to finalize resource manager! Error: {:?}", err);
            }
        }

        info!("Successfully finalized all kernel managers.");

        self.state = ManagerState::Finalized;

        Ok(())
    }

    fn get_state(&self) -> &ManagerState {
        &self.state
    }
}

impl MainManager {
    fn new() -> MainManager {
        MainManager {
            state: ManagerState::Created,
        }
    }
}

// Module Functions
