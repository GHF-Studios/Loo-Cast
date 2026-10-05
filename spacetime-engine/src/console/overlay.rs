//! Developer overlay presentation, history, and input handling.

use super::command::{char_to_byte_index, complete_command_input, completion_candidates};
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
const MAX_HISTORY: usize = 128;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(super) enum ConsoleTab {
    #[default]
    Console,
    Scripts,
}

#[derive(Resource)]
pub(super) struct ConsoleOverlay {
    open: bool,
    opened_this_frame: bool,
    tab: ConsoleTab,
    scripts_fullscreen: bool,
    input: String,
    history: Vec<String>,
    history_cursor: Option<usize>,
    scrollback: VecDeque<ConsoleRecord>,
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
            input: String::new(),
            history: Vec::new(),
            history_cursor: None,
            scrollback,
        }
    }
}

impl ConsoleOverlay {
    pub(super) fn close_for_gameplay(&mut self) {
        self.open = false;
        self.history_cursor = None;
    }

    pub(super) fn accept_record(&mut self, record: ConsoleRecord) {
        if record.kind == ConsoleRecordKind::Clear {
            self.scrollback.clear();
        } else {
            self.push(record);
        }
    }

    fn push(&mut self, record: ConsoleRecord) {
        self.scrollback.push_back(record);
        while self.scrollback.len() > MAX_SCROLLBACK {
            self.scrollback.pop_front();
        }
    }

    fn submit(&mut self) -> Option<String> {
        let command = self.input.trim().to_string();
        self.input.clear();
        self.history_cursor = None;
        if command.is_empty() {
            return None;
        }

        if self.history.last() != Some(&command) {
            self.history.push(command.clone());
            if self.history.len() > MAX_HISTORY {
                self.history.remove(0);
            }
        }

        Some(command)
    }

    fn history_up(&mut self) {
        if self.history.is_empty() {
            return;
        }
        let next = match self.history_cursor {
            None => self.history.len() - 1,
            Some(0) => 0,
            Some(index) => index - 1,
        };
        self.history_cursor = Some(next);
        self.input.clone_from(&self.history[next]);
    }

