use crate::game::events::*;
use crate::save_game::events::DeleteSaveGame;
use crate::save_game::resources::*;
use crate::ui::save_games_menu::components::*;
use crate::ui::save_games_menu::events::*;
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
    mut delete_save_game_event_writer: EventWriter<DeleteSaveGame>,
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
                delete_save_game_event_writer.send(DeleteSaveGame {
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
    mut load_save_game_instance_event_writer: EventWriter<LoadSaveGameInstance>,
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
                load_save_game_instance_event_writer.send(LoadSaveGameInstance {
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

pub fn handle_load_save_game_instance(
    mut load_save_game_instance_event_reader: EventReader<LoadSaveGameInstance>,
    mut load_game_event_writer: EventWriter<LoadGame>,
    save_game_manager: Res<SaveGameManager>,
) {
    if let Some(event) = load_save_game_instance_event_reader.iter().last() {
        if let Some(save_game) = save_game_manager.get_save_game_info(event.save_game_name.clone())
        {
            load_game_event_writer.send(LoadGame {
                save_game: save_game.clone(),
            });
        }
    }
}

pub fn handle_delete_save_game_ui(
    mut commands: Commands,
    mut delete_save_game_ui_event_reader: EventReader<DeleteSaveGameUI>,
    mut save_game_query: Query<(Entity, &SaveGame)>,
) {
    if let Some(event) = delete_save_game_ui_event_reader.iter().next() {
        for (entity, save_game) in save_game_query.iter_mut() {
            if save_game.name == event.save_game_name {
                commands.entity(entity).despawn_recursive();
                return;
            }
        }
    }
}
