use bevy::prelude::*;

use super::resources::GameTimeControl;

pub fn run_if_game_running(game_time_control: Option<Res<GameTimeControl>>) -> bool {
    //if let Some(gtc) = game_time_control {
    //    !gtc.paused || gtc.step_once
    //} else {
    //    warn!("GameTimeControl resource not found when checking if game is running. Assuming game is running.");
    //    true
    //}
    true
}
