// Internal imports
use crate::AppState;

// External imports
use bevy::prelude::*;

// Events
#[derive(Event)]
pub struct LoadUniverse {}

// Resources
#[derive(Resource)]
pub struct UniverseManager {
    pub current_scale_level: i8,
    pub current_chunk_offset_x: i16,
    pub current_chunk_offset_y: i16,
}

// Structs
pub struct UniversePlugin;

// Implementations
impl Plugin for UniversePlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<LoadUniverse>()
            // Update Systems
            .add_systems(
                Update,
                (
                    UniverseManager::handle_load_universe,
                )
                    .run_if(in_state(AppState::Game)),
            );
    }
}

impl UniverseManager {
    pub fn new() -> Self {
        Self {
            current_scale_level: 0,
            current_chunk_offset_x: 0,
            current_chunk_offset_y: 0,
        }
    }

    pub fn handle_load_universe(
        mut commands: Commands,
        mut load_universe_event_reader: EventReader<LoadUniverse>,
    ) {
        if let Some(_) = load_universe_event_reader.iter().last() {
            commands.insert_resource(UniverseManager {
                current_scale_level: 0,
                current_chunk_offset_x: 0,
                current_chunk_offset_y: 0,
            });
        }
    }
}
