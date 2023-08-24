use super::components::*;
use super::events::*;

use bevy::prelude::*;

pub fn text_input_system(
    mut received_character_event_reader: EventReader<ReceivedCharacter>,
    //mut received_string_event_writer: EventWriter<ReceivedInput>,
    keyboard_input: Res<Input<KeyCode>>,
    mut input_field_query: Query<(&mut InputField, Entity)>,
) {
    for (mut input_field, input_field_entity) in input_field_query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::Return) {
            //received_string_event_writer.send(ReceivedInput {
            //    sender: input_field_entity,
            //    input: input_field.value.clone(),
            //});
            println!("Text input: {}", input_field.value);
            input_field.value.clear();
            continue;
        } else if keyboard_input.just_pressed(KeyCode::Back) {
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
