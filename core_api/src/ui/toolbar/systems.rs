use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::ui::toolbar::resources::ToolbarState;

#[deprecated]
pub(super) fn show_toolbar_ui(mut egui_ctx: EguiContexts, keys: Res<ButtonInput<KeyCode>>, mut toolbar_state: ResMut<ToolbarState>) {
    if keys.just_pressed(KeyCode::F3) {
        if toolbar_state.enabled {
            toolbar_state.disable_all();
        } else {
            toolbar_state.enable();
            toolbar_state.show_perf_ui = true;
        }
    }

    if !toolbar_state.enabled {
        return;
    }

    // let ctx = match egui_ctx.ctx_mut() {
    //     Ok(ctx) => ctx,
    //     Err(_) => {
    //         return;
    //     }
    // };
    //
    // egui::Window::new("Toolbar")
    //     .anchor(egui::Align2::LEFT_TOP, [8.0, 8.0])
    //     .title_bar(false)
    //     .resizable(false)
    //     .collapsible(false)
    //     .show(ctx, |ui| {
    //         ui.horizontal(|ui| {
    //             if ui.button("PUI").clicked() {
    //                 toolbar_state.show_perf_ui = !toolbar_state.show_perf_ui;
    //             }
    //             if ui.button("LVUI").clicked() {
    //                 toolbar_state.show_log_viewer_ui = !toolbar_state.show_log_viewer_ui;
    //             }
    //             if ui.button("CMDUI").clicked() {
    //                 toolbar_state.show_chunk_manager_debug_ui = !toolbar_state.show_chunk_manager_debug_ui;
    //             }
    //             if ui.button("LRDUI").clicked() {
    //                 toolbar_state.show_log_registry_debug_ui = !toolbar_state.show_log_registry_debug_ui;
    //             }
    //         })
    //     });
}
