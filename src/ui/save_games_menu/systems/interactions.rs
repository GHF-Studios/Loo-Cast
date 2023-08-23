use crate::save_game::events::*;
use crate::ui::save_games_menu::components::*;
use crate::ui::styles::*;
use crate::AppState;

use bevy::prelude::*;

pub fn interact_with_back_to_main_menu_button(
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

pub fn interact_with_create_save_game_button(
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

pub fn interact_with_delete_save_game_button(
    mut game_over_event_writer: EventWriter<DeleteSaveGameEvent>,
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
                game_over_event_writer.send(DeleteSaveGameEvent {
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

pub fn interact_with_load_save_game_button(
    mut game_over_event_writer: EventWriter<LoadSaveGameEvent>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<LoadSaveGameButton>),
    >,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                game_over_event_writer.send(LoadSaveGameEvent {
                    save_game_name: "save_game_name".to_string(),
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
