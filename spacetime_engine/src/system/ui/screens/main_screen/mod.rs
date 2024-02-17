// Modules

// Local imports

// Internal imports
use crate::system::ui::*;
use crate::system::AppState;

// External imports
use bevy::app::AppExit;
use bevy::prelude::*;

// Static variables

// Constant variables
pub const MAIN_SCREEN_STYLE: Style = {
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
pub struct MainScreenPlugin;

#[derive(Component)]
pub struct MainScreen {}

#[derive(Component)]
pub struct PlayButton {}

#[derive(Component)]
pub struct QuitButton {}

#[derive(Resource, Default)]
pub struct MainScreenManager;

// Implementations
impl Plugin for MainScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            // Enter State Systems
            .add_systems(OnEnter(AppState::Main), MainScreenManager::startup)
            // Update State Systems
            .add_systems(
                Update,
                (
                    MainScreenManager::handle_play_button,
                    MainScreenManager::handle_quit_button,
                )
                    .run_if(in_state(AppState::Main)),
            )
            // Exit State Systems
            .add_systems(OnExit(AppState::Main), MainScreenManager::shutdown);
    }
}

impl MainScreenManager {
    fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.insert_resource(MainScreenManager {});
        Self::build_main_screen(&mut commands, &asset_server);
    }

    fn shutdown(mut commands: Commands, main_screen_query: Query<Entity, With<MainScreen>>) {
        commands.remove_resource::<MainScreenManager>();
        if let Ok(main_screen_entity) = main_screen_query.get_single() {
            commands.entity(main_screen_entity).despawn_recursive();
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

                    info!("Switching to games screen...");

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

                    info!("Quitting the application...");

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

    fn build_main_screen(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
        let main_screen_entity = commands
            .spawn((
                NodeBundle {
                    style: MAIN_SCREEN_STYLE,
                    ..default()
                },
                MainScreen {},
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

        main_screen_entity
    }
}

// Module Functions
