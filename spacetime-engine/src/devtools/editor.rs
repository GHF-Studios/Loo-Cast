//! Runtime editor/composer shell around the already-running game.
//!
//! This is intentionally a reconciliation layer:
//!
//! - the mature legacy dock layout returns (Game / Hierarchy / Inspector /
//!   Resources / Assets / lower tooling tabs);
//! - current semantic inspection, focus and visualization resources remain the
//!   authority for developer meaning;
//! - the running game is still the same direct-to-window Bevy camera, merely
//!   constrained to the live `Game` dock rectangle while the shell is visible.
//!
//! Nothing here makes simulation depend on egui or reflection tooling.

use bevy::{
    camera::{CameraOutputMode, ClearColorConfig, Viewport},
    camera::visibility::RenderLayers,
    prelude::*,
    render::render_resource::BlendState,
    window::PrimaryWindow,
};
use bevy_egui::{
    EguiContext, EguiGlobalSettings, EguiPlugin, EguiPrimaryContextPass, PrimaryEguiContext, egui,
};
use bevy_inspector_egui::{
    DefaultInspectorConfigPlugin,
    bevy_inspector::{
        self,
        hierarchy::{SelectedEntities, hierarchy_ui_filtered},
    },
};
use egui_dock::{DockArea, DockState, NodeIndex, Style, TabViewer};

use crate::{
    diagnostics::RuntimeDiagnostics,
    view::{PrimaryGameView, PrimaryViewPresentation},
};

use super::{
    DeveloperArtifact, DeveloperFocus, DeveloperTools, FocusTarget, InspectionFrame, ui::inspector,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum EditorTab {
    Game,
    Hierarchy,
    Inspector,
    Resources,
    Assets,
    Visualizations,
    Diagnostics,
    ChunkManager,
    IntentBuffer,
    IntentCommit,
    ChunkInspector,
}

#[derive(Resource)]
struct EditorShell {
    dock_state: DockState<EditorTab>,
    selected_entities: SelectedEntities,
}

impl Default for EditorShell {
    fn default() -> Self {
        // Preserve the mature legacy spatial grammar:
        //
        // Hierarchy |       Game       | Inspector
        //           |------------------|
        //           | lower tool tabs  |
        let mut dock_state = DockState::new(vec![EditorTab::Game]);
        let tree = dock_state.main_surface_mut();
        let [game, _inspector] =
            tree.split_right(NodeIndex::root(), 0.75, vec![EditorTab::Inspector]);
        let [game, _hierarchy] = tree.split_left(game, 0.2, vec![EditorTab::Hierarchy]);
        let [_game, _bottom] = tree.split_below(
            game,
            0.666,
            vec![
                EditorTab::Resources,
                EditorTab::Assets,
                EditorTab::Visualizations,
                EditorTab::Diagnostics,
                EditorTab::ChunkManager,
                EditorTab::IntentBuffer,
                EditorTab::IntentCommit,
                EditorTab::ChunkInspector,
            ],
        );

        Self {
            dock_state,
            selected_entities: SelectedEntities::default(),
        }
    }
}

#[derive(Component)]
struct EditorShellCamera;

pub(super) fn configure(app: &mut App) {
    app.add_plugins((EguiPlugin::default(), DefaultInspectorConfigPlugin));
    app.world_mut()
        .resource_mut::<EguiGlobalSettings>()
        .auto_create_primary_context = false;

    app.init_resource::<EditorShell>()
        .init_resource::<PrimaryViewPresentation>()
        .add_systems(Startup, spawn_editor_camera)
        .add_systems(PreUpdate, toggle_editor_shell)
        // Exclusive on purpose: the reflected hierarchy/Inspector is a tooling
        // surface over `World`, while semantic inspection remains ordinary ECS
        // collection outside this renderer.
        .add_systems(EguiPrimaryContextPass, draw_editor_shell);
}

fn spawn_editor_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Editor Shell Camera"),
        DeveloperArtifact,
        EditorShellCamera,
        Camera2d,
        Camera {
            order: 1_000,
            output_mode: CameraOutputMode::Write {
                blend_state: Some(BlendState::ALPHA_BLENDING),
                clear_color: ClearColorConfig::None,
            },
            clear_color: ClearColorConfig::Custom(Color::NONE),
            ..default()
        },
        RenderLayers::none(),
        PrimaryEguiContext,
    ));
}

fn toggle_editor_shell(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut presentation: ResMut<PrimaryViewPresentation>,
) {
    if !keyboard.just_pressed(KeyCode::F2) {
        return;
    }

    *presentation = match *presentation {
        PrimaryViewPresentation::Immersive => PrimaryViewPresentation::Embedded,
        PrimaryViewPresentation::Embedded => PrimaryViewPresentation::Immersive,
    };
}

