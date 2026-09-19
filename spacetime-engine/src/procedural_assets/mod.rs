//! Runtime-generated presentation assets.
//!
//! Procedural assets are recipes and disposable generated representations. They
//! deliberately do not own semantic world state. A world/Phenomenon may choose
//! recipe parameters or seeds later, but texture/model/audio generation remains
//! a separate replaceable layer.

mod debug_grid;
mod systems;
mod texture;

pub use texture::CrackedClayRecipe;

use bevy::{
    asset::RenderAssetUsages,
    image::{ImageAddressMode, ImageSampler, ImageSamplerDescriptor},
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

use texture::generate_cracked_clay;

/// Stable handles for one generated PBR material set.
///
/// Regeneration replaces the assets behind these handles, so every current
/// consumer updates without respawning entities or changing semantic identity.
#[derive(Debug, Clone)]
pub struct ProceduralPbrMaterial {
    pub material: Handle<StandardMaterial>,
    pub albedo: Handle<Image>,
    pub normal: Handle<Image>,
    pub height: Handle<Image>,
    pub orm: Handle<Image>,
}

/// Built-in generated assets currently used by the game vertical slice.
#[derive(Resource, Debug, Clone)]
pub struct ProceduralAssetLibrary {
    pub cracked_clay: ProceduralPbrMaterial,
    /// High-contrast development grid. Vertex colors encode 3D chunk lineage.
    pub debug_grid: Handle<StandardMaterial>,
}

use systems::{initialize_procedural_assets, regenerate_changed_procedural_assets};

pub struct ProceduralAssetsPlugin;

impl Plugin for ProceduralAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CrackedClayRecipe>()
            .init_resource::<CrackedClayRecipe>()
            .add_systems(PreStartup, initialize_procedural_assets)
            .add_systems(Update, regenerate_changed_procedural_assets);
    }
}

pub(crate) fn rgba_image(size: UVec2, data: Vec<u8>, srgb: bool) -> Image {
    let format = if srgb {
        TextureFormat::Rgba8UnormSrgb
    } else {
        TextureFormat::Rgba8Unorm
    };
    let mut image = Image::new(
        Extent3d {
            width: size.x,
            height: size.y,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        format,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        ..ImageSamplerDescriptor::linear()
    });
    image
}

#[cfg(test)]
mod tests;
