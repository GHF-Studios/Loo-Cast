// Modules

// Local imports

// Internal imports
use crate::system::*;
use crate::system::game::*;
use crate::system::ui::*;

// External imports
use bevy::prelude::*;

// Static variables

// Constant variables
pub const SAVE_GAMES_SCREEN_STYLE: Style = {
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
pub struct GamesScreenPlugin;

#[derive(Component)]
pub struct GamesScreen {}

#[derive(Component)]
pub struct GamesContainer {}

#[derive(Component)]
pub struct Game {
    pub name: String,
}

#[derive(Component)]
pub struct CreateGameButton {}

#[derive(Component)]
pub struct BackToMainScreenButton {}

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
pub struct GamesScreenManager;

// Implementations
impl Plugin for GamesScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<LoadGameInstance>()
            .add_event::<DeleteGameUI>()
            // Enter State Systems
            .add_systems(OnEnter(AppState::Games), GamesScreenManager::startup)
            // Update State Systems
            .add_systems(
                Update,
                (
                    GamesScreenManager::handle_back_to_main_screen_button,
                    GamesScreenManager::handle_create_game_button,
                    GamesScreenManager::handle_delete_game_button,
                    GamesScreenManager::handle_load_game_button,
                    GamesScreenManager::handle_delete_game_ui,
                )
                    .run_if(in_state(AppState::Games)),
            )
            // Exit State Systems
            .add_systems(OnExit(AppState::Games), GamesScreenManager::shutdown);
    }
}

impl GamesScreenManager {
    fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
        Self::build_games_screen(&mut commands, &asset_server);
    }

    fn shutdown(mut commands: Commands, games_screen_query: Query<Entity, With<GamesScreen>>) {
        if let Ok(games_screen_entity) = games_screen_query.get_single() {
            commands.entity(games_screen_entity).despawn_recursive();
        }
    }

    fn handle_back_to_main_screen_button(
        mut app_state_next_state: ResMut<NextState<AppState>>,
        mut button_query: Query<
            (&Interaction, &mut BackgroundColor),
            (Changed<Interaction>, With<BackToMainScreenButton>),
        >,
    ) {
        if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
            match *interaction {
                Interaction::Pressed => {
                    *background_color = PRESSED_BUTTON_COLOR.into();

                    info!("Transitioning to main screen...");

                    app_state_next_state.set(AppState::Main);
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

                    info!("Transitioning to game creation screen...");

                    app_state_next_state.set(AppState::CreateGame);
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
        mut enter_game_event_writer: EventWriter<EnterGame>,
        mut next_app_state: ResMut<NextState<AppState>>,
        mut next_game_state: ResMut<NextState<GameState>>,
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

                    warn!("Games and game management don't actually exist; the shown games are just placeholders and cannot be loaded.");

                    // TODO: Load the game associated with the button

                    enter_game_event_writer.send(EnterGame {});
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

    fn build_games_screen(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
        warn!(
            "Games and game management don't actually exist; the save games are just placeholders."
        );

        let game_names = vec!["save_game_1", "save_game_2", "save_game_3"];

        let games_screen_entity = commands
            .spawn((
                NodeBundle {
                    style: SAVE_GAMES_SCREEN_STYLE,
                    ..default()
                },
                GamesScreen {},
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
                        // Back To Main Screen Button
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: BUTTON_STYLE,
                                    background_color: NORMAL_BUTTON_COLOR.into(),
                                    ..default()
                                },
                                BackToMainScreenButton {},
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

        games_screen_entity
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
