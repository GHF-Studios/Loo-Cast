// Modules
pub mod background;
pub mod camera;
pub mod game;
pub mod iteration_test;
pub mod player;
pub mod savegame;
pub mod ui;
pub mod universe;

// Local imports

// Internal imports
use super::kernel::manager::*;
use background::BackgroundPlugin;
use camera::CameraPlugin;
use game::GamePlugin;
use iteration_test::IterationTestPlugin;
use player::PlayerPlugin;
use savegame::SavegamePlugin;
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
    SavegamesMenu,
    CreateSavegameMenu,
    Game,
}

// Structs
pub struct SystemManager {
    state: ManagerState,
}

pub struct SystemPlugins;

pub struct RapierPlugins;

// Implementations
impl Manager for SystemManager {
    fn initialize(&mut self) -> Result<(), ManagerInitializeError> {
        info!("Initializing system...");

        match self.state {
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
                trace!("Successfully locked background manager mutex.");
                background_manager
            },
            Err(err) => {
                panic!("Failed to lock background manager mutex! Error: {:?}", err);
            },
        };
        let camera_manager = camera::CAMERA_MANAGER.clone();
        let mut camera_manager = match camera_manager.lock() {
            Ok(camera_manager) => {
                trace!("Successfully locked camera manager mutex.");
                camera_manager
            },
            Err(err) => {
                panic!("Failed to lock camera manager mutex! Error: {:?}", err);
            },
        };
        let game_manager = game::GAME_MANAGER.clone();
        let mut game_manager = match game_manager.lock() {
            Ok(game_manager) => {
                trace!("Successfully locked game manager mutex.");
                game_manager
            },
            Err(err) => {
                panic!("Failed to lock game manager mutex! Error: {:?}", err);
            },
        };
        let iteration_test_manager = iteration_test::ITERATION_TEST_MANAGER.clone();
        let mut iteration_test_manager = match iteration_test_manager.lock() {
            Ok(iteration_test_manager) => {
                trace!("Successfully locked iteration test manager mutex.");
                iteration_test_manager
            },
            Err(err) => {
                panic!("Failed to lock iteration test manager mutex! Error: {:?}", err);
            },
        };
        let player_manager = player::PLAYER_MANAGER.clone();
        let mut player_manager = match player_manager.lock() {
            Ok(player_manager) => {
                trace!("Successfully locked player manager mutex.");
                player_manager
            },
            Err(err) => {
                panic!("Failed to lock player manager mutex! Error: {:?}", err);
            },
        };
        let savegame_manager = savegame::SAVE_GAME_MANAGER.clone();
        let mut savegame_manager = match savegame_manager.lock() {
            Ok(savegame_manager) => {
                trace!("Successfully locked save game manager mutex.");
                savegame_manager
            },
            Err(err) => {
                panic!("Failed to lock save game manager mutex! Error: {:?}", err);
            },
        };
        let ui_manager = ui::UI_MANAGER.clone();
        let mut ui_manager = match ui_manager.lock() {
            Ok(ui_manager) => {
                trace!("Successfully locked UI manager mutex.");
                ui_manager
            },
            Err(err) => {
                panic!("Failed to lock UI manager mutex! Error: {:?}", err);
            },
        };
        let universe_manager = universe::UNIVERSE_MANAGER.clone();
        let mut universe_manager = match universe_manager.lock() {
            Ok(universe_manager) => {
                trace!("Successfully locked universe manager mutex.");
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
                debug!("Successfully initialized background module.");
            },
            Err(err) => {
                panic!("Failed to initialize background module! Error: {:?}", err);
            },
        }
        match camera_manager.initialize() {
            Ok(_) => {
                debug!("Successfully initialized camera module.");
            },
            Err(err) => {
                panic!("Failed to initialize camera module! Error: {:?}", err);
            },
        }
        match game_manager.initialize() {
            Ok(_) => {
                debug!("Successfully initialized game module.");
            },
            Err(err) => {
                panic!("Failed to initialize game module! Error: {:?}", err);
            },
        }
        match iteration_test_manager.initialize() {
            Ok(_) => {
                debug!("Successfully initialized iteration test module.");
            },
            Err(err) => {
                panic!("Failed to initialize iteration test module! Error: {:?}", err);
            },
        }
        match player_manager.initialize() {
            Ok(_) => {
                debug!("Successfully initialized player module.");
            },
            Err(err) => {
                panic!("Failed to initialize player module! Error: {:?}", err);
            },
        }
        match savegame_manager.initialize() {
            Ok(_) => {
                debug!("Successfully initialized save game module.");
            },
            Err(err) => {
                panic!("Failed to initialize save game module! Error: {:?}", err);
            },
        }
        match ui_manager.initialize() {
            Ok(_) => {
                debug!("Successfully initialized UI module.");
            },
            Err(err) => {
                panic!("Failed to initialize UI module! Error: {:?}", err);
            },
        }
        match universe_manager.initialize() {
            Ok(_) => {
                debug!("Successfully initialized universe module.");
            },
            Err(err) => {
                panic!("Failed to initialize universe module! Error: {:?}", err);
            },
        }

        info!("Initialized system modules.");

        self.state = ManagerState::Initialized;

