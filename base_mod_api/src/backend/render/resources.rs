use crate::bevy::prelude::*;
use egui::TextureId;
use egui_dock::{DockState, NodeIndex};

use crate::config::statics::CONFIG;
use crate::debug::types::DebugSuiteTab;
pub use crate::core_mod_api::render::resources::{PauseMenuWindow, PrimaryWindowUiState, RuntimeDebugToggles};
use crate::usf::scale::Scale;

/// The current scale of the camera (0 = base, +1 = one scale up, -1 = one down, etc.)
#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct ViewScale {
    pub discrete: i32, // Current scale
    pub offset: f32,   // Fractional offset between this and next (for blending)
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct RenderPrecisionAnchor {
    pub active_scale: Scale,
    pub active_scale_index: i16,
    pub player_root_native: [f64; 3],
}
impl Default for RenderPrecisionAnchor {
    fn default() -> Self {
        Self {
            active_scale: Scale::MAX,
            active_scale_index: Scale::MAX.index_from_top() as i16,
            player_root_native: [0.0, 0.0, 0.0],
        }
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
