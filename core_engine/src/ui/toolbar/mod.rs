pub mod resources;
pub mod systems;

use bevy::prelude::*;

use resources::ToolbarState;
use systems::show_toolbar_ui;

pub(crate) struct ToolbarPlugin;
impl Plugin for ToolbarPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ToolbarState::default()).add_systems(Update, show_toolbar_ui);
    }
}
