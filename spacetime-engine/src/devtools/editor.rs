//! Runtime editor/composer shell around the already-running game.
//!
//! Hierarchy is the raw ECS entity chooser. Structure refines that canonical
//! entity focus into semantic parts. Semantic Inspector and contextual Gizmos
//! consume the refinement; ECS Inspector remains a deliberately raw whole-entity
//! reflection surface.

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
    ecs::{UsfManifestationAuthority, UsfManifestationOf, UsfManifestations},
    view::{PrimaryGameView, PrimaryViewPresentation},
};

use super::{
    DeveloperArtifact, DeveloperFocus, DeveloperTools, EditorTransformGizmoSettings,
    EditorTransformSpace, EditorTransformWritable, FocusTarget, InspectionFrame,
    SemanticInspectionSelection, ui::inspector,
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
    mut semantic_selection: ResMut<SemanticInspectionSelection>,
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
            }
            *presentation = PrimaryViewPresentation::Embedded;
        }
        PrimaryViewPresentation::Embedded => {
            focus.clear_selection();
            semantic_selection.clear();
            *presentation = PrimaryViewPresentation::Immersive;
        }
    }
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
    let space = world
        .resource::<EditorTransformGizmoSettings>()
        .transform_space();
    for (candidate, label) in [
        (EditorTransformSpace::World, "World"),
        (EditorTransformSpace::Local, "Local"),
    ] {
        if ui.selectable_label(space == candidate, label).clicked() {
            world
                .resource_mut::<EditorTransformGizmoSettings>()
                .set_transform_space(candidate);
        }
    }
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
    world.resource_mut::<SemanticInspectionSelection>().clear();
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
        ui.weak("Select an entity to inspect its semantic structure.");
        return;
    };

    ui.heading(focus_name(Some(target), world));
    ui.weak(format!("ECS {:?}", target.spatial_entity));
    ui.add_space(4.0);

    let selected_section = world
        .resource::<SemanticInspectionSelection>()
        .section_for(target.spatial_entity);
    if ui
        .selectable_label(selected_section.is_none(), "Entity")
        .clicked()
    {
        world
            .resource_mut::<SemanticInspectionSelection>()
            .clear_for(target.spatial_entity);
    }

    let sections = world
        .resource::<InspectionFrame>()
        .sorted_sections()
        .into_iter()
        .map(|section| (section.id, section.title.clone()))
        .collect::<Vec<_>>();
    for (id, title) in sections {
        if ui
            .selectable_label(selected_section == Some(id), format!("  {title}"))
            .clicked()
        {
            world
                .resource_mut::<SemanticInspectionSelection>()
                .select(target.spatial_entity, id);
        }
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
    world.resource_mut::<SemanticInspectionSelection>().clear();
}

fn draw_semantic_inspector(ui: &mut egui::Ui, world: &World) {
    let tools = world.resource::<DeveloperTools>();
    let focus = world.resource::<DeveloperFocus>();
    let frame = world.resource::<InspectionFrame>();
    let target = focus.current();
    let focus_name = focus_name(target, world);
    let selected_section = target.and_then(|target| {
        world
            .resource::<SemanticInspectionSelection>()
            .section_for(target.spatial_entity)
    });

    inspector::draw_editor_inspector(
        ui,
        tools,
        target,
        &focus_name,
        focus.pinned().is_some(),
        frame,
        selected_section,
    );
}

fn draw_gizmos(ui: &mut egui::Ui, world: &mut World) {
    let Some(target) = world.resource::<DeveloperFocus>().current() else {
        ui.weak("Select an entity to discover contextual gizmos.");
        return;
    };
    let entity = target.spatial_entity;
    let selected_section = world
        .resource::<SemanticInspectionSelection>()
        .section_for(entity);
    if selected_section.is_some() && selected_section != Some(super::gizmo::TRANSFORM_SECTION) {
        ui.weak("No contextual gizmo is implemented for the selected Structure item yet.");
        return;
    }

    let Some(transform) = world.get::<Transform>(entity).cloned() else {
        ui.weak("No contextual gizmo is registered for the current Structure selection yet.");
        return;
    };

    ui.heading("Transform");
    ui.horizontal(|ui| {
        ui.label("Space");
        draw_transform_space_control(ui, world);
    });

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
        ui.weak("Generic direct editing is disabled for parented Transforms: local/world authority needs a domain-aware adapter.");
    } else {
        ui.weak("Runtime-only authority grant. Generated/simulated/asset-authored state should use a domain adapter instead.");
    }

    let editable = writable && !parented;
    draw_transform_values(ui, world, entity, transform, editable);

    ui.add_space(6.0);
    ui.separator();
    ui.label("Viewport: translation arrows + rotation rings + scale handles are active simultaneously.");
    if !editable {
        ui.weak("The viewport gizmo remains visible for observation, but dragging is read-only.");
    }
}

fn draw_transform_values(
    ui: &mut egui::Ui,
    world: &mut World,
    entity: Entity,
    transform: Transform,
    editable: bool,
) {
    let mut translation = transform.translation;
    let (rx, ry, rz) = transform.rotation.to_euler(EulerRot::XYZ);
    let mut rotation_degrees = Vec3::new(rx.to_degrees(), ry.to_degrees(), rz.to_degrees());
    let mut scale = transform.scale;

    let translation_changed = vec3_editor(ui, "Position", &mut translation, editable, 0.02);
    let rotation_changed = vec3_editor(ui, "Rotation °", &mut rotation_degrees, editable, 0.2);
    let scale_changed = vec3_editor(ui, "Scale", &mut scale, editable, 0.01);

    if !editable || !(translation_changed || rotation_changed || scale_changed) {
        return;
    }
    let Some(mut target) = world.get_mut::<Transform>(entity) else {
        return;
    };
    if translation_changed {
        target.translation = translation;
    }
    if rotation_changed {
        target.rotation = Quat::from_euler(
            EulerRot::XYZ,
            rotation_degrees.x.to_radians(),
            rotation_degrees.y.to_radians(),
            rotation_degrees.z.to_radians(),
        );
    }
    if scale_changed {
        // `Direct` really means direct here: do not silently impose domain
        // validation such as forbidding negative/mirrored scale. Validated
        // authoring belongs to an explicit validated/domain adapter.
        target.scale = scale;
    }
}

fn vec3_editor(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut Vec3,
    editable: bool,
    speed: f64,
) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        ui.label(label);
        for component in [&mut value.x, &mut value.y, &mut value.z] {
            if editable {
                changed |= ui
                    .add(egui::DragValue::new(component).speed(speed))
                    .changed();
            } else {
                ui.monospace(format!("{component:.3}"));
            }
        }
    });
    changed
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
