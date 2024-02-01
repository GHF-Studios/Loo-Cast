// Modules

// Local imports

// Internal imports
use crate::system::ui::*;
use crate::system::AppState;

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
pub struct GamesMenuPlugin;

#[derive(Component)]
pub struct GamesMenu {}

#[derive(Component)]
pub struct GamesContainer {}

#[derive(Component)]
pub struct Game {
    pub name: String,
}

#[derive(Component)]
pub struct CreateGameButton {}

#[derive(Component)]
pub struct BackToMainMenuButton {}

#[derive(Component)]
pub struct LoadGameButton {
    pub game_name: String,
}

#[derive(Component)]
pub struct DeleteGameButton {
    pub game_name: String,
}

#[derive(Event)]
pub struct LoadGameInstance {
    pub game_name: String,
}

#[derive(Event)]
pub struct DeleteGameUI {
    pub game_name: String,
}

#[derive(Resource)]
pub struct GamesMenuManager;

// Implementations
impl Plugin for GamesMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<LoadGameInstance>()
            .add_event::<DeleteGameUI>()
            // Enter State Systems
            .add_systems(
                OnEnter(AppState::GamesMenu),
                GamesMenuManager::startup,
            )
            // Update State Systems
            .add_systems(
                Update,
                (
                    GamesMenuManager::handle_back_to_main_menu_button,
                    GamesMenuManager::handle_create_game_button,
                    GamesMenuManager::handle_delete_game_button,
                    GamesMenuManager::handle_load_game_button,
                    GamesMenuManager::handle_delete_game_ui,
                )
                    .run_if(in_state(AppState::GamesMenu)),
            )
            // Exit State Systems
            .add_systems(
                OnExit(AppState::GamesMenu),
                GamesMenuManager::shutdown,
            );
    }
}

impl GamesMenuManager {
    fn startup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
    ) {
        Self::build_games_menu(&mut commands, &asset_server);
    }

    fn shutdown(
        mut commands: Commands,
        games_menu_query: Query<Entity, With<GamesMenu>>,
    ) {
        if let Ok(games_menu_entity) = games_menu_query.get_single() {
            commands.entity(games_menu_entity).despawn_recursive();
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
                    
                    info!("Transitioning to main menu...");

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

    fn handle_create_game_button(
        mut app_state_next_state: ResMut<NextState<AppState>>,
        mut button_query: Query<
            (&Interaction, &mut BackgroundColor),
            (Changed<Interaction>, With<CreateGameButton>),
        >,
    ) {
        if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
            match *interaction {
                Interaction::Pressed => {
                    *background_color = PRESSED_BUTTON_COLOR.into();

                    info!("Transitioning to game creation menu...");

                    app_state_next_state.set(AppState::CreateGameMenu);
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

    fn handle_delete_game_button(
        mut button_query: Query<
            (&Interaction, &mut BackgroundColor, &DeleteGameButton),
            Changed<Interaction>,
        >,
    ) {
        if let Ok((interaction, mut background_color, delete_game_button)) =
            button_query.get_single_mut()
        {
            match *interaction {
                Interaction::Pressed => {
                    *background_color = PRESSED_BUTTON_COLOR.into();

                    error!("Games and game management don't actually exist; the shown games are just placeholders and cannot be deleted.");

                    // TODO: Delete the game associated with the button

                    todo!();
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

    fn handle_load_game_button(
        mut button_query: Query<
            (&Interaction, &mut BackgroundColor, &LoadGameButton),
            Changed<Interaction>,
        >,
    ) {
        if let Ok((interaction, mut background_color, load_game_button)) =
            button_query.get_single_mut()
        {
            match *interaction {
                Interaction::Pressed => {
                    *background_color = PRESSED_BUTTON_COLOR.into();

                    error!("Games and game management don't actually exist; the shown games are just placeholders and cannot be loaded.");

                    // TODO: Load the game associated with the button
                    
                    todo!();
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

    fn handle_delete_game_ui(
        mut commands: Commands,
        mut delete_game_ui_event_reader: EventReader<DeleteGameUI>,
        mut game_query: Query<(Entity, &Game)>,
    ) {
        if let Some(event) = delete_game_ui_event_reader.iter().next() {
            for (entity, game) in game_query.iter_mut() {
                if game.name == event.game_name {
                    commands.entity(entity).despawn_recursive();

                    info!("Deleted ui for game '{}'.", event.game_name);

                    return;
                }
            }
        }
    }

    fn build_games_menu(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
    ) -> Entity {
        warn!("Games and game management don't actually exist; the save games are just placeholders.");

        let game_names = vec!["save_game_1", "save_game_2", "save_game_3"];

        let games_menu_entity = commands
            .spawn((
                NodeBundle {
                    style: SAVE_GAMES_MENU_STYLE,
                    ..default()
                },
                GamesMenu {},
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
                        GamesContainer {},
                    ))
                    .with_children(|parent| {
                        // Save Games
                        for game_name in game_names.iter() {
                            parent
                                .spawn((
                                    NodeBundle {
                                        style: SAVE_GAME_STYLE,
                                        background_color: BACKGROUND_COLOR.into(),
                                        ..default()
                                    },
                                    Game {
                                        name: game_name.to_string(),
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
                                            DeleteGameButton {
                                                game_name: game_name.to_string(),
                                            },
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn(ImageBundle {
                                                style: BUTTON_IMAGE_STYLE,
                                                image: asset_server
                                                    .load("loo_cast_base_mod/resources/sprites/cross.png")
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
                                                        game_name.clone(),
                                                        Self::get_game_name_text_style(
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
                                            LoadGameButton {
                                                game_name: game_name.to_string(),
                                            },
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn(ImageBundle {
                                                style: BUTTON_IMAGE_STYLE,
                                                image: asset_server
                                                    .load("loo_cast_base_mod/resources/sprites/enter.png")
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
                                    image: asset_server.load("loo_cast_base_mod/resources/sprites/return.png").into(),
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
                                CreateGameButton {},
                            ))
                            .with_children(|parent| {
                                parent.spawn(ImageBundle {
                                    style: BUTTON_IMAGE_STYLE,
                                    image: asset_server.load("loo_cast_base_mod/resources/sprites/plus.png").into(),
                                    ..default()
                                });
                            });
                    });
            })
            .id();

        games_menu_entity
    }

    fn get_game_name_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
        TextStyle {
            font: asset_server.load("loo_cast_base_mod/resources/fonts/FiraSans-Bold.ttf"),
            font_size: 64.0,
            color: Color::WHITE,
        }
    }
}

// Module Functions
