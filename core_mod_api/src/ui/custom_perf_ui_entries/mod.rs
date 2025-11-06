pub mod player_position;

use bevy::prelude::*;
use iyes_perf_ui::PerfUiAppExt;
use player_position::{PerfUiEntryPlayerGridOffset, PerfUiEntryPlayerUnitOffset};

pub(crate) struct CustomPerfUiEntriesPlugin;
impl Plugin for CustomPerfUiEntriesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_perf_ui_simple_entry::<PerfUiEntryPlayerGridOffset>()
            .add_perf_ui_simple_entry::<PerfUiEntryPlayerUnitOffset>();
    }
}
