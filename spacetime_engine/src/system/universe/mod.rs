// Modules
pub mod chunk;
pub mod entity;
pub mod global;
pub mod local;

// Local imports
use chunk::*;
use entity::*;
use global::*;
use local::id::*;
use local::*;

// Internal imports
use crate::kernel::manager::*;
use crate::system::player::*;
use crate::system::AppState;
use crate::system::game::SimulationState;

// External imports
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::HashMap;
use lazy_static::*;
use std::sync::{Arc, Mutex};

// Static variables
lazy_static! {
    pub static ref UNIVERSE_MANAGER: Arc<Mutex<UniverseManager>> = Arc::new(Mutex::new(UniverseManager::new()));
}

// Constant variables

// Types

// Enums
pub enum UniverseState {
    
}

// Structs
pub struct UniversePlugin;

#[derive(Resource)]
pub struct UniverseManager {
    universe_state: UniverseState,
    manager_state: ManagerState,
    registered_global_universe: Option<Arc<Mutex<GlobalUniverse>>>,
    registered_local_universes: HashMap<LocalUniverseID, Arc<Mutex<LocalUniverse>>>,
}

// Implementations
impl Plugin for UniversePlugin {
    fn build(&self, app: &mut App) {
        app
            // Enter Systems
            .add_systems(OnEnter(AppState::Game), UniverseManager::startup)
            // Update Systems
            .add_systems(
                Update,(
                    Chunk::debug_render_system, 
                    GlobalUniverse::handle_operation_requests,
                    LocalUniverse::detect_local_chunks_system)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            // Exit Systems
            .add_systems(OnExit(AppState::Game), UniverseManager::shutdown);
    }
}

impl Manager for UniverseManager {
    fn initialize(&mut self) -> Result<(), ManagerInitializeError> {
        info!("Initializing universe main module...");

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

        info!("Initialized universe main module.");

        Ok(())
    }

    fn finalize(&mut self) -> Result<(), ManagerFinalizeError> {
        info!("Finalizing universe main module...");

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

        info!("Finalized universe main module.");

        Ok(())
    }

    fn get_manager_state(&self) -> &ManagerState {
        &self.manager_state
    }
}

impl UniverseManager {
    fn new() -> Self {
        UniverseManager {
            manager_state: ManagerState::Created,
            registered_global_universe: None,
            registered_local_universes: HashMap::new(),
        }
    }

    fn startup(
        mut rapier_configuration: ResMut<RapierConfiguration>,
    ) {
        rapier_configuration.gravity = Vec2::splat(0.0);
    }

    fn shutdown() {
    }

    fn register_global_universe() {
        let universe_manager = UNIVERSE_MANAGER.clone();
        let mut universe_manager = match universe_manager.lock() {
            Ok(mut universe_manager) => {
                trace!("Universe manager locked.");
                universe_manager
            },
            Err(_) => {
                panic!("Failed to lock universe manager.");
            }
        };

        match universe_manager.registered_global_universe {
            Some(_) => {
                error!("Global universe is already registered.");
                return;
            }
            None => {}
        };

        universe_manager.registered_global_universe =
            Some(Arc::new(Mutex::new(GlobalUniverse {
                registered_root_chunks: HashMap::new(),
                operation_requests: Arc::new(Mutex::new(Vec::new())),
                chunk_entity_info_hierarchy: ChunkEntityInfoHierarchy::new(),
            })));
    }

    pub fn get_global_universe(&self) -> Option<Arc<Mutex<GlobalUniverse>>> {
        self.registered_global_universe.clone()
    }

    pub fn register_local_universe(&mut self, local_universe: LocalUniverse) -> Result<(), String> {
        let local_universe_id = local_universe.get_id();

        if self
            .registered_local_universes
            .contains_key(local_universe_id)
        {
            return Err(format!(
                "Local universe with ID {} is already registered.",
                local_universe_id.get_id()
            ));
        }

        self.registered_local_universes
            .insert(*local_universe_id, Arc::new(Mutex::new(local_universe)));

        Ok(())
    }

    pub fn unregister_local_universe(
        &mut self,
        local_universe_id: LocalUniverseID,
    ) -> Result<(), String> {
        match self.registered_local_universes.remove(&local_universe_id) {
            Some(_) => Ok(()),
            None => Err(format!(
                "Local universe with ID {:?} is already unregistered.",
                local_universe_id
            )),
        }
    }

    pub fn get_local_universe(
        &self,
        local_universe_id: LocalUniverseID,
    ) -> Option<Arc<Mutex<LocalUniverse>>> {
        self.registered_local_universes
            .get(&local_universe_id)
            .cloned()
    }
}

// Module Functions
