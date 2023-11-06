// Modules
pub mod chunk;
pub mod cluster;
pub mod entity;

// Local imports
use chunk::*;
use cluster::*;
use entity::*;

// Internal imports
use crate::AppState;

// External imports
use bevy::prelude::*;

// Static variables

// Constant variables

// Types

// Enums

// Structs
pub struct UniversePlugin;

#[derive(Event)]
pub struct LoadUniverse {}

#[derive(Resource)]
pub struct Universe {
    pub current_scale_level: i8,
    pub current_chunk_offset_x: i16,
    pub current_chunk_offset_y: i16,
}

// Implementations
impl Plugin for UniversePlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<LoadUniverse>()
            // Plugins
            .add_plugins(ChunkPlugin)
            // Update Systems
            .add_systems(
                Update,
                (Universe::handle_load_universe,).run_if(in_state(AppState::Game)),
            );
    }
}

impl Universe {
    pub fn handle_load_universe(
        mut commands: Commands,
        mut load_universe_event_reader: EventReader<LoadUniverse>,
    ) {
        if let Some(_) = load_universe_event_reader.iter().last() {
            commands.insert_resource(Universe {
                current_scale_level: 0,
                current_chunk_offset_x: 0,
                current_chunk_offset_y: 0,
            });
        }
    }
}

// Module Functions
