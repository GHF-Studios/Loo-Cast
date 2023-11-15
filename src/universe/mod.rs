// Modules
pub mod chunk;
pub mod entity;
pub mod global;
pub mod local;

// Local imports
use chunk::data::*;
use chunk::id::*;
use chunk::metadata::*;
use chunk::pos::*;
use chunk::*;
use entity::data::*;
use entity::id::*;
use entity::metadata::*;
use entity::pos::*;
use entity::*;
use global::*;
use local::*;
use local::id::*;

// Internal imports
use crate::game::SimulationState;
use crate::AppState;

// External imports
use bevy::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Static variables

// Constant variables

// Types

// Enums

// Structs
pub struct UniversePlugin;

#[derive(Event)]
pub struct LoadGlobalUniverse {}

#[derive(Resource)]
pub struct UniverseManager {
    registered_global_universe: Option<GlobalUniverse>,
    registered_local_universes: HashMap<LocalUniverseID, LocalUniverse>,
}

// Implementations
impl Plugin for UniversePlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<LoadGlobalUniverse>()
            // Plugins
            .add_plugins((ChunkPlugin, EntityPlugin, GlobalUniversePlugin, LocalUniversePlugin))
            // Enter Systems
            .add_systems(OnEnter(AppState::Game), UniverseManager::initialize)
            // Update Systems
            .add_systems(
                Update,
                (UniverseManager::handle_load_global_universe,).run_if(in_state(AppState::Game)),
            )
            // Exit Systems
            .add_systems(OnExit(AppState::Game), UniverseManager::terminate);
    }
}

impl UniverseManager {
    fn initialize(mut commands: Commands) {
        let universe_manager = Self {
            registered_global_universe: None,
            registered_local_universes: HashMap::new(),
        };

        commands.insert_resource(universe_manager);
    }

    fn terminate(mut commands: Commands) {
        commands.remove_resource::<Self>();
    }

    pub fn handle_load_global_universe(
        mut load_global_universe_event_reader: EventReader<LoadGlobalUniverse>,
        mut universe_manager: ResMut<UniverseManager>
    ) {
        if let Some(_) = load_global_universe_event_reader.iter().last() {
            universe_manager.registered_global_universe = Some(GlobalUniverse {
                registered_root_chunks: Arc::new(Mutex::new(HashMap::new())),
                chunk_operation_requests: Arc::new(Mutex::new(Vec::new())),
                entity_operation_requests: Arc::new(Mutex::new(Vec::new())),
            });
        }
    }
}

// Module Functions
