// Modules

// Local imports

// Internal imports
use crate::engine::kernel::game::*;
use crate::engine::kernel::save_game::*;
use crate::engine::kernel::ui::*;
use crate::engine::kernel::AppState;

// External imports
use bevy::prelude::*;

// Static variables

// Constant variables
pub const SAVE_GAMES_MENU_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style.row_gap = Val::Px(8.0);
    style.column_gap = Val::Px(8.0);
    style
};

pub const SAVE_GAMES_CONTAINER_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.display = Display::Flex;
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Start;
    style.align_items = AlignItems::Center;
    style.width = Val::Px(1000.0);
    style.height = Val::Px(500.0);
    style.row_gap = Val::Px(8.0);
    style.column_gap = Val::Px(8.0);
    style
};

pub const SAVE_GAME_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.display = Display::Flex;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::SpaceBetween;
    style.align_items = AlignItems::Center;
    style.width = Val::Px(1000.0);
    style.height = Val::Px(80.0);
    style.row_gap = Val::Px(8.0);
    style.column_gap = Val::Px(8.0);
    style
};

pub const BUTTON_CONTAINER_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.display = Display::Flex;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::SpaceBetween;
    style.align_items = AlignItems::Center;
    style.width = Val::Px(1000.0);
    style.height = Val::Px(64.0);
    style.row_gap = Val::Px(8.0);
    style.column_gap = Val::Px(8.0);
    style
};

pub const BUTTON_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Px(80.0);
    style.height = Val::Px(80.0);
    style
};

pub const BUTTON_IMAGE_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Px(64.0);
    style.height = Val::Px(64.0);
    style.margin = UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0));
    style
};

pub const SAVE_GAME_NAME_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Px(500.0);
    style.height = Val::Px(80.0);
    style
};

pub const TITLE_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Px(500.0);
    style.height = Val::Px(80.0);
    style
};

// Types

// Enums

// Structs
pub struct SaveGamesMenuPlugin;

#[derive(Component)]
pub struct SaveGamesMenu {}

#[derive(Component)]
pub struct SaveGamesContainer {}

#[derive(Component)]
pub struct SaveGame {
    pub name: String,
}

#[derive(Component)]
pub struct CreateSaveGameButton {}

#[derive(Component)]
pub struct BackToMainMenuButton {}

#[derive(Component)]
pub struct LoadSaveGameButton {
    pub save_game_name: String,
}

#[derive(Component)]
pub struct DeleteSaveGameButton {
    pub save_game_name: String,
}

#[derive(Event)]
pub struct LoadSaveGameInstance {
    pub save_game_name: String,
}

#[derive(Event)]
pub struct DeleteSaveGameUI {
    pub save_game_name: String,
}

#[derive(Resource)]
pub struct SaveGamesMenuManager;

// Implementations
impl Plugin for SaveGamesMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<LoadSaveGameInstance>()
            .add_event::<DeleteSaveGameUI>()
            // Enter State Systems
            .add_systems(
                OnEnter(AppState::SaveGamesMenu),
                SaveGamesMenuManager::initialize,
            )
            // Update Systems
            .add_systems(
                Update,
                (
                    SaveGamesMenuManager::handle_back_to_main_menu_button,
                    SaveGamesMenuManager::handle_create_save_game_button,
                    SaveGamesMenuManager::handle_delete_save_game_button,
                    SaveGamesMenuManager::handle_load_save_game_button,
                    SaveGamesMenuManager::handle_load_save_game_instance,
                    SaveGamesMenuManager::handle_delete_save_game_ui,
                )
                    .run_if(in_state(AppState::SaveGamesMenu)),
            )
            // Exit State Systems
            .add_systems(
                OnExit(AppState::SaveGamesMenu),
                SaveGamesMenuManager::terminate,
            );
    }
}

impl SaveGamesMenuManager {
    fn initialize(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        save_game_manager: Res<SaveGameManager>,
    ) {
        Self::build_save_games_menu(&mut commands, &asset_server, &save_game_manager);
    }

    fn terminate(
        mut commands: Commands,
        save_games_menu_query: Query<Entity, With<SaveGamesMenu>>,
    ) {
        if let Ok(save_games_menu_entity) = save_games_menu_query.get_single() {
            commands.entity(save_games_menu_entity).despawn_recursive();
        }
    }

