use crate::bevy::prelude::*;

use crate::input::states::InputMode;

#[tracing::instrument(skip_all)]
pub(super) fn toggle_input_mode(keys: Res<ButtonInput<KeyCode>>, mode: Res<State<InputMode>>, mut next_mode: ResMut<NextState<InputMode>>) {
    let hotkey_help_chord = keys.pressed(KeyCode::F2) && (keys.pressed(KeyCode::KeyH) || keys.just_pressed(KeyCode::KeyH));
    if keys.just_pressed(KeyCode::F2) && !hotkey_help_chord {
        match mode.get() {
            InputMode::Debug => next_mode.set(InputMode::Release),
            InputMode::Release => next_mode.set(InputMode::Debug),
        }
    }
}
