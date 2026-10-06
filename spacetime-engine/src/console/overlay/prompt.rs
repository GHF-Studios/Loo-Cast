//! Command prompt, completion and history input.

use super::*;

pub(super) fn draw_command_console(
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
    let output = egui::TextEdit::singleline(&mut console.prompt.input)
        .font(egui::TextStyle::Monospace)
        .desired_width(f32::INFINITY)
        .hint_text("command")
        .show(ui);
    let response = output.response;
    let mut cursor = output.cursor_range.map_or_else(
        || console.prompt.input.len(),
        |range| char_to_byte_index(&console.prompt.input, range.primary.index),
    );

    if console.opened_this_frame {
        console.prompt.input.clear();
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
        console.prompt.history_up();
        cursor = console.prompt.input.len();
        response.request_focus();
    }
    if history_down {
        console.prompt.history_down();
        cursor = console.prompt.input.len();
        response.request_focus();
    }
    if complete {
        cursor = complete_command_input(
            &mut console.prompt.input,
            cursor,
            registry,
            runtime_variables,
        );
        response.request_focus();
    }
    if submit {
        if let Some(command) = console.prompt.submit() {
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
    let completions =
        completion_candidates(&console.prompt.input, cursor, registry, runtime_variables);
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