    fn handle_back_to_main_menu_button(
        mut app_state_next_state: ResMut<NextState<AppState>>,
        mut button_query: Query<
            (&Interaction, &mut BackgroundColor),
            (Changed<Interaction>, With<BackToMainMenuButton>),
        >,
    ) {
        if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
            match *interaction {
                Interaction::Pressed => {
                    *background_color = PRESSED_BUTTON_COLOR.into();
                    app_state_next_state.set(AppState::MainMenu);
                }
                Interaction::Hovered => {
                    *background_color = HOVERED_BUTTON_COLOR.into();
                }
                Interaction::None => {
                    *background_color = NORMAL_BUTTON_COLOR.into();
                }
            }
        }
    }

    fn handle_create_save_game_button(
        mut app_state_next_state: ResMut<NextState<AppState>>,
        mut button_query: Query<
            (&Interaction, &mut BackgroundColor),
            (Changed<Interaction>, With<CreateSaveGameButton>),
        >,
    ) {
        if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
            match *interaction {
                Interaction::Pressed => {
                    *background_color = PRESSED_BUTTON_COLOR.into();
                    app_state_next_state.set(AppState::CreateSaveGameMenu);
                }
                Interaction::Hovered => {
                    *background_color = HOVERED_BUTTON_COLOR.into();
                }
                Interaction::None => {
                    *background_color = NORMAL_BUTTON_COLOR.into();
                }
            }
        }
    }

    fn handle_delete_save_game_button(
        mut delete_save_game_event_writer: EventWriter<DeleteSaveGame>,
        mut button_query: Query<
            (&Interaction, &mut BackgroundColor, &DeleteSaveGameButton),
            Changed<Interaction>,
        >,
    ) {
        if let Ok((interaction, mut background_color, delete_save_game_button)) =
            button_query.get_single_mut()
        {
            match *interaction {
                Interaction::Pressed => {
                    delete_save_game_event_writer.send(DeleteSaveGame {
                        save_game_name: delete_save_game_button.save_game_name.clone(),
                    });
                    *background_color = PRESSED_BUTTON_COLOR.into();
                }
                Interaction::Hovered => {
                    *background_color = HOVERED_BUTTON_COLOR.into();
                }
                Interaction::None => {
                    *background_color = NORMAL_BUTTON_COLOR.into();
                }
            }
        }
    }

    fn handle_load_save_game_button(
        mut load_save_game_instance_event_writer: EventWriter<LoadSaveGameInstance>,
        mut button_query: Query<
            (&Interaction, &mut BackgroundColor, &LoadSaveGameButton),
            Changed<Interaction>,
        >,
    ) {
        if let Ok((interaction, mut background_color, load_save_game_button)) =
            button_query.get_single_mut()
        {
            match *interaction {
                Interaction::Pressed => {
                    load_save_game_instance_event_writer.send(LoadSaveGameInstance {
                        save_game_name: load_save_game_button.save_game_name.clone(),
                    });
                    *background_color = PRESSED_BUTTON_COLOR.into();
                }
                Interaction::Hovered => {
                    *background_color = HOVERED_BUTTON_COLOR.into();
                }
                Interaction::None => {
                    *background_color = NORMAL_BUTTON_COLOR.into();
                }
            }
        }
    }

    fn handle_load_save_game_instance(
        mut load_save_game_instance_event_reader: EventReader<LoadSaveGameInstance>,
        mut load_game_event_writer: EventWriter<LoadGame>,
        save_game_manager: Res<SaveGameManager>,
    ) {
        if let Some(event) = load_save_game_instance_event_reader.iter().last() {
            if let Some(save_game) =
                save_game_manager.get_save_game_info(event.save_game_name.clone())
            {
                load_game_event_writer.send(LoadGame {
                    save_game: save_game.clone(),
                });
            }
        }
    }

    fn handle_delete_save_game_ui(
        mut commands: Commands,
        mut delete_save_game_ui_event_reader: EventReader<DeleteSaveGameUI>,
        mut save_game_query: Query<(Entity, &SaveGame)>,
    ) {
        if let Some(event) = delete_save_game_ui_event_reader.iter().next() {
            for (entity, save_game) in save_game_query.iter_mut() {
                if save_game.name == event.save_game_name {
                    commands.entity(entity).despawn_recursive();
                    return;
                }
            }
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
                                    UIManager::get_title_text_style(asset_server),
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
                                                image: asset_server
                                                    .load("sprites/cross.png")
                                                    .into(),
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
                                                        Self::get_save_game_name_text_style(
                                                            asset_server,
                                                        ),
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
                                                image: asset_server
                                                    .load("sprites/enter.png")
                                                    .into(),
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

    fn get_save_game_name_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
        TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 64.0,
            color: Color::WHITE,
        }
    }
}

// Module Functions
