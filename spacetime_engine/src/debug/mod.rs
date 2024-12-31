pub mod components;
pub mod functions;
pub mod systems;

use bevy::prelude::*;
use systems::test_object_movement_system;

pub(in crate) struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, test_object_movement_system);
    }
}