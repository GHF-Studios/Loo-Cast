use bevy::prelude::*;

use super::resources::TimeInfo;

pub fn run_if_not_paused(
    time_info: Option<Res<TimeInfo>>, 
) -> bool {
    //if let Some(time_info) = time_info {
    //    !time_info.pause_state.is_paused()
    //} else {
    //    warn!("TimeInfo resource not found. Assuming Virtual Time is not paused.");
    //    true
    //}
    true
}
