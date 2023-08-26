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
    mut game_over_event_writer: EventWriter<DeleteSaveGame>,
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
                game_over_event_writer.send(DeleteSaveGame {
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
    mut load_save_game_event_writer: EventWriter<LoadSaveGame>,
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
                load_save_game_event_writer.send(LoadSaveGame {
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

pub fn handle_confirm_created_save_game_event(
    mut confirm_created_save_game_event_reader: EventReader<ConfirmCreatedSaveGame>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Some(_) = confirm_created_save_game_event_reader.iter().next() {
        app_state_next_state.set(AppState::SaveGamesMenu);
    }
}

pub fn handle_confirm_deleted_save_game_event(
    mut commands: Commands,
    mut confirm_deleted_save_game_event_reader: EventReader<ConfirmDeletedSaveGame>,
    mut save_game_query: Query<(Entity, &SaveGame)>,
) {
    if let Some(event) = confirm_deleted_save_game_event_reader.iter().next() {
        for (entity, save_game) in save_game_query.iter_mut() {
            if save_game.name == event.save_game.clone() {
                commands.entity(entity).despawn_recursive();
                return;
            }
        }
    }
}
