use crate::bevy::ecs::system::SystemState;
use crate::bevy::prelude::*;
use egui::Color32;
use egui_dock::{DockArea, Style};
use once_cell::sync::OnceCell;

use crate::{
    chunk::components::ChunkLoader,
    chunk::resources::{ChunkLoadGate, ChunkLoadGateState},
    config::statics::CONFIG,
    debug::types::DebugSuiteTabViewer,
    input::states::InputMode,
    player::{components::Player, resources::PlayerControlSettings},
    render::resources::{DevZoomFactor, GameViewRenderTarget, PauseMenuWindow, PrimaryWindowUiDockState, PrimaryWindowUiState, RuntimeDebugToggles, ViewScale},
    time::{
        resources::TimeInfo,
        types::{PauseState, StepConfig},
    },
    usf::phenomenon::PhenomenonDebugStats,
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
                ui.checkbox(&mut state.remap_pick_targets_to_source_entities, "Remap Picks");

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

                if let Some(stats) = world.get_resource::<PhenomenonDebugStats>() {
                    ui.separator();
                    ui.label(format!("Nodes {}", stats.active_nodes));
                    ui.label(format!("Proxies {}", stats.active_frontier_proxies));
                    ui.label(format!(
                        "Frontier s={} seed={} w={}‰ (+{} / -{})",
                        stats.frontier_primary_scale_index,
                        stats.frontier_primary_seed,
                        stats.frontier_primary_window_size_milli,
                        stats.frontier_proxy_spawns_frame,
                        stats.frontier_proxy_despawns_frame
                    ));
                    ui.label(format!("Meshes {} (+{})", stats.generated_meshes_total, stats.generated_meshes_frame));
                    ui.label(format!("Cache {} (+{})", stats.mesh_cache_hits_total, stats.mesh_cache_hits_frame));
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

    draw_pause_menu_ui(state, world, ctx);
    draw_local_zoom_indicator(state, world, ctx);
    draw_runtime_debug_overlay(state, world, ctx);
}

fn draw_local_zoom_indicator(state: &PrimaryWindowUiState, world: &mut World, ctx: &egui::Context) {
    if state.enabled {
        return;
    }

    let mut query = world.query_filtered::<&ChunkLoader, With<Player>>();
    let Some((scale_index, local_zoom, local_min, local_max, commit_min, commit_max)) = query.single(world).ok().map(|loader| {
        let policy = loader.usf_transform.scale.policy;
        (
            loader.scale.index_from_top(),
            loader.usf_transform.scale.local_f32(),
            policy.local_min as f32,
            policy.local_max as f32,
            policy.commit_min() as f32,
            policy.commit_max() as f32,
        )
    }) else {
        return;
    };

    egui::Area::new(egui::Id::new("local_zoom_indicator"))
        .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-12.0, 12.0))
        .interactable(false)
        .show(ctx, |ui| {
            egui::Frame::new()
                .fill(egui::Color32::from_black_alpha(185))
                .stroke(egui::Stroke::new(1.0, egui::Color32::from_white_alpha(80)))
                .corner_radius(egui::CornerRadius::same(4))
                .inner_margin(egui::Margin::same(8))
                .show(ui, |ui| {
                    ui.monospace(format!("local zoom: {:.6}x   scale: {}", local_zoom, scale_index));
                    ui.monospace(format!("window [{:.6}, {:.6}]  commit [{:.6}, {:.6}]", local_min, local_max, commit_min, commit_max));
                });
        });
}

fn keybind_options() -> &'static [(KeyCode, &'static str)] {
    &[
        (KeyCode::KeyW, "W"),
        (KeyCode::KeyA, "A"),
        (KeyCode::KeyS, "S"),
        (KeyCode::KeyD, "D"),
        (KeyCode::KeyQ, "Q"),
        (KeyCode::KeyE, "E"),
        (KeyCode::KeyR, "R"),
        (KeyCode::KeyF, "F"),
        (KeyCode::Space, "Space"),
        (KeyCode::ShiftLeft, "Shift Left"),
        (KeyCode::ShiftRight, "Shift Right"),
        (KeyCode::ControlLeft, "Ctrl Left"),
        (KeyCode::ControlRight, "Ctrl Right"),
    ]
}

fn keybind_combo(ui: &mut egui::Ui, label: &str, keybind: &mut KeyCode) {
    egui::ComboBox::from_label(label).selected_text(format!("{keybind:?}")).show_ui(ui, |ui| {
        for (candidate, candidate_label) in keybind_options() {
            ui.selectable_value(keybind, *candidate, *candidate_label);
        }
    });
}

