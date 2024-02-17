// Modules

// Local imports

// Internal imports
use crate::system::ui::components::input_field::*;
use crate::system::ui::*;
use crate::system::AppState;

// External imports
use bevy::prelude::*;

// Static variables

// Constant variables
pub const SAVE_GAME_CREATION_SCREEN_STYLE: Style = {
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

pub const NEW_SAVE_GAME_PANEL_STYLE: Style = {
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
pub const NEW_SAVE_GAME_NAME_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.display = Display::Flex;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Start;
    style.width = Val::Px(1000.0);
    style.height = Val::Px(64.0);
    style.row_gap = Val::Px(8.0);
    style.column_gap = Val::Px(8.0);
    style.min_width = Val::Px(1000.0);
    style.min_height = Val::Px(64.0);
    style.max_width = Val::Px(1000.0);
    style.max_height = Val::Px(64.0);
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
pub struct GameCreationScreenPlugin;

#[derive(Component)]
pub struct GameCreationScreen {}

#[derive(Component)]
pub struct GameName {}

#[derive(Component)]
pub struct CancelGameCreationButton {}

#[derive(Component)]
pub struct ConfirmGameCreationButton {}

#[derive(Resource)]
pub struct GameCreationScreenManager;

// Implementations
impl Plugin for GameCreationScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            // Enter State Systems
            .add_systems(
                OnEnter(AppState::CreateGame),
                GameCreationScreenManager::initialize,
            )
            // Update State Systems
            .add_systems(
                Update,
                (
                    GameCreationScreenManager::handle_cancel_game_creation_button,
                    GameCreationScreenManager::handle_confirm_game_creation_button,
                )
                    .run_if(in_state(AppState::CreateGame)),
            )
            // Exit State Systems
            .add_systems(
                OnExit(AppState::CreateGame),
                GameCreationScreenManager::terminate,
            );
    }
}

impl GameCreationScreenManager {
    fn initialize(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.insert_resource(GameCreationScreenManager {});
        Self::build_game_creation_screen(&mut commands, &asset_server);
    }

    fn terminate(
        mut commands: Commands,
        create_game_info_screen_query: Query<Entity, With<GameCreationScreen>>,
    ) {
        commands.remove_resource::<GameCreationScreenManager>();
        if let Ok(create_game_info_screen_entity) = create_game_info_screen_query.get_single() {
            commands
                .entity(create_game_info_screen_entity)
                .despawn_recursive();
        }
    }

    fn handle_cancel_game_creation_button(
        mut app_state_next_state: ResMut<NextState<AppState>>,
        mut button_query: Query<
            (&Interaction, &mut BackgroundColor),
            (Changed<Interaction>, With<CancelGameCreationButton>),
        >,
    ) {
        if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
            match *interaction {
                Interaction::Pressed => {
                    *background_color = PRESSED_BUTTON_COLOR.into();

                    info!("Transitioning to games screen...");

                    app_state_next_state.set(AppState::Games);
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

    fn handle_confirm_game_creation_button(
        mut button_query: Query<
            (&Interaction, &mut BackgroundColor),
            (Changed<Interaction>, With<ConfirmGameCreationButton>),
        >,
        name_input_field_query: Query<&InputField, With<GameName>>,
    ) {
        if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
            let name_input_field = name_input_field_query.iter().next().unwrap();

            match *interaction {
                Interaction::Pressed => {
                    *background_color = PRESSED_BUTTON_COLOR.into();

                    // TODO: Create game with selected parameters, or use default parameters

                    error!("Games and game management don't actually exist; the shown games are just placeholders and cannot be created.");

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

    fn build_game_creation_screen(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
    ) -> Entity {
        let create_game_info_screen_entity = commands
            .spawn((
                NodeBundle {
                    style: SAVE_GAME_CREATION_SCREEN_STYLE,
                    ..default()
                },
                GameCreationScreen {},
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
                                    UIManager::get_title_text_style(asset_server),
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
                        ..default()
                    })
                    .with_children(|parent| {
                        // Save Game Name Input Field
                        parent.spawn((
                            InputFieldBundle {
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Sex",
                                        UIManager::get_label_text_style(asset_server),
                                    )],
                                    ..default()
                                },
                                style: NEW_SAVE_GAME_NAME_STYLE,
                                background_color: UNFOCUSED_COLOR.into(),
                                ..default()
                            },
                            GameName {},
                        ));
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
                                CancelGameCreationButton {},
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

                        // Confirm Create Save Game Button
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: BUTTON_STYLE,
                                    background_color: NORMAL_BUTTON_COLOR.into(),
                                    ..default()
                                },
                                ConfirmGameCreationButton {},
                            ))
                            .with_children(|parent| {
                                parent.spawn(ImageBundle {
                                    style: BUTTON_IMAGE_STYLE,
                                    image: asset_server
                                        .load("loo_cast_base_mod/resources/sprites/checkMark.png")
                                        .into(),
                                    ..default()
                                });
                            });
                    });
            })
            .id();

        create_game_info_screen_entity
    }
}

// Module Functions
