//! Runtime editor/composer shell around the already-running game.
//!
//! Hierarchy is the raw ECS entity chooser. Structure refines that canonical
//! entity focus into semantic parts. Semantic Inspector and contextual Gizmos
//! consume the refinement; ECS Inspector remains a deliberately raw whole-entity
//! reflection surface.

use bevy::{
    camera::visibility::RenderLayers,
    camera::{CameraOutputMode, ClearColorConfig, Viewport},
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
    ecs::{UsfManifestationAuthority, UsfManifestationOf, UsfManifestations},
    view::{PrimaryGameView, PrimaryViewPresentation},
};

use super::{
    DeveloperArtifact, DeveloperFocus, DeveloperTools, EditorTransformGizmoSettings,
    EditorTransformWritable, FocusTarget, InspectEditRequest, InspectTypeRegistry, InspectValue,
    InspectionFrame, StructureFrame, StructureSelection,
    inspect_ui::{
        self, InspectWidgetContext, InspectorWidget, InspectorWidgetRegistry, TransformWidget,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum EditorTab {
    Game,
    Hierarchy,
    Structure,
    SemanticInspector,
    Gizmos,
    EcsInspector,
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
        // Hierarchy |       Game       | Semantic Inspector / Gizmos / ECS Inspector
        // Structure |------------------|
        //           | lower tool tabs  |
        let mut dock_state = DockState::new(vec![EditorTab::Game]);
        let tree = dock_state.main_surface_mut();
        let [game, _right] = tree.split_right(
            NodeIndex::root(),
            0.75,
            vec![
                EditorTab::SemanticInspector,
                EditorTab::Gizmos,
                EditorTab::EcsInspector,
            ],
        );
        let [game, hierarchy] = tree.split_left(game, 0.2, vec![EditorTab::Hierarchy]);
        let [_hierarchy, _structure] =
            tree.split_below(hierarchy, 0.58, vec![EditorTab::Structure]);
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
    mut focus: ResMut<DeveloperFocus>,
    mut structure: ResMut<StructureSelection>,
) {
    if !keyboard.just_pressed(KeyCode::F2) {
        return;
    }

    match *presentation {
        PrimaryViewPresentation::Immersive => {
            if focus.selected().is_none()
                && let Some(target) = focus.current()
            {
                focus.select(target);
                structure.select_entity(target);
            }
            *presentation = PrimaryViewPresentation::Embedded;
        }
        PrimaryViewPresentation::Embedded => {
            focus.clear_selection();
            structure.clear();
            *presentation = PrimaryViewPresentation::Immersive;
        }
    }
}

fn draw_editor_shell(world: &mut World) {
    if !world.resource::<PrimaryViewPresentation>().is_embedded() {
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
        let focused_entity = world
            .resource::<DeveloperFocus>()
            .current()
            .map(|target| target.spatial_entity);
        sync_hierarchy_selection(&mut shell.selected_entities, focused_entity);
        let selection_before = shell.selected_entities.as_slice().to_vec();

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

        let selection_after = shell.selected_entities.as_slice().to_vec();
        if selection_after != selection_before {
            apply_hierarchy_selection(world, &selection_after);
        }
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
            draw_transform_space_control(ui, world);

            ui.separator();
            ui.weak("Unified Transform gizmo: translate + rotate + scale. Esc cancels a drag or toggles gameplay capture.");
        });
    });
}

fn draw_transform_space_control(ui: &mut egui::Ui, world: &mut World) {
    ui.label("Transform");
    world.resource_scope(|world, mut settings: Mut<EditorTransformGizmoSettings>| {
        let registration = world
            .resource::<InspectTypeRegistry>()
            .get::<EditorTransformGizmoSettings>()
            .expect("Transform gizmo settings must be registered for inspection");
        let widgets = world.resource::<InspectorWidgetRegistry>();
        let _ = inspect_ui::edit_registered_inspectable(ui, registration, &mut *settings, widgets);
    });
}

fn sync_hierarchy_selection(selected: &mut SelectedEntities, entity: Option<Entity>) {
    let desired = entity.map(|entity| vec![entity]).unwrap_or_default();
    if selected.as_slice() == desired.as_slice() {
        return;
    }

    selected.clear();
    if let Some(entity) = entity {
        selected.select_replace(entity);
    }
}

fn apply_hierarchy_selection(world: &mut World, selected: &[Entity]) {
    world.resource_mut::<StructureSelection>().clear();
    let mut focus = world.resource_mut::<DeveloperFocus>();
    focus.clear_pin();

    let [entity] = selected else {
        focus.clear_selection();
        return;
    };
    let entity = *entity;
    drop(focus);

    let target = focus_target_for_entity(world, entity);
    world.resource_mut::<DeveloperFocus>().select(target);
    world
        .resource_mut::<StructureSelection>()
        .select_entity(target);
}

