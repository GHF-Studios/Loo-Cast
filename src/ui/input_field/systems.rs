use crate::ui::events::*;
use crate::ui::resources::FocusManager;
use crate::ui::styles::*;

use super::components::*;
use super::events::*;

use bevy::prelude::*;

pub fn text_input_system(
    mut received_character_event_reader: EventReader<ReceivedCharacter>,
    mut received_string_event_writer: EventWriter<ReceivedInput>,
    keyboard_input: Res<Input<KeyCode>>,
    focus_manager: Res<FocusManager>,
    mut input_field_query: Query<(&mut InputField, Entity)>,
) {
    for (mut input_field, input_field_entity) in input_field_query.iter_mut() {
        if Some(input_field_entity) != focus_manager.focus {
            continue;
        }

        if keyboard_input.just_pressed(KeyCode::Return) {
            received_string_event_writer.send(ReceivedInput {
                sender: input_field_entity,
                input: input_field.value.clone(),
            });
            println!("Text input: {}", input_field.value);
            input_field.value.clear();
            continue;
        }

        if keyboard_input.just_pressed(KeyCode::Back) {
            if !input_field.value.is_empty() {
                input_field.value.pop();
            }
            continue;
        }

        for received_char_event in received_character_event_reader.iter() {
            let received_char = received_char_event.char;
            if received_char.is_alphanumeric() || received_char.is_whitespace() {
                input_field.value.push(received_char);
            }
        }
    }
}

pub fn text_render_system(mut input_field_query: Query<(&InputField, &mut Text)>) {
    for (input_field, mut input_field_text) in input_field_query.iter_mut() {
        input_field_text.sections[0].value = input_field.value.clone();
    }
}

pub fn interact_with_input_field(
    mut gained_focus_event_writer: EventWriter<GainedFocus>,
    focus_manager: Res<FocusManager>,
    mut input_field_query: Query<(&Interaction, Entity), (Changed<Interaction>, With<InputField>)>,
) {
    if let Ok((interaction, input_field_entity)) = input_field_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                if focus_manager.focus != Some(input_field_entity) {
                    gained_focus_event_writer.send(GainedFocus {
                        entity: input_field_entity,
                    });
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn handle_gained_focus_event(
    mut gained_focus_event_reader: EventReader<GainedFocus>,
    mut input_field_query: Query<(&mut BackgroundColor, Entity), With<InputField>>,
) {
    if let Some(gained_focus_event) = gained_focus_event_reader.iter().last() {
        if let Ok((mut background_color, input_field_entity)) =
            input_field_query.get_mut(gained_focus_event.entity)
        {
            if gained_focus_event.entity == input_field_entity {
                *background_color = FOCUSED_COLOR.into();
            }
        }
    }
}

pub fn handle_lost_focus_event(
    mut lost_focus_event_reader: EventReader<LostFocus>,
    mut input_field_query: Query<(&mut BackgroundColor, Entity), With<InputField>>,
) {
    if let Some(lost_focus_event) = lost_focus_event_reader.iter().last() {
        if let Ok((mut background_color, input_field_entity)) =
            input_field_query.get_mut(lost_focus_event.entity)
        {
            if lost_focus_event.entity == input_field_entity {
                *background_color = UNFOCUSED_COLOR.into();
            }
        }
    }
}
