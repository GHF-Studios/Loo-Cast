use bevy::prelude::*;

use super::resources::GameTime;

pub fn run_if_game_running(game_time: Option<Res<GameTime>>) -> bool {
    if let Some(game_time) = game_time {
        !game_time.paused || game_time.step_once
    } else {
        warn!("GameTime resource not found when checking if game is running. Assuming game is running.");
        true
    }
}
