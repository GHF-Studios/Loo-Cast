mod components;
mod resources;
mod systems;

use components::*;
use resources::*;
use systems::*;

use crate::AppState;

use bevy::prelude::*;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct UniversePlugin;

impl Plugin for UniversePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(PreStartup, create_universe)
        .add_systems(Startup, save_universe);
    }
}