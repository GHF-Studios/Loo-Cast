use crate::bevy::prelude::*;
use bevy_inspector_egui::bevy_inspector::hierarchy::SelectedEntities;
use egui::TextureId;
use egui_dock::{DockState, NodeIndex};
use std::collections::{HashMap, VecDeque};

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
    pub show_runtime_debug_overlay: bool,
    pub show_chunk_manager: bool,
    pub show_intent_buffer: bool,
    pub show_intent_commit: bool,
    pub show_chunk_inspector: bool,
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

#[derive(Resource, Debug, Clone, Reflect)]
#[reflect(Resource)]
pub struct PhenomenonSurfaceMeshingBudget {
    pub max_builds_per_frame: usize,
}
impl Default for PhenomenonSurfaceMeshingBudget {
    fn default() -> Self {
        Self { max_builds_per_frame: 3 }
    }
}

#[derive(Resource, Debug, Default)]
pub struct PhenomenonSurfaceMeshCache {
    pub entries: HashMap<u64, Handle<Mesh>>,
    pub lru: VecDeque<u64>,
    pub max_entries: usize,
}
impl PhenomenonSurfaceMeshCache {
    pub fn with_max_entries(max_entries: usize) -> Self {
        Self {
            entries: HashMap::new(),
            lru: VecDeque::new(),
            max_entries: max_entries.max(1),
        }
    }

    pub fn get(&mut self, signature: u64) -> Option<Handle<Mesh>> {
        let handle = self.entries.get(&signature).cloned()?;
        self.touch(signature);
        Some(handle)
    }

    pub fn insert(&mut self, signature: u64, handle: Handle<Mesh>) {
        self.entries.insert(signature, handle);
        self.touch(signature);
        while self.entries.len() > self.max_entries {
            let Some(evict_key) = self.lru.pop_front() else {
                break;
            };
            self.entries.remove(&evict_key);
        }
    }

    fn touch(&mut self, signature: u64) {
        if let Some(idx) = self.lru.iter().position(|key| *key == signature) {
            self.lru.remove(idx);
        }
        self.lru.push_back(signature);
    }
}
