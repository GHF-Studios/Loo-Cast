use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{log::{resources::LogRegistryHandle, ui::{functions::{format_log, gather_logs, render_console, render_console_toolbar, render_selection_tree, render_selection_tree_toolbar}, resources::LogViewerState}}, ui::toolbar::resources::ToolbarState};

pub(super) fn show_log_viewer_ui(
    mut egui_ctx: EguiContexts,
    toolbar_state: Res<ToolbarState>,
    mut log_viewer_state: ResMut<LogViewerState>,
    log_registry_handle: Res<LogRegistryHandle>,
) {
    if !toolbar_state.show_log_viewer_ui { return; }

    egui::Window::new("Log Viewer")
        .default_size([700.0, 450.0])
        .min_width(350.0)
        .min_height(250.0)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.columns(2, |cols| {
                // Left panel (SelectionTree + Toolbar)
                cols[0].vertical(|ui| {
                    render_selection_tree_toolbar(ui, &mut log_viewer_state);
                    render_selection_tree(ui, &mut log_viewer_state, &mut log_registry_handle.0.lock().unwrap());
                });

                // Right panel (Console + Toolbar)
                cols[1].vertical(|ui| {
                    render_console_toolbar(ui, &mut log_viewer_state);
                    render_console(ui, &log_viewer_state, &log_registry_handle.0.lock().unwrap());
                });
            });
        });
}