fn focus_target_for_entity(world: &World, entity: Entity) -> FocusTarget {
    let semantic_entity = world
        .get::<UsfManifestationOf>(entity)
        .map(|manifestation| manifestation.0)
        .unwrap_or(entity);
    FocusTarget::entity(entity, semantic_entity)
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

    let size = UVec2::new(max.x.saturating_sub(min.x), max.y.saturating_sub(min.y));
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
            EditorTab::Structure => "Structure",
            EditorTab::SemanticInspector => "Semantic Inspector",
            EditorTab::Gizmos => "Gizmos",
            EditorTab::EcsInspector => "ECS Inspector",
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
            EditorTab::Structure => draw_structure(ui, self.world),
            EditorTab::SemanticInspector => draw_semantic_inspector(ui, self.world),
            EditorTab::Gizmos => draw_gizmos(ui, self.world),
            EditorTab::EcsInspector => self.draw_ecs_inspector(ui),
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

fn draw_structure(ui: &mut egui::Ui, world: &mut World) {
    let Some(target) = world.resource::<DeveloperFocus>().current() else {
        ui.weak("Select or focus an entity to inspect its semantic structure.");
        return;
    };

    ui.heading(focus_name(Some(target), world));
    ui.weak(format!("ECS {:?}", target.spatial_entity));
    if target.semantic_entity != target.spatial_entity {
        ui.weak(format!("Semantic {:?}", target.semantic_entity));
    }
    ui.add_space(4.0);

    let selected_item = world.resource::<StructureSelection>().item_for(target);
    if ui
        .selectable_label(selected_item.is_none(), "Entity")
        .clicked()
    {
        world.resource_mut::<DeveloperFocus>().select(target);
        world
            .resource_mut::<StructureSelection>()
            .select_entity(target);
    }

    let items = world
        .resource::<StructureFrame>()
        .sorted_items()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    for item in items {
        ui.horizontal(|ui| {
            ui.add_space(12.0);
            let clicked = ui
                .selectable_label(selected_item == Some(item.id), &item.label)
                .clicked();
            if let Some(detail) = &item.detail {
                ui.weak(detail);
            }
            if clicked {
                world.resource_mut::<DeveloperFocus>().select(target);
                world
                    .resource_mut::<StructureSelection>()
                    .select_item(target, item.id);
            }
        });
    }

    let manifestations = world
        .get::<UsfManifestations>(target.semantic_entity)
        .map(|manifestations| manifestations.iter().collect::<Vec<_>>())
        .unwrap_or_default();
    if target.spatial_entity != target.semantic_entity || !manifestations.is_empty() {
        ui.add_space(6.0);
        ui.separator();
        ui.strong("USF relationships");

        if target.spatial_entity != target.semantic_entity
            && ui
                .button(format!("Semantic entity  {:?}", target.semantic_entity))
                .clicked()
        {
            select_related_entity(world, target.semantic_entity);
            return;
        }

        for manifestation in manifestations {
            let name = entity_name(world, manifestation);
            let authority = world
                .get::<UsfManifestationAuthority>(manifestation)
                .is_some();
            let suffix = if authority { "  [authority]" } else { "" };
            if ui
                .button(format!("Manifestation  {name}  {manifestation:?}{suffix}"))
                .clicked()
            {
                select_related_entity(world, manifestation);
                return;
            }
        }
    }
}

fn select_related_entity(world: &mut World, entity: Entity) {
    let target = focus_target_for_entity(world, entity);
    world.resource_mut::<DeveloperFocus>().clear_pin();
    world.resource_mut::<DeveloperFocus>().select(target);
    world
        .resource_mut::<StructureSelection>()
        .select_entity(target);
}

fn draw_semantic_inspector(ui: &mut egui::Ui, world: &mut World) {
    let focus = *world.resource::<DeveloperFocus>();
    let Some(target) = focus.current() else {
        ui.weak("No semantic focus.");
        ui.label("Hover the Game view or select an entity in Hierarchy.");
        return;
    };

    let scope = world.resource::<StructureSelection>().item_for(target);
    ui.heading(focus_name(Some(target), world));
    if focus.selected().is_some() {
        ui.weak("selected");
    } else if let Some(hit) = target.hit {
        ui.weak(format!("hover focus · {:.3} m", hit.distance_meters));
    } else {
        ui.weak("focus");
    }
    ui.separator();

    let output = {
        let frame = world.resource::<InspectionFrame>();
        inspect_ui::draw_sections(ui, frame, target, scope, true)
    };
    dispatch_inspection_ui_output(world, output);
}

fn draw_gizmos(ui: &mut egui::Ui, world: &mut World) {
    let Some(target) = world.resource::<DeveloperFocus>().current() else {
        ui.weak("Select or focus an entity to discover contextual gizmos.");
        return;
    };
    let scope = world.resource::<StructureSelection>().item_for(target);

    if scope == Some(super::gizmo::TRANSFORM_STRUCTURE) || scope.is_none() {
        if world.get::<Transform>(target.spatial_entity).is_some() {
            draw_transform_gizmo_panel(ui, world, target);
            return;
        }
    }

    let Some(scope) = scope else {
        ui.weak("Select a Structure item to open its contextual gizmo.");
        return;
    };

    ui.heading("Context gizmo");
    ui.weak("Rich domain context for the selected Structure item. Observation is independent from edit authority.");
    ui.separator();
    let output = {
        let frame = world.resource::<InspectionFrame>();
        inspect_ui::draw_contextual_gizmo(ui, frame, target, scope)
    };
    dispatch_inspection_ui_output(world, output);
}

fn draw_transform_gizmo_panel(ui: &mut egui::Ui, world: &mut World, target: FocusTarget) {
    let entity = target.spatial_entity;
    let Some(transform) = world.get::<Transform>(entity).copied() else {
        ui.weak("No Transform gizmo applies to this focus.");
        return;
    };

    ui.heading("Transform");
    ui.weak(
        "Combined translate + rotate + scale viewport gizmo. Translate/Rotate honor World/Local; Scale remains local to match Transform.scale.",
    );
    ui.horizontal(|ui| {
        ui.label("Space");
        draw_transform_space_control(ui, world);
    });
    ui.separator();

    let parented = world.get::<ChildOf>(entity).is_some();
    let mut writable = world.get::<EditorTransformWritable>(entity).is_some();
    let response = ui.add_enabled(
        !parented,
        egui::Checkbox::new(&mut writable, "Allow direct runtime Transform editing"),
    );
    if response.changed() {
        if writable {
            world.entity_mut(entity).insert(EditorTransformWritable);
        } else {
            world.entity_mut(entity).remove::<EditorTransformWritable>();
        }
    }

    if parented {
        ui.weak(
            "Read-only generic gizmo: parented transforms need a parent-aware/domain authoring adapter before direct mutation is safe.",
        );
    } else if !writable {
        ui.weak("Read-only. Inspectability does not grant mutation authority.");
    } else {
        ui.weak("Runtime-direct authority only; generated or authored sources may still overwrite this value.");
    }

    ui.add_space(6.0);
    let editable = writable && !parented;
    if editable {
        let mut proposed = transform;
        if TransformWidget.edit(ui, &mut proposed, &InspectWidgetContext::new("Transform")) {
            let mut edits = Vec::new();
            if proposed.translation != transform.translation {
                edits.push((
                    super::gizmo::TRANSLATION_FIELD,
                    InspectValue::Vec3(proposed.translation),
                ));
            }
            if proposed.rotation != transform.rotation {
                let (rx, ry, rz) = proposed.rotation.to_euler(EulerRot::XYZ);
                edits.push((
                    super::gizmo::ROTATION_FIELD,
                    InspectValue::Vec3(Vec3::new(
                        rx.to_degrees(),
                        ry.to_degrees(),
                        rz.to_degrees(),
                    )),
                ));
            }
            if proposed.scale != transform.scale {
                edits.push((
                    super::gizmo::SCALE_FIELD,
                    InspectValue::Vec3(proposed.scale),
                ));
            }
            for (field, value) in edits {
                world.write_message(InspectEditRequest {
                    target,
                    section: super::gizmo::TRANSFORM_SECTION,
                    field,
                    value,
                });
            }
        }
    } else {
        TransformWidget.show(ui, &transform, &InspectWidgetContext::new("Transform"));
    }

    ui.add_space(6.0);
    ui.separator();
    ui.label(
        "Viewport: translation arrows + rotation rings + scale handles are active simultaneously.",
    );
    if !editable {
        ui.weak("The viewport gizmo remains visible for observation, but dragging is read-only.");
    }
}

fn dispatch_inspection_ui_output(world: &mut World, output: inspect_ui::InspectionUiOutput) {
    for edit in output.edits {
        world.write_message(edit);
    }
    for action in output.actions {
        world.write_message(action);
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

fn entity_name(world: &World, entity: Entity) -> String {
    world
        .get::<Name>(entity)
        .map(Name::as_str)
        .map(str::to_owned)
        .unwrap_or_else(|| "Unnamed".to_owned())
}

fn draw_visualizations(ui: &mut egui::Ui, world: &mut World) {
    ui.heading("Developer visualizations");
    ui.add_space(4.0);

    let mut tools = world.resource_mut::<DeveloperTools>();
    let mut enabled = tools.enabled();
    if ui
        .checkbox(&mut enabled, "Master developer output")
        .changed()
    {
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
            diagnostic_row(ui, "1% low FPS", diagnostics.frame.one_percent_low_fps, "");
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
            diagnostic_row(ui, "System CPU", diagnostics.system.system_cpu_percent, "%");
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