    fn history_down(&mut self) {
        let Some(index) = self.history_cursor else {
            return;
        };
        if index + 1 >= self.history.len() {
            self.history_cursor = None;
            self.input.clear();
        } else {
            let next = index + 1;
            self.history_cursor = Some(next);
            self.input.clone_from(&self.history[next]);
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
        console.history_cursor = None;
    } else if console.open && keyboard.just_pressed(KeyCode::Escape) {
        if console.tab == ConsoleTab::Scripts && console.scripts_fullscreen {
            console.scripts_fullscreen = false;
        } else {
            console.open = false;
            console.history_cursor = None;
        }
    }

    focus.set_modal_claim(CONSOLE_FOCUS_OWNER, console.open);
}

fn console_record_layout(record: &ConsoleRecord) -> egui::text::LayoutJob {
    let mut job = egui::text::LayoutJob::default();
    let font = egui::FontId::new(12.5, egui::FontFamily::Monospace);

    let timestamp = egui::Color32::from_rgb(105, 112, 120);
    let separator = egui::Color32::from_rgb(92, 100, 108);
    let target = egui::Color32::from_rgb(100, 184, 214);
    let message = egui::Color32::from_rgb(218, 222, 226);
    let field_key = egui::Color32::from_rgb(192, 146, 224);
    let field_value = egui::Color32::from_rgb(218, 188, 116);
    let command = egui::Color32::from_rgb(96, 190, 220);
    let output = egui::Color32::from_rgb(202, 208, 214);
    let error = egui::Color32::from_rgb(248, 105, 96);

    let mut append = |text: &str, color: egui::Color32| {
        job.append(
            text,
            0.0,
            egui::text::TextFormat {
                font_id: font.clone(),
                color,
                ..default()
            },
        );
    };

    append(&timestamp_label(record.timestamp_millis), timestamp);
    append("  ", separator);

    match record.kind {
        ConsoleRecordKind::Trace(level) => {
            append(&format!("{:<5}", level.label()), level_color(level));
            append("  ", separator);
            append(record.target.as_deref().unwrap_or("<unknown>"), target);
            append("  ", separator);
            append(&record.text, message);

            for (key, value) in &record.fields {
                append("  ", separator);
                append(key, field_key);
                append("=", separator);
                append(value, field_value);
            }
        }
        ConsoleRecordKind::Command => {
            append("CMD  ", command);
            append(
                &format!(
                    "[{}]",
                    record.source.map_or("?", ConsoleCommandSource::label)
                ),
                separator,
            );
            append("  › ", command);
            append(&record.text, message);
        }
        ConsoleRecordKind::Output => {
            append("OUT  ", separator);
            append(&record.text, output);
        }
        ConsoleRecordKind::Error => {
            append("ERR  ", error);
            append(&record.text, error);
        }
        ConsoleRecordKind::Clear => {}
    }

    job
}

fn level_color(level: ConsoleLogLevel) -> egui::Color32 {
    match level {
        ConsoleLogLevel::Trace => egui::Color32::from_rgb(172, 132, 205),
        ConsoleLogLevel::Debug => egui::Color32::from_rgb(104, 158, 220),
        ConsoleLogLevel::Info => egui::Color32::from_rgb(108, 190, 126),
        ConsoleLogLevel::Warn => egui::Color32::from_rgb(236, 185, 82),
        ConsoleLogLevel::Error => egui::Color32::from_rgb(248, 105, 96),
    }
}

pub(super) fn draw_console(
    mut contexts: Query<&mut EguiContext, With<PrimaryEguiContext>>,
    mut console: ResMut<ConsoleOverlay>,
    registry: Res<ConsoleCommandRegistry>,
    runtime_variables: Res<RuntimeVariableRegistry>,
    transport: Res<ConsoleTransport>,
    mut script_workbench: Option<ResMut<DeveloperScriptWorkbench>>,
) {
    if !console.open {
        return;
    }
    let Ok(mut context) = contexts.single_mut() else {
        return;
    };
    let ctx = context.get_mut();
    let content_rect = ctx.input(|input| input.content_rect());
    let (size, position) =
        console_window_geometry(content_rect, console.tab, console.scripts_fullscreen);

    egui::Area::new(egui::Id::new("spacetime_developer_console"))
        .order(egui::Order::Foreground)
        .fixed_pos(position)
        .default_size(size)
        .show(ctx, |ui| {
            ui.set_width(size.x);
            ui.set_height(size.y);
            egui::Frame::new()
                .fill(egui::Color32::from_rgba_unmultiplied(10, 12, 14, 248))
                .stroke(egui::Stroke::new(
                    1.0_f32,
                    egui::Color32::from_rgb(72, 82, 92),
                ))
                .show(ui, |ui| {
                    ui.set_width(size.x);
                    ui.set_height(size.y);
                    draw_console_header(ui, &mut console);
                    ui.separator();
                    match console.tab {
                        ConsoleTab::Console => draw_command_console(
                            ui,
                            &mut console,
                            &registry,
                            &runtime_variables,
                            &transport,
                        ),
                        ConsoleTab::Scripts => {
                            if let Some(workbench) = script_workbench.as_mut() {
                                draw_script_workspace(ui, &mut *workbench);
                            } else {
                                ui.colored_label(
                                    egui::Color32::LIGHT_RED,
                                    "Script workspace resource is unavailable.",
                                );
                            }
                        }
                    }
                });
        });
}

fn console_window_geometry(
    content_rect: egui::Rect,
    tab: ConsoleTab,
    scripts_fullscreen: bool,
) -> (egui::Vec2, egui::Pos2) {
    let margin = 12.0_f32;
    let available_width = (content_rect.width() - margin * 2.0).max(320.0);
    let available_height = (content_rect.height() - margin * 2.0).max(220.0);
    match (tab, scripts_fullscreen) {
        (ConsoleTab::Scripts, true) => (content_rect.size(), content_rect.left_top()),
        (ConsoleTab::Scripts, false) => {
            let width = (content_rect.width() * 0.88)
                .clamp(760.0, 1320.0)
                .min(available_width);
            let height = (content_rect.height() * 0.78)
                .clamp(500.0, 900.0)
                .min(available_height);
            (
                egui::vec2(width, height),
                egui::pos2(
                    content_rect.center().x - width * 0.5,
                    content_rect.center().y - height * 0.5,
                ),
            )
        }
        (ConsoleTab::Console, _) => {
            let width = (content_rect.width() * 0.62)
                .clamp(620.0, 900.0)
                .min(available_width);
            let height = (content_rect.height() * 0.36)
                .clamp(240.0, 380.0)
                .min(available_height);
            (
                egui::vec2(width, height),
                egui::pos2(content_rect.left() + margin, content_rect.top() + margin),
            )
        }
    }
}

fn draw_console_header(ui: &mut egui::Ui, console: &mut ConsoleOverlay) {
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new("SPACETIME DEV")
                .monospace()
                .strong()
                .color(egui::Color32::from_rgb(220, 224, 228)),
        );
        ui.separator();

