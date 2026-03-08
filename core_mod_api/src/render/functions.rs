use crate::bevy::ecs::system::SystemState;
use crate::bevy::prelude::*;
use egui::Color32;
use egui_dock::{DockArea, Style};
use once_cell::sync::OnceCell;
use std::hash::{Hash, Hasher};

use crate::{
    chunk::resources::{ChunkLoadGate, ChunkLoadGateState},
    config::statics::CONFIG,
    debug::types::DebugSuiteTabViewer,
    render::{
        components::{ProxySyncRevision, RenderProxy, RenderProxyWindowMode},
        resources::{GameViewRenderTarget, PrimaryWindowUiDockState, PrimaryWindowUiState},
    },
    time::{
        resources::TimeInfo,
        types::{PauseState, StepConfig},
    },
    usf::pos::grid::types::GridVec,
    usf::scale::Scale,
};

static RESERVED_EGUI_CAMERA_ENTITY: OnceCell<Entity> = OnceCell::new();
static RESERVED_UI_CAMERA_ENTITY: OnceCell<Entity> = OnceCell::new();
static RESERVED_MAIN_CAMERA_ENTITY: OnceCell<Entity> = OnceCell::new();
static RESERVED_CHUNK_CUBE_CAMERA_ENTITY: OnceCell<Entity> = OnceCell::new();

static RESERVED_GAME_VIEW_RENDER_TARGET_HANDLE: OnceCell<Handle<Image>> = OnceCell::new();
static RESERVED_GAME_VIEW_RENDER_TARGET_SIZE_UVEC2: OnceCell<UVec2> = OnceCell::new();

pub(super) fn reserve_camera_entities(egui_camera: Entity, ui_camera: Entity, main_camera: Entity, chunk_cube_camera: Entity) {
    RESERVED_EGUI_CAMERA_ENTITY.set(egui_camera).expect("RESERVED_EGUI_CAMERA_ENTITY already set");
    RESERVED_UI_CAMERA_ENTITY.set(ui_camera).expect("RESERVED_UI_CAMERA_ENTITY already set");
    RESERVED_MAIN_CAMERA_ENTITY.set(main_camera).expect("RESERVED_MAIN_CAMERA_ENTITY already set");
    RESERVED_CHUNK_CUBE_CAMERA_ENTITY
        .set(chunk_cube_camera)
        .expect("RESERVED_CHUNK_CUBE_CAMERA_ENTITY already set");
}
pub(super) fn get_reserved_camera_entities() -> (Entity, Entity, Entity, Entity) {
    (
        RESERVED_EGUI_CAMERA_ENTITY.clone().into_inner().expect("RESERVED_EGUI_CAMERA_ENTITY not set"),
        RESERVED_UI_CAMERA_ENTITY.clone().into_inner().expect("RESERVED_UI_CAMERA_ENTITY not set"),
        RESERVED_MAIN_CAMERA_ENTITY.clone().into_inner().expect("RESERVED_MAIN_CAMERA_ENTITY not set"),
        RESERVED_CHUNK_CUBE_CAMERA_ENTITY
            .clone()
            .into_inner()
            .expect("RESERVED_CHUNK_CUBE_CAMERA_ENTITY not set"),
    )
}

pub(super) fn reserve_game_view_render_target(handle: Handle<Image>, size_uvec2: UVec2) {
    RESERVED_GAME_VIEW_RENDER_TARGET_HANDLE
        .set(handle)
        .expect("RESERVED_GAME_VIEW_RENDER_TARGET_HANDLE already set");
    RESERVED_GAME_VIEW_RENDER_TARGET_SIZE_UVEC2
        .set(size_uvec2)
        .expect("RESERVED_GAME_VIEW_RENDER_TARGET_SIZE already set");
}
pub(super) fn get_reserved_game_view_render_target() -> (Handle<Image>, UVec2) {
    (
        RESERVED_GAME_VIEW_RENDER_TARGET_HANDLE
            .clone()
            .into_inner()
            .expect("RESERVED_GAME_VIEW_RENDER_TARGET_HANDLE not set"),
        RESERVED_GAME_VIEW_RENDER_TARGET_SIZE_UVEC2
            .clone()
            .into_inner()
            .expect("RESERVED_GAME_VIEW_RENDER_TARGET_SIZE not set"),
    )
}

