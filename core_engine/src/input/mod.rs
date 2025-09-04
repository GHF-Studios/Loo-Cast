pub mod states;
pub mod systems;

use bevy::prelude::*;
use states::InputMode;
use systems::toggle_input_mode;

use crate::core::run_conditions::run_after_startup_finished;

pub(crate) struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<InputMode>()
            .add_systems(Update, toggle_input_mode.run_if(run_after_startup_finished))
            .register_type::<states::InputMode>();
    }
}