        if ui
            .selectable_label(console.tab == ConsoleTab::Console, "Console")
            .clicked()
        {
            console.tab = ConsoleTab::Console;
            console.scripts_fullscreen = false;
            console.opened_this_frame = true;
        }
        if ui
            .selectable_label(console.tab == ConsoleTab::Scripts, "Scripts")
            .clicked()
        {
            console.tab = ConsoleTab::Scripts;
            console.opened_this_frame = false;
        }

        if console.tab == ConsoleTab::Scripts {
            ui.separator();
            let label = if console.scripts_fullscreen {
                "Windowed"
            } else {
                "Fullscreen"
            };
            if ui.button(label).clicked() {
                console.scripts_fullscreen = !console.scripts_fullscreen;
            }
        }

        ui.separator();
        ui.label(
            egui::RichText::new(if console.tab == ConsoleTab::Console {
                "` toggle   ↑/↓ history   Tab complete"
            } else {
                "host-managed Rhai workspace   Save ≠ Commit"
            })
            .monospace()
            .small()
            .color(egui::Color32::from_rgb(130, 140, 150)),
        );
    });
}

fn draw_command_console(
    ui: &mut egui::Ui,
    console: &mut ConsoleOverlay,
    registry: &ConsoleCommandRegistry,
    runtime_variables: &RuntimeVariableRegistry,
    transport: &ConsoleTransport,
) {
    let scroll_height = (ui.available_height() - 52.0).max(80.0);
    egui::ScrollArea::vertical()
        .stick_to_bottom(true)
        .auto_shrink([false, false])
        .max_height(scroll_height)
        .show(ui, |ui| {
            for record in &console.scrollback {
                ui.add(egui::Label::new(console_record_layout(record)).selectable(true));
            }
        });
    ui.separator();
    draw_command_prompt(ui, console, registry, runtime_variables, transport);
}

fn draw_command_prompt(
    ui: &mut egui::Ui,
    console: &mut ConsoleOverlay,
    registry: &ConsoleCommandRegistry,
    runtime_variables: &RuntimeVariableRegistry,
    transport: &ConsoleTransport,
) {
    let output = egui::TextEdit::singleline(&mut console.input)
        .font(egui::TextStyle::Monospace)
        .desired_width(f32::INFINITY)
        .hint_text("command")
        .show(ui);
    let response = output.response;
    let mut cursor = output.cursor_range.map_or_else(
        || console.input.len(),
        |range| char_to_byte_index(&console.input, range.primary.index),
    );

    if console.opened_this_frame {
        console.input.clear();
        cursor = 0;
        response.request_focus();
        console.opened_this_frame = false;
    }
    if response.has_focus() || response.lost_focus() {
        cursor = handle_prompt_keys(
            ui,
            console,
            registry,
            runtime_variables,
            transport,
            &response,
            cursor,
        );
    }
    draw_completion_preview(ui, console, cursor, registry, runtime_variables);
}

fn handle_prompt_keys(
    ui: &egui::Ui,
    console: &mut ConsoleOverlay,
    registry: &ConsoleCommandRegistry,
    runtime_variables: &RuntimeVariableRegistry,
    transport: &ConsoleTransport,
    response: &egui::Response,
    mut cursor: usize,
) -> usize {
    let history_up = ui.input(|input| input.key_pressed(egui::Key::ArrowUp));
    let history_down = ui.input(|input| input.key_pressed(egui::Key::ArrowDown));
    let complete = ui.input(|input| input.key_pressed(egui::Key::Tab));
    let submit = ui.input(|input| input.key_pressed(egui::Key::Enter));

    if history_up {
        console.history_up();
        cursor = console.input.len();
        response.request_focus();
    }
    if history_down {
        console.history_down();
        cursor = console.input.len();
        response.request_focus();
    }
    if complete {
        cursor = complete_command_input(&mut console.input, cursor, registry, runtime_variables);
        response.request_focus();
    }
    if submit {
        if let Some(command) = console.submit() {
            transport.submit(ConsoleCommandSource::Overlay, command);
        }
        cursor = 0;
        response.request_focus();
    }
    cursor
}

fn draw_completion_preview(
    ui: &mut egui::Ui,
    console: &ConsoleOverlay,
    cursor: usize,
    registry: &ConsoleCommandRegistry,
    runtime_variables: &RuntimeVariableRegistry,
) {
    let completions = completion_candidates(&console.input, cursor, registry, runtime_variables);
    if completions.is_empty() {
        return;
    }
    let preview = completions
        .into_iter()
        .take(8)
        .collect::<Vec<_>>()
        .join("  ");
    ui.label(
        egui::RichText::new(preview)
            .monospace()
            .small()
            .color(egui::Color32::from_rgb(108, 136, 160)),
    );
}