pub fn new_sprite_proxy_bundle(image: Handle<Image>, pos: Vec2, visual_scale: f32, source_entity: Entity, coord_scale: Scale, depth_bias: f32) -> impl Bundle {
    (
        Transform {
            translation: pos.extend(coord_scale.compute_z() + depth_bias),
            scale: Vec3::splat(visual_scale),
            ..Default::default()
        },
        Sprite { image, ..Default::default() },
        Pickable::default(),
        RenderProxy {
            source: source_entity,
            layer_index: coord_scale.render_layer_index(),
            depth_bias,
            window_mode: RenderProxyWindowMode::WindowedSubsection,
            window_center_local: Vec2::ZERO,
            window_size_local: Vec2::ONE,
            coarse_context_persistent: true,
        },
        ProxySyncRevision::default(),
    )
}

pub const CHUNK_DEV_CUBE_SUBGRID_SIZE: i32 = 10;
pub const CHUNK_DEV_CUBE_SIZE_LOCAL_UNITS: f32 = 100.0;
pub const CHUNK_DEV_CUBE_DEFAULT_COUNT: usize = 8;
pub const CHUNK_DEV_CUBE_DEFAULT_DEPTH_LAYERS: i32 = 10;
pub const CHUNK_DEV_SURFACE_CUBE_DEFAULT_COUNT: usize = 24;

pub fn new_chunk_cube_proxy_bundle(pos: Vec2, visual_scale: f32, source_entity: Entity, coord_scale: Scale, depth_bias: f32) -> impl Bundle {
    (
        Transform {
            translation: pos.extend(coord_scale.compute_z() + depth_bias),
            scale: Vec3::splat(visual_scale),
            ..Default::default()
        },
        Visibility::Visible,
        RenderProxy {
            source: source_entity,
            layer_index: coord_scale.render_layer_index(),
            depth_bias,
            window_mode: RenderProxyWindowMode::WindowedSubsection,
            window_center_local: Vec2::ZERO,
            window_size_local: Vec2::ONE,
            coarse_context_persistent: true,
        },
        ProxySyncRevision::default(),
    )
}

pub fn compute_chunk_dev_cube_local_offsets(grid_coord: &GridVec, requested_count: usize, depth_layers: i32) -> Vec<Vec3> {
    let depth_layers = depth_layers.clamp(1, CHUNK_DEV_CUBE_SUBGRID_SIZE);
    let max_slots = (CHUNK_DEV_CUBE_SUBGRID_SIZE * CHUNK_DEV_CUBE_SUBGRID_SIZE * depth_layers) as usize;
    let count = requested_count.min(max_slots);
    if count == 0 {
        return Vec::new();
    }

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    grid_coord.hash(&mut hasher);
    let seed = hasher.finish();

    let mut slots = (0..depth_layers)
        .flat_map(|z| (0..CHUNK_DEV_CUBE_SUBGRID_SIZE).flat_map(move |y| (0..CHUNK_DEV_CUBE_SUBGRID_SIZE).map(move |x| IVec3::new(x, y, z))))
        .map(|cell| {
            let packed = ((cell.x as u64) << 42) | ((cell.y as u64) << 21) | cell.z as u64;
            let score = splitmix64(seed ^ packed);
            (score, cell)
        })
        .collect::<Vec<_>>();
    slots.sort_by_key(|(score, _)| *score);

    let half_xy = (CHUNK_DEV_CUBE_SUBGRID_SIZE as f32 - 1.0) * 0.5;
    let half_z = (depth_layers as f32 - 1.0) * 0.5;
    slots
        .into_iter()
        .take(count)
        .map(|(_, cell)| {
            let x = (cell.x as f32 - half_xy) * CHUNK_DEV_CUBE_SIZE_LOCAL_UNITS;
            let y = (cell.y as f32 - half_xy) * CHUNK_DEV_CUBE_SIZE_LOCAL_UNITS;
            let z = (cell.z as f32 - half_z) * CHUNK_DEV_CUBE_SIZE_LOCAL_UNITS;
            Vec3::new(x, y, z)
        })
        .collect()
}

