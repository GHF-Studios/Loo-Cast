use super::components::*;
use super::events::*;

use crate::game::resources::GameManager;
use crate::save_game::structs::SaveGameInfo;

use bevy::prelude::*;

pub fn handle_load_universe(
    mut load_universe_event_reader: EventReader<LoadUniverse>,
    game_manager: Res<GameManager>,
) {
    if let Some(_) = load_universe_event_reader.iter().last() {
        let save_game_info: SaveGameInfo = game_manager.current_save_game.clone();
    }
}
