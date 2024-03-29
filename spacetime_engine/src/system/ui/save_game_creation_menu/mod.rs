// Modules

// Local imports

// Internal imports
use crate::system::save_game::*;
use crate::system::ui::input_field::*;
use crate::system::ui::*;
use crate::system::AppState;

// External imports
use bevy::prelude::*;

// Static variables

// Constant variables
pub const SAVE_GAME_CREATION_MENU_STYLE: Style = {
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
pub struct SaveGameCreationMenuPlugin;

#[derive(Component)]
pub struct SaveGameCreationMenu {}

#[derive(Component)]
pub struct SaveGameName {}

#[derive(Component)]
pub struct CancelSaveGameCreationButton {}

#[derive(Component)]
pub struct ConfirmSaveGameCreationButton {}

#[derive(Resource)]
pub struct SaveGameCreationMenuManager;

// Implementations
impl Plugin for SaveGameCreationMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Enter State Systems
            .add_systems(
                OnEnter(AppState::CreateSaveGameMenu),
                SaveGameCreationMenuManager::initialize,
            )
            // Update Systems
            .add_systems(
                Update,
                (
                    SaveGameCreationMenuManager::handle_cancel_save_game_creation_button,
                    SaveGameCreationMenuManager::handle_confirm_save_game_creation_button,
                )
                    .run_if(in_state(AppState::CreateSaveGameMenu)),
            )
            // Exit State Systems
            .add_systems(
                OnExit(AppState::CreateSaveGameMenu),
                SaveGameCreationMenuManager::terminate,
            );
    }
}

impl SaveGameCreationMenuManager {
    fn initialize(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.insert_resource(SaveGameCreationMenuManager {});
        Self::build_save_game_creation_menu(&mut commands, &asset_server);
    }

    fn terminate(
        mut commands: Commands,
        create_save_game_menu_query: Query<Entity, With<SaveGameCreationMenu>>,
    ) {
        commands.remove_resource::<SaveGameCreationMenuManager>();
        if let Ok(create_save_game_menu_entity) = create_save_game_menu_query.get_single() {
            commands
                .entity(create_save_game_menu_entity)
                .despawn_recursive();
        }
    }

    fn handle_cancel_save_game_creation_button(
        mut app_state_next_state: ResMut<NextState<AppState>>,
        mut button_query: Query<
            (&Interaction, &mut BackgroundColor),
            (Changed<Interaction>, With<CancelSaveGameCreationButton>),
        >,
    ) {
        if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
            match *interaction {
                Interaction::Pressed => {
                    *background_color = PRESSED_BUTTON_COLOR.into();
                    app_state_next_state.set(AppState::SaveGamesMenu);
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

    fn handle_confirm_save_game_creation_button(
        mut create_save_game_event_writer: EventWriter<CreateSaveGame>,
        mut button_query: Query<
            (&Interaction, &mut BackgroundColor),
            (Changed<Interaction>, With<ConfirmSaveGameCreationButton>),
        >,
        name_input_field_query: Query<&InputField, With<SaveGameName>>,
    ) {
        if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
            let name_input_field = name_input_field_query.iter().next().unwrap();

            match *interaction {
                Interaction::Pressed => {
                    *background_color = PRESSED_BUTTON_COLOR.into();
                    create_save_game_event_writer.send(CreateSaveGame {
                        save_game_name: name_input_field.value.clone(),
                    });
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

    fn build_save_game_creation_menu(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
    ) -> Entity {
        let create_save_game_menu_entity = commands
            .spawn((
                NodeBundle {
                    style: SAVE_GAME_CREATION_MENU_STYLE,
                    ..default()
                },
                SaveGameCreationMenu {},
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
                            SaveGameName {},
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
                                CancelSaveGameCreationButton {},
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
                                ConfirmSaveGameCreationButton {},
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

        create_save_game_menu_entity
    }
}

// Module Functions
