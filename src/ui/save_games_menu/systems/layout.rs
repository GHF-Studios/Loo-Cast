use crate::ui::save_games_menu::components::*;
use crate::ui::save_games_menu::styles::*;
use crate::ui::styles::*;

use bevy::prelude::*;

pub fn spawn_save_games_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_save_games_menu(&mut commands, &asset_server);
}

pub fn despawn_save_games_menu(
    mut commands: Commands,
    save_games_menu_query: Query<Entity, With<SaveGamesMenu>>,
) {
    if let Ok(save_games_menu_entity) = save_games_menu_query.get_single() {
        commands.entity(save_games_menu_entity).despawn_recursive();
    }
}

pub fn spawn_save_game(
    commands: &mut Commands,
    save_games_container_entity: Entity,
    asset_server: &Res<AssetServer>,
    save_game_name: String,
) {
    build_save_game(
        commands,
        &asset_server,
        save_games_container_entity,
        save_game_name,
    );
}

pub fn despawn_save_game(
    commands: &mut Commands,
    save_game_query: &Query<(Entity, &SaveGame)>,
    save_game_name: String,
) {
    if let Ok((save_game_entity, save_game)) = save_game_query.get_single() {
        if save_game.name == save_game_name {
            commands.entity(save_game_entity).despawn_recursive();
            return;
        }
    }
}

pub fn build_save_games_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
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
            parent.spawn((
                NodeBundle {
                    style: SAVE_GAMES_CONTAINER_STYLE,
                    ..default()
                },
                SaveGamesContainer {},
            ));
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

pub fn build_save_game(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    save_games_container_entity: Entity,
    save_game_name: String,
) -> Entity {
    commands
        .entity(save_games_container_entity)
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        style: SAVE_GAME_STYLE,
                        background_color: BACKGROUND_COLOR.into(),
                        ..default()
                    },
                    SaveGame {
                        name: save_game_name.clone(),
                    },
                ))
                .with_children(|parent| {
                    // Button Container
                    parent
                        .spawn(NodeBundle {
                            style: BUTTON_CONTAINER_STYLE,
                            ..default()
                        })
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
                                        save_game_name: save_game_name.clone(),
                                    },
                                ))
                                .with_children(|parent| {
                                    parent.spawn(ImageBundle {
                                        style: BUTTON_IMAGE_STYLE,
                                        image: asset_server.load("sprites/cross.png").into(),
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
                                        save_game_name: save_game_name.clone(),
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
                });
        })
        .id()
}
