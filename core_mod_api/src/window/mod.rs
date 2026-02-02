pub mod systems;

use crate::bevy::prelude::*;
use systems::*;

use crate::core::run_conditions::run_after_startup_finished;

pub(crate) struct WindowPlugin;
impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, toggle_window_mode.run_if(run_after_startup_finished));
    }
}
