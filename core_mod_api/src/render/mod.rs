pub mod components;
pub mod functions;
pub mod resources;
pub mod systems;

pub mod custom_egui_widgets;
pub mod custom_perf_ui_entries;
pub mod workflows;

use bevy::prelude::*;
use bevy_egui::EguiPrimaryContextPass;
use components::{MainCamera, UiCamera, RenderProxyHandle, RenderProxy};
use resources::{PrimaryWindowUiDockState, PrimaryWindowUiState, ZoomFactor, ViewScale};
use systems::{pre_setup_phase_0, pre_setup_phase_1, resize_render_texture, main_camera_zoom_system, update_view_scale_from_zoom, update_render_proxies, despawn_orphaned_render_proxies, primary_window_ui_system};

use crate::core::run_conditions::run_after_startup_finished;

pub(crate) struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(custom_egui_widgets::CustomEguiWidgetsPlugin)
            .add_plugins(custom_perf_ui_entries::CustomPerfUiEntriesPlugin)
            
            .init_resource::<PrimaryWindowUiState>()
            .init_resource::<PrimaryWindowUiDockState>()
            .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
            .insert_resource(ZoomFactor::default())
            .insert_resource(ViewScale::default())

            .add_systems(PreStartup, (
                pre_setup_phase_0.before(pre_setup_phase_1),
                pre_setup_phase_1,
            ))
            .add_systems(Update, (
                resize_render_texture,
                main_camera_zoom_system.before(update_view_scale_from_zoom),
                update_view_scale_from_zoom,
                despawn_orphaned_render_proxies.before(update_render_proxies),
                update_render_proxies,
            ).run_if(run_after_startup_finished))
            .add_systems(EguiPrimaryContextPass, primary_window_ui_system)

            .register_type::<MainCamera>()
            .register_type::<UiCamera>()
            .register_type::<ZoomFactor>()
            .register_type::<ViewScale>()
            .register_type::<RenderProxyHandle>()
            .register_type::<RenderProxy>()
            .register_type::<PrimaryWindowUiState>();
    }
}
