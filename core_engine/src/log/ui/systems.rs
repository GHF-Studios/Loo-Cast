use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{log::{resources::LogRegistry, ui::{functions::{format_log, gather_logs, render_console, render_console_toolbar, render_selection_tree, render_selection_tree_toolbar}, resources::LogViewerState}}, ui::toolbar::resources::ToolbarState};

pub(super) fn show_log_viewer_ui(
    mut egui_ctx: EguiContexts,
    toolbar_state: Res<ToolbarState>,
    mut log_viewer_state: ResMut<LogViewerState>,
    mut log_registry: ResMut<LogRegistry>,
) {
    if !toolbar_state.show_log_viewer_ui { return; }
    let ctx = match egui_ctx.try_ctx_mut() {
        Some(ctx) => ctx,
        None => {
            return;
        },
    };

    egui::Window::new("Log Viewer")
        .default_size([700.0, 450.0])
        .min_width(350.0)
        .min_height(250.0)
        .show(ctx, |ui| {
            ui.columns(2, |cols| {
                // Left panel (SelectionTree + Toolbar)
                cols[0].vertical(|ui| {
                    render_selection_tree_toolbar(ui, &mut log_registry);

                    let height = ui.available_height();
                    let width = ui.available_width();

                    egui::Frame::none()
                        .fill(egui::Color32::from_rgb(20, 30, 50))
                        .show(ui, |ui| {
                            ui.set_min_height(height);
                            ui.set_min_width(width);
                        
                            render_selection_tree(ui, &mut log_registry);
                        });
                });

                // Right panel (Console + Toolbar)
                cols[1].vertical(|ui| {
                    render_console_toolbar(ui, &mut log_viewer_state);
                    
                    let height = ui.available_height();
                    let width = ui.available_width();

                    egui::Frame::none()
                        .fill(egui::Color32::from_rgb(50, 20, 20))
                        .show(ui, |ui| {
                            ui.set_min_height(height);
                            ui.set_min_width(width);
                        
                            render_console(ui, &log_viewer_state, &mut log_registry);
                        });
                });
            });
        });
}
