//! Runtime-generated presentation assets.
//!
//! Procedural assets are recipes and disposable generated representations. They
//! deliberately do not own semantic world state. A world/Phenomenon may choose
//! recipe parameters or seeds later, but texture/model/audio generation remains
//! a separate replaceable layer.

mod texture;

pub use texture::CrackedClayRecipe;

use bevy::{
    asset::RenderAssetUsages,
    image::{ImageAddressMode, ImageSampler, ImageSamplerDescriptor},
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

use texture::{GeneratedPbrTextures, generate_cracked_clay};

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
}

pub struct ProceduralAssetsPlugin;

impl Plugin for ProceduralAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CrackedClayRecipe>()
            .init_resource::<CrackedClayRecipe>()
            .add_systems(PreStartup, initialize_procedural_assets)
            .add_systems(Update, regenerate_changed_procedural_assets);
    }
}

fn initialize_procedural_assets(
    mut commands: Commands,
    recipe: Res<CrackedClayRecipe>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let generated = generate_cracked_clay(&recipe);
    let albedo = images.add(generated.albedo);
    let normal = images.add(generated.normal);
    let height = images.add(generated.height);
    let orm = images.add(generated.orm);

    let material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        base_color_texture: Some(albedo.clone()),
        normal_map_texture: Some(normal.clone()),
        // R = ambient occlusion, G = roughness, B = metallic.
        metallic_roughness_texture: Some(orm.clone()),
        occlusion_texture: Some(orm.clone()),
        perceptual_roughness: 1.0,
        metallic: 1.0,
        ..default()
    });

    commands.insert_resource(ProceduralAssetLibrary {
        cracked_clay: ProceduralPbrMaterial {
            material,
            albedo,
            normal,
            height,
            orm,
        },
    });
}

/// Regenerates changed recipes in-place.
///
/// This is the first deliberately small piece of the continuous procedural-asset
/// path: recipe mutation changes already-instantiated users because their asset
/// handles stay stable. Async scheduling/cache policy can grow around this seam
/// without changing the recipe/output contract.
fn regenerate_changed_procedural_assets(
    recipe: Res<CrackedClayRecipe>,
    library: Option<Res<ProceduralAssetLibrary>>,
    mut images: ResMut<Assets<Image>>,
) {
    if !recipe.is_changed() {
        return;
    }
    let Some(library) = library else {
        return;
    };

    let generated = generate_cracked_clay(&recipe);
    replace_image(&mut images, &library.cracked_clay.albedo, generated.albedo);
    replace_image(&mut images, &library.cracked_clay.normal, generated.normal);
    replace_image(&mut images, &library.cracked_clay.height, generated.height);
    replace_image(&mut images, &library.cracked_clay.orm, generated.orm);
}

fn replace_image(images: &mut Assets<Image>, handle: &Handle<Image>, replacement: Image) {
    let Some(mut image) = images.get_mut(handle) else {
        warn!("procedural image handle disappeared before regeneration");
        return;
    };
    *image = replacement;
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
mod tests {
    use super::*;

    #[test]
    fn regenerated_texture_set_keeps_expected_pixel_storage() {
        let recipe = CrackedClayRecipe {
            resolution: UVec2::new(32, 24),
            ..default()
        };
        let generated = generate_cracked_clay(&recipe);
        let expected = (32 * 24 * 4) as usize;

        for image in [
            &generated.albedo,
            &generated.normal,
            &generated.height,
            &generated.orm,
        ] {
            assert_eq!(image.data.as_ref().unwrap().len(), expected);
        }
    }
}
