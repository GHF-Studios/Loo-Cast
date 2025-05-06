pub mod components;
pub mod systems;

pub mod workflows;

// pub mod workflows_MACROINPUT;
// pub mod workflows_MACROOUTPUT;

use bevy::prelude::*;
use systems::{
    chunk_inspection_system, chunk_loader_inspection_system, debug_object_movement_system,
};

pub(crate) struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, debug_ui_startup)
            .add_systems(
                Update,
                (
                    debug_object_movement_system,
                    chunk_inspection_system,
                    chunk_loader_inspection_system,
                ),
            );
    }
}

fn debug_ui_startup(
    mut has_spawned: Local<bool>,
    mut commands: Commands,
) {
    use iyes_perf_ui::{
        entries::{PerfUiFramerateEntries, PerfUiSystemEntries},
        prelude::{PerfUiEntryEntityCount, PerfUiRoot},
    };

    if !*has_spawned {
        *has_spawned = true;
        commands.spawn((
            PerfUiRoot::default(),
            PerfUiFramerateEntries::default(),
            PerfUiSystemEntries::default(),
            PerfUiEntryEntityCount::default(),
        ));
    }
}