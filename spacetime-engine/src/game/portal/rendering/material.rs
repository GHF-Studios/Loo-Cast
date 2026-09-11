use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
};

use super::PORTAL_SHADER;

/// A one-sided screen-space aperture material.
///
/// Two-sided portals are represented by two explicitly oriented surface
/// entities, so this material can retain ordinary backface culling.
#[derive(
    Asset,
    TypePath,
    AsBindGroup,
    Debug,
    Clone,
)]
pub struct PortalMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,
}

impl Material for PortalMaterial {
    fn fragment_shader() -> ShaderRef {
        ShaderRef::Handle(
            PORTAL_SHADER.clone(),
        )
    }
}
