pub mod components;
pub mod functions;
pub mod resources;
pub mod run_conditions;
pub mod schedules;
pub mod statics;
pub mod systems;
pub mod types;

pub mod workflows;

use crate::bevy::prelude::*;

use systems::startup_system;
use types::{Diegetic, Meta, ShortTime};

pub(crate) struct CorePlugin;
impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup_system)
            .register_type::<ShortTime>()
            .register_type::<Diegetic>()
            .register_type::<Meta>();
    }
}
