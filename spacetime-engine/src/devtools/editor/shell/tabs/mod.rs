//! Dock-tab routing and ECS-inspector tab presentation.

use super::*;

pub(super) struct EditorTabViewer<'a> {
    pub(super) world: &'a mut World,
    pub(super) game_rect: &'a mut Option<egui::Rect>,
    pub(super) selected_entities: &'a mut SelectedEntities,
}

impl TabViewer for EditorTabViewer<'_> {
    type Tab = EditorTab;

    fn id(&mut self, tab: &mut Self::Tab) -> egui::Id {
        egui::Id::new(*tab)
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            EditorTab::Game => "Game",
            EditorTab::Hierarchy => "Hierarchy",
            EditorTab::Structure => "Structure",
            EditorTab::SemanticInspector => "Semantic Inspector",
            EditorTab::Gizmos => "Gizmos",
            EditorTab::EcsInspector => "ECS Inspector",
            EditorTab::Resources => "Resources",
            EditorTab::Assets => "Assets",
            EditorTab::Visualizations => "Visualizations",
            EditorTab::ChunkManager => "Chunk Manager",
            EditorTab::IntentBuffer => "Intent Buffer",
            EditorTab::IntentCommit => "Intent Commit",
            EditorTab::ChunkInspector => "Chunk Inspector",
        }
        .into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            EditorTab::Game => {
                let rect = ui.available_rect_before_wrap();
                *self.game_rect = Some(rect);
                ui.allocate_rect(rect, egui::Sense::hover());
            }
            EditorTab::Hierarchy => {
                hierarchy_ui_filtered::<Without<DeveloperArtifact>>(
                    self.world,
                    ui,
                    self.selected_entities,
                );
            }
            EditorTab::Structure => draw_structure(ui, self.world),
            EditorTab::SemanticInspector => draw_semantic_inspector(ui, self.world),
            EditorTab::Gizmos => draw_gizmos(ui, self.world),
            EditorTab::EcsInspector => self.draw_ecs_inspector(ui),
            EditorTab::Resources => bevy_inspector::ui_for_resources(self.world, ui),
            EditorTab::Assets => bevy_inspector::ui_for_all_assets(self.world, ui),
            EditorTab::Visualizations => draw_visualizations(ui, self.world),
            EditorTab::ChunkManager => draw_legacy_slot(ui, "Chunk Manager"),
            EditorTab::IntentBuffer => draw_legacy_slot(ui, "Intent Buffer"),
            EditorTab::IntentCommit => draw_legacy_slot(ui, "Intent Commit"),
            EditorTab::ChunkInspector => draw_legacy_slot(ui, "Chunk Inspector"),
        }
    }

    fn clear_background(&self, tab: &Self::Tab) -> bool {
        !matches!(tab, EditorTab::Game)
    }

    fn is_closeable(&self, _tab: &Self::Tab) -> bool {
        false
    }

    fn allowed_in_windows(&self, _tab: &mut Self::Tab) -> bool {
        false
    }

    fn scroll_bars(&self, tab: &Self::Tab) -> [bool; 2] {
        match tab {
            EditorTab::Game => [false, false],
            _ => [true, true],
        }
    }
}

impl EditorTabViewer<'_> {
    fn draw_ecs_inspector(&mut self, ui: &mut egui::Ui) {
        match self.selected_entities.as_slice() {
            [] => {
                ui.weak("Select an entity in Hierarchy or the Game view.");
            }
            [entity] => {
                bevy_inspector::ui_for_entity_with_children(self.world, *entity, ui);
            }
            entities => {
                bevy_inspector::ui_for_entities_shared_components(self.world, entities, ui);
            }
        };
    }
}
