use bevy::prelude::*;
use bevy_inspector_egui::egui;
use egui_dock::{DockArea, DockState, NodeIndex, Style, SurfaceIndex, TabViewer};

#[derive(Default, Resource, Reflect, PartialEq)]
#[reflect(Resource)]
pub struct ChunkDebugUIState {
    pub show_chunk_manager: bool,
    pub show_intent_buffer: bool,
    pub show_intent_commit: bool,
    pub show_chunk_inspector: bool,
    pub is_paused: bool,
    pub step_mode: StepMode,
    pub step_config: StepConfig,
}

#[derive(Default, Reflect, Resource, PartialEq, Eq)]
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

pub struct ChunkDebugPlugin;

impl Plugin for ChunkDebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<ChunkDebugUIState>()
            .register_type::<StepMode>()
            .register_type::<StepConfig>()
            .init_resource::<ChunkDebugUIState>()
            .insert_resource(ChunkDebugDock::default())
            .add_systems(Update, render_chunk_debug_ui);
    }
}

#[derive(Resource)]
pub struct ChunkDebugDock {
    pub dock_state: DockState<String>,
}
impl Default for ChunkDebugDock {
    fn default() -> Self {
        Self {
            dock_state: DockState::new(vec![]),
        }
    }
}

struct DebugTabViewer;

impl TabViewer for DebugTabViewer {
    type Tab = String;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.clone().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        ui.label(format!("Window: {tab}"));
    }
}

fn render_chunk_debug_ui(
    mut egui_contexts: bevy_inspector_egui::bevy_egui::EguiContexts,
    mut state: ResMut<ChunkDebugUIState>,
    mut dock: ResMut<ChunkDebugDock>,
) {
    let ctx = egui_contexts.ctx_mut();

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
                // Step execution logic will go here later
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

    // Build a fresh dock state based on toggles
    let mut tabs = vec![];
    if state.show_chunk_manager {
        tabs.push("Chunk Manager".to_string());
    }
    if state.show_intent_buffer {
        tabs.push("Intent Buffer".to_string());
    }
    if state.show_intent_commit {
        tabs.push("Intent Commit".to_string());
    }
    if state.show_chunk_inspector {
        tabs.push("Chunk Inspector".to_string());
    }

    // Reset dock state with current tabs
    dock.dock_state = DockState::new(tabs);

    egui::CentralPanel::default().show(ctx, |ui| {
        DockArea::new(&mut dock.dock_state)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut DebugTabViewer);
    });
}