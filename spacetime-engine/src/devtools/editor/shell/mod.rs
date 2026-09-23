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
        .add_systems(
            PreUpdate,
            toggle_editor_shell.before(crate::input_focus::InputFocusSet::Resolve),
        )
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

mod tabs;
mod toolbar;
mod viewport;

use tabs::EditorTabViewer;
use toolbar::draw_toolbar;
pub(super) use toolbar::draw_transform_space_control;
use viewport::{egui_rect_to_viewport, set_primary_game_viewport};
