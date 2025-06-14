use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::log::{functions::*, resources::{LogTreeHandle, LogViewerState}};

pub(super) fn show_debug_ui(
    mut egui_ctx: EguiContexts,
    log_tree:     Res<LogTreeHandle>,
    mut state:    ResMut<LogViewerState>,
) {
    egui::Window::new("Tracing Viewer").show(egui_ctx.ctx_mut(), |ui| {
        ui.columns(2, |cols| {
            // ───────────────── left = tree ─────────────────
            cols[0].vertical(|ui| {
                ui.heading("Hierarchy");
                render_selectable_tree(ui, &log_tree.0, &mut state);
            });

            // ───────────────── right = console ─────────────
            cols[1].vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.heading("Logs");
                    ui.checkbox(&mut state.autoscroll, "autoscroll");
                });

                let logs: Vec<_> = gather_logs(&log_tree.0, &state)
                    .iter()
                    .cloned()
                    .collect();

                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .stick_to_bottom(state.autoscroll)
                    .show(ui, |ui| {
                        for log in logs {
                            ui.label(format_log_line(&log));
                        }
                    });
            });
        });
    });
}

