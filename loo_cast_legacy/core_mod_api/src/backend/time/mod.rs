pub mod errors;
pub mod functions;
pub mod resources;
pub mod run_conditions;
pub mod statics;
pub mod systems;
pub mod types;

use crate::bevy::prelude::*;
use crate::bevy::render::RenderApp;
use resources::{TimeInfo, VirtualPaused};
use systems::{
    configure_virtual_time, extract_game_time_info, extract_virtual_paused, post_update_game_time_info, sync_elapsed_virtual_time, sync_virtual_paused,
    wake_virtual_sleeps_system,
};
use types::{PauseState, StepConfig};

pub(crate) struct TimePlugin;
impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TimeInfo::default())
            .insert_resource(VirtualPaused::default())
            .add_systems(Startup, configure_virtual_time)
            .add_systems(PreUpdate, (sync_virtual_paused, sync_elapsed_virtual_time))
            .add_systems(PreUpdate, wake_virtual_sleeps_system.after(sync_elapsed_virtual_time))
            .add_systems(PostUpdate, post_update_game_time_info)
            .register_type::<TimeInfo>()
            .register_type::<VirtualPaused>()
            .register_type::<PauseState>()
            .register_type::<StepConfig>();

        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .add_systems(ExtractSchedule, extract_game_time_info)
            .add_systems(ExtractSchedule, extract_virtual_paused);
    }
}
