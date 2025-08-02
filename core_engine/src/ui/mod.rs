pub mod custom_egui_widgets;
pub mod toolbar;

use bevy::prelude::*;

pub(crate) struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(toolbar::ToolbarPlugin);
    }
}