fn draw_editor_shell(world: &mut World) {
    if !world
        .resource::<PrimaryViewPresentation>()
        .is_embedded()
    {
        set_primary_game_viewport(world, None, true);
        return;
    }

    let mut egui_context = {
        let mut contexts = world.query_filtered::<&mut EguiContext, With<PrimaryEguiContext>>();
        let Ok(context) = contexts.single_mut(world) else {
            return;
        };
        context.clone()
    };
    let ctx = egui_context.get_mut();

    let mut game_rect = None;

    world.resource_scope(|world, mut shell: Mut<EditorShell>| {
        draw_toolbar(ctx, world);

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(egui::Color32::TRANSPARENT))
            .show(ctx, |ui| {
                let EditorShell {
                    dock_state,
                    selected_entities,
                } = &mut *shell;
                let mut viewer = EditorTabViewer {
                    world,
                    game_rect: &mut game_rect,
                    selected_entities,
                };

                DockArea::new(dock_state)
                    .style(Style::from_egui(ui.style().as_ref()))
                    .show_inside(ui, &mut viewer);
            });
    });

    let window_metrics = {
        let mut windows = world.query_filtered::<&Window, With<PrimaryWindow>>();
        windows.single(world).ok().map(|window| {
            (
                window.resolution.scale_factor(),
                window.resolution.physical_size(),
            )
        })
    };

    let viewport = window_metrics.and_then(|(scale, target_size)| {
        game_rect.and_then(|rect| egui_rect_to_viewport(rect, scale, target_size))
    });
    let active = viewport.is_some();
    set_primary_game_viewport(world, viewport, active);
}

fn set_primary_game_viewport(world: &mut World, viewport: Option<Viewport>, active: bool) {
    let mut cameras = world.query_filtered::<&mut Camera, With<PrimaryGameView>>();
    let Ok(mut camera) = cameras.single_mut(world) else {
        return;
    };

    camera.viewport = viewport;
    camera.is_active = active;
}

fn draw_toolbar(ctx: &egui::Context, world: &mut World) {
    egui::TopBottomPanel::top("spacetime_editor_toolbar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.strong("SPACETIME");
            ui.separator();
            ui.label("F2  return to game");

            ui.separator();
            let mut enabled = world.resource::<DeveloperTools>().enabled();
            if ui.checkbox(&mut enabled, "Developer output").changed() {
                world.resource_mut::<DeveloperTools>().set_enabled(enabled);
            }

            ui.separator();
            ui.weak("Game remains live; click its viewport to recapture gameplay input.");
        });
    });
}

fn egui_rect_to_viewport(
    rect: egui::Rect,
    scale_factor: f32,
    target_size: UVec2,
) -> Option<Viewport> {
    let logical_min = Vec2::new(rect.min.x, rect.min.y).max(Vec2::ZERO);
    let logical_max = Vec2::new(rect.max.x, rect.max.y).max(logical_min);

    let mut min = (logical_min * scale_factor).floor().as_uvec2();
    let mut max = (logical_max * scale_factor).ceil().as_uvec2();

    min.x = min.x.min(target_size.x);
    min.y = min.y.min(target_size.y);
    max.x = max.x.min(target_size.x);
    max.y = max.y.min(target_size.y);

    let size = UVec2::new(
        max.x.saturating_sub(min.x),
        max.y.saturating_sub(min.y),
    );
    if size.x < 2 || size.y < 2 {
        return None;
    }

    Some(Viewport {
        physical_position: min,
        physical_size: size,
        ..default()
    })
}

