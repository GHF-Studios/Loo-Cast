use bevy::prelude::*;
use egui_dock::{DockState, NodeIndex};

use super::types::{DebugSuiteTab, StepMode, StepConfig};

#[derive(Resource, Default, PartialEq)]
pub struct DebugSuiteUIState {
    pub show_chunk_manager: bool,
    pub show_intent_buffer: bool,
    pub show_intent_commit: bool,
    pub show_chunk_inspector: bool,
    pub is_paused: bool,
    pub step_mode: StepMode,
    pub step_config: StepConfig,
    pub viewport_rect: Option<egui::Rect>,
}

#[derive(Resource)]
pub struct DebugSuiteDock {
    pub dock_state: DockState<DebugSuiteTab>,
}

impl Default for DebugSuiteDock {
    fn default() -> Self {
        let mut dock_state = DockState::new(vec![DebugSuiteTab::GameView]);
        let tree = dock_state.main_surface_mut();

        // Split right for Inspector
        let game = NodeIndex::root();
        let inspector = tree.split_right(game, 0.75, vec![DebugSuiteTab::Inspector])[1];

        // Split left for Hierarchy
        let game = tree.split_left(game, 0.20, vec![DebugSuiteTab::Hierarchy])[1];

        // Split bottom for Assets/Resources
        tree.split_below(game, 0.75, vec![DebugSuiteTab::Resources, DebugSuiteTab::Assets]);

        // Add custom debug tabs under Inspector
        tree.split_below(inspector, 0.5, vec![
            DebugSuiteTab::ChunkManager,
            DebugSuiteTab::IntentBuffer,
            DebugSuiteTab::IntentCommit,
            DebugSuiteTab::ChunkInspector,
        ]);

        Self { dock_state }
    }
}