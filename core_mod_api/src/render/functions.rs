use bevy::prelude::*;
use bevy::ecs::system::SystemState;
use egui::{Color32, emath::GuiRounding};
use egui_dock::{DockArea, Style};
use once_cell::sync::OnceCell;

use crate::{
    debug::types::DebugSuiteTabViewer,
    render::{
        components::RenderProxy,
        resources::{PrimaryWindowUiDockState, PrimaryWindowUiState, GameViewRenderTarget},
        
    },
    time::{
        resources::TimeInfo,
        types::{PauseState, StepConfig},
    },
};

static RESERVED_EGUI_CAMERA_ENTITY: OnceCell<Entity> = OnceCell::new();
static RESERVED_UI_CAMERA_ENTITY: OnceCell<Entity> = OnceCell::new();
static RESERVED_MAIN_CAMERA_ENTITY: OnceCell<Entity> = OnceCell::new();

static RESERVED_GAME_VIEW_RENDER_TARGET_HANDLE: OnceCell<Handle<Image>> = OnceCell::new();
static RESERVED_GAME_VIEW_RENDER_TARGET_SIZE_UVEC2: OnceCell<UVec2> = OnceCell::new();

pub(super) fn reserve_camera_entities(egui_camera: Entity, ui_camera: Entity, main_camera: Entity) {
    RESERVED_EGUI_CAMERA_ENTITY.set(egui_camera).expect("RESERVED_EGUI_CAMERA_ENTITY already set");
    RESERVED_UI_CAMERA_ENTITY.set(ui_camera).expect("RESERVED_UI_CAMERA_ENTITY already set");
    RESERVED_MAIN_CAMERA_ENTITY.set(main_camera).expect("RESERVED_MAIN_CAMERA_ENTITY already set");
}
pub(super) fn get_reserved_camera_entities() -> (Entity, Entity, Entity) {
    (
        RESERVED_EGUI_CAMERA_ENTITY.clone().into_inner().expect("RESERVED_EGUI_CAMERA_ENTITY not set"),
        RESERVED_UI_CAMERA_ENTITY.clone().into_inner().expect("RESERVED_UI_CAMERA_ENTITY not set"),
        RESERVED_MAIN_CAMERA_ENTITY.clone().into_inner().expect("RESERVED_MAIN_CAMERA_ENTITY not set"),
    )
}

pub(super) fn reserve_game_view_render_target(handle: Handle<Image>, size_uvec2: UVec2) {
    RESERVED_GAME_VIEW_RENDER_TARGET_HANDLE.set(handle).expect("RESERVED_GAME_VIEW_RENDER_TARGET_HANDLE already set");
    RESERVED_GAME_VIEW_RENDER_TARGET_SIZE_UVEC2.set(size_uvec2).expect("RESERVED_GAME_VIEW_RENDER_TARGET_SIZE already set");
}
pub(super) fn get_reserved_game_view_render_target() -> (Handle<Image>, UVec2) {
    (
        RESERVED_GAME_VIEW_RENDER_TARGET_HANDLE.clone().into_inner().expect("RESERVED_GAME_VIEW_RENDER_TARGET_HANDLE not set"),
        RESERVED_GAME_VIEW_RENDER_TARGET_SIZE_UVEC2.clone().into_inner().expect("RESERVED_GAME_VIEW_RENDER_TARGET_SIZE not set"),
    )
}

pub fn new_sprite_proxy_bundle(
    image: Handle<Image>,
    pos: Vec2,
    scale: f32,
    source_entity: Entity,
    chunk_z: f32,
) -> impl Bundle {
    (
        Transform {
            translation: pos.extend(chunk_z),
            scale: Vec3::splat(scale),
            ..Default::default()
        },
        Sprite {
            image,
            ..Default::default()
        },
        Pickable::default(),
        RenderProxy {
            source: source_entity,
        },
    )
}

