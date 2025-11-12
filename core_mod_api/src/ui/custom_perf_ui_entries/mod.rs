pub mod cursor_position;
pub mod player_position;

use bevy::prelude::*;
use iyes_perf_ui::PerfUiAppExt;
use cursor_position::{PerfUiEntryCursorScreenPos, PerfUiEntryCursorUnitPos, PerfUiEntryGamePickingCursorScreenPos, PerfUiEntryDebugPickingCursorScreenPos};
use player_position::{PerfUiEntryPlayerGridPos, PerfUiEntryPlayerUnitPos};

pub(crate) struct CustomPerfUiEntriesPlugin;
impl Plugin for CustomPerfUiEntriesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_perf_ui_simple_entry::<PerfUiEntryCursorScreenPos>()
            .add_perf_ui_simple_entry::<PerfUiEntryCursorUnitPos>()
            .add_perf_ui_simple_entry::<PerfUiEntryPlayerGridPos>()
            .add_perf_ui_simple_entry::<PerfUiEntryPlayerUnitPos>()
            .add_perf_ui_simple_entry::<PerfUiEntryGamePickingCursorScreenPos>()
            .add_perf_ui_simple_entry::<PerfUiEntryDebugPickingCursorScreenPos>()
            .register_type::<PerfUiEntryCursorScreenPos>()
            .register_type::<PerfUiEntryCursorUnitPos>()
            .register_type::<PerfUiEntryPlayerGridPos>()
            .register_type::<PerfUiEntryPlayerUnitPos>()
            .register_type::<PerfUiEntryGamePickingCursorScreenPos>()
            .register_type::<PerfUiEntryDebugPickingCursorScreenPos>();
    }
}
