pub mod resources;
pub mod run_conditions;
pub mod systems;
pub mod types;

use bevy::prelude::*;
use bevy::render::RenderApp;
use resources::GameTimeInfo;
use systems::{extract_game_time_info, post_update_game_time_info};
use types::PauseState;

pub(crate) struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GameTimeInfo::default())
            .add_systems(PostUpdate, post_update_game_time_info)
            .register_type::<GameTimeInfo>()
            .register_type::<PauseState>();

        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .add_systems(ExtractSchedule, extract_game_time_info);
    }
}