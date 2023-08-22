use crate::ui::create_save_game_menu::components::*;
use crate::ui::create_save_game_menu::styles::*;
use crate::ui::styles::*;

use bevy::prelude::*;

pub fn spawn_create_save_game_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_create_save_game_menu(&mut commands, &asset_server);
}

pub fn despawn_create_save_game_menu(
    mut commands: Commands,
    create_save_game_menu_query: Query<Entity, With<CreateSaveGameMenu>>,
) {
    if let Ok(create_save_game_menu_entity) = create_save_game_menu_query.get_single() {
        commands.entity(create_save_game_menu_entity).despawn_recursive();
    }
}

pub fn build_create_save_game_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let create_save_game_menu_entity = commands
        .spawn((
            NodeBundle {
                style: CREATE_SAVE_GAME_MENU_STYLE,
                background_color: BACKGROUND_COLOR,
                ..default()
            },
            CreateSaveGameMenu {},
        ))
        .id();

    create_save_game_menu_entity
}
