use bevy::prelude::*;
use bevy::ecs::system::SystemState;
use bevy::render::MainWorld;

use super::resources::GameTimeInfo;
use super::types::PauseState;

pub(super) fn post_update_game_time_info(
    mut gti: ResMut<GameTimeInfo>,
    mut virtual_time: ResMut<Time<Virtual>>,
) {
    match (gti.pause_state.clone(), virtual_time.is_paused()) {
        (PauseState::Running, true) => warn!("GameTimeInfo indicates Running but Virtual Time is paused. Correcting GameTimeInfo to Paused."),
        (PauseState::Running, false) => warn!("GameTimeInfo indicates Running and Virtual Time is running. All good."),
        (PauseState::Paused, true) => warn!("GameTimeInfo indicates Paused and Virtual Time is paused. All good."),
        (PauseState::Paused, false) => warn!("GameTimeInfo indicates Paused but Virtual Time is running. Correcting GameTimeInfo to Running."),
        (PauseState::Step, true) => warn!("GameTimeInfo indicates Step and Virtual Time is paused. Allowing one frame to pass and unpausing Virtual Time."),
        (PauseState::Step, false) => warn!("GameTimeInfo indicates Step and Virtual Time is running. Resetting GameTimeInfo to Paused and pausing Virtual Time."),
    }

    if virtual_time.is_paused() {
        match gti.pause_state {
            PauseState::Running => gti.pause_state = PauseState::Paused, // Correct to Paused
            PauseState::Paused => {}, // All good
            PauseState::Step => {
                virtual_time.unpause(); // Allow one frame to pass
            }
        }
    } else {
        match gti.pause_state {
            PauseState::Running => {}, // All good
            PauseState::Paused => gti.pause_state = PauseState::Running, // Correct to Running
            PauseState::Step => {
                virtual_time.pause();
                gti.pause_state = PauseState::Paused; // Reset to paused after stepping
            }
        }
    }
}

pub(super) fn extract_game_time_info(
    world: &mut World,
) {
    let main_world = SystemState::<Res<MainWorld>>::new(world).get(world);
    let extracted_gti = match main_world.get_resource::<GameTimeInfo>() {
        Some(extracted_gti) => extracted_gti.clone(),
        None => unreachable!("Game time control resource not found"),
    };

    world.insert_resource(extracted_gti);
}