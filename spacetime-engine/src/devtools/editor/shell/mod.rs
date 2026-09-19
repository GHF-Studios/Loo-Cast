//! Runtime editor shell, docking, toolbar and game viewport embedding.

use super::*;

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

#[expect(
    deprecated,
    reason = "egui 0.34 deprecated root Context panel entry points before bevy_egui exposes a root Ui"
)]
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

#[expect(
    deprecated,
    reason = "egui 0.34 deprecated root Context panel entry points before bevy_egui exposes a root Ui"
)]
fn draw_toolbar(ctx: &egui::Context, world: &mut World) {
    egui::Panel::top("spacetime_editor_toolbar").show(ctx, |ui| {
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

pub(super) fn draw_transform_space_control(ui: &mut egui::Ui, world: &mut World) {
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
