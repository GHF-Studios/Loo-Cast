// Modules
pub mod background;
pub mod camera;
pub mod game;
pub mod iteration_test;
pub mod player;
pub mod save_game;
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
use save_game::SaveGamePlugin;
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
    SaveGamesMenu,
    CreateSaveGameMenu,
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

        debug!("Locked system module manager mutexes.");

        info!("Initializing system modules...");

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

        debug!("Locked system module manager mutexes.");

        info!("Finalizing system modules...");

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
            .add(SaveGamePlugin)
            .add(UIPlugin)
            .add(UniversePlugin)
            // External Modules
            .add(RapierDebugRenderPlugin::default())
            .add(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));

        group
    }
}

// Module Functions