fn draw_pause_menu_ui(state: &mut PrimaryWindowUiState, world: &mut World, ctx: &egui::Context) {
    if !state.pause_menu_open {
        return;
    }

    state.ensure_pause_menu_stack();

    match state.active_pause_menu_window() {
        PauseMenuWindow::Root => draw_pause_root_window(state, ctx),
        PauseMenuWindow::Settings => draw_pause_settings_window(state, world, ctx),
    }
}

fn draw_pause_root_window(state: &mut PrimaryWindowUiState, ctx: &egui::Context) {
    let mut close_requested = false;
    let mut open_settings_requested = false;

    egui::Window::new("Pause Menu")
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.heading("Paused");
            ui.label("Esc to continue.");
            ui.add_space(8.0);

            let resume_button = egui::Button::new(egui::RichText::new("RESUME").strong().size(18.0)).min_size(egui::vec2(ui.available_width(), 44.0));
            if ui.add(resume_button).clicked() {
                close_requested = true;
            }

            ui.add_space(4.0);
            if ui
                .add_sized(
                    egui::vec2(ui.available_width(), 34.0),
                    egui::Button::new(egui::RichText::new("Settings").size(16.0)),
                )
                .clicked()
            {
                open_settings_requested = true;
            }
        });

    if close_requested {
        state.close_pause_menu();
    } else if open_settings_requested {
        state.push_pause_menu_window(PauseMenuWindow::Settings);
    }
}

fn draw_pause_settings_window(state: &mut PrimaryWindowUiState, world: &mut World, ctx: &egui::Context) {
    let mut system_state = SystemState::<ResMut<PlayerControlSettings>>::new(world);
    let mut control_settings = system_state.get_mut(world);
    let mut close_requested = false;
    let mut back_requested = false;

    egui::Window::new("Settings")
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Back").clicked() {
                    back_requested = true;
                }
                ui.separator();
                if ui.button("Resume").clicked() {
                    close_requested = true;
                }
            });

            ui.separator();

            ui.label("First-Person Camera");
            ui.add(
                egui::Slider::new(&mut control_settings.first_person_fov_degrees, 45.0..=179.0)
                    .text("First-person FOV")
                    .suffix(" deg"),
            );
            ui.add(
                egui::Slider::new(&mut control_settings.mouse_look_sensitivity, 0.0005..=0.03)
                    .logarithmic(true)
                    .text("Look sensitivity"),
            );
            ui.checkbox(&mut control_settings.invert_look_x_axis, "Invert look X (horizontal)");
            ui.checkbox(&mut control_settings.invert_look_y_axis, "Invert look Y (vertical)");

            ui.separator();
            ui.label("Third-Person Camera");
            ui.add(
                egui::Slider::new(&mut control_settings.third_person_mouse_look_sensitivity, 0.0005..=0.03)
                    .logarithmic(true)
                    .text("Orbit sensitivity"),
            );
            ui.checkbox(&mut control_settings.invert_third_person_look_x_axis, "Invert orbit X (horizontal)");
            ui.checkbox(&mut control_settings.invert_third_person_look_y_axis, "Invert orbit Y (vertical)");

            ui.separator();
            ui.label("Keybinds");
            keybind_combo(ui, "Forward", &mut control_settings.move_forward);
            keybind_combo(ui, "Backward", &mut control_settings.move_backward);
            keybind_combo(ui, "Left", &mut control_settings.move_left);
            keybind_combo(ui, "Right", &mut control_settings.move_right);
            keybind_combo(ui, "Up", &mut control_settings.move_up);
            keybind_combo(ui, "Down", &mut control_settings.move_down);
            keybind_combo(ui, "Sprint", &mut control_settings.sprint);
            keybind_combo(ui, "Roll Left", &mut control_settings.roll_left);
            keybind_combo(ui, "Roll Right", &mut control_settings.roll_right);

            ui.separator();
            ui.label("Axis Inversion");
            ui.checkbox(&mut control_settings.invert_move_x_axis, "Invert movement X (left/right)");
            ui.checkbox(&mut control_settings.invert_move_y_axis, "Invert movement Y (forward/back)");
            ui.checkbox(&mut control_settings.invert_move_z_axis, "Invert movement Z (up/down)");
            ui.checkbox(&mut control_settings.invert_roll_axis, "Invert roll");

            ui.separator();
            if ui.button("Reset Defaults").clicked() {
                *control_settings = PlayerControlSettings::default();
            }
        });

    control_settings.first_person_fov_degrees = control_settings.first_person_fov_degrees.clamp(45.0, 179.0);
    control_settings.mouse_look_sensitivity = control_settings.mouse_look_sensitivity.clamp(0.0005, 0.03);
    control_settings.third_person_mouse_look_sensitivity = control_settings.third_person_mouse_look_sensitivity.clamp(0.0005, 0.03);
    if close_requested {
        state.close_pause_menu();
    } else if back_requested {
        state.pop_pause_menu_window_or_close();
    }
}

