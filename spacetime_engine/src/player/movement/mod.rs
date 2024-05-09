pub(in crate) mod constants;
pub(in crate) mod systems;

use bevy::prelude::*;

pub(in crate) struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, systems::update);
    }
}