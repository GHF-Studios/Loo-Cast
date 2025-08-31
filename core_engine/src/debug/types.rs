use bevy::prelude::Reflect;
use egui_dock::TabViewer;

use super::functions::draw_game_view;
use super::resources::DebugSuiteUIState;

#[derive(Default, Reflect)]
pub enum DebugObjectMovement {
    #[default]
    Static,
    Circle {
        radius: f32,
        speed: f32,
    },
    Line {
        distance: f32,
        speed: f32,
    },
}

#[derive(Default, PartialEq, Eq, Clone, Copy, Reflect)]
pub enum StepMode {
    #[default]
    None,
    Cycles,
    Seconds,
}

#[derive(Default, PartialEq, Reflect)]
pub struct StepConfig {
    pub cycles: u32,
    pub seconds: f32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DebugSuiteTab {
    GameView,
    Hierarchy,
    Resources,
    Assets,
    Inspector,
    ChunkManager,
    IntentBuffer,
    IntentCommit,
    ChunkInspector,
}

pub(super) struct DebugSuiteTabViewer<'a> {
    pub state: &'a mut DebugSuiteUIState,
    pub game_view_texture_id: Option<egui::TextureId>,
    pub game_view_texture_size: Option<egui::Vec2>
}

impl TabViewer for DebugSuiteTabViewer<'_> {
    type Tab = DebugSuiteTab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        format!("{:?}", tab).into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            DebugSuiteTab::GameView => {
                self.state.viewport_rect = Some(ui.clip_rect());

                if let (Some(texture_id), Some(texture_size)) = (self.game_view_texture_id, self.game_view_texture_size) {
                    draw_game_view(ui, texture_id, texture_size);
                } else {
                    ui.label("Game View Render Texture not available yet.");
                }
            }
            DebugSuiteTab::Hierarchy => { ui.label("Hierarchy (todo)"); },
            DebugSuiteTab::Resources => { ui.label("Resources (todo)"); },
            DebugSuiteTab::Assets => { ui.label("Assets (todo)"); },
            DebugSuiteTab::Inspector => { ui.label("Inspector (todo)"); },
            DebugSuiteTab::ChunkManager => { ui.label("Chunk Manager (todo)"); },
            DebugSuiteTab::IntentBuffer => { ui.label("Intent Buffer (todo)"); },
            DebugSuiteTab::IntentCommit => { ui.label("Intent Commit (todo)"); },
            DebugSuiteTab::ChunkInspector => { ui.label("Chunk Inspector (todo)"); },
        };
    }

    fn clear_background(&self, tab: &Self::Tab) -> bool {
        !matches!(tab, DebugSuiteTab::GameView)
    }
}