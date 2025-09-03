use bevy::prelude::*;
use bevy_egui::EguiContexts;

use crate::{debug::resources::DebugSuiteUiState, input::states::InputMode};

#[tracing::instrument(skip_all)]
pub fn on_click_select(
    click: Trigger<Pointer<Click>>,
    mut ui_state: ResMut<DebugSuiteUiState>,
    mut egui_contexts: EguiContexts,
    input_mode: Res<State<InputMode>>,
) {
    error!("Click select triggered");

    let ctx = match egui_contexts.ctx_mut() {
        Ok(ctx) => ctx,
        Err(_) => {
            error!("Egui context not available");
            return;
        }
    };

    if !input_mode.is_debug_suite() {
        error!("Ignoring click select because input mode is not DebugSuite");
        return;
    }

    let pointer_pos = match ctx.pointer_hover_pos() {
        Some(pos) => pos,
        None => {
            error!("Pointer position not available");
            return;
        }
    };

    if let Some(viewport_rect) = ui_state.viewport_rect {
        if !viewport_rect.contains(pointer_pos) {
            error!("Pointer not in viewport rect");
            return;
        }
    } else {
        error!("Viewport rect not set");
        return;
    };

    let modifiers = ctx.input(|i| i.modifiers);
    let add = modifiers.ctrl || modifiers.shift;

    ui_state.selected_entities.select_maybe_add(click.target(), add);
}