        info!("Initialized system.");

        Ok(())
    }

    fn finalize(&mut self) -> Result<(), ManagerFinalizeError> {
        info!("Finalizing system...");

        match self.state {
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
                trace!("Successfully locked background manager mutex.");
                background_manager
            },
            Err(err) => {
                panic!("Failed to lock background manager mutex! Error: {:?}", err);
            },
        };
        let camera_manager = camera::CAMERA_MANAGER.clone();
        let mut camera_manager = match camera_manager.lock() {
            Ok(camera_manager) => {
                trace!("Successfully locked camera manager mutex.");
                camera_manager
            },
            Err(err) => {
                panic!("Failed to lock camera manager mutex! Error: {:?}", err);
            },
        };
        let game_manager = game::GAME_MANAGER.clone();
        let mut game_manager = match game_manager.lock() {
            Ok(game_manager) => {
                trace!("Successfully locked game manager mutex.");
                game_manager
            },
            Err(err) => {
                panic!("Failed to lock game manager mutex! Error: {:?}", err);
            },
        };
        let iteration_test_manager = iteration_test::ITERATION_TEST_MANAGER.clone();
        let mut iteration_test_manager = match iteration_test_manager.lock() {
            Ok(iteration_test_manager) => {
                trace!("Successfully locked iteration test manager mutex.");
                iteration_test_manager
            },
            Err(err) => {
                panic!("Failed to lock iteration test manager mutex! Error: {:?}", err);
            },
        };
        let player_manager = player::PLAYER_MANAGER.clone();
        let mut player_manager = match player_manager.lock() {
            Ok(player_manager) => {
                trace!("Successfully locked player manager mutex.");
                player_manager
            },
            Err(err) => {
                panic!("Failed to lock player manager mutex! Error: {:?}", err);
            },
        };
        let savegame_manager = savegame::SAVE_GAME_MANAGER.clone();
        let mut savegame_manager = match savegame_manager.lock() {
            Ok(savegame_manager) => {
                trace!("Successfully locked save game manager mutex.");
                savegame_manager
            },
            Err(err) => {
                panic!("Failed to lock save game manager mutex! Error: {:?}", err);
            },
        };
        let ui_manager = ui::UI_MANAGER.clone();
        let mut ui_manager = match ui_manager.lock() {
            Ok(ui_manager) => {
                trace!("Successfully locked UI manager mutex.");
                ui_manager
            },
            Err(err) => {
                panic!("Failed to lock UI manager mutex! Error: {:?}", err);
            },
        };
        let universe_manager = universe::UNIVERSE_MANAGER.clone();
        let mut universe_manager = match universe_manager.lock() {
            Ok(universe_manager) => {
                trace!("Successfully locked universe manager mutex.");
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
                debug!("Successfully finalized background module.");
            },
            Err(err) => {
                panic!("Failed to finalize background module! Error: {:?}", err);
            },
        }
        match camera_manager.finalize() {
            Ok(_) => {
                debug!("Successfully finalized camera module.");
            },
            Err(err) => {
                panic!("Failed to finalize camera module! Error: {:?}", err);
            },
        }
        match game_manager.finalize() {
            Ok(_) => {
                debug!("Successfully finalized game module.");
            },
            Err(err) => {
                panic!("Failed to finalize game module! Error: {:?}", err);
            },
        }
        match iteration_test_manager.finalize() {
            Ok(_) => {
                debug!("Successfully finalized iteration test module.");
            },
            Err(err) => {
                panic!("Failed to finalize iteration test module! Error: {:?}", err);
            },
        }
        match player_manager.finalize() {
            Ok(_) => {
                debug!("Successfully finalized player module.");
            },
            Err(err) => {
                panic!("Failed to finalize player module! Error: {:?}", err);
            },
        }
        match savegame_manager.finalize() {
            Ok(_) => {
                debug!("Successfully finalized save game module.");
            },
            Err(err) => {
                panic!("Failed to finalize save game module! Error: {:?}", err);
            },
        }
        match ui_manager.finalize() {
            Ok(_) => {
                debug!("Successfully finalized UI module.");
            },
            Err(err) => {
                panic!("Failed to finalize UI module! Error: {:?}", err);
            },
        }
        match universe_manager.finalize() {
            Ok(_) => {
                debug!("Successfully finalized universe module.");
            },
            Err(err) => {
                panic!("Failed to finalize universe module! Error: {:?}", err);
            },
        }

        info!("Finalized system modules.");

        self.state = ManagerState::Finalized;

        info!("Finalized system.");

        Ok(())
    }

    fn get_state(&self) -> &ManagerState {
        &self.state
    }
}

impl SystemManager {
    pub fn new() -> Self {
        Self {
            state: ManagerState::Created,
        }
    }
}

impl PluginGroup for SystemPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        group = group
            // Internal Modules
            .add(BackgroundPlugin)
            .add(CameraPlugin)
            .add(GamePlugin)
            .add(IterationTestPlugin)
            .add(PlayerPlugin)
            .add(SavegamePlugin)
            .add(UIPlugin)
            .add(UniversePlugin)
            // External Modules
            .add(RapierDebugRenderPlugin::default())
            .add(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));

        group
    }
}

// Module Functions
