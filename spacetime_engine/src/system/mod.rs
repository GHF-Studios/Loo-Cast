// Modules
pub mod background;
pub mod camera;
pub mod game;
pub mod test;
pub mod player;
pub mod ui;
pub mod universe;

// Local imports

// Internal imports
use super::kernel::manager::*;
use background::BackgroundPlugin;
use camera::CameraPlugin;
use game::GamePlugin;
use test::TestPlugin;
use player::PlayerPlugin;
use ui::UIPlugin;
use universe::UniversePlugin;

// External imports
use lazy_static::*;
use std::sync::{Arc, Mutex};
use bevy::log::*;
use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_rapier2d::prelude::*;

// Static variables
lazy_static! {
    pub static ref SYSTEM_MANAGER: Arc<Mutex<SystemManager>> = Arc::new(Mutex::new(SystemManager::new()));
}

// Constant variables

// Types

// Enums
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    GamesMenu,
    CreateGameInfoMenu,
    Game,
}

// Structs
pub struct SystemManager {
    manager_state: ManagerState,
}

pub struct SystemPlugins;

pub struct RapierPlugins;

// Implementations
impl PluginGroup for SystemPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        group = group
            // Internal Modules
            .add(BackgroundPlugin)
            .add(CameraPlugin)
            .add(GamePlugin)
            .add(TestPlugin)
            .add(PlayerPlugin)
            .add(GamePlugin)
            .add(UIPlugin)
            .add(UniversePlugin)
            // External Modules
            .add(RapierDebugRenderPlugin::default())
            .add(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));

        group
    }
}

impl Manager for SystemManager {
    fn initialize(&mut self) -> Result<(), ManagerInitializeError> {
        info!("Initializing system main module...");

        match self.manager_state {
            ManagerState::Created => {},
            ManagerState::Initialized => {
                return Err(ManagerInitializeError::ManagerAlreadyInitialized);
            },
            ManagerState::Finalized => {
                return Err(ManagerInitializeError::ManagerAlreadyFinalized);
            },
        }

        debug!("Locking system module manager mutexes...");

        let background_manager = background::BACKGROUND_MANAGER.clone();
        let mut background_manager = match background_manager.lock() {
            Ok(background_manager) => {
                trace!("Locked background manager mutex.");
                background_manager
            },
            Err(err) => {
                panic!("Failed to lock background manager mutex! Error: {:?}", err);
            },
        };
        let camera_manager = camera::CAMERA_MANAGER.clone();
        let mut camera_manager = match camera_manager.lock() {
            Ok(camera_manager) => {
                trace!("Locked camera manager mutex.");
                camera_manager
            },
            Err(err) => {
                panic!("Failed to lock camera manager mutex! Error: {:?}", err);
            },
        };
        let game_manager = game::GAME_MANAGER.clone();
        let mut game_manager = match game_manager.lock() {
            Ok(game_manager) => {
                trace!("Locked game manager mutex.");
                game_manager
            },
            Err(err) => {
                panic!("Failed to lock game manager mutex! Error: {:?}", err);
            },
        };
        let player_manager = player::PLAYER_MANAGER.clone();
        let mut player_manager = match player_manager.lock() {
            Ok(player_manager) => {
                trace!("Locked player manager mutex.");
                player_manager
            },
            Err(err) => {
                panic!("Failed to lock player manager mutex! Error: {:?}", err);
            },
        };
        let test_manager = test::TEST_MANAGER.clone();
        let mut test_manager = match test_manager.lock() {
            Ok(test_manager) => {
                trace!("Locked iteration test manager mutex.");
                test_manager
            },
            Err(err) => {
                panic!("Failed to lock iteration test manager mutex! Error: {:?}", err);
            },
        };
        let ui_manager = ui::UI_MANAGER.clone();
        let mut ui_manager = match ui_manager.lock() {
            Ok(ui_manager) => {
                trace!("Locked UI manager mutex.");
                ui_manager
            },
            Err(err) => {
                panic!("Failed to lock UI manager mutex! Error: {:?}", err);
            },
        };
        let universe_manager = universe::UNIVERSE_MANAGER.clone();
        let mut universe_manager = match universe_manager.lock() {
            Ok(universe_manager) => {
                trace!("Locked universe manager mutex.");
                universe_manager
            },
            Err(err) => {
                panic!("Failed to lock universe manager mutex! Error: {:?}", err);
            },
        };

        debug!("Locked system module manager mutexes.");

        info!("Initializing system modules...");

        match background_manager.initialize() {
            Ok(_) => {
                debug!("Initialized background main module.");
            },
            Err(err) => {
                panic!("Failed to initialize background main module! Error: {:?}", err);
            },
        }
        match camera_manager.initialize() {
            Ok(_) => {
                debug!("Initialized camera main module.");
            },
            Err(err) => {
                panic!("Failed to initialize camera main module! Error: {:?}", err);
            },
        }
        match game_manager.initialize() {
            Ok(_) => {
                debug!("Initialized game main module.");
            },
            Err(err) => {
                panic!("Failed to initialize game main module! Error: {:?}", err);
            },
        }
        match player_manager.initialize() {
            Ok(_) => {
                debug!("Initialized player main module.");
            },
            Err(err) => {
                panic!("Failed to initialize player main module! Error: {:?}", err);
            },
        }
        match test_manager.initialize() {
            Ok(_) => {
                debug!("Initialized iteration test module.");
            },
            Err(err) => {
                panic!("Failed to initialize iteration test module! Error: {:?}", err);
            },
        }
        match ui_manager.initialize() {
            Ok(_) => {
                debug!("Initialized UI main module.");
            },
            Err(err) => {
                panic!("Failed to initialize UI main module! Error: {:?}", err);
            },
        }
        match universe_manager.initialize() {
            Ok(_) => {
                debug!("Initialized universe main module.");
            },
            Err(err) => {
                panic!("Failed to initialize universe main module! Error: {:?}", err);
            },
        }

        info!("Initialized system modules.");

        self.manager_state = ManagerState::Initialized;

        info!("Initialized system main module.");

        Ok(())
    }

