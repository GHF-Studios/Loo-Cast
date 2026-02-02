pub mod bundles;
pub mod components;
pub mod systems;

use crate::bevy::prelude::*;
use bundles::PlayerBundle;
use components::Player;
use systems::update_player_system;

use crate::core::run_conditions::run_after_startup_finished;
use crate::time::run_conditions::run_if_not_paused;

pub(crate) struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_player_system.run_if(run_after_startup_finished.and(run_if_not_paused)))
            .register_type::<PlayerBundle>()
            .register_type::<Player>();
    }
}