pub fn compute_chunk_dev_surface_cube_local_offsets(grid_coord: &GridVec, parent_cube_index: usize, requested_count: usize) -> Vec<Vec3> {
    let face_subgrid = CHUNK_DEV_CUBE_SUBGRID_SIZE;
    let max_slots = (6 * face_subgrid * face_subgrid) as usize;
    let count = requested_count.min(max_slots);
    if count == 0 {
        return Vec::new();
    }

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    grid_coord.hash(&mut hasher);
    parent_cube_index.hash(&mut hasher);
    let seed = hasher.finish();

    let mut slots = (0..6_u8)
        .flat_map(|face| (0..face_subgrid).flat_map(move |v| (0..face_subgrid).map(move |u| (face, u, v))))
        .map(|(face, u, v)| {
            let packed = ((face as u64) << 56) | ((u as u64) << 28) | (v as u64);
            let score = splitmix64(seed ^ packed);
            (score, (face, u, v))
        })
        .collect::<Vec<_>>();
    slots.sort_by_key(|(score, _)| *score);

    let micro_size = CHUNK_DEV_CUBE_SIZE_LOCAL_UNITS / CHUNK_DEV_CUBE_SUBGRID_SIZE as f32;
    let micro_half = micro_size * 0.5;
    let parent_half = CHUNK_DEV_CUBE_SIZE_LOCAL_UNITS * 0.5;
    let uv_half = (face_subgrid as f32 - 1.0) * 0.5;

    slots
        .into_iter()
        .take(count)
        .map(|(_, (face, u, v))| {
            let ux = (u as f32 - uv_half) * micro_size;
            let vy = (v as f32 - uv_half) * micro_size;

            match face {
                0 => Vec3::new(parent_half + micro_half, ux, vy),    // +X
                1 => Vec3::new(-(parent_half + micro_half), ux, vy), // -X
                2 => Vec3::new(ux, parent_half + micro_half, vy),    // +Y
                3 => Vec3::new(ux, -(parent_half + micro_half), vy), // -Y
                4 => Vec3::new(ux, vy, parent_half + micro_half),    // +Z
                _ => Vec3::new(ux, vy, -(parent_half + micro_half)), // -Z
            }
        })
        .collect()
}

