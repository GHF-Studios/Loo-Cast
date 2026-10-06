//! Developer overlay presentation, history, and input handling.
//!
//! ## Module map
//!
//! - `prompt`: Command prompt, completion and history input.
//! - `record`: Structured diagnostic record styling.
//! - `render`: Console window and script workspace presentation.
//! - `state`: Prompt editing and bounded command history.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

use super::completion::{char_to_byte_index, complete_command_input, completion_candidates};
use super::transport::{ConsoleLogLevel, ConsoleRecord, ConsoleRecordKind, timestamp_label};
use super::{
    CONSOLE_FOCUS_OWNER, ConsoleCommandRegistry, ConsoleCommandSource, ConsoleTransport,
    RuntimeVariableRegistry,
};
use crate::{
    devtools::{DeveloperScriptWorkbench, draw_script_workspace},
    input_focus::InputFocus,
};
use bevy::prelude::*;
use bevy_egui::{EguiContext, PrimaryEguiContext, egui};
use std::collections::VecDeque;
const MAX_SCROLLBACK: usize = 512;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(super) enum ConsoleTab {
    #[default]
    Console,
    Scripts,
}

#[derive(Resource)]
pub(super) struct ConsoleOverlay {
    pub(super) open: bool,
    pub(super) opened_this_frame: bool,
    pub(super) tab: ConsoleTab,
    pub(super) scripts_fullscreen: bool,
    pub(super) prompt: ConsolePrompt,
    pub(super) scrollback: VecDeque<ConsoleRecord>,
}

impl Default for ConsoleOverlay {
    fn default() -> Self {
        let mut scrollback = VecDeque::new();
        scrollback.push_back(ConsoleRecord::output(
            "Spacetime Engine developer console — overlay + stdin + Bevy tracing.",
        ));
        Self {
            open: false,
            opened_this_frame: false,
            tab: ConsoleTab::Console,
            scripts_fullscreen: false,
            prompt: ConsolePrompt::default(),
            scrollback,
        }
    }
}

impl ConsoleOverlay {
    pub(super) fn close_for_gameplay(&mut self) {
        self.open = false;
        self.prompt.reset_navigation();
    }

    pub(super) fn accept_record(&mut self, record: ConsoleRecord) {
        if record.kind == ConsoleRecordKind::Clear {
            self.scrollback.clear();
        } else {
            self.push(record);
        }
    }

    pub(super) fn push(&mut self, record: ConsoleRecord) {
        self.scrollback.push_back(record);
        while self.scrollback.len() > MAX_SCROLLBACK {
            self.scrollback.pop_front();
        }
    }
}

pub(super) fn toggle_console(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut console: ResMut<ConsoleOverlay>,
    mut focus: ResMut<InputFocus>,
) {
    if keyboard.just_pressed(KeyCode::Backquote) {
        console.open = !console.open;
        console.opened_this_frame = console.open;
        console.prompt.reset_navigation();
    } else if console.open && keyboard.just_pressed(KeyCode::Escape) {
        if console.tab == ConsoleTab::Scripts && console.scripts_fullscreen {
            console.scripts_fullscreen = false;
        } else {
            console.open = false;
            console.prompt.reset_navigation();
        }
    }

    focus.set_modal_claim(CONSOLE_FOCUS_OWNER, console.open);
}

mod prompt;
mod record;
mod render;
mod state;

use prompt::draw_command_console;
use record::console_record_layout;
pub(super) use render::draw_console;
use state::ConsolePrompt;
