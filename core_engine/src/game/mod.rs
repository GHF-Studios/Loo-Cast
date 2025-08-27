pub mod resources;
pub mod run_conditions;
pub mod states;
pub mod systems;

use bevy::prelude::*;
use bevy::render::RenderApp;
use resources::GameTimeControl;
use systems::{extract_game_time_control, apply_game_time_control};

use crate::game::run_conditions::run_if_game_running;

pub(crate) struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GameTimeControl::default())
            .add_systems(PostUpdate, apply_game_time_control)
            .register_type::<GameTimeControl>();

        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .add_systems(ExtractSchedule, extract_game_time_control);
    }
}