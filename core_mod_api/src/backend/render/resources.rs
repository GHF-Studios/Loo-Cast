use crate::bevy::prelude::*;
use bevy_inspector_egui::bevy_inspector::hierarchy::SelectedEntities;

use crate::debug::types::{InspectorSelection, StepConfig, StepMode};

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct PrimaryWindowUiState {
    pub enabled: bool,
    pub pause_menu_open: bool,
    pub pause_menu_window_stack: Vec<PauseMenuWindow>,
    pub pause_menu_forced_pause: bool,
    pub show_runtime_debug_overlay: bool,
    pub show_chunk_manager: bool,
    pub show_intent_buffer: bool,
    pub show_intent_commit: bool,
    pub show_chunk_inspector: bool,
    pub remap_pick_targets_to_source_entities: bool,
    pub step_mode: StepMode,
    pub step_config: StepConfig,
    #[reflect(ignore)]
    pub viewport_rect: Option<egui::Rect>,
    #[reflect(ignore)]
    pub viewport_rect_precision_proxy: Option<egui::Rect>,
    #[reflect(ignore)]
    pub selected_entities: SelectedEntities,
    pub selection: InspectorSelection,
}
impl PrimaryWindowUiState {
    pub fn open_pause_menu(&mut self) {
        self.pause_menu_open = true;
        self.pause_menu_window_stack.clear();
        self.pause_menu_window_stack.push(PauseMenuWindow::Root);
    }

    pub fn close_pause_menu(&mut self) {
        self.pause_menu_open = false;
        self.pause_menu_window_stack.clear();
    }

    pub fn ensure_pause_menu_stack(&mut self) {
        if self.pause_menu_open && self.pause_menu_window_stack.is_empty() {
            self.pause_menu_window_stack.push(PauseMenuWindow::Root);
        }
    }

    pub fn active_pause_menu_window(&self) -> PauseMenuWindow {
        self.pause_menu_window_stack.last().copied().unwrap_or(PauseMenuWindow::Root)
    }

    pub fn push_pause_menu_window(&mut self, window: PauseMenuWindow) {
        self.ensure_pause_menu_stack();
        if self.pause_menu_window_stack.last().copied() != Some(window) {
            self.pause_menu_window_stack.push(window);
        }
    }

    pub fn pop_pause_menu_window_or_close(&mut self) {
        self.ensure_pause_menu_stack();
        if self.pause_menu_window_stack.len() > 1 {
            self.pause_menu_window_stack.pop();
        } else {
            self.close_pause_menu();
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Default)]
pub enum PauseMenuWindow {
    #[default]
    Root,
    Settings,
}

#[derive(Resource, Reflect, Debug, Clone, Copy, PartialEq, Eq)]
#[reflect(Resource)]
pub struct RuntimeDebugToggles {
    pub chunk_locator_enabled: bool,
    pub show_hotkey_help: bool,
}
impl Default for RuntimeDebugToggles {
    fn default() -> Self {
        Self {
            chunk_locator_enabled: true,
            show_hotkey_help: false,
        }
    }
}