    fn finalize(&mut self) -> Result<(), ManagerFinalizeError> {
        info!("Finalizing system main module...");

        match self.manager_state {
            ManagerState::Created => {
                return Err(ManagerFinalizeError::ManagerNotInitialized);
            },
            ManagerState::Initialized => {},
            ManagerState::Finalized => {
                return Err(ManagerFinalizeError::ManagerAlreadyFinalized);
            },
        }

        debug!("Locking system module manager mutexes...");

        let background_manager = background::BACKGROUND_MANAGER.clone();
        let mut background_manager = match background_manager.lock() {
            Ok(background_manager) => {
                trace!("Locked background manager mutex.");
                background_manager
            },
            Err(err) => {
                panic!("Failed to lock background manager mutex! Error: {:?}", err);
            },
        };
        let camera_manager = camera::CAMERA_MANAGER.clone();
        let mut camera_manager = match camera_manager.lock() {
            Ok(camera_manager) => {
                trace!("Locked camera manager mutex.");
                camera_manager
            },
            Err(err) => {
                panic!("Failed to lock camera manager mutex! Error: {:?}", err);
            },
        };
        let game_manager = game::GAME_MANAGER.clone();
        let mut game_manager = match game_manager.lock() {
            Ok(game_manager) => {
                trace!("Locked game manager mutex.");
                game_manager
            },
            Err(err) => {
                panic!("Failed to lock game manager mutex! Error: {:?}", err);
            },
        };
        let player_manager = player::PLAYER_MANAGER.clone();
        let mut player_manager = match player_manager.lock() {
            Ok(player_manager) => {
                trace!("Locked player manager mutex.");
                player_manager
            },
            Err(err) => {
                panic!("Failed to lock player manager mutex! Error: {:?}", err);
            },
        };
        let test_manager = test::TEST_MANAGER.clone();
        let mut test_manager = match test_manager.lock() {
            Ok(test_manager) => {
                trace!("Locked iteration test manager mutex.");
                test_manager
            },
            Err(err) => {
                panic!("Failed to lock iteration test manager mutex! Error: {:?}", err);
            },
        };
        let ui_manager = ui::UI_MANAGER.clone();
        let mut ui_manager = match ui_manager.lock() {
            Ok(ui_manager) => {
                trace!("Locked UI manager mutex.");
                ui_manager
            },
            Err(err) => {
                panic!("Failed to lock UI manager mutex! Error: {:?}", err);
            },
        };
        let universe_manager = universe::UNIVERSE_MANAGER.clone();
        let mut universe_manager = match universe_manager.lock() {
            Ok(universe_manager) => {
                trace!("Locked universe manager mutex.");
                universe_manager
            },
            Err(err) => {
                panic!("Failed to lock universe manager mutex! Error: {:?}", err);
            },
        };

        debug!("Locked system module manager mutexes.");

        info!("Finalizing system modules...");

        match background_manager.finalize() {
            Ok(_) => {
                debug!("Finalized background main module.");
            },
            Err(err) => {
                panic!("Failed to finalize background main module. Error: {:?}", err);
            },
        }
        match camera_manager.finalize() {
            Ok(_) => {
                debug!("Finalized camera main module.");
            },
            Err(err) => {
                panic!("Failed to finalize camera main module. Error: {:?}", err);
            },
        }
        match game_manager.finalize() {
            Ok(_) => {
                debug!("Finalized game main module.");
            },
            Err(err) => {
                panic!("Failed to finalize game main module. Error: {:?}", err);
            },
        }
        match player_manager.finalize() {
            Ok(_) => {
                debug!("Finalized player main module.");
            },
            Err(err) => {
                panic!("Failed to finalize player main module. Error: {:?}", err);
            },
        }
        match test_manager.finalize() {
            Ok(_) => {
                debug!("Finalized iteration test module.");
            },
            Err(err) => {
                panic!("Failed to finalize iteration test module! Error: {:?}", err);
            },
        }
        match ui_manager.finalize() {
            Ok(_) => {
                debug!("Finalized UI main module.");
            },
            Err(err) => {
                panic!("Failed to finalize UI main module. Error: {:?}", err);
            },
        }
        match universe_manager.finalize() {
            Ok(_) => {
                debug!("Finalized universe main module.");
            },
            Err(err) => {
                panic!("Failed to finalize universe main module. Error: {:?}", err);
            },
        }

        info!("Finalized system modules..");

        self.manager_state = ManagerState::Finalized;

        info!("Finalized system main module.");

        Ok(())
    }

    fn get_manager_state(&self) -> &ManagerState {
        &self.manager_state
    }
}

impl SystemManager {
    pub fn new() -> Self {
        Self {
            manager_state: ManagerState::Created,
        }
    }
}

// Module Functions
