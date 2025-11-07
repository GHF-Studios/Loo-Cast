pub mod custom_egui_widgets;
pub mod custom_perf_ui_entries;

use bevy::prelude::*;

pub(crate) struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(custom_egui_widgets::CustomEguiWidgetsPlugin)
            .add_plugins(custom_perf_ui_entries::CustomPerfUiEntriesPlugin);
    }
}
