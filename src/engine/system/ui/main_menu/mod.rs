// Modules

// Local imports

// Internal imports
use crate::engine::system::ui::*;
use crate::engine::system::AppState;

// External imports
use bevy::app::AppExit;
use bevy::prelude::*;

// Static variables

// Constant variables
pub const MAIN_MENU_STYLE: Style = {
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

pub const BUTTON_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Px(200.0);
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
pub struct MainMenuPlugin;

#[derive(Component)]
pub struct MainMenu {}

#[derive(Component)]
pub struct PlayButton {}

#[derive(Component)]
pub struct QuitButton {}

#[derive(Resource)]
pub struct MainMenuManager;

// Implementations
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Enter State Systems
            .add_systems(OnEnter(AppState::MainMenu), MainMenuManager::initialize)
            // Update Systems
            .add_systems(
                Update,
                (
                    MainMenuManager::handle_play_button,
                    MainMenuManager::handle_quit_button,
                )
                    .run_if(in_state(AppState::MainMenu)),
            )
            // Exit State Systems
            .add_systems(OnExit(AppState::MainMenu), MainMenuManager::terminate);
    }
}

impl MainMenuManager {
    fn initialize(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.insert_resource(MainMenuManager {});
        Self::build_main_menu(&mut commands, &asset_server);
    }

    fn terminate(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
        commands.remove_resource::<MainMenuManager>();
        if let Ok(main_menu_entity) = main_menu_query.get_single() {
            commands.entity(main_menu_entity).despawn_recursive();
        }
    }

    fn handle_play_button(
        mut button_query: Query<
            (&Interaction, &mut BackgroundColor),
            (Changed<Interaction>, With<PlayButton>),
        >,
        mut app_state_next_state: ResMut<NextState<AppState>>,
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

    fn handle_quit_button(
        mut app_exit_event_writer: EventWriter<AppExit>,
        mut button_query: Query<
            (&Interaction, &mut BackgroundColor),
            (Changed<Interaction>, With<QuitButton>),
        >,
    ) {
        if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
            match *interaction {
                Interaction::Pressed => {
                    *background_color = PRESSED_BUTTON_COLOR.into();
                    app_exit_event_writer.send(AppExit);
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

    fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
        let main_menu_entity = commands
            .spawn((
                NodeBundle {
                    style: MAIN_MENU_STYLE,
                    ..default()
                },
                MainMenu {},
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
                                    "Loo Cast",
                                    UIManager::get_title_text_style(asset_server),
                                )],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        });
                    });
                // === Play Button ===
                parent
                    .spawn((
                        ButtonBundle {
                            style: BUTTON_STYLE,
                            background_color: NORMAL_BUTTON_COLOR.into(),
                            ..default()
                        },
                        PlayButton {},
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    "Play",
                                    UIManager::get_button_text_style(asset_server),
                                )],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        });
                    });
                // === Quit Button ===
                parent
                    .spawn((
                        ButtonBundle {
                            style: BUTTON_STYLE,
                            background_color: NORMAL_BUTTON_COLOR.into(),
                            ..default()
                        },
                        QuitButton {},
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    "Quit",
                                    UIManager::get_button_text_style(asset_server),
                                )],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        });
                    });
            })
            .id();

        main_menu_entity
    }
}

// Module Functions
