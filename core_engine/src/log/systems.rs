use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::log::{
    functions::*,
    resources::*,
};

pub(super) fn show_toolbar_ui(mut egui_ctx: EguiContexts, mut win: ResMut<UiWindows>) {
    let ctx = egui_ctx.ctx_mut();

    egui::Window::new("toolbar")
        .anchor(egui::Align2::LEFT_TOP, [8.0, 8.0])
        .title_bar(false)
        .resizable(false)
        .collapsible(false)
        .show(ctx, |ui| {
            if ui.button("📜").clicked() {
                win.show_log_viewer = !win.show_log_viewer;
            }
        });
}

pub(super) fn show_log_viewer_ui(
    mut egui_ctx: EguiContexts,
    win:          Res<UiWindows>,
    log_tree:     Res<LogTreeHandle>,
    mut state:    ResMut<LogViewerState>,
) {
    if !win.show_log_viewer { return; }

    let ctx = egui_ctx.ctx_mut();
    egui::Window::new("Log Viewer")
        .default_size([600.0, 400.0])
        .min_height(200.0)
        .min_width(300.0)
        .show(ctx, |ui| {
            egui::SidePanel::left("tree")
                .resizable(true)
                .default_width(250.0)
                .show_inside(ui, |ui| {
                    ui.heading("Hierarchy");
                    render_selectable_tree(ui, &log_tree.0, &mut state);
                });

            egui::CentralPanel::default().show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("Logs");
                });

                let logs = gather_logs(&log_tree.0, &state);
                let row_h = ui.text_style_height(&egui::TextStyle::Monospace);
                egui::ScrollArea::vertical()
                    .stick_to_bottom(true)
                    .show_rows(ui, row_h, logs.len(), |ui, range| {
                        for i in range {
                            ui.label(format_log_line(&logs[i]));
                        }
                    });
            });
        });
}

