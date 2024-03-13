// Modules

// Local imports

// Internal imports
use crate::system::game::*;
use crate::system::savegame::*;
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
pub struct SavegamesMenuPlugin;

#[derive(Component)]
pub struct SavegamesMenu {}

#[derive(Component)]
pub struct SavegamesContainer {}

#[derive(Component)]
pub struct Savegame {
    pub name: String,
}

#[derive(Component)]
pub struct CreateSavegameButton {}

#[derive(Component)]
pub struct BackToMainMenuButton {}

#[derive(Component)]
pub struct LoadSavegameButton {
    pub savegame_name: String,
}

#[derive(Component)]
pub struct DeleteSavegameButton {
    pub savegame_name: String,
}

#[derive(Event)]
pub struct LoadSavegameInstance {
    pub savegame_name: String,
}

#[derive(Event)]
pub struct DeleteSavegameUI {
    pub savegame_name: String,
}

#[derive(Resource)]
pub struct SavegamesMenuManager;

// Implementations
impl Plugin for SavegamesMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<LoadSavegameInstance>()
            .add_event::<DeleteSavegameUI>()
            // Enter State Systems
            .add_systems(
                OnEnter(AppState::SavegamesMenu),
                SavegamesMenuManager::initialize,
            )
            // Update Systems
            .add_systems(
                Update,
                (
                    SavegamesMenuManager::handle_back_to_main_menu_button,
                    SavegamesMenuManager::handle_create_savegame_button,
                    SavegamesMenuManager::handle_delete_savegame_button,
                    SavegamesMenuManager::handle_load_savegame_button,
                    SavegamesMenuManager::handle_load_savegame_instance,
                    SavegamesMenuManager::handle_delete_savegame_ui,
                )
                    .run_if(in_state(AppState::SavegamesMenu)),
            )
            // Exit State Systems
            .add_systems(
                OnExit(AppState::SavegamesMenu),
                SavegamesMenuManager::terminate,
            );
    }
}

impl SavegamesMenuManager {
    fn initialize(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        savegame_manager: Res<SavegameManager>,
    ) {
        Self::build_savegames_menu(&mut commands, &asset_server, &savegame_manager);
    }

    fn terminate(
        mut commands: Commands,
        savegames_menu_query: Query<Entity, With<SavegamesMenu>>,
    ) {
        if let Ok(savegames_menu_entity) = savegames_menu_query.get_single() {
            commands.entity(savegames_menu_entity).despawn_recursive();
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

    fn handle_create_savegame_button(
        mut app_state_next_state: ResMut<NextState<AppState>>,
        mut button_query: Query<
            (&Interaction, &mut BackgroundColor),
            (Changed<Interaction>, With<CreateSavegameButton>),
        >,
    ) {
        if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
            match *interaction {
                Interaction::Pressed => {
                    *background_color = PRESSED_BUTTON_COLOR.into();
                    app_state_next_state.set(AppState::CreateSavegameMenu);
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

    fn handle_delete_savegame_button(
        mut delete_savegame_event_writer: EventWriter<DeleteSavegame>,
        mut button_query: Query<
            (&Interaction, &mut BackgroundColor, &DeleteSavegameButton),
            Changed<Interaction>,
        >,
    ) {
        if let Ok((interaction, mut background_color, delete_savegame_button)) =
            button_query.get_single_mut()
        {
            match *interaction {
                Interaction::Pressed => {
                    delete_savegame_event_writer.send(DeleteSavegame {
                        savegame_name: delete_savegame_button.savegame_name.clone(),
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

    fn handle_load_savegame_button(
        mut load_savegame_instance_event_writer: EventWriter<LoadSavegameInstance>,
        mut button_query: Query<
            (&Interaction, &mut BackgroundColor, &LoadSavegameButton),
            Changed<Interaction>,
        >,
    ) {
        if let Ok((interaction, mut background_color, load_savegame_button)) =
            button_query.get_single_mut()
        {
            match *interaction {
                Interaction::Pressed => {
                    load_savegame_instance_event_writer.send(LoadSavegameInstance {
                        savegame_name: load_savegame_button.savegame_name.clone(),
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

    fn handle_load_savegame_instance(
        mut load_savegame_instance_event_reader: EventReader<LoadSavegameInstance>,
        mut load_game_event_writer: EventWriter<LoadGame>,
        savegame_manager: Res<SavegameManager>,
    ) {
        if let Some(event) = load_savegame_instance_event_reader.iter().last() {
            if let Some(savegame) =
                savegame_manager.get_savegame_info(event.savegame_name.clone())
            {
                load_game_event_writer.send(LoadGame {
                    savegame: savegame.clone(),
                });
            }
        }
    }

    fn handle_delete_savegame_ui(
        mut commands: Commands,
        mut delete_savegame_ui_event_reader: EventReader<DeleteSavegameUI>,
        mut savegame_query: Query<(Entity, &Savegame)>,
    ) {
        if let Some(event) = delete_savegame_ui_event_reader.iter().next() {
            for (entity, savegame) in savegame_query.iter_mut() {
                if savegame.name == event.savegame_name {
                    commands.entity(entity).despawn_recursive();
                    return;
                }
            }
        }
    }

    fn build_savegames_menu(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        savegame_manager: &Res<SavegameManager>,
    ) -> Entity {
        let savegames_menu_entity = commands
            .spawn((
                NodeBundle {
                    style: SAVE_GAMES_MENU_STYLE,
                    ..default()
                },
                SavegamesMenu {},
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
                        SavegamesContainer {},
                    ))
                    .with_children(|parent| {
                        // Save Games
                        for savegame_info in savegame_manager.registered_savegames.iter() {
                            parent
                                .spawn((
                                    NodeBundle {
                                        style: SAVE_GAME_STYLE,
                                        background_color: BACKGROUND_COLOR.into(),
                                        ..default()
                                    },
                                    Savegame {
                                        name: savegame_info.name.clone(),
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
                                            DeleteSavegameButton {
                                                savegame_name: savegame_info.name.clone(),
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
                                                        savegame_info.name.clone(),
                                                        Self::get_savegame_name_text_style(
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
                                            LoadSavegameButton {
                                                savegame_name: savegame_info.name.clone(),
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
                                CreateSavegameButton {},
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

        savegames_menu_entity
    }

    fn get_savegame_name_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
        TextStyle {
            font: asset_server.load("loo_cast_base_mod/resources/fonts/FiraSans-Bold.ttf"),
            font_size: 64.0,
            color: Color::WHITE,
        }
    }
}

// Module Functions
