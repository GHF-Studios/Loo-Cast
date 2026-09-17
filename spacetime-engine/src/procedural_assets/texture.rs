use bevy::prelude::*;

use super::rgba_image;

/// Editable recipe for the first procedural material vertical slice.
///
/// Two independent periodic Voronoi fields create large clay plates and a finer
/// secondary fracture network. The same scalar surface drives color, height,
/// tangent-space normals, roughness and ambient occlusion so the outputs remain
/// internally coherent instead of being unrelated noise textures.
#[derive(Resource, Reflect, Debug, Clone, PartialEq)]
#[reflect(Resource)]
pub struct CrackedClayRecipe {
    pub resolution: UVec2,
    pub seed: u32,
    pub macro_cells: u32,
    pub micro_cells: u32,
    pub macro_crack_width: f32,
    pub micro_crack_width: f32,
    pub macro_crack_depth: f32,
    pub micro_crack_depth: f32,
    pub normal_strength: f32,
    pub dark_clay_srgb: Vec3,
    pub light_clay_srgb: Vec3,
    pub crack_srgb: Vec3,
}

impl Default for CrackedClayRecipe {
    fn default() -> Self {
        Self {
            resolution: UVec2::splat(256),
            seed: 0xC1A4_C1A4,
            macro_cells: 7,
            micro_cells: 23,
            macro_crack_width: 0.10,
            micro_crack_width: 0.055,
            macro_crack_depth: 0.52,
            micro_crack_depth: 0.17,
            normal_strength: 0.11,
            dark_clay_srgb: Vec3::new(0.34, 0.12, 0.055),
            light_clay_srgb: Vec3::new(0.72, 0.34, 0.16),
            crack_srgb: Vec3::new(0.075, 0.028, 0.018),
        }
    }
}

pub(crate) struct GeneratedPbrTextures {
    pub(crate) albedo: Image,
    pub(crate) normal: Image,
    pub(crate) height: Image,
    pub(crate) orm: Image,
}

#[derive(Debug, Clone, Copy)]
struct SurfaceSample {
    height: f32,
    crack: f32,
    color_srgb: Vec3,
    roughness: f32,
    occlusion: f32,
}

#[derive(Debug, Clone, Copy)]
struct VoronoiSample {
    nearest: f32,
    edge: f32,
    cell_value: f32,
}

pub(crate) fn generate_cracked_clay(recipe: &CrackedClayRecipe) -> GeneratedPbrTextures {
    let size = recipe.resolution.max(UVec2::ONE);
    let pixel_count = (size.x as usize) * (size.y as usize);
    let mut albedo = Vec::with_capacity(pixel_count * 4);
    let mut normal = Vec::with_capacity(pixel_count * 4);
    let mut height = Vec::with_capacity(pixel_count * 4);
    let mut orm = Vec::with_capacity(pixel_count * 4);

    let texel = Vec2::new(1.0 / size.x as f32, 1.0 / size.y as f32);

    for y in 0..size.y {
        for x in 0..size.x {
            let uv = Vec2::new((x as f32 + 0.5) * texel.x, (y as f32 + 0.5) * texel.y);
            let sample = sample_surface(recipe, uv);
            let left = sample_surface(recipe, uv - Vec2::new(texel.x, 0.0)).height;
            let right = sample_surface(recipe, uv + Vec2::new(texel.x, 0.0)).height;
            let up = sample_surface(recipe, uv - Vec2::new(0.0, texel.y)).height;
            let down = sample_surface(recipe, uv + Vec2::new(0.0, texel.y)).height;
            let dhdu = (right - left) / (2.0 * texel.x);
            let dhdv = (down - up) / (2.0 * texel.y);
            let n = Vec3::new(
                -dhdu * recipe.normal_strength.max(0.0),
                -dhdv * recipe.normal_strength.max(0.0),
                1.0,
            )
            .normalize_or_zero();

            push_rgba(&mut albedo, sample.color_srgb, 1.0);
            push_rgba(&mut normal, n * 0.5 + Vec3::splat(0.5), 1.0);
            push_gray(&mut height, sample.height);
            push_rgba(
                &mut orm,
                Vec3::new(sample.occlusion, sample.roughness, 0.0),
                1.0,
            );
        }
    }

    GeneratedPbrTextures {
        albedo: rgba_image(size, albedo, true),
        normal: rgba_image(size, normal, false),
        height: rgba_image(size, height, false),
        orm: rgba_image(size, orm, false),
    }
}

