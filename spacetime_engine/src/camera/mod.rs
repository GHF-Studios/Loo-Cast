pub(in crate) mod constants;
pub(in crate) mod systems;

use bevy::prelude::*;

pub(in crate) struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::startup);
    }
}