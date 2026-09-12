use bevy::{
    prelude::*, reflect::TypePath, render::render_resource::AsBindGroup, shader::ShaderRef,
};

use super::PORTAL_SHADER;

/// Material for one directed portal face.
///
/// Ordinary backface culling is intentionally retained. A physically
/// two-sided portal is represented by two separate [`PortalFace`] mechanisms,
/// not by one double-sided polygon.
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct PortalMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,
}

impl Material for PortalMaterial {
    fn fragment_shader() -> ShaderRef {
        ShaderRef::Handle(PORTAL_SHADER.clone())
    }
}
