pub mod cursor_position;
pub mod player_position;

use bevy::prelude::*;
use iyes_perf_ui::PerfUiAppExt;
use cursor_position::{PerfUiEntryViewportRect, PerfUiEntryCursorWindowPos, PerfUiEntryCursorPointerPos, PerfUiEntryCursorUnitPos, PerfUiEntryCursorUnitPosUnprojected};
use player_position::{PerfUiEntryPlayerGridPos, PerfUiEntryPlayerUnitPos};

pub(crate) struct CustomPerfUiEntriesPlugin;
impl Plugin for CustomPerfUiEntriesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_perf_ui_simple_entry::<PerfUiEntryViewportRect>()
            .add_perf_ui_simple_entry::<PerfUiEntryCursorWindowPos>()
            .add_perf_ui_simple_entry::<PerfUiEntryCursorPointerPos>()
            .add_perf_ui_simple_entry::<PerfUiEntryCursorUnitPos>()
            .add_perf_ui_simple_entry::<PerfUiEntryCursorUnitPosUnprojected>()
            .add_perf_ui_simple_entry::<PerfUiEntryPlayerGridPos>()
            .add_perf_ui_simple_entry::<PerfUiEntryPlayerUnitPos>()
            .register_type::<PerfUiEntryViewportRect>()
            .register_type::<PerfUiEntryCursorWindowPos>()
            .register_type::<PerfUiEntryCursorPointerPos>()
            .register_type::<PerfUiEntryCursorUnitPos>()
            .register_type::<PerfUiEntryCursorUnitPosUnprojected>()
            .register_type::<PerfUiEntryPlayerGridPos>()
            .register_type::<PerfUiEntryPlayerUnitPos>();
    }
}
