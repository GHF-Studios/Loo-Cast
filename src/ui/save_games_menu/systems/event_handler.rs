use crate::save_game::events::*;
use crate::ui::save_games_menu::components::*;
use crate::ui::save_games_menu::systems::layout::*;

use bevy::prelude::*;

pub fn handle_created_save_game(
    mut commands: Commands,
    mut created_save_game_event_reader: EventReader<CreatedSaveGame>,
    asset_server: Res<AssetServer>,
    save_games_container_query: Query<Entity, With<SaveGamesContainer>>,
) {
    if let Ok(save_games_container_entity) = save_games_container_query.get_single() {
        for event in created_save_game_event_reader.iter() {
            spawn_save_game(
                &mut commands,
                save_games_container_entity,
                &asset_server,
                event.save_game_name.to_string(),
            );
        }
    }
}

pub fn handle_deleted_save_game(
    mut commands: Commands,
    mut deleted_save_game_event_reader: EventReader<DeletedSaveGame>,
    save_game_query: Query<(Entity, &SaveGame)>,
) {
    for event in deleted_save_game_event_reader.iter() {
        despawn_save_game(
            &mut commands,
            &save_game_query,
            event.save_game_name.to_string(),
        );
    }
}
