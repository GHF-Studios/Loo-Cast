use crate::bevy::prelude::*;
use bevy_inspector_egui::bevy_inspector::hierarchy::SelectedEntities;
use egui::TextureId;
use egui_dock::{DockState, NodeIndex};

use crate::config::statics::CONFIG;
use crate::debug::types::{DebugSuiteTab, InspectorSelection, StepConfig, StepMode};

/// The current scale of the camera (0 = base, +1 = one scale up, -1 = one down, etc.)
#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct ViewScale {
    pub discrete: i32, // Current scale
    pub offset: f32,   // Fractional offset between this and next (for blending)
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub(crate) struct ZoomFactor(pub f32);
impl Default for ZoomFactor {
    fn default() -> Self {
        Self(CONFIG().get::<f32>("camera/default_zoom"))
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub(crate) struct DevZoomFactor(pub f32);
impl Default for DevZoomFactor {
    fn default() -> Self {
        Self(CONFIG().get::<f32>("camera/default_dev_zoom"))
    }
}

#[derive(Resource)]
pub struct GameViewRenderTarget {
    pub handle: Handle<Image>,
    pub size: UVec2,
    pub id: TextureId,
}

#[derive(Resource)]
pub struct PrimaryWindowUiDockState {
    pub dock_state: DockState<DebugSuiteTab>,
}
impl Default for PrimaryWindowUiDockState {
    fn default() -> Self {
        let mut dock_state = DockState::new(vec![DebugSuiteTab::GameView]);
        let tree = dock_state.main_surface_mut();
        let [game, _inspector] = tree.split_right(NodeIndex::root(), 0.75, vec![DebugSuiteTab::Inspector]);
        let [game, _hierarchy] = tree.split_left(game, 0.2, vec![DebugSuiteTab::Hierarchy]);
        let [_game, _bottom] = tree.split_below(
            game,
            0.666,
            vec![
                DebugSuiteTab::Resources,
                DebugSuiteTab::Assets,
                DebugSuiteTab::ChunkManager,
                DebugSuiteTab::IntentBuffer,
                DebugSuiteTab::IntentCommit,
                DebugSuiteTab::ChunkInspector,
            ],
        );

        Self { dock_state }
    }
}

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
