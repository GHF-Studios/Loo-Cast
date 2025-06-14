use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::log::{
    arena::Arena,
    functions::*,
    resources::*,
};

pub(super) fn show_debug_ui(
    mut egui_ctx: EguiContexts,
    log_tree:     Res<LogTreeHandle>,
    mut state:    ResMut<LogViewerState>,
) {
    let ctx = egui_ctx.ctx_mut();

    // ───────── LEFT: resizable tree panel ─────────
    egui::SidePanel::left("log_tree")
        .resizable(true)                  // <- draggable edge!
        .default_width(250.0)
        .min_width(150.0)
        .show(ctx, |ui| {
            ui.heading("Hierarchy");
            render_selectable_tree(ui, &log_tree.0, &mut state);
        });

    // ───────── RIGHT: central console ─────────────
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.heading("Logs");
            ui.checkbox(&mut state.autoscroll, "autoscroll");
        });

        // collect once
        let logs = gather_logs(&log_tree.0, &state);

        let row_h = ui.text_style_height(&egui::TextStyle::Monospace);
        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .stick_to_bottom(state.autoscroll)
            .show_rows(ui, row_h, logs.len(), |ui, range| {
                for i in range {
                    ui.label(format_log_line(&logs[i]));
                }
            });
    });
}
