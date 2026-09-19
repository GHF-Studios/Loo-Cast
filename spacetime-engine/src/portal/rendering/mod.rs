//! Portal presentation.
//!
//! Simulation/topology never depends on this module. Presentation derives from
//! portal-domain state, including whether a complete pair is currently active.

pub mod layout;
pub mod material;
pub mod recursion;
pub mod scene;
mod split;
mod visibility;

use bevy::{
    asset::{load_internal_asset, uuid_handle},
    prelude::*,
    shader::Shader,
};

use super::PortalUpdateSet;

pub const MAIN_PORTAL_LAYER: usize = 1;
/// Entities that should be visible to derived world views (portal cameras,
/// mirrors, etc.) while remaining hidden from the primary first-person view.
/// Recursive portal-context layers start at 4, so layer 2 is reserved here.
pub const DERIVED_VIEW_LAYER: usize = 2;
pub const WORLD_LAYER: usize = 0;

pub use split::PortalSplitVisual;

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
                    visibility::sync_derived_view_lights,
                    split::sync_split_visuals,
                    recursion::camera::update_portal_cameras,
                )
                    .chain()
                    .in_set(PortalUpdateSet::DerivedViews),
            )
            .add_systems(
                Update,
                recursion::targets::resize_render_targets.in_set(PortalUpdateSet::Presentation),
            );
    }
}

pub fn render_size(window: &Window, scale: f32) -> UVec2 {
    scaled_render_size(
        UVec2::new(window.physical_width(), window.physical_height()),
        scale,
    )
}

/// Scales a concrete view's physical render size for derived portal views.
///
/// This is intentionally independent from `Window`: embedded, split-screen or
/// image-backed primary views can all drive the same derived-view sizing path.
pub fn scaled_render_size(size: UVec2, scale: f32) -> UVec2 {
    let scale = scale.clamp(0.1, 1.0);

    (size.as_vec2() * scale).round().as_uvec2().max(UVec2::ONE)
}
