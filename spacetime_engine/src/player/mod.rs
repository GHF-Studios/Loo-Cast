pub mod bundles;
pub mod components;
pub mod constants;
pub mod systems;

use bevy::prelude::*;
use systems::update_player_system;

pub(in crate) struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update_player_system);
    }
}