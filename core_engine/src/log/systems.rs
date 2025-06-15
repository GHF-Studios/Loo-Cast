use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use egui::{vec2, Sense, Rect, Color32};

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

    egui::Window::new("Log Viewer")
        .default_size([700.0, 450.0])
        .min_width(350.0)
        .min_height(250.0)
        .show(egui_ctx.ctx_mut(), |ui| {
            let avail      = ui.available_size();
            let splitter_w = 5.0;
            state.split_ratio = state.split_ratio.clamp(0.15, 0.85);

            let tree_w    = (avail.x * state.split_ratio).clamp(150.0, avail.x - 150.0 - splitter_w);
            let console_w = avail.x - tree_w - splitter_w;

            // tree rect
            let tree_rect = Rect::from_min_size(ui.min_rect().min, vec2(tree_w, avail.y));

            // splitter rect
            let split_rect = Rect::from_min_max(
                tree_rect.right_top(),
                tree_rect.right_top() + vec2(splitter_w, avail.y),
            );
            let split_resp = ui.allocate_rect(split_rect, Sense::click_and_drag());

            // console rect
            let console_rect = Rect::from_min_size(split_rect.right_top(), vec2(console_w, avail.y));

            // ----- drag logic
            let mut new_split_x = tree_rect.width();

            if split_resp.dragged_by(egui::PointerButton::Primary) {
                new_split_x += split_resp.drag_delta().x;
            }
            
            let min_tree = 150.0;
            let max_tree = avail.x - 150.0 - splitter_w;
            
            new_split_x = new_split_x.clamp(min_tree, max_tree);
            state.split_ratio = new_split_x / avail.x;

            // paint splitter bar
            ui.painter().rect_filled(split_rect, 0.0, Color32::DARK_GRAY);

            // -------- left pane (tree) --------
            let tree_inner = ui.allocate_ui_at_rect(tree_rect, |ui| {
                let old_spacing = ui.spacing().item_spacing;
                ui.spacing_mut().item_spacing.x = 4.0;

                render_selectable_tree(ui, &log_tree.0, &mut state);

                ui.spacing_mut().item_spacing = old_spacing;
            });

            // -------- right pane (console) --------
            let console_inner = ui.allocate_ui_at_rect(console_rect, |ui| {
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
