//! Portal presentation.
//!
//! Simulation/topology never depends on this module. Presentation derives from
//! portal-domain state, including whether a complete pair is currently active.

pub mod layout;
pub mod material;
pub mod recursion;
pub mod scene;
mod visibility;

use bevy::{
    asset::{load_internal_asset, uuid_handle},
    prelude::*,
    shader::Shader,
};

use crate::game::{GameSet, PresentationSet};

pub const MAIN_PORTAL_LAYER: usize = 1;
pub const WORLD_LAYER: usize = 0;

pub const PORTAL_SHADER: Handle<Shader> = uuid_handle!("14bc976d-50ec-4c03-a025-2d39405bb2fa");

pub struct PortalRenderingPlugin;

impl Plugin for PortalRenderingPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, PORTAL_SHADER, "shader/portal.wgsl", Shader::from_wgsl);

        app.add_plugins(MaterialPlugin::<material::PortalMaterial>::default())
            .add_systems(Startup, scene::setup_portals)
            .add_systems(
                Update,
                (
                    visibility::sync_portal_visibility,
                    recursion::camera::update_portal_cameras,
                )
                    .chain()
                    .in_set(PresentationSet::DerivedViews),
            )
            .add_systems(
                Update,
                recursion::targets::resize_render_targets.in_set(GameSet::Presentation),
            );
    }
}

pub fn render_size(window: &Window, scale: f32) -> UVec2 {
    let scale = scale.clamp(0.1, 1.0);

    UVec2::new(
        (window.physical_width() as f32 * scale).round() as u32,
        (window.physical_height() as f32 * scale).round() as u32,
    )
    .max(UVec2::ONE)
}
