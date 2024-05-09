pub(in crate) mod constants;
pub(in crate) mod systems;

use systems::*;
use bevy::prelude::*;

pub(in crate) struct CreativePlugin;

impl Plugin for CreativePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update);
    }
}