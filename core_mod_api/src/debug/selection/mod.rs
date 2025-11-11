pub mod systems;

use bevy::prelude::*;
use systems::handle_selection;

use crate::core::run_conditions::run_after_startup_finished;

pub(crate) struct SelectionPlugin;
impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                handle_selection,
            ).run_if(run_after_startup_finished));
    }
}
