use std::any::TypeId;

use bevy::asset::UntypedAssetId;
use bevy::ecs::reflect::AppTypeRegistry;
use bevy::ecs::world::World;
use bevy::prelude::Reflect;
use bevy::state::state::State;
use bevy_inspector_egui::bevy_inspector::by_type_id::{ui_for_asset, ui_for_resource};
use bevy_inspector_egui::bevy_inspector::hierarchy::hierarchy_ui;
use bevy_inspector_egui::bevy_inspector::{ui_for_entities_shared_components, ui_for_entity_with_children};
use egui_dock::TabViewer;

use crate::debug::functions::{select_asset, select_resource};
use crate::input::states::InputMode;

use super::functions::draw_game_view;
use super::resources::DebugSuiteUiState;

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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect)]
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

#[derive(Clone, Default, Eq, PartialEq, Reflect)]
pub enum InspectorSelection {
    #[default]
    Entities,
    Resource(TypeId, String),
    Asset(TypeId, String, #[reflect(ignore)] Option<UntypedAssetId>),
}

pub(super) struct DebugSuiteTabViewer<'a> {
    pub world: &'a mut World,
    pub state: &'a mut DebugSuiteUiState,
    pub game_view_texture_id: Option<egui::TextureId>,
    pub game_view_texture_size: Option<egui::Vec2>,
}

impl TabViewer for DebugSuiteTabViewer<'_> {
    type Tab = DebugSuiteTab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        format!("{:?}", tab).into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        let type_registry = self.world.resource::<AppTypeRegistry>().0.clone();
        let type_registry = type_registry.read();

        match tab {
            DebugSuiteTab::GameView => {
                self.state.viewport_rect = Some(ui.clip_rect());

                let input_mode = self.world.resource::<State<InputMode>>();
                if input_mode.is_game() {
                    let rect = ui.clip_rect();
                    let stroke = egui::Stroke::new(12.0, egui::Color32::RED);
                    ui.painter().rect_stroke(rect, 0.0, stroke, egui::StrokeKind::Middle);
                }

                if let (Some(texture_id), Some(texture_size)) = (self.game_view_texture_id, self.game_view_texture_size) {
                    draw_game_view(ui, texture_id, texture_size);
                } else {
                    ui.label("Game View Render Texture not available yet.");
                }
            }
            DebugSuiteTab::Hierarchy => {
                let selected = hierarchy_ui(self.world, ui, &mut self.state.selected_entities);
                if selected {
                    self.state.selection = InspectorSelection::Entities;
                }
            }
            DebugSuiteTab::Resources => select_resource(ui, &type_registry, &mut self.state.selection),
            DebugSuiteTab::Assets => select_asset(ui, &type_registry, self.world, &mut self.state.selection),
            DebugSuiteTab::Inspector => match self.state.selection {
                InspectorSelection::Entities => match self.state.selected_entities.as_slice() {
                    &[entity] => ui_for_entity_with_children(self.world, entity, ui),
                    entities => ui_for_entities_shared_components(self.world, entities, ui),
                },
                InspectorSelection::Resource(type_id, ref name) => {
                    ui.label(name);
                    ui_for_resource(self.world, type_id, ui, name, &type_registry)
                }
                InspectorSelection::Asset(type_id, ref name, handle) => {
                    let handle = handle.unwrap();
                    ui.label(name);
                    ui_for_asset(self.world, type_id, handle, ui, &type_registry);
                }
            },
            DebugSuiteTab::ChunkManager => {
                ui.label("Chunk Manager (todo)");
            }
            DebugSuiteTab::IntentBuffer => {
                ui.label("Intent Buffer (todo)");
            }
            DebugSuiteTab::IntentCommit => {
                ui.label("Intent Commit (todo)");
            }
            DebugSuiteTab::ChunkInspector => {
                ui.label("Chunk Inspector (todo)");
            }
        };
    }

    fn clear_background(&self, tab: &Self::Tab) -> bool {
        !matches!(tab, DebugSuiteTab::GameView)
    }
}