struct EditorTabViewer<'a> {
    world: &'a mut World,
    game_rect: &'a mut Option<egui::Rect>,
    selected_entities: &'a mut SelectedEntities,
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
            EditorTab::Inspector => "Inspector",
            EditorTab::Resources => "Resources",
            EditorTab::Assets => "Assets",
            EditorTab::Visualizations => "Visualizations",
            EditorTab::Diagnostics => "Diagnostics",
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
            EditorTab::Inspector => self.draw_inspector(ui),
            EditorTab::Resources => bevy_inspector::ui_for_resources(self.world, ui),
            EditorTab::Assets => bevy_inspector::ui_for_all_assets(self.world, ui),
            EditorTab::Visualizations => draw_visualizations(ui, self.world),
            EditorTab::Diagnostics => draw_runtime_diagnostics(ui, self.world),
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
    fn draw_inspector(&mut self, ui: &mut egui::Ui) {
        // Current semantic inspection remains first-class and presentation
        // independent. The reflected ECS inspector below it is deliberately a
        // separate developer mechanism rather than the source of semantic data.
        {
            let tools = self.world.resource::<DeveloperTools>();
            let focus = self.world.resource::<DeveloperFocus>();
            let frame = self.world.resource::<InspectionFrame>();
            let target = focus.current();
            let focus_name = focus_name(target, self.world);

            inspector::draw_editor_inspector(
                ui,
                tools,
                target,
                &focus_name,
                focus.pinned().is_some(),
                frame,
            );
        }

        ui.add_space(8.0);
        ui.separator();
        ui.add_space(4.0);
        ui.heading("ECS selection");

        let selected_entities = self.selected_entities.as_slice().to_vec();
        match selected_entities.as_slice() {
            [] => {
                ui.weak("Select an entity in Hierarchy to inspect reflected ECS state.");
            }
            [entity] => {
                bevy_inspector::ui_for_entity_with_children(self.world, *entity, ui);
            }
            entities => {
                bevy_inspector::ui_for_entities_shared_components(self.world, entities, ui);
            }
        }
    }
}

fn focus_name(target: Option<FocusTarget>, world: &World) -> String {
    let Some(target) = target else {
        return "No semantic focus".to_owned();
    };

    world
        .get::<Name>(target.semantic_entity)
        .or_else(|| world.get::<Name>(target.spatial_entity))
        .map(Name::as_str)
        .map(str::to_owned)
        .unwrap_or_else(|| format!("{:?}", target.semantic_entity))
}

fn draw_visualizations(ui: &mut egui::Ui, world: &mut World) {
    ui.heading("Developer visualizations");
    ui.add_space(4.0);

    let mut tools = world.resource_mut::<DeveloperTools>();
    let mut enabled = tools.enabled();
    if ui.checkbox(&mut enabled, "Master developer output").changed() {
        tools.set_enabled(enabled);
    }

    ui.separator();

    let mut specs = tools.visualizations().copied().collect::<Vec<_>>();
    specs.sort_by_key(|spec| (spec.order, spec.label));

    for spec in specs {
        let mut selected = tools.visualization_selected(spec.id);
        if ui.checkbox(&mut selected, spec.label).changed() {
            tools.set_visualization_enabled(spec.id, selected);
        }
    }
}

fn draw_runtime_diagnostics(ui: &mut egui::Ui, world: &World) {
    let diagnostics = world.resource::<RuntimeDiagnostics>();

    ui.heading("Runtime diagnostics");
    ui.add_space(4.0);

    egui::Grid::new("runtime_diagnostics_grid")
        .num_columns(2)
        .striped(true)
        .show(ui, |ui| {
            diagnostic_row(ui, "FPS", diagnostics.frame.fps, "");
            diagnostic_row(ui, "Frame time", diagnostics.frame.frame_time_ms, " ms");
            diagnostic_row(
                ui,
                "Average frame time",
                diagnostics.frame.average_frame_time_ms,
                " ms",
            );
            diagnostic_row(
                ui,
                "1% low FPS",
                diagnostics.frame.one_percent_low_fps,
                "",
            );
            ui.label("Entities");
            ui.monospace(diagnostics.world.entities.to_string());
            ui.end_row();
            ui.label("Component instances");
            ui.monospace(diagnostics.world.component_instances.to_string());
            ui.end_row();
            ui.label("Archetypes");
            ui.monospace(diagnostics.world.archetypes.to_string());
            ui.end_row();
            diagnostic_row(
                ui,
                "Process CPU",
                diagnostics.system.process_cpu_percent,
                "%",
            );
            diagnostic_row(
                ui,
                "System CPU",
                diagnostics.system.system_cpu_percent,
                "%",
            );
            diagnostic_row(
                ui,
                "Process memory",
                diagnostics.system.process_memory_gib,
                " GiB",
            );
            diagnostic_row(
                ui,
                "System memory",
                diagnostics.system.system_memory_percent,
                "%",
            );
        });
}

fn diagnostic_row(ui: &mut egui::Ui, label: &str, value: Option<f64>, suffix: &str) {
    ui.label(label);
    match value {
        Some(value) => ui.monospace(format!("{value:.2}{suffix}")),
        None => ui.monospace("—"),
    };
    ui.end_row();
}

fn draw_legacy_slot(ui: &mut egui::Ui, title: &str) {
    ui.heading(title);
    ui.weak("Legacy editor slot restored; current subsystem integration is intentionally still pending.");
}
