use bevy::prelude::*;

use super::resources::GameTimeInfo;

pub fn run_if_game_running(
    game_time_info: Option<Res<GameTimeInfo>>, 
) -> bool {
    //if let Some(gtc) = game_time_info {
    //    !gtc.pause_state.is_paused()
    //} else {
    //    warn!("GameTimeInfo resource not found when checking if game is running. Assuming game is running.");
    //    true
    //}
    true
}
