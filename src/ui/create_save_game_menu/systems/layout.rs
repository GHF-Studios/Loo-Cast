use crate::ui::create_save_game_menu::components::*;
use crate::ui::create_save_game_menu::styles::*;
use crate::ui::input_field::components::*;
use crate::ui::input_field::node_bundles::*;
use crate::ui::styles::*;

use bevy::prelude::*;
use bevy::text::*;

pub fn spawn_create_save_game_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_create_save_game_menu(&mut commands, &asset_server);
}

pub fn despawn_create_save_game_menu(
    mut commands: Commands,
    create_save_game_menu_query: Query<Entity, With<CreateSaveGameMenu>>,
) {
    if let Ok(create_save_game_menu_entity) = create_save_game_menu_query.get_single() {
        commands
            .entity(create_save_game_menu_entity)
            .despawn_recursive();
    }
}

fn build_create_save_game_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let create_save_game_menu_entity = commands
        .spawn((
            NodeBundle {
                style: CREATE_SAVE_GAME_MENU_STYLE,
                ..default()
            },
            CreateSaveGameMenu {},
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
                                "New Save Game",
                                get_title_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            // New Save Game Panel
            parent
                .spawn(NodeBundle {
                    style: NEW_SAVE_GAME_PANEL_STYLE,
                    background_color: PANEL_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Name Input Field
                    parent.spawn((InputFieldBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Sex",
                                get_title_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            linebreak_behavior: BreakLineOn::WordBoundary,
                        },
                        ..default()
                    },));
                });
            // Button Container
            parent
                .spawn(NodeBundle {
                    style: BUTTON_CONTAINER_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    // Cancel Create Save Button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: BUTTON_STYLE,
                                background_color: NORMAL_BUTTON_COLOR.into(),
                                ..default()
                            },
                            CancelCreateSaveGameButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(ImageBundle {
                                style: BUTTON_IMAGE_STYLE,
                                image: asset_server.load("sprites/cross.png").into(),
                                ..default()
                            });
                        });

                    // Confirm Create Save Game Button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: BUTTON_STYLE,
                                background_color: NORMAL_BUTTON_COLOR.into(),
                                ..default()
                            },
                            ConfirmCreateSaveGameButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(ImageBundle {
                                style: BUTTON_IMAGE_STYLE,
                                image: asset_server.load("sprites/checkMark.png").into(),
                                ..default()
                            });
                        });
                });
        })
        .id();

    create_save_game_menu_entity
}
