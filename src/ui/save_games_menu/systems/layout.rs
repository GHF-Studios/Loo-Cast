use crate::save_game::resources::*;
use crate::ui::save_games_menu::components::*;
use crate::ui::save_games_menu::styles::*;
use crate::ui::styles::*;

use bevy::prelude::*;

pub fn spawn_save_games_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    save_game_manager: Res<SaveGameManager>,
) {
    build_save_games_menu(&mut commands, &asset_server, &save_game_manager);
}

pub fn despawn_save_games_menu(
    mut commands: Commands,
    save_games_menu_query: Query<Entity, With<SaveGamesMenu>>,
) {
    if let Ok(save_games_menu_entity) = save_games_menu_query.get_single() {
        commands.entity(save_games_menu_entity).despawn_recursive();
    }
}

fn build_save_games_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    save_game_manager: &Res<SaveGameManager>,
) -> Entity {
    let save_games_menu_entity = commands
        .spawn((
            NodeBundle {
                style: SAVE_GAMES_MENU_STYLE,
                ..default()
            },
            SaveGamesMenu {},
        ))
        .with_children(|parent| {
            // === Title ===
            parent
                .spawn(NodeBundle {
                    style: TITLE_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    // Text
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Save Games",
                                get_title_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            // Save Games Container
            parent
                .spawn((
                    NodeBundle {
                        style: SAVE_GAMES_CONTAINER_STYLE,
                        ..default()
                    },
                    SaveGamesContainer {},
                ))
                .with_children(|parent| {
                    // Save Games
                    for save_game_info in save_game_manager.registered_save_games.iter() {
                        parent
                            .spawn((
                                NodeBundle {
                                    style: SAVE_GAME_STYLE,
                                    background_color: BACKGROUND_COLOR.into(),
                                    ..default()
                                },
                                SaveGame {
                                    name: save_game_info.name.clone(),
                                },
                            ))
                            .with_children(|parent| {
                                // Delete Save Game Button
                                parent
                                    .spawn((
                                        ButtonBundle {
                                            style: BUTTON_STYLE,
                                            background_color: NORMAL_BUTTON_COLOR.into(),
                                            ..default()
                                        },
                                        DeleteSaveGameButton {
                                            save_game_name: save_game_info.name.clone(),
                                        },
                                    ))
                                    .with_children(|parent| {
                                        parent.spawn(ImageBundle {
                                            style: BUTTON_IMAGE_STYLE,
                                            image: asset_server.load("sprites/cross.png").into(),
                                            ..default()
                                        });
                                    });
                                // Save Game Name Text
                                parent
                                    .spawn(NodeBundle {
                                        style: SAVE_GAME_NAME_STYLE,
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        // Text
                                        parent.spawn(TextBundle {
                                            text: Text {
                                                sections: vec![TextSection::new(
                                                    save_game_info.name.clone(),
                                                    get_save_game_name_text_style(&asset_server),
                                                )],
                                                alignment: TextAlignment::Center,
                                                ..default()
                                            },
                                            ..default()
                                        });
                                    });
                                // Load Save Game Button
                                parent
                                    .spawn((
                                        ButtonBundle {
                                            style: BUTTON_STYLE,
                                            background_color: NORMAL_BUTTON_COLOR.into(),
                                            ..default()
                                        },
                                        LoadSaveGameButton {
                                            save_game_name: save_game_info.name.clone(),
                                        },
                                    ))
                                    .with_children(|parent| {
                                        parent.spawn(ImageBundle {
                                            style: BUTTON_IMAGE_STYLE,
                                            image: asset_server.load("sprites/enter.png").into(),
                                            ..default()
                                        });
                                    });
                            });
                    }
                });
            // Button Container
            parent
                .spawn(NodeBundle {
                    style: BUTTON_CONTAINER_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    // Back To Main Menu Button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: BUTTON_STYLE,
                                background_color: NORMAL_BUTTON_COLOR.into(),
                                ..default()
                            },
                            BackToMainMenuButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(ImageBundle {
                                style: BUTTON_IMAGE_STYLE,
                                image: asset_server.load("sprites/return.png").into(),
                                ..default()
                            });
                        });

                    // Create Save Game Button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: BUTTON_STYLE,
                                background_color: NORMAL_BUTTON_COLOR.into(),
                                ..default()
                            },
                            CreateSaveGameButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(ImageBundle {
                                style: BUTTON_IMAGE_STYLE,
                                image: asset_server.load("sprites/plus.png").into(),
                                ..default()
                            });
                        });
                });
        })
        .id();

    save_games_menu_entity
}
