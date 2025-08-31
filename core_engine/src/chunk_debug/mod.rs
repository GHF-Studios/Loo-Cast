// THIS WHOLE FILE IS A WIP AND WILL BE SPLIT INTO MODULES

use bevy::prelude::*;
use bevy_egui::{EguiContextSettings, EguiPrimaryContextPass};
use bevy::render::camera::Viewport;
use bevy_inspector_egui::egui;
use egui::Color32;
use egui_dock::{DockArea, DockState, Style, TabViewer, NodeIndex};

use crate::camera::{components::MainCamera, resources::GameViewRenderTarget};

// === Resources ===

#[derive(Resource, Default, PartialEq)]
pub struct GameViewUIState {
    pub show_chunk_manager: bool,
    pub show_intent_buffer: bool,
    pub show_intent_commit: bool,
    pub show_chunk_inspector: bool,
    pub is_paused: bool,
    pub step_mode: StepMode,
    pub step_config: StepConfig,
    pub viewport_rect: Option<egui::Rect>,
}

#[derive(Default, Reflect, Resource, PartialEq, Eq, Clone, Copy)]
#[reflect(Resource)]
pub enum StepMode {
    #[default]
    None,
    Cycles,
    Seconds,
}

#[derive(Default, Reflect, Resource, PartialEq)]
#[reflect(Resource)]
pub struct StepConfig {
    pub cycles: u32,
    pub seconds: f32,
}

// TODO: Move this entire module to the 'debug' module, except for maybe the implementations for all the Tabs except the GameView
#[derive(Resource)]
pub struct DebugDock {
    pub dock_state: DockState<EguiWindow>,
}

impl Default for DebugDock {
    fn default() -> Self {
        let mut dock_state = DockState::new(vec![EguiWindow::GameView]);
        let tree = dock_state.main_surface_mut();

        // Split right for Inspector
        let game = NodeIndex::root();
        let inspector = tree.split_right(game, 0.75, vec![EguiWindow::Inspector])[1];

        // Split left for Hierarchy
        let game = tree.split_left(game, 0.20, vec![EguiWindow::Hierarchy])[1];

        // Split bottom for Assets/Resources
        tree.split_below(game, 0.75, vec![EguiWindow::Resources, EguiWindow::Assets]);

        // Add custom debug tabs under Inspector
        tree.split_below(inspector, 0.5, vec![
            EguiWindow::ChunkManager,
            EguiWindow::IntentBuffer,
            EguiWindow::IntentCommit,
            EguiWindow::ChunkInspector,
        ]);

        Self { dock_state }
    }
}

// === Plugin ===

pub struct ChunkDebugPlugin;

impl Plugin for ChunkDebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<StepMode>()
            .register_type::<StepConfig>()
            .init_resource::<GameViewUIState>()
            .init_resource::<DebugDock>()
            .add_systems(EguiPrimaryContextPass, render_chunk_debug_ui)
            //.add_systems(PostUpdate, set_camera_viewport.after(render_chunk_debug_ui))
            ;
    }
}

// === Windows/Tabs ===

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EguiWindow {
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

struct DebugTabViewer<'a> {
    state: &'a mut GameViewUIState,
    texture_id: Option<egui::TextureId>,
    texture_size: Option<egui::Vec2>
}

impl TabViewer for DebugTabViewer<'_> {
    type Tab = EguiWindow;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        format!("{:?}", tab).into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            EguiWindow::GameView => {
                self.state.viewport_rect = Some(ui.clip_rect());

                if let (Some(texture_id), Some(image_size)) = (self.texture_id, self.texture_size) {
                    draw_game_view(ui, texture_id, image_size);
                } else {
                    ui.label("Game View Render Texture not available yet.");
                }
            }
            EguiWindow::Hierarchy => { ui.label("Hierarchy (todo)"); },
            EguiWindow::Resources => { ui.label("Resources (todo)"); },
            EguiWindow::Assets => { ui.label("Assets (todo)"); },
            EguiWindow::Inspector => { ui.label("Inspector (todo)"); },
            EguiWindow::ChunkManager => { ui.label("Chunk Manager (todo)"); },
            EguiWindow::IntentBuffer => { ui.label("Intent Buffer (todo)"); },
            EguiWindow::IntentCommit => { ui.label("Intent Commit (todo)"); },
            EguiWindow::ChunkInspector => { ui.label("Chunk Inspector (todo)"); },
        };
    }

    fn clear_background(&self, tab: &Self::Tab) -> bool {
        !matches!(tab, EguiWindow::GameView)
    }
}

// === Functions ===

