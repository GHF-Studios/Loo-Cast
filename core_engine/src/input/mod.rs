pub mod states;
pub mod systems;

use bevy::prelude::*;
use states::InputMode;
use systems::toggle_input_mode;

pub(crate) struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<InputMode>()
            .add_systems(Update, toggle_input_mode)
            .register_type::<states::InputMode>();
    }
}
