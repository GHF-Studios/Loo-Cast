pub mod components;
pub mod functions;
pub mod resources;
pub mod systems;

pub mod custom_egui_widgets;
// pub mod custom_perf_ui_entries;
pub mod workflows;

use crate::bevy::prelude::*;
use bevy_egui::EguiPrimaryContextPass;
use components::{ChunkCubeCamera, EntityProxyLink, LogicProxy, MainCamera, ProxySyncRevision, RenderProxy, RenderProxyWindowMode, UiCamera};
use resources::{DevZoomFactor, PrimaryWindowUiDockState, PrimaryWindowUiState, ViewScale, ZoomFactor};
use systems::{
    apply_usf_player_pivots_system, despawn_orphaned_render_proxies, enforce_chunk_cube_camera_depth_contract_system,
    enforce_main_camera_depth_contract_system, main_camera_zoom_system, pre_setup_phase_0, pre_setup_phase_1, primary_window_ui_system,
    resize_render_texture, update_render_proxies, update_view_scale_from_zoom,
};

use crate::core::{components::Meta, orchestration::AppSet, run_conditions::run_after_startup_finished};

pub(crate) struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(custom_egui_widgets::CustomEguiWidgetsPlugin)
            // .add_plugins(custom_perf_ui_entries::CustomPerfUiEntriesPlugin)
            .init_resource::<PrimaryWindowUiState>()
            .init_resource::<PrimaryWindowUiDockState>()
            .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
            .insert_resource(ZoomFactor::default())
            .insert_resource(DevZoomFactor::default())
            .insert_resource(ViewScale::default())
            .add_systems(PreStartup, (pre_setup_phase_0.before(pre_setup_phase_1), pre_setup_phase_1))
            .add_systems(
                Update,
                (
                    resize_render_texture.in_set(AppSet::Presentation),
                    main_camera_zoom_system.in_set(AppSet::InputGather),
                    apply_usf_player_pivots_system.in_set(AppSet::BoundaryResolve),
                    enforce_main_camera_depth_contract_system.in_set(AppSet::Camera),
                    enforce_chunk_cube_camera_depth_contract_system.in_set(AppSet::Camera),
                    update_view_scale_from_zoom.in_set(AppSet::Camera),
                    despawn_orphaned_render_proxies.in_set(AppSet::Presentation),
                    update_render_proxies.in_set(AppSet::Presentation).after(despawn_orphaned_render_proxies),
                )
                    .run_if(run_after_startup_finished),
            )
            .add_systems(EguiPrimaryContextPass, primary_window_ui_system)
            .register_type::<MainCamera>()
            .register_type::<ChunkCubeCamera>()
            .register_type::<UiCamera>()
            .register_type::<ViewScale>()
            .register_type::<EntityProxyLink>()
            .register_type::<LogicProxy>()
            .register_type::<RenderProxy>()
            .register_type::<RenderProxyWindowMode>()
            .register_type::<ProxySyncRevision>()
            .register_type::<Meta<Sprite>>()
            .register_type::<PrimaryWindowUiState>()
            .register_type::<ZoomFactor>()
            .register_type::<DevZoomFactor>();
    }
}
