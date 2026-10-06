//! Console bind dispatch after focus/cursor arbitration.

use super::PlayerInputBindings;
use crate::game::player::cursor::CursorCapture;
use crate::{
    console::{ConsoleCommandSource, ConsoleTransport},
    input_focus::InputFocus,
};
use bevy::prelude::*;

pub(in crate::game::player) fn dispatch_bound_console_commands(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    bindings: Res<PlayerInputBindings>,
    focus: Res<InputFocus>,
    capture: Res<CursorCapture>,
    transport: Res<ConsoleTransport>,
) {
    if focus.gameplay_claimed() || !capture.active() {
        return;
    }

    for (button, command) in bindings.console_commands() {
        if button.just_pressed(&keyboard, &mouse) {
            transport.submit(ConsoleCommandSource::Binding, command.clone());
        }
        if button.just_released(&keyboard, &mouse)
            && let Some(release) = command.strip_prefix('+')
        {
            transport.submit(ConsoleCommandSource::Binding, format!("-{release}"));
        }
    }
}
