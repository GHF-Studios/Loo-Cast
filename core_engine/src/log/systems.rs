use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use egui::{vec2, Sense, Rect, Color32};

use crate::log::{
    arena::*,
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
            // -------- split geometry --------
            let avail      = ui.available_size();
            let splitter_w = 5.0;
            state.split_ratio = state.split_ratio.clamp(0.15, 0.85);

            let tree_w   = (avail.x * state.split_ratio).clamp(150.0, avail.x - 150.0 - splitter_w);
            let console_w = avail.x - tree_w - splitter_w;

            // tree rect
            let tree_rect = ui.allocate_rect(
                Rect::from_min_size(ui.min_rect().min, vec2(tree_w, avail.y)),
                Sense::hover(),
            ).rect;

            // splitter rect
            let split_rect = Rect::from_min_max(
                tree_rect.right_top(),
                tree_rect.right_top() + vec2(splitter_w, avail.y),
            );
            let split_resp = ui.allocate_rect(split_rect, Sense::click_and_drag());

            // console rect
            let console_rect = Rect::from_min_max(
                split_rect.right_top(),
                ui.min_rect().max,
            );
            // ----- drag logic
            if split_resp.dragged() {
                state.split_ratio += split_resp.drag_delta().x / avail.x;
            }
            // paint splitter bar
            ui.painter().rect_filled(split_rect, 0.0, Color32::DARK_GRAY);

            // -------- left pane (tree) --------
            let mut tree_ui = ui.child_ui_with_id_source(tree_rect, *ui.layout(), "tree_area", None);
            // tighter spacing:
            let old_spacing = tree_ui.spacing().item_spacing;
            tree_ui.spacing_mut().item_spacing.x = 4.0;
            render_selectable_tree(&mut tree_ui, &log_tree.0, &mut state);
            tree_ui.spacing_mut().item_spacing = old_spacing;

            // -------- right pane (console) --------
            let mut con_ui = ui.child_ui_with_id_source(console_rect, *ui.layout(), "console_area", None);

            con_ui.horizontal(|ui| {
                ui.heading("Logs");
            });

            let logs = gather_logs(&log_tree.0, &state);
            let row_h = con_ui.text_style_height(&egui::TextStyle::Monospace);
            egui::ScrollArea::vertical()
                .stick_to_bottom(true)
                .show_rows(&mut con_ui, row_h, logs.len(), |ui, range| {
                    for i in range { ui.label(format_log_line(&logs[i])); }
                });
        });
}

