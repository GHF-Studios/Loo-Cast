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
use local::id::*;
use local::*;

// Internal imports
use crate::game::SimulationState;
use crate::AppState;
use crate::player::*;

// External imports
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
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
    registered_global_universe: Option<Arc<Mutex<GlobalUniverse>>>,
    registered_local_universes: HashMap<LocalUniverseID, Arc<Mutex<LocalUniverse>>>,
}

// Implementations
impl Plugin for UniversePlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<LoadGlobalUniverse>()
            // Plugins
            .add_plugins((
                ChunkPlugin,
                EntityPlugin,
                GlobalUniversePlugin,
                LocalUniversePlugin,
            ))
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
    fn initialize(
        mut commands: Commands,
        mut initialize_player_event_writer: EventWriter<InitializePlayer>,
        mut rapier_configuration: ResMut<RapierConfiguration>,
    ) {
        rapier_configuration.gravity = Vec2::splat(0.0);

        let universe_manager = Self {
            registered_global_universe: None,
            registered_local_universes: HashMap::new(),
        };

        commands.insert_resource(universe_manager);

        initialize_player_event_writer.send(InitializePlayer {});
    }

    fn terminate(
        mut commands: Commands,
        mut terminate_player_event_writer: EventWriter<TerminatePlayer>,
    ) {
        terminate_player_event_writer.send(TerminatePlayer {});

        commands.remove_resource::<Self>();
    }

    pub fn handle_load_global_universe(
        mut load_global_universe_event_reader: EventReader<LoadGlobalUniverse>,
        mut universe_manager: ResMut<UniverseManager>,
    ) {
        if let Some(_) = load_global_universe_event_reader.iter().last() {
            universe_manager.registered_global_universe = Some(Arc::new(Mutex::new(GlobalUniverse {
                registered_root_chunks: Arc::new(Mutex::new(HashMap::new())),
                operation_requests: Arc::new(Mutex::new(Vec::new())),
            })));
        }
    }

    pub fn get_global_universe(&self) -> Option<Arc<Mutex<GlobalUniverse>>> {
        self.registered_global_universe.clone()
    }

    pub fn register_local_universe(&mut self, local_universe: LocalUniverse) -> Result<(), String> {
        let local_universe_id = local_universe.get_id();

        if self.registered_local_universes.contains_key(local_universe_id) {
            return Err(format!(
                "Local universe with ID {} is already registered.",
                local_universe_id.get_id()
            ));
        }

        self.registered_local_universes.insert(local_universe_id.clone(), Arc::new(Mutex::new(local_universe)));

        Ok(())
    }

    pub fn unregister_local_universe(&mut self, local_universe_id: LocalUniverseID) -> Result<(), String> {
        match self.registered_local_universes.remove(&local_universe_id) {
            Some(_) => Ok(()),
            None => Err(format!(
                "Local universe with ID {:?} is already unregistered.",
                local_universe_id
            )),
        }
    }

    pub fn get_local_universe(&self, local_universe_id: LocalUniverseID) -> Option<Arc<Mutex<LocalUniverse>>> {
        self.registered_local_universes.get(&local_universe_id).cloned()
    }
}

// Module Functions
