use std::time::Instant;

use bevy::prelude::*;
use bevy::ecs::system::SystemState;
use bevy::render::MainWorld;

use crate::time::resources::VirtualPaused;

use super::resources::TimeInfo;
use super::types::{PauseState, StepConfig};

pub(super) fn post_update_game_time_info(
    mut time_info: ResMut<TimeInfo>,
    mut virtual_time: ResMut<Time<Virtual>>,
    mut cycle_counter: Local<Option<u32>>,
    mut step_timestamp: Local<Option<Instant>>,
    mut step_config: Local<Option<StepConfig>>
) {
    if virtual_time.is_paused() {
        match time_info.pause_state {
            PauseState::Running => time_info.pause_state = PauseState::Paused,
            PauseState::Paused => {},
            PauseState::Step => {
                virtual_time.unpause();
                match &time_info.step_config {
                    StepConfig::Cycles(max_cycles) => {
                        *cycle_counter = Some(0);
                        *step_config = Some(StepConfig::Cycles(*max_cycles));
                    },
                    StepConfig::Seconds(max_seconds) => {
                        *step_timestamp = Some(Instant::now());
                        *step_config = Some(StepConfig::Seconds(*max_seconds));
                    }
                }
            }
        }
    } else {
        match time_info.pause_state {
            PauseState::Running => {},
            PauseState::Paused => time_info.pause_state = PauseState::Running,
            PauseState::Step => {
                match (*step_config, time_info.step_config) {
                    (Some(StepConfig::Cycles(max_cycles)), StepConfig::Cycles(_)) => {
                        let counter = cycle_counter.as_mut().unwrap();
                        *counter += 1;

                        warn!("Stepping by cycles({}/{})...", *counter, max_cycles);

                        if *counter >= max_cycles {
                            virtual_time.pause();
                            time_info.pause_state = PauseState::Paused;
                            *cycle_counter = None;
                            *step_config = None;
                        }
                    },
                    (Some(StepConfig::Seconds(max_seconds)), StepConfig::Seconds(_)) => {
                        let start_time = &mut step_timestamp.unwrap();
                        let elapsed = start_time.elapsed().as_secs_f32();

                        warn!("Stepping by seconds({}/{})...", elapsed, max_seconds);

                        if elapsed >= max_seconds {
                            virtual_time.pause();
                            time_info.pause_state = PauseState::Paused;
                            *step_timestamp = None;
                            *step_config = None;
                        }
                    },
                    (None, StepConfig::Cycles(max_cycles)) => {
                        warn!("Initializing missing step config to cycles...");
                        *cycle_counter = Some(0);
                        *step_timestamp = None;
                        *step_config = Some(StepConfig::Cycles(max_cycles));
                    },
                    (None, StepConfig::Seconds(max_seconds)) => {
                        warn!("Initializing missing step config to seconds...");
                        *cycle_counter = None;
                        *step_timestamp = Some(Instant::now());
                        *step_config = Some(StepConfig::Seconds(max_seconds));
                    },
                    (Some(StepConfig::Cycles(_)), StepConfig::Seconds(max_seconds)) => {
                        warn!("Switching step config to seconds...");
                        *cycle_counter = None;
                        *step_timestamp = Some(Instant::now());
                        *step_config = Some(StepConfig::Seconds(max_seconds));
                    },
                    (Some(StepConfig::Seconds(_)), StepConfig::Cycles(max_cycles)) => {
                        warn!("Switching step config to cycles...");
                        *cycle_counter = Some(0);
                        *step_timestamp = None;
                        *step_config = Some(StepConfig::Cycles(max_cycles));
                    }
                }
            }
        }
    }
}

pub(super) fn extract_game_time_info(
    world: &mut World,
) {
    let main_world = SystemState::<Res<MainWorld>>::new(world).get(world);
    let extracted_time_info = match main_world.get_resource::<TimeInfo>() {
        Some(extracted_time_info) => extracted_time_info.clone(),
        None => unreachable!("TimeInfo resource not found"),
    };

    world.insert_resource(extracted_time_info);
}

pub(super) fn extract_virtual_paused(
    world: &mut World,
) {
    let main_world = SystemState::<Res<MainWorld>>::new(world).get(world);
    let extracted_virtual_paused = match main_world.get_resource::<VirtualPaused>() {
        Some(extracted_virtual_paused) => extracted_virtual_paused.clone(),
        None => unreachable!("VirtualPaused resource not found"),
    };

    world.insert_resource(extracted_virtual_paused);
}

pub(super) fn sync_virtual_paused(mut paused: ResMut<VirtualPaused>, time: Res<Time<Virtual>>) {
    paused.0 = time.is_paused();
}