fn draw_runtime_debug_overlay(state: &PrimaryWindowUiState, world: &mut World, ctx: &egui::Context) {
    if !state.show_runtime_debug_overlay {
        return;
    }

    let input_mode_label = world
        .get_resource::<State<InputMode>>()
        .map(|mode| if mode.is_game() { "release" } else { "debug" })
        .unwrap_or("n/a");
    let dev_zoom = world.get_resource::<DevZoomFactor>().map(|value| value.0).unwrap_or(0.0);
    let (view_scale_discrete, view_scale_offset) = world
        .get_resource::<ViewScale>()
        .map(|value| (value.discrete, value.offset))
        .unwrap_or((0, 0.0));
    let (loader_scale_index, loader_local_zoom) = {
        let mut query = world.query_filtered::<&ChunkLoader, With<Player>>();
        query
            .single(world)
            .ok()
            .map(|loader| (loader.scale.index_from_top() as i32, loader.usf_transform.scale.local_f32()))
            .unwrap_or((-1, 0.0))
    };

    let runtime_toggles = world.get_resource::<RuntimeDebugToggles>().copied().unwrap_or_default();

    let mut lines = Vec::with_capacity(12);
    lines.push("USF Runtime Debug  (F6 toggle)".to_string());
    lines.push(format!("input_mode={input_mode_label}"));
    lines.push(format!(
        "camera_zoom={:.6} dev_zoom={:.6} effective_zoom={:.6}",
        loader_local_zoom,
        dev_zoom,
        loader_local_zoom * dev_zoom
    ));
    lines.push(format!(
        "view_scale_discrete={} view_scale_offset={:.4} loader_scale_index={}",
        view_scale_discrete, view_scale_offset, loader_scale_index
    ));
    lines.push(format!(
        "chunk_locator_debug={} hotkey_help={}",
        runtime_toggles.chunk_locator_enabled, runtime_toggles.show_hotkey_help
    ));

    if let Some(stats) = world.get_resource::<PhenomenonDebugStats>() {
        lines.push(format!(
            "nodes={} proxies={} frontier_scale={} frontier_seed={}",
            stats.active_nodes, stats.active_frontier_proxies, stats.frontier_primary_scale_index, stats.frontier_primary_seed
        ));
        lines.push(format!(
            "frontier_window={}‰ proxy_spawns(+{}) proxy_despawns(-{})",
            stats.frontier_primary_window_size_milli, stats.frontier_proxy_spawns_frame, stats.frontier_proxy_despawns_frame
        ));
        lines.push(format!(
            "meshes_total={} meshes_frame={} cache_total={} cache_frame={}",
            stats.generated_meshes_total, stats.generated_meshes_frame, stats.mesh_cache_hits_total, stats.mesh_cache_hits_frame
        ));
    }

    if runtime_toggles.show_hotkey_help {
        lines.push("Hotkeys: Esc=pause menu, F2=input mode, F4=debug suite, F5=camera mode, F6=runtime overlay".to_string());
        lines.push("F3 menu: F3+C toggles chunk wiregrid/wiremesh debug visuals".to_string());
        lines.push("Help toggle: F2+H".to_string());
    }

    egui::Area::new(egui::Id::new("runtime_debug_overlay"))
        .anchor(egui::Align2::LEFT_TOP, egui::vec2(10.0, 10.0))
        .interactable(false)
        .show(ctx, |ui| {
            egui::Frame::new()
                .fill(egui::Color32::from_black_alpha(185))
                .stroke(egui::Stroke::new(1.0, egui::Color32::from_white_alpha(80)))
                .corner_radius(egui::CornerRadius::same(4))
                .inner_margin(egui::Margin::same(8))
                .show(ui, |ui| {
                    for line in lines {
                        ui.monospace(line);
                    }
                });
        });
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