fn draw_game_view(
    ui: &mut egui::Ui,
    texture_id: egui::TextureId,
    image_size: egui::Vec2, // actual render texture size
) {
    let available_size = ui.available_size();
    let available_aspect = available_size.x / available_size.y;
    let image_aspect = image_size.x / image_size.y;

    warn!("Drawing game view with: AvailableSize '{}'", available_size);

    // Fit with letterboxing or pillarboxing
    let (final_size, offset) = if image_aspect > available_aspect {
        // Image is wider → fit width, add vertical padding
        let width = available_size.x;
        let height = width / image_aspect;
        let y_offset = (available_size.y - height) * 0.5;
        (egui::Vec2::new(width, height), egui::Vec2::new(0.0, y_offset))
    } else {
        // Image is taller → fit height, add horizontal padding
        let height = available_size.y;
        let width = height * image_aspect;
        let x_offset = (available_size.x - width) * 0.5;
        (egui::Vec2::new(width, height), egui::Vec2::new(x_offset, 0.0))
    };

    let (rect, _) = ui.allocate_exact_size(available_size, egui::Sense::hover());

    let image_rect = egui::Rect::from_min_size(rect.min + offset, final_size);
    ui.painter().image(texture_id, image_rect, egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), Color32::WHITE);
}

// === Systems ===

fn render_chunk_debug_ui(
    mut egui_contexts: bevy_egui::EguiContexts,
    mut state: ResMut<GameViewUIState>,
    mut dock: ResMut<DebugDock>,
    target: Res<GameViewRenderTarget>,
) {
    let ctx = match egui_contexts.ctx_mut() {
        Ok(ctx) => ctx,
        Err(_) => {
            return
        }
    };

    // Toolbar
    egui::TopBottomPanel::top("chunk_debug_toolbar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.checkbox(&mut state.show_chunk_manager, "Chunk Manager");
            ui.checkbox(&mut state.show_intent_buffer, "Intent Buffer");
            ui.checkbox(&mut state.show_intent_commit, "Intent Commit");
            ui.checkbox(&mut state.show_chunk_inspector, "Chunk Inspector");

            ui.separator();

            if ui.button(if state.is_paused { "▶ Resume" } else { "⏸ Pause" }).clicked() {
                state.is_paused = !state.is_paused;
            }

            if ui.button("⏭ Step").clicked() {
                // step logic later
            }

            ui.label("Step Mode:");
            egui::ComboBox::from_label("")
                .selected_text(match state.step_mode {
                    StepMode::None => "None",
                    StepMode::Cycles => "Cycles",
                    StepMode::Seconds => "Seconds",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut state.step_mode, StepMode::None, "None");
                    ui.selectable_value(&mut state.step_mode, StepMode::Cycles, "Cycles");
                    ui.selectable_value(&mut state.step_mode, StepMode::Seconds, "Seconds");
                });

            match state.step_mode {
                StepMode::Cycles => {
                    ui.add(egui::DragValue::new(&mut state.step_config.cycles).speed(1).range(1..=100));
                }
                StepMode::Seconds => {
                    ui.add(egui::DragValue::new(&mut state.step_config.seconds).speed(0.1).range(0.1..=10.0));
                }
                _ => {}
            }
        });
    });

    // Dock area
    egui::CentralPanel::default().show(ctx, |_ui| {
        DockArea::new(&mut dock.dock_state)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut DebugTabViewer {
                state: &mut state,
                texture_id: Some(target.texture_id),
                texture_size: Some(egui::Vec2::new(target.image_size.x as f32, target.image_size.y as f32)),
            });
    });
}

fn set_camera_viewport(
    state: Res<GameViewUIState>,
    window: Query<&Window, With<bevy::window::PrimaryWindow>>,
    mut cam: Query<&mut Camera, With<MainCamera>>,
    egui_settings: Single<&EguiContextSettings>,
) {
    let Ok(window) = window.single() else { return };
    let Ok(mut cam) = cam.single_mut() else { return };
    let Some(rect) = state.viewport_rect else { return };

    let scale_factor = window.scale_factor() * egui_settings.scale_factor;

    let pos = rect.left_top().to_vec2() * scale_factor;
    let size = rect.size() * scale_factor;

    let physical_position = UVec2::new(pos.x.max(0.0) as u32, pos.y.max(0.0) as u32);
    let physical_size = UVec2::new(size.x.max(0.0) as u32, size.y.max(0.0) as u32);

    // skip invalid rects
    if physical_size.x == 0 || physical_size.y == 0 {
        cam.viewport = None;
        return;
    }

    let rect_end = physical_position + physical_size;
    let window_size = window.physical_size();

    warn!("Setting camera viewport to: Pos '{}' | Size '{}'", physical_position, physical_size);

    if rect_end.x <= window_size.x && rect_end.y <= window_size.y {
        cam.viewport = Some(Viewport {
            physical_position,
            physical_size,
            depth: 0.0..1.0,
        });
    }
}