// TODO: Move this (and other similar/related) to "render/functions.rs" (and other similar/related)
#[tracing::instrument(skip_all)]
pub(crate) fn draw_primary_window_ui(
    state: &mut PrimaryWindowUiState,
    dock_state: &mut PrimaryWindowUiDockState,
    target: &GameViewRenderTarget,
    world: &mut World,
    ctx: &mut egui::Context,
) {
    if !state.enabled {
        // Game view only
        let central_panel = egui::CentralPanel::default();
        let central_panel = central_panel.frame(egui::Frame {
            inner_margin: egui::Margin::same(0),
            fill: Color32::default(),
            stroke: egui::Stroke::new(0.0, Color32::default()),
            corner_radius: egui::CornerRadius::same(0),
            outer_margin: egui::Margin::same(0),
            shadow: egui::Shadow::NONE,
        });

        central_panel.show(ctx, |ui| {
            super::functions::draw_game_view(
                ui,
                target.id,
                egui::Vec2::new(target.size.x as f32, target.size.y as f32),
                &mut state.viewport_rect_precision_proxy,
            );
        });
    } else {
        // Toolbar
        egui::TopBottomPanel::top("debug_suite_toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.checkbox(&mut state.show_chunk_manager, "Chunk Manager");
                ui.checkbox(&mut state.show_intent_buffer, "Intent Buffer");
                ui.checkbox(&mut state.show_intent_commit, "Intent Commit");
                ui.checkbox(&mut state.show_chunk_inspector, "Chunk Inspector");

                ui.separator();

                let mut system_state: SystemState<_> = SystemState::<(ResMut<TimeInfo>, ResMut<Time<Virtual>>)>::new(world);
                let (mut time_info, mut virtual_time) = system_state.get_mut(world);
                let pause_state = &mut time_info.pause_state;

                if ui.button(if pause_state.is_paused() { "▶ Resume" } else { "⏸ Pause" }).clicked() {
                    match pause_state {
                        PauseState::Running => {
                            *pause_state = PauseState::Paused;
                            virtual_time.pause();
                        }
                        PauseState::Paused => {
                            *pause_state = PauseState::Running;
                            virtual_time.unpause();
                        }
                        PauseState::Step => {}
                    }
                }

                if ui.button("⏭ Step").clicked() {
                    match time_info.pause_state {
                        PauseState::Running => {
                            return;
                        }
                        PauseState::Paused => {
                            time_info.pause_state = PauseState::Step;
                        }
                        PauseState::Step => {
                            return;
                        }
                    }
                }

                ui.label("Step Mode:");
                egui::ComboBox::from_label("")
                    .selected_text(match time_info.step_config {
                        StepConfig::Cycles(_) => "Cycles",
                        StepConfig::Seconds(_) => "Seconds",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut time_info.step_config, StepConfig::Cycles(1), "Cycles");
                        ui.selectable_value(&mut time_info.step_config, StepConfig::Seconds(1.0), "Seconds");
                    });

                match &mut time_info.step_config {
                    StepConfig::Cycles(cycles) => {
                        ui.add(egui::DragValue::new(cycles).speed(1));
                    }
                    StepConfig::Seconds(seconds) => {
                        ui.add(egui::DragValue::new(seconds).speed(1));
                    }
                }
            });
        });

        // Dock area
        egui::CentralPanel::default().show(ctx, |_ui| {
            DockArea::new(&mut dock_state.dock_state).style(Style::from_egui(ctx.style().as_ref())).show(
                ctx,
                &mut DebugSuiteTabViewer {
                    world,
                    state,
                    game_view_texture_id: Some(target.id),
                    game_view_texture_size: Some(egui::Vec2::new(target.size.x as f32, target.size.y as f32)),
                },
            );
        });
    }
}

#[tracing::instrument(skip_all)]
pub(crate) fn draw_game_view(
    ui: &mut egui::Ui,
    texture_id: egui::TextureId,
    image_size: egui::Vec2, // actual render texture size
    viewport_rect_precision_proxy: &mut Option<egui::Rect>,
) {
    let available_size = ui.available_size();
    let available_aspect = available_size.x / available_size.y;
    let image_aspect = image_size.x / image_size.y;

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

    *viewport_rect_precision_proxy = Some(image_rect);

    ui.painter().image(
        texture_id,
        image_rect,
        egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
        Color32::WHITE,
    );
}
