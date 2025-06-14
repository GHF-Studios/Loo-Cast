use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::log::{functions::render_log_tree, resources::LogTreeHandle};

pub(super) fn show_debug_ui(
    mut egui_ctx: EguiContexts,
    log_tree_handle: Res<LogTreeHandle>,
) {
    egui::Window::new("Tracing Viewer").show(egui_ctx.ctx_mut(), |ui| {
        let tree = log_tree_handle.0.lock().unwrap();
        render_log_tree(ui, &tree);
    });
}
