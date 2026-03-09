use crate::bevy::pbr::Material;
use crate::bevy::prelude::*;
use crate::bevy::reflect::TypePath;
use crate::bevy::render::render_resource::{AsBindGroup, ShaderType};
use crate::bevy::shader::ShaderRef;
use crate::usf::phenomenon::PhenomenonKind;

const PHENOMENON_SURFACE_SHADER_PATH: &str = "core_mod/shaders/phenomena/mandelbulb_surface.wgsl";

#[derive(Debug, Clone, ShaderType)]
pub struct PhenomenonSurfaceParams {
    pub primary: Vec4,
    pub secondary: Vec4,
    pub glow: Vec4,
    /// x=layer_norm, y=window_scale, z=time, w=emissive_strength
    pub meta: Vec4,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct PhenomenonSurfaceMaterial {
    #[uniform(0)]
    pub params: PhenomenonSurfaceParams,
    pub alpha_mode: AlphaMode,
}

impl PhenomenonSurfaceMaterial {
    pub fn for_mandelbulb_surface() -> Self {
        Self {
            params: PhenomenonSurfaceParams {
                primary: Vec4::new(0.25, 0.62, 0.92, 1.0),
                secondary: Vec4::new(0.95, 0.76, 0.38, 1.0),
                glow: Vec4::new(0.36, 0.82, 1.0, 1.0),
                meta: Vec4::new(0.5, 1.0, 0.0, 0.6),
            },
            alpha_mode: AlphaMode::Opaque,
        }
    }

    pub fn for_phenomenon_kind(kind: PhenomenonKind) -> Self {
        match kind {
            PhenomenonKind::Mandelbulb => Self::for_mandelbulb_surface(),
            PhenomenonKind::SierpinskiSponge => Self {
                params: PhenomenonSurfaceParams {
                    primary: Vec4::new(0.82, 0.92, 0.97, 1.0),
                    secondary: Vec4::new(0.36, 0.55, 0.72, 1.0),
                    glow: Vec4::new(0.82, 0.98, 1.0, 1.0),
                    meta: Vec4::new(0.5, 1.0, 0.0, 0.65),
                },
                alpha_mode: AlphaMode::Opaque,
            },
        }
    }
}

impl Material for PhenomenonSurfaceMaterial {
    fn fragment_shader() -> ShaderRef {
        PHENOMENON_SURFACE_SHADER_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }

    fn enable_prepass() -> bool {
        false
    }
}
