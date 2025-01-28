pub mod errors;
pub mod events;
pub mod resources;
pub mod structs;
pub mod systems;
pub mod traits;

pub mod stage_io;

use bevy::prelude::*;
use events::*;
use resources::*;
use systems::*;

pub(in crate) struct ActionPlugin;
impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
    }
}