use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use egui::Color32;

use crate::{log::{resources::LogRegistryHandle, ui::{functions::{format_log, gather_logs, left_panel_toolbar_ui, render_selection_tree, right_panel_toolbar_ui}, resources::LogViewerState}}, ui::toolbar::resources::ToolbarState};

pub(super) fn show_log_viewer_ui(
    mut egui_ctx: EguiContexts,
    toolbar_state: Res<ToolbarState>,
    mut log_viewer_state: ResMut<LogViewerState>,
    log_registry: Res<LogRegistryHandle>,
) {
    if !toolbar_state.show_log_viewer_ui { return; }

    egui::Window::new("Log Viewer")
        .default_size([700.0, 450.0])
        .min_width(350.0)
        .min_height(250.0)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.columns(2, |cols| {
                // Left panel (Tree + Toolbar)
                cols[0].vertical(|ui| {
                    left_panel_toolbar_ui(ui, &mut log_viewer_state.tree_mode);
                    render_selection_tree(ui, &mut log_viewer_state, &log_registry.0, &span_tree.0, &location_tree.0);
                });

                // Right panel (Console + Toolbar)
                cols[1].vertical(|ui| {
                    right_panel_toolbar_ui(ui, &mut log_viewer_state);
                    let logs = gather_logs(&log_viewer_state, &log_registry.0, &span_tree.0, &location_tree.0);
                    let row_h = ui.text_style_height(&egui::TextStyle::Monospace);
                    egui::ScrollArea::vertical()
                        .stick_to_bottom(true)
                        .show_rows(ui, row_h, logs.len(), |ui, range| {
                            for i in range {
                                ui.label(format_log(&logs[i]));
                            }
                        });
                });
            });
        });
}


