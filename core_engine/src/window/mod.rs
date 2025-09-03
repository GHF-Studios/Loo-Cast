pub mod systems;

use bevy::prelude::*;
use systems::*;

pub(crate) struct WindowPlugin;
impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, toggle_window_mode);
    }
}
