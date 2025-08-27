use std::time::Instant;

use bevy::prelude::*;
use bevy::ecs::system::SystemState;
use bevy::render::MainWorld;

use crate::game::resources::GameTime;

pub(super) fn update_game_time(
    mut game_time: ResMut<GameTime>,
) {
    if game_time.paused && game_time.step_once {
        game_time.step_once = false;
    }

    let delta_seconds = Instant::now().duration_since(game_time.update_timestamp).as_secs_f32() * game_time.time_scale;
    game_time.elapsed_seconds += delta_seconds;
    game_time.delta_seconds = delta_seconds;
    game_time.update_timestamp = Instant::now();
}

pub(super) fn extract_game_time(
    world: &mut World,
) {
    let mut main_world = SystemState::<ResMut<MainWorld>>::new(world).get_mut(world);
    let extracted_game_time = match main_world.get_resource_mut::<GameTime>() {
        Some(mut buffer) => std::mem::take(&mut *buffer),
        None => unreachable!("Game time resource not found"),
    };

    world.insert_resource(extracted_game_time.clone());
}