#[inline]
fn splitmix64(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^ (x >> 31)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_dev_cube_layout_is_deterministic() {
        let coord = GridVec::new_root(IVec2::new(1, -2));
        let a = compute_chunk_dev_cube_local_offsets(&coord, CHUNK_DEV_CUBE_DEFAULT_COUNT, CHUNK_DEV_CUBE_DEFAULT_DEPTH_LAYERS);
        let b = compute_chunk_dev_cube_local_offsets(&coord, CHUNK_DEV_CUBE_DEFAULT_COUNT, CHUNK_DEV_CUBE_DEFAULT_DEPTH_LAYERS);
        assert_eq!(a, b);
    }

    #[test]
    fn chunk_dev_cube_layout_is_subgrid_aligned_and_bounded_3d() {
        let coord = GridVec::new_root(IVec2::new(0, 0));
        let offsets = compute_chunk_dev_cube_local_offsets(&coord, 100, CHUNK_DEV_CUBE_DEFAULT_DEPTH_LAYERS);
        assert_eq!(offsets.len(), 100);

        for pos in offsets {
            assert!(pos.x >= -450.0 && pos.x <= 450.0);
            assert!(pos.y >= -450.0 && pos.y <= 450.0);
            assert!(pos.z >= -450.0 && pos.z <= 450.0);
            assert!(((pos.x + 450.0) / CHUNK_DEV_CUBE_SIZE_LOCAL_UNITS).fract().abs() < 1e-6);
            assert!(((pos.y + 450.0) / CHUNK_DEV_CUBE_SIZE_LOCAL_UNITS).fract().abs() < 1e-6);
            assert!(((pos.z + 450.0) / CHUNK_DEV_CUBE_SIZE_LOCAL_UNITS).fract().abs() < 1e-6);
        }
    }

    #[test]
    fn chunk_dev_cube_depth_one_stays_single_slice() {
        let coord = GridVec::new_root(IVec2::new(2, 3));
        let offsets = compute_chunk_dev_cube_local_offsets(&coord, 32, 1);
        assert!(!offsets.is_empty());
        assert!(offsets.iter().all(|p| p.z == 0.0));
    }

    #[test]
    fn surface_cube_layout_is_deterministic_and_on_surfaces() {
        let coord = GridVec::new_root(IVec2::new(-1, 4));
        let a = compute_chunk_dev_surface_cube_local_offsets(&coord, 2, CHUNK_DEV_SURFACE_CUBE_DEFAULT_COUNT);
        let b = compute_chunk_dev_surface_cube_local_offsets(&coord, 2, CHUNK_DEV_SURFACE_CUBE_DEFAULT_COUNT);
        assert_eq!(a, b);
        assert!(a.iter().all(|p| {
            let ax = p.x.abs();
            let ay = p.y.abs();
            let az = p.z.abs();
            (ax - 55.0).abs() < 1e-6 || (ay - 55.0).abs() < 1e-6 || (az - 55.0).abs() < 1e-6
        }));
    }
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
    let overload_overlay = if CONFIG().get::<bool>("workflow/chunk_load_gate_enabled") {
        world.get_resource::<ChunkLoadGate>().and_then(|gate| match gate.state {
            ChunkLoadGateState::Open => None,
            ChunkLoadGateState::LockedByTimeout => {
                let label = if let Some(info) = gate.lock_info {
                    format!("TIMEOUT {}::{} | Retry #{}", info.module_name, info.workflow_name, info.timeout_count)
                } else {
                    "TIMEOUT".to_string()
                };
                Some((egui::Color32::RED, label))
            }
            ChunkLoadGateState::LockedByInFlightBoundary => Some((egui::Color32::YELLOW, "BOUNDARY OVERLAP".to_string())),
        })
    } else {
        None
    };

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

            if let Some((border_color, label)) = overload_overlay.as_ref() {
                if let Some(viewport_rect) = state.viewport_rect_precision_proxy {
                    let stroke = egui::Stroke::new(12.0, *border_color);
                    ui.painter().rect_stroke(viewport_rect, 0.0, stroke, egui::StrokeKind::Middle);

                    let text_anchor = viewport_rect.center_top() + egui::vec2(0.0, 8.0);
                    let font_id = egui::TextStyle::Heading.resolve(ui.style());
                    let galley = ui.painter().layout_no_wrap(label.clone(), font_id, egui::Color32::WHITE);
                    let text_size = galley.size();
                    let text_top_left = egui::pos2(text_anchor.x - text_size.x * 0.5, text_anchor.y);

                    let padding = egui::vec2(10.0, 6.0);
                    let bg_rect = egui::Rect::from_min_max(text_top_left - padding, text_top_left + text_size + padding);
                    ui.painter().rect_filled(bg_rect, 4.0, egui::Color32::from_black_alpha(220));
                    ui.painter().galley(text_top_left, galley, egui::Color32::WHITE);
                }
            }
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
