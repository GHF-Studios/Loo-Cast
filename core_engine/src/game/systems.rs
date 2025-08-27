use bevy::prelude::*;
use bevy::ecs::system::SystemState;
use bevy::render::MainWorld;

use crate::game::resources::GameTimeControl;

pub(super) fn apply_game_time_control(
    mut game_time_control: ResMut<GameTimeControl>,
    mut time: ResMut<Time<Virtual>>,
) {
    if game_time_control.paused {
        if game_time_control.step_once {
            game_time_control.step_once = false;
        } else {
            time.pause();
        }
    } else {
        time.unpause();
    }
}

pub(super) fn extract_game_time_control(
    world: &mut World,
) {
    let mut main_world = SystemState::<ResMut<MainWorld>>::new(world).get_mut(world);
    let extracted_game_time_control = match main_world.get_resource_mut::<GameTimeControl>() {
        Some(mut buffer) => std::mem::take(&mut *buffer),
        None => unreachable!("Game time control resource not found"),
    };

    world.insert_resource(extracted_game_time_control.clone());
}