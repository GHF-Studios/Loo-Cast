mod components;
mod resources;
mod systems;

use components::*;
use resources::*;
use systems::*;

use bevy::prelude::*;

pub struct UniversePlugin;

impl Plugin for UniversePlugin {
    fn build(&self, app: &mut App) {}
}
