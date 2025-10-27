pub mod constants;
pub mod functions;
pub mod resources;
pub mod run_conditions;
pub mod schedule_systems;
pub mod schedules;
pub mod statics;
pub mod systems;
pub mod types;

pub mod workflows;

use bevy::prelude::*;

use systems::startup_system;

pub(crate) struct CorePlugin;
impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, startup_system);
    }
}
