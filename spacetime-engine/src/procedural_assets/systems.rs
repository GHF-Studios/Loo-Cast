//! Initialization and in-place regeneration of procedural presentation assets.

use bevy::{color::LinearRgba, prelude::*};

use super::{ProceduralAssetLibrary, ProceduralPbrMaterial};
use super::texture::{
    CrackedClayRecipe, generate_cracked_clay, generate_planet_surface_image,
    generate_star_surface_image,
};

pub(super) fn initialize_procedural_assets(
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

    let star_surface_texture = images.add(generate_star_surface_image());
    let star_surface = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        base_color_texture: Some(star_surface_texture),
        emissive: LinearRgba::rgb(8.0, 2.8, 0.55),
        emissive_exposure_weight: 0.0,
        perceptual_roughness: 0.9,
        metallic: 0.0,
        ..default()
    });

    let planet_surface_texture = images.add(generate_planet_surface_image());
    let planet_surface = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        base_color_texture: Some(planet_surface_texture),
        perceptual_roughness: 0.96,
        metallic: 0.0,
        ..default()
    });

    let debug_grid_texture = images.add(super::debug_grid::generate_debug_grid_image());
    let debug_grid = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        base_color_texture: Some(debug_grid_texture),
        perceptual_roughness: 0.92,
        metallic: 0.0,
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
        debug_grid,
        star_surface,
        planet_surface,
    });
}

/// Regenerates changed recipes in-place.
///
/// Recipe mutation updates already-instantiated users in place because asset
/// handles remain stable. Scheduling/cache policy can evolve independently of
/// the recipe/output contract.
pub(super) fn regenerate_changed_procedural_assets(
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
