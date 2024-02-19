// Modules
mod chunk;
pub mod commands;
mod entity;
mod global;
mod local;

// Local imports
use chunk::*;
use entity::*;
use global::*;
use local::id::*;
use local::*;

// Internal imports
use crate::system::player::*;
use crate::system::AppState;

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
            // Startup Systems
            .add_systems(Startup, UniverseManager::startup)
            // Enter Systems
            .add_systems(OnEnter(AppState::Game), UniverseManager::load_global_universe)
            // Update Systems
            .add_systems(
                Update,
                (UniverseManager::load_global_universe).run_if(in_state(AppState::Game)),
            )
            // Exit Systems
            .add_systems(OnExit(AppState::Game), UniverseManager::unload_global_universe);
    }
}

impl UniverseManager {
    fn startup(
        mut commands: Commands,
        mut initialize_player_event_writer: EventWriter<StartupPlayer>,
        mut rapier_configuration: ResMut<RapierConfiguration>,
    ) {
        rapier_configuration.gravity = Vec2::splat(0.0);

        let universe_manager = Self {
            registered_global_universe: None,
            registered_local_universes: HashMap::new(),
        };

        commands.insert_resource(universe_manager);

        initialize_player_event_writer.send(StartupPlayer {});
    }

    fn shutdown(
        mut commands: Commands,
        mut terminate_player_event_writer: EventWriter<TerminatePlayer>,
    ) {
        terminate_player_event_writer.send(TerminatePlayer {});
    }

    fn load_global_universe(mut universe_manager: ResMut<UniverseManager>) {
        universe_manager.registered_global_universe =
            Some(Arc::new(Mutex::new(GlobalUniverse {
                registered_root_chunks: HashMap::new(),
                operation_requests: Arc::new(Mutex::new(Vec::new())),
                chunk_entity_info_hierarchy: ChunkEntityInfoHierarchy::new(),
            })));
    }

    fn unload_global_universe(mut universe_manager: ResMut<UniverseManager>) {
        universe_manager.registered_global_universe = None;
    }

    pub(in self) fn get_global_universe(&self) -> Option<Arc<Mutex<GlobalUniverse>>> {
        self.registered_global_universe.clone()
    }

    pub(in self) fn register_local_universe(&mut self, local_universe: LocalUniverse) -> Result<(), String> {
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

    pub(in self) fn unregister_local_universe(
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

    pub(in self) fn get_local_universe(
        &self,
        local_universe_id: LocalUniverseID,
    ) -> Option<Arc<Mutex<LocalUniverse>>> {
        self.registered_local_universes
            .get(&local_universe_id)
            .cloned()
    }
}

// Module Functions
