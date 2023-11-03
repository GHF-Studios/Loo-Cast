// Modules


// Local imports


// Internal imports
use crate::game::*;
use crate::save_game::*;
use crate::ui::*;

// External imports
use bevy::prelude::*;

// Static variables


// Constant variables
pub const PAUSE_MENU_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.position_type = PositionType::Absolute; // Needed to display separately from HUD.
    style.display = Display::Flex; // Hidden by Default
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style
};

pub const PAUSE_MENU_CONTAINER_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.display = Display::Flex;
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Px(400.0);
    style.height = Val::Px(400.0);
    style.row_gap = Val::Px(8.0);
    style.column_gap = Val::Px(8.0);
    style
};

pub const BUTTON_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Px(200.0);
    style.height = Val::Px(80.0);
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style
};

// Types


// Enums


// Structs
pub struct PauseMenuPlugin;

#[derive(Component)]
pub struct PauseMenu {}

#[derive(Component)]
pub struct ResumeButton {}

#[derive(Component)]
pub struct MainMenuButton {}

#[derive(Component)]
pub struct QuitButton {}

#[derive(Resource)]
pub struct PauseMenuManager;

// Implementations
impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Enter Systems
            .add_systems(
                OnEnter(SimulationState::Paused),
                PauseMenuManager::initialize,
            )
            // Update Systems
            .add_systems(
                Update,
                (
                    PauseMenuManager::interact_with_resume_button,
                    PauseMenuManager::interact_with_main_menu_button,
                    PauseMenuManager::interact_with_quit_button,
                )
                    .run_if(in_state(SimulationState::Paused)),
            )
            // Exit Systems
            .add_systems(OnExit(SimulationState::Paused), PauseMenuManager::terminate);
    }
}

impl PauseMenuManager {
    fn initialize(mut commands: Commands, asset_server: Res<AssetServer>) {
        Self::build_pause_menu(&mut commands, &asset_server);
    }

    fn terminate(mut commands: Commands, pause_menu_query: Query<Entity, With<PauseMenu>>) {
        if let Ok(pause_menu_entity) = pause_menu_query.get_single() {
            commands.entity(pause_menu_entity).despawn_recursive();
        }
    }

    fn interact_with_resume_button(
        mut button_query: Query<
            (&Interaction, &mut BackgroundColor),
            (Changed<Interaction>, With<ResumeButton>),
        >,
        mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
    ) {
        for (interaction, mut color) in button_query.iter_mut() {
            match *interaction {
                Interaction::Pressed => {
                    *color = PRESSED_BUTTON_COLOR.into();
                    simulation_state_next_state.set(SimulationState::Running);
                }
                Interaction::Hovered => {
                    *color = HOVERED_BUTTON_COLOR.into();
                }
                Interaction::None => {
                    *color = NORMAL_BUTTON_COLOR.into();
                }
            }
        }
    }

    fn interact_with_main_menu_button(
        mut unload_save_game_event_writer: EventWriter<UnloadGame>,
        mut button_query: Query<
            (&Interaction, &mut BackgroundColor),
            (Changed<Interaction>, With<MainMenuButton>),
        >,
    ) {
        for (interaction, mut color) in button_query.iter_mut() {
            match *interaction {
                Interaction::Pressed => {
                    *color = PRESSED_BUTTON_COLOR.into();
                    unload_save_game_event_writer.send(UnloadGame {
                        quit_mode: GameQuitMode::QuitToMainMenu,
                    });
                }
                Interaction::Hovered => {
                    *color = HOVERED_BUTTON_COLOR.into();
                }
                Interaction::None => {
                    *color = NORMAL_BUTTON_COLOR.into();
                }
            }
        }
    }

    fn interact_with_quit_button(
        mut unload_save_game_event_writer: EventWriter<UnloadGame>,
        mut button_query: Query<
            (&Interaction, &mut BackgroundColor),
            (Changed<Interaction>, With<QuitButton>),
        >,
    ) {
        for (interaction, mut color) in button_query.iter_mut() {
            match *interaction {
                Interaction::Pressed => {
                    *color = PRESSED_BUTTON_COLOR.into();
                    unload_save_game_event_writer.send(UnloadGame {
                        quit_mode: GameQuitMode::QuitToDesktop,
                    });
                }
                Interaction::Hovered => {
                    *color = HOVERED_BUTTON_COLOR.into();
                }
                Interaction::None => {
                    *color = NORMAL_BUTTON_COLOR.into();
                }
            }
        }
    }

    fn build_pause_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
        let pause_menu_entity = commands
            .spawn((
                NodeBundle {
                    style: PAUSE_MENU_STYLE,
                    z_index: ZIndex::Local(1), // See Ref. 1
                    ..default()
                },
                PauseMenu {},
            ))
            .with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: PAUSE_MENU_CONTAINER_STYLE,
                        background_color: BACKGROUND_COLOR.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        // Title
                        parent.spawn(TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    "Pause Menu",
                                    UIManager::get_title_text_style(&asset_server),
                                )],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        });
                        // Resume Button
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: BUTTON_STYLE,
                                    background_color: NORMAL_BUTTON_COLOR.into(),
                                    ..default()
                                },
                                ResumeButton {},
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle {
                                    style: Style { ..default() },
                                    text: Text {
                                        sections: vec![TextSection::new(
                                            "Resume",
                                            UIManager::get_button_text_style(&asset_server),
                                        )],
                                        alignment: TextAlignment::Center,
                                        ..default()
                                    },
                                    ..default()
                                });
                            });
                        // Main Menu Button
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: BUTTON_STYLE,
                                    background_color: NORMAL_BUTTON_COLOR.into(),
                                    ..default()
                                },
                                MainMenuButton {},
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle {
                                    style: Style { ..default() },
                                    text: Text {
                                        sections: vec![TextSection::new(
                                            "Main Menu",
                                            UIManager::get_button_text_style(&asset_server),
                                        )],
                                        alignment: TextAlignment::Center,
                                        ..default()
                                    },
                                    ..default()
                                });
                            });
                        // Quit Button
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
                                    style: Style { ..default() },
                                    text: Text {
                                        sections: vec![TextSection::new(
                                            "Quit",
                                            UIManager::get_button_text_style(&asset_server),
                                        )],
                                        alignment: TextAlignment::Center,
                                        ..default()
                                    },
                                    ..default()
                                });
                            });
                    });
            })
            .id();

        pause_menu_entity
    }
}

// Module Functions
