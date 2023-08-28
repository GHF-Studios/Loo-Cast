pub mod components;
pub mod events;
pub mod resources;
mod systems;

use events::*;
use systems::*;

use crate::AppState;

use bevy::prelude::*;

pub struct UniversePlugin;

impl Plugin for UniversePlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<LoadUniverse>()
            .add_event::<ConfirmLoadedUniverse>()
            // Update Systems
            .add_systems(
                Update,
                handle_load_universe.run_if(in_state(AppState::Game)),
            );
    }
}
