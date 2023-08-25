use crate::save_game::events::*;
use crate::ui::create_save_game_menu::components::*;
use crate::ui::input_field::components::InputField;
use crate::ui::styles::*;
use crate::AppState;

use bevy::prelude::*;

pub fn interact_with_cancel_create_save_game_button(
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<CancelCreateSaveGameButton>),
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

pub fn interact_with_confirm_create_save_game_button(
    mut create_save_game_event_writer: EventWriter<CreateSaveGame>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ConfirmCreateSaveGameButton>),
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