fn sample_surface(recipe: &CrackedClayRecipe, uv: Vec2) -> SurfaceSample {
    let uv = Vec2::new(uv.x.rem_euclid(1.0), uv.y.rem_euclid(1.0));
    let macro_v = periodic_voronoi(uv, recipe.macro_cells.max(1), recipe.seed);
    let micro_v = periodic_voronoi(uv, recipe.micro_cells.max(1), recipe.seed ^ 0x9E37_79B9);

    let macro_crack = crack_mask(macro_v.edge, recipe.macro_crack_width);
    let micro_crack = crack_mask(micro_v.edge, recipe.micro_crack_width);
    let crack = (macro_crack + micro_crack * 0.55).clamp(0.0, 1.0);

    // Large cells are slightly domed while the two fracture networks cut into
    // the same scalar surface. This scalar is retained as a height output and
    // differentiated for the normal map below.
    let dome = (1.0 - macro_v.nearest / 0.9).clamp(0.0, 1.0);
    let micro_relief = (1.0 - micro_v.nearest / 0.8).clamp(0.0, 1.0);
    let height = (0.60 + dome * 0.12 + micro_relief * 0.025
        - macro_crack * recipe.macro_crack_depth.max(0.0)
        - micro_crack * recipe.micro_crack_depth.max(0.0))
    .clamp(0.0, 1.0);

    let plate_tone = (0.22 + macro_v.cell_value * 0.58 + micro_v.cell_value * 0.20).clamp(0.0, 1.0);
    let clay = recipe
        .dark_clay_srgb
        .lerp(recipe.light_clay_srgb, plate_tone);
    let crack_mix = (0.25 + crack * 0.72).clamp(0.0, 1.0);
    let color_srgb = clay
        .lerp(recipe.crack_srgb, crack * crack_mix)
        .clamp(Vec3::ZERO, Vec3::ONE);
    let roughness = (0.72 + crack * 0.27 + (1.0 - height) * 0.08).clamp(0.0, 1.0);
    let occlusion = (1.0 - crack * 0.55 - (1.0 - height) * 0.10).clamp(0.0, 1.0);

    SurfaceSample {
        height,
        crack,
        color_srgb,
        roughness,
        occlusion,
    }
}

fn periodic_voronoi(uv: Vec2, cells: u32, seed: u32) -> VoronoiSample {
    let cells_i = cells as i32;
    let p = uv * cells as f32;
    let base = p.floor().as_ivec2();
    let mut nearest = f32::INFINITY;
    let mut second = f32::INFINITY;
    let mut nearest_cell = IVec2::ZERO;

    for oy in -1..=1 {
        for ox in -1..=1 {
            let cell = base + IVec2::new(ox, oy);
            let wrapped = IVec2::new(cell.x.rem_euclid(cells_i), cell.y.rem_euclid(cells_i));
            let feature = cell.as_vec2() + feature_point(wrapped, seed);
            let distance = p.distance(feature);

            if distance < nearest {
                second = nearest;
                nearest = distance;
                nearest_cell = wrapped;
            } else if distance < second {
                second = distance;
            }
        }
    }

    VoronoiSample {
        nearest,
        edge: (second - nearest).max(0.0),
        cell_value: hash01(hash_cell(nearest_cell, seed ^ 0xA511_E9B3)),
    }
}

fn feature_point(cell: IVec2, seed: u32) -> Vec2 {
    let h = hash_cell(cell, seed);
    Vec2::new(
        hash01(hash32(h ^ 0x68BC_21EB)),
        hash01(hash32(h ^ 0x02E5_BE93)),
    )
}

fn hash_cell(cell: IVec2, seed: u32) -> u32 {
    let x = cell.x as u32;
    let y = cell.y as u32;
    hash32(seed ^ x.wrapping_mul(0x9E37_79B1) ^ y.wrapping_mul(0x85EB_CA77))
}

fn hash32(mut x: u32) -> u32 {
    x ^= x >> 16;
    x = x.wrapping_mul(0x7FEB_352D);
    x ^= x >> 15;
    x = x.wrapping_mul(0x846C_A68B);
    x ^ (x >> 16)
}

fn hash01(value: u32) -> f32 {
    (value as f64 / u32::MAX as f64) as f32
}

fn crack_mask(edge_distance: f32, width: f32) -> f32 {
    let width = width.max(1.0e-5);
    1.0 - smoothstep(width * 0.20, width, edge_distance)
}

fn smoothstep(a: f32, b: f32, x: f32) -> f32 {
    let t = ((x - a) / (b - a).max(f32::EPSILON)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn push_gray(buffer: &mut Vec<u8>, value: f32) {
    push_rgba(buffer, Vec3::splat(value), 1.0);
}

fn push_rgba(buffer: &mut Vec<u8>, rgb: Vec3, alpha: f32) {
    for value in [rgb.x, rgb.y, rgb.z, alpha] {
        buffer.push((value.clamp(0.0, 1.0) * 255.0).round() as u8);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cracked_clay_generation_is_deterministic() {
        let recipe = CrackedClayRecipe {
            resolution: UVec2::splat(24),
            ..default()
        };
        let a = generate_cracked_clay(&recipe);
        let b = generate_cracked_clay(&recipe);

        assert_eq!(a.albedo.data, b.albedo.data);
        assert_eq!(a.normal.data, b.normal.data);
        assert_eq!(a.height.data, b.height.data);
        assert_eq!(a.orm.data, b.orm.data);
    }

    #[test]
    fn surface_recipe_is_periodic_in_both_axes() {
        let recipe = CrackedClayRecipe::default();
        let uv = Vec2::new(0.173, 0.681);
        let base = sample_surface(&recipe, uv);
        let x = sample_surface(&recipe, uv + Vec2::X);
        let y = sample_surface(&recipe, uv + Vec2::Y);

        for other in [x, y] {
            assert!((base.height - other.height).abs() < 1.0e-5);
            assert!((base.crack - other.crack).abs() < 1.0e-5);
            assert!((base.color_srgb - other.color_srgb).length() < 1.0e-5);
        }
    }

    #[test]
    fn changing_seed_changes_generated_surface() {
        let a = CrackedClayRecipe::default();
        let mut b = a.clone();
        b.seed = b.seed.wrapping_add(1);
        let uv = Vec2::new(0.37, 0.42);

        let a = sample_surface(&a, uv);
        let b = sample_surface(&b, uv);
        assert!(
            (a.height - b.height).abs() > 1.0e-5 || (a.color_srgb - b.color_srgb).length() > 1.0e-5
        );
    }
}
