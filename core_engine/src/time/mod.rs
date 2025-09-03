pub mod resources;
pub mod run_conditions;
pub mod systems;
pub mod types;

use bevy::prelude::*;
use bevy::render::RenderApp;
use resources::{TimeInfo, VirtualPaused};
use systems::{configure_virtual_time, extract_game_time_info, extract_virtual_paused, post_update_game_time_info, sync_virtual_paused};
use types::PauseState;

pub(crate) struct TimePlugin;
impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TimeInfo::default())
            .insert_resource(VirtualPaused::default())
            .add_systems(Startup, configure_virtual_time)
            .add_systems(PreUpdate, sync_virtual_paused)
            .add_systems(PostUpdate, post_update_game_time_info)
            .register_type::<TimeInfo>()
            .register_type::<VirtualPaused>()
            .register_type::<PauseState>();

        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .add_systems(ExtractSchedule, extract_game_time_info)
            .add_systems(ExtractSchedule, extract_virtual_paused);
    }
}
