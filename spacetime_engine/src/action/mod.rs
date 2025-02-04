pub mod errors;
pub mod events;
pub mod functions;
pub mod resources;
pub mod types;
pub mod systems;

pub mod stage_io;
pub mod stage;
pub mod target;

use bevy::prelude::*;
use events::*;
use resources::*;
use systems::*;

pub(in crate) struct ActionPlugin;
impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreUpdate, async_stage_event_relay_system);
    }
}