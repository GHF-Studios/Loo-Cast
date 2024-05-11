pub(in crate) mod systems;

use systems::*;
use bevy::prelude::*;

pub(in crate) struct TeleportationPlugin;

impl Plugin for TeleportationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update);
    }
}