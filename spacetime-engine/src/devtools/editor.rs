//! Runtime editor/composer shell around the already-running game.
//!
//! The shell does not own simulation, inspection or developer semantics. It hosts
//! those existing surfaces while the primary game camera remains the same logical
//! view, merely constrained to the live `Game` dock rectangle.

use bevy::{
    camera::{CameraOutputMode, ClearColorConfig, Viewport},
    camera::visibility::RenderLayers,
    prelude::*,
    render::render_resource::BlendState,
    window::PrimaryWindow,
};
use bevy_egui::{
    EguiContexts, EguiGlobalSettings, EguiPlugin, EguiPrimaryContextPass, PrimaryEguiContext, egui,
};
use egui_dock::{DockArea, DockState, NodeIndex, Style, TabViewer};

use crate::view::{PrimaryGameView, PrimaryViewPresentation};

use super::{
    DeveloperFocus, DeveloperTools, FocusTarget, InspectionFrame,
    ui::inspector,
};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum EditorTab {
    Game,
    Inspector,
    Visualizations,
}

#[derive(Resource)]
struct EditorShell {
    dock_state: DockState<EditorTab>,
}

impl Default for EditorShell {
    fn default() -> Self {
        let mut dock_state = DockState::new(vec![EditorTab::Game]);
        let tree = dock_state.main_surface_mut();
        let [_game, right] =
            tree.split_right(NodeIndex::root(), 0.74, vec![EditorTab::Inspector]);
        tree.split_below(right, 0.64, vec![EditorTab::Visualizations]);

        Self { dock_state }
    }
}

#[derive(Component)]
struct EditorShellCamera;

pub(super) fn configure(app: &mut App) {
    app.add_plugins(EguiPlugin::default());
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

fn draw_editor_shell(
    mut contexts: EguiContexts,
    presentation: Res<PrimaryViewPresentation>,
    mut shell: ResMut<EditorShell>,
    mut tools: ResMut<DeveloperTools>,
    focus: Res<DeveloperFocus>,
    frame: Res<InspectionFrame>,
    names: Query<&Name>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut game_camera: Single<&mut Camera, With<PrimaryGameView>>,
) -> Result {
    if !presentation.is_embedded() {
        game_camera.viewport = None;
        game_camera.is_active = true;
        return Ok(());
    }

    let ctx = contexts.ctx_mut()?;
    let mut viewport_ui = egui::Ui::new(
        ctx.clone(),
        "spacetime_editor_root".into(),
        egui::UiBuilder::new()
            .layer_id(egui::LayerId::background())
            .max_rect(ctx.viewport_rect()),
    );
    draw_toolbar(&mut viewport_ui, &mut tools);

    let target = focus.current();
    let focus_name = focus_name(target, &names);
    let pinned = focus.pinned().is_some();
    let mut game_rect = None;

    egui::CentralPanel::default()
        .frame(egui::Frame::new().fill(egui::Color32::TRANSPARENT))
        .show(&mut viewport_ui, |ui| {
            let mut viewer = EditorTabViewer {
                game_rect: &mut game_rect,
                tools: &mut tools,
                target,
                focus_name: &focus_name,
                pinned,
                frame: &frame,
            };

            DockArea::new(&mut shell.dock_state)
                .style(Style::from_egui(ui.style().as_ref()))
                .show_inside(ui, &mut viewer);
        });

    let viewport = game_rect.and_then(|rect| egui_rect_to_viewport(rect, &window));
    game_camera.is_active = viewport.is_some();
    game_camera.viewport = viewport;

    Ok(())
}

fn draw_toolbar(ui: &mut egui::Ui, tools: &mut DeveloperTools) {
    egui::Panel::top("spacetime_editor_toolbar").show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.strong("SPACETIME");
            ui.separator();
            ui.label("F2  return to game");

            ui.separator();
            let mut enabled = tools.enabled();
            if ui.checkbox(&mut enabled, "Developer output").changed() {
                tools.set_enabled(enabled);
            }

            ui.separator();
            ui.weak("Click the Game view to recapture gameplay input.");
        });
    });
}

fn focus_name(target: Option<FocusTarget>, names: &Query<&Name>) -> String {
    let Some(target) = target else {
        return "No focus".to_owned();
    };

    names
        .get(target.semantic_entity)
        .or_else(|_| names.get(target.spatial_entity))
        .map(Name::as_str)
        .map(str::to_owned)
        .unwrap_or_else(|_| format!("{:?}", target.semantic_entity))
}

fn egui_rect_to_viewport(rect: egui::Rect, window: &Window) -> Option<Viewport> {
    let scale = window.resolution.scale_factor();
    let target_size = window.resolution.physical_size();

    let logical_min = Vec2::new(rect.min.x, rect.min.y).max(Vec2::ZERO);
    let logical_max = Vec2::new(rect.max.x, rect.max.y).max(logical_min);

    let mut min = (logical_min * scale).floor().as_uvec2();
    let mut max = (logical_max * scale).ceil().as_uvec2();

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
    game_rect: &'a mut Option<egui::Rect>,
    tools: &'a mut DeveloperTools,
    target: Option<FocusTarget>,
    focus_name: &'a str,
    pinned: bool,
    frame: &'a InspectionFrame,
}

impl TabViewer for EditorTabViewer<'_> {
    type Tab = EditorTab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            EditorTab::Game => "Game",
            EditorTab::Inspector => "Inspector",
            EditorTab::Visualizations => "Visualizations",
        }
        .into()
    }

    fn id(&mut self, tab: &mut Self::Tab) -> egui::Id {
        egui::Id::new(*tab)
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            EditorTab::Game => {
                let rect = ui.available_rect_before_wrap();
                *self.game_rect = Some(rect);
                ui.allocate_rect(rect, egui::Sense::hover());
            }
            EditorTab::Inspector => {
                inspector::draw_editor_inspector(
                    ui,
                    &*self.tools,
                    self.target,
                    self.focus_name,
                    self.pinned,
                    self.frame,
                );
            }
            EditorTab::Visualizations => draw_visualizations(ui, self.tools),
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
            EditorTab::Inspector | EditorTab::Visualizations => [true, true],
        }
    }
}

fn draw_visualizations(ui: &mut egui::Ui, tools: &mut DeveloperTools) {
    ui.heading("Developer visualizations");
    ui.add_space(4.0);

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
