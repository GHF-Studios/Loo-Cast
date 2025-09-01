use bevy::prelude::*;
use bevy_inspector_egui::bevy_inspector::hierarchy::SelectedEntities;
use egui_dock::{DockState, NodeIndex};

use crate::debug::types::InspectorSelection;

use super::types::{DebugSuiteTab, StepMode, StepConfig};

#[derive(Resource)]
pub struct DebugSuiteUiDockState {
    pub dock_state: DockState<DebugSuiteTab>,
}
impl Default for DebugSuiteUiDockState {
    fn default() -> Self {
        
        let mut dock_state = DockState::new(vec![DebugSuiteTab::GameView]);
        let tree = dock_state.main_surface_mut();
        let [game, _inspector] = tree.split_right(NodeIndex::root(), 0.75, vec![DebugSuiteTab::Inspector]);
        let [game, _hierarchy] = tree.split_left(game, 0.2, vec![DebugSuiteTab::Hierarchy]);
        let [_game, _bottom] = tree.split_below(game, 0.666, vec![
            DebugSuiteTab::Resources, 
            DebugSuiteTab::Assets,
            DebugSuiteTab::ChunkManager,
            DebugSuiteTab::IntentBuffer,
            DebugSuiteTab::IntentCommit,
            DebugSuiteTab::ChunkInspector
        ]);

        Self { dock_state }
    }
}

#[derive(Resource, Default)]
pub struct DebugSuiteUiState {
    pub enabled: bool,
    pub show_chunk_manager: bool,
    pub show_intent_buffer: bool,
    pub show_intent_commit: bool,
    pub show_chunk_inspector: bool,
    pub step_mode: StepMode,
    pub step_config: StepConfig,
    pub viewport_rect: Option<egui::Rect>,
    pub selected_entities: SelectedEntities,
    pub selection: InspectorSelection,
}