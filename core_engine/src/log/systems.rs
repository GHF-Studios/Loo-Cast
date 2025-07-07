use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use egui::Color32;

use crate::log::{
    arena::FilterTreeMode, functions::*, resources::*
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
    log_storage:     Res<LogStorageHandle>,
    span_tree:     Res<SpanTreeHandle>,
    location_tree:     Res<LocationTreeHandle>,
    mut viewer_state:    ResMut<LogViewerState>,
) {
    if !win.show_log_viewer { return; }

    egui::Window::new("Log Viewer")
        .default_size([700.0, 450.0])
        .min_width(350.0)
        .min_height(250.0)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.columns(2, |cols| {
                // Left panel (Tree + Toolbar)
                cols[0].vertical(|ui| {
                    left_panel_toolbar_ui(ui, &mut viewer_state.tree_mode);
                    render_selectable_tree(ui, &mut viewer_state, &log_storage.0, &span_tree.0, &location_tree.0);
                });

                // Right panel (Console)
                cols[1].vertical(|ui| {
                    right_panel_toolbar_ui(ui, &mut viewer_state);
                    let logs = gather_logs(&viewer_state, &log_storage.0, &span_tree.0, &location_tree.0);
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
        });
}

fn left_panel_toolbar_ui(ui: &mut egui::Ui, tree_mode: &mut FilterTreeMode) {
    egui::Frame::none()
        .fill(Color32::from_gray(25))
        .stroke(egui::Stroke::new(1.0, Color32::DARK_GRAY))
        .inner_margin(egui::Margin::symmetric(6.0, 4.0))
        .show(ui, |ui| {
            ui.label("Filter Mode:");
            ui.horizontal(|ui| {
                let selected = matches!(tree_mode, FilterTreeMode::Span);
                if ui.selectable_label(selected, "↔ Spans").clicked() {
                    *tree_mode = FilterTreeMode::Span;
                }

                let selected = matches!(tree_mode, FilterTreeMode::Loc);
                if ui.selectable_label(selected, "📦 Locations").clicked() {
                    *tree_mode = FilterTreeMode::Loc;
                }
            });
        });
}

fn right_panel_toolbar_ui(ui: &mut egui::Ui, viewer_state: &mut LogViewerState) {
    egui::Frame::none()
        .fill(Color32::from_gray(25))
        .stroke(egui::Stroke::new(1.0, Color32::DARK_GRAY))
        .inner_margin(egui::Margin::symmetric(6.0, 4.0))
        .show(ui, |ui| {
            right_panel_filter_ui(ui, &mut viewer_state.threshold);
        });
}
