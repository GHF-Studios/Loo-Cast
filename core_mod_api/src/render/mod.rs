pub mod camera_contract;
pub mod components;
pub mod functions;
pub mod resources;
pub mod systems;

pub mod custom_egui_widgets;
// pub mod custom_perf_ui_entries;
pub mod workflows;

use crate::bevy::prelude::*;
use bevy_egui::EguiPrimaryContextPass;
use components::{EguiCamera, EntityProxyLink, LogicProxy, MainCamera, ProxySyncRevision, RenderProxy, RenderProxyWindowMode, UiCamera, WorldPresentationRoot};
use resources::{DevZoomFactor, PauseMenuWindow, PrimaryWindowUiDockState, PrimaryWindowUiState, RuntimeDebugToggles, ViewScale, ZoomFactor};
use systems::{
    apply_usf_player_pivots_system, bind_render_proxies_to_world_presentation_root_system, despawn_orphaned_render_proxies, draw_chunk_locator_gizmos_system,
    enforce_main_camera_depth_contract_system, main_camera_zoom_system, pre_setup_phase_0, pre_setup_phase_1, primary_window_ui_system, resize_render_texture,
    update_render_proxies, update_view_scale_from_zoom, update_world_presentation_root_transform_system, validate_camera_contract_system,
};

use crate::core::{components::Meta, orchestration::AppSet, run_conditions::run_after_startup_finished};
use crate::follower::systems::update_follower_system;

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
            .init_resource::<RuntimeDebugToggles>()
            .add_systems(PreStartup, (pre_setup_phase_0.before(pre_setup_phase_1), pre_setup_phase_1))
            .add_systems(
                Update,
                (
                    resize_render_texture.in_set(AppSet::Presentation),
                    main_camera_zoom_system.in_set(AppSet::InputGather),
                    apply_usf_player_pivots_system.in_set(AppSet::BoundaryResolve),
                    enforce_main_camera_depth_contract_system.in_set(AppSet::Camera).after(update_follower_system),
                    update_view_scale_from_zoom.in_set(AppSet::Camera),
                    validate_camera_contract_system.in_set(AppSet::Diagnostics).after(update_view_scale_from_zoom),
                    despawn_orphaned_render_proxies.in_set(AppSet::Presentation),
                    update_world_presentation_root_transform_system
                        .in_set(AppSet::Presentation)
                        .after(apply_usf_player_pivots_system),
                    bind_render_proxies_to_world_presentation_root_system
                        .in_set(AppSet::Presentation)
                        .after(despawn_orphaned_render_proxies),
                    update_render_proxies
                        .in_set(AppSet::Presentation)
                        .after(bind_render_proxies_to_world_presentation_root_system),
                    draw_chunk_locator_gizmos_system.in_set(AppSet::Presentation).after(update_render_proxies),
                )
                    .run_if(run_after_startup_finished),
            )
            .add_systems(EguiPrimaryContextPass, primary_window_ui_system)
            .register_type::<MainCamera>()
            .register_type::<UiCamera>()
            .register_type::<EguiCamera>()
            .register_type::<WorldPresentationRoot>()
            .register_type::<ViewScale>()
            .register_type::<EntityProxyLink>()
            .register_type::<LogicProxy>()
            .register_type::<RenderProxy>()
            .register_type::<RenderProxyWindowMode>()
            .register_type::<ProxySyncRevision>()
            .register_type::<Meta<Sprite>>()
            .register_type::<Meta<Mesh3d>>()
            .register_type::<PrimaryWindowUiState>()
            .register_type::<PauseMenuWindow>()
            .register_type::<RuntimeDebugToggles>()
            .register_type::<ZoomFactor>()
            .register_type::<DevZoomFactor>();
    }
}
