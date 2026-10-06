//! Script explorer and editor surface embedded in the developer console.

use super::DeveloperScriptWorkbench;
use bevy_egui::egui;

mod editor;
mod explorer;

use editor::draw_script_editor;
use explorer::draw_script_explorer;

/// Starfall/E2-style script workspace embedded by the developer-console shell.
pub(crate) fn draw_script_workspace(ui: &mut egui::Ui, workbench: &mut DeveloperScriptWorkbench) {
    let available_height = ui.available_height().max(240.0);
    let explorer_width = (ui.available_width() * 0.22).clamp(170.0, 260.0);

    ui.horizontal(|ui| {
        ui.allocate_ui_with_layout(
            egui::vec2(explorer_width, available_height),
            egui::Layout::top_down(egui::Align::Min),
            |ui| draw_script_explorer(ui, workbench),
        );
        ui.separator();
        ui.allocate_ui_with_layout(
            egui::vec2(ui.available_width(), available_height),
            egui::Layout::top_down(egui::Align::Min),
            |ui| draw_script_editor(ui, workbench),
        );
    });
}
