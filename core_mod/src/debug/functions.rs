use bevy::{asset::ReflectAsset, ecs::system::SystemState, prelude::*, reflect::TypeRegistry};
use egui::Color32;
use egui_dock::{DockArea, Style};

use crate::{
    camera::resources::GameViewRenderTarget,
    debug::types::{DebugSuiteTabViewer, InspectorSelection},
    time::{
        resources::TimeInfo,
        types::{PauseState, StepConfig},
    },
};

use super::resources::{DebugSuiteUiDockState, DebugSuiteUiState};

#[tracing::instrument(skip_all)]
pub(super) fn draw_debug_suite(
    state: &mut DebugSuiteUiState,
    dock_state: &mut DebugSuiteUiDockState,
    target: &GameViewRenderTarget,
    world: &mut World,
    ctx: &mut egui::Context,
) {
    if !state.enabled {
        return;
    }

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

#[tracing::instrument(skip_all)]
pub(super) fn draw_game_view(
    ui: &mut egui::Ui,
    texture_id: egui::TextureId,
    image_size: egui::Vec2, // actual render texture size
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
    ui.painter().image(
        texture_id,
        image_rect,
        egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
        Color32::WHITE,
    );
}

#[tracing::instrument(skip_all)]
pub(super) fn select_resource(ui: &mut egui::Ui, type_registry: &TypeRegistry, selection: &mut InspectorSelection) {
    let mut resources: Vec<_> = type_registry
        .iter()
        .filter(|registration| registration.data::<ReflectResource>().is_some())
        .map(|registration| (registration.type_info().type_path_table().short_path(), registration.type_id()))
        .collect();
    resources.sort_by(|(name_a, _), (name_b, _)| name_a.cmp(name_b));

    for (resource_name, type_id) in resources {
        let selected = match *selection {
            InspectorSelection::Resource(selected, _) => selected == type_id,
            _ => false,
        };

        if ui.selectable_label(selected, resource_name).clicked() {
            *selection = InspectorSelection::Resource(type_id, resource_name.to_string());
        }
    }
}

#[tracing::instrument(skip_all)]
pub(super) fn select_asset(ui: &mut egui::Ui, type_registry: &TypeRegistry, world: &World, selection: &mut InspectorSelection) {
    let mut assets: Vec<_> = type_registry
        .iter()
        .filter_map(|registration| {
            let reflect_asset = registration.data::<ReflectAsset>()?;
            Some((registration.type_info().type_path_table().short_path(), registration.type_id(), reflect_asset))
        })
        .collect();
    assets.sort_by(|(name_a, ..), (name_b, ..)| name_a.cmp(name_b));

    for (asset_name, asset_type_id, reflect_asset) in assets {
        let handles: Vec<_> = reflect_asset.ids(world).collect();

        ui.collapsing(format!("{asset_name} ({})", handles.len()), |ui| {
            for handle in handles {
                let selected = match *selection {
                    InspectorSelection::Asset(_, _, selected_id) => selected_id.unwrap() == handle,
                    _ => false,
                };

                if ui.selectable_label(selected, format!("{handle:?}")).clicked() {
                    *selection = InspectorSelection::Asset(asset_type_id, asset_name.to_string(), Some(handle));
                }
            }
        });
    }
}
