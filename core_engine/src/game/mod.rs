pub mod resources;
pub mod run_conditions;
pub mod states;
pub mod systems;

use bevy::prelude::*;
use bevy::render::RenderApp;
use resources::GameTime;
use systems::{extract_game_time, update_game_time};

use crate::game::run_conditions::run_if_game_running;

pub(crate) struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GameTime::default())
            .add_systems(PostUpdate, update_game_time.run_if(run_if_game_running))
            .register_type::<GameTime>();

        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .add_systems(ExtractSchedule, extract_game_time.run_if(run_if_game_running));
    }
}