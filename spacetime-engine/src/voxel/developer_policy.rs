//! Developer-only reconstructible celestial presentation policy.
//!
//! developer-celestial-terrain-script-api-v2
//!
//! Canonical celestial terrain remains authoritative and unchanged. This module
//! exposes a narrow, deterministic Rhai domain API used to derive optional
//! PRESENTATION terrain. Scripts own the formula, while the host owns sampling
//! coordinates, seed, semantic scale, bounds, scheduling and publication.

use bevy::{math::DVec3, prelude::Vec3};
use rhai::{Engine, ImmutableString};

use crate::{
    devtools::DeveloperScalarPolicyRuntime,
};

use super::{
    CelestialVoxelField,
    base::script_value_noise_3d,
};

/// Keep live presentation experiments representable by the existing working set.
const MAX_SCRIPTED_RELIEF_METRES: f64 = 100_000.0;
const MAX_SCRIPTED_RELIEF_RADIUS_FRACTION: f64 = 0.10;

const NOISE_MIN_FREQUENCY: f64 = 1.0e-4;
const NOISE_MAX_FREQUENCY: f64 = 1.0e6;
const NOISE_MAX_OCTAVES: i64 = 12;

/// One immutable terrain sample context passed into Rhai.
///
/// This is intentionally data-only. It contains no ECS handle, no entity, no
/// filesystem access and no mutable simulation authority.
#[derive(Debug, Clone)]
pub(crate) struct TerrainScriptContext {
    direction: Vec3,
    canonical_height_metres: f64,
    body_radius_metres: f64,
    seed: u32,
    semantic_scale: i8,
}

impl TerrainScriptContext {
    fn new(
        field: CelestialVoxelField,
        direction: Vec3,
        canonical_height_metres: f64,
    ) -> Self {
        Self {
            direction: normalized_direction(direction),
            canonical_height_metres,
            body_radius_metres: field.radius_metres(),
            seed: field.seed(),
            semantic_scale: field.surface_detail_scale().exponent(),
        }
    }

    pub(crate) fn preview(canonical_height_metres: f64) -> Self {
        Self {
            direction: Vec3::new(0.31, 0.72, -0.61).normalize(),
            canonical_height_metres,
            body_radius_metres: 6_371_000.0,
            seed: 0x4541_5254,
            semantic_scale: 4,
        }
    }
}

/// Stateless deterministic spherical noise handle.
///
/// Despite familiar RNG-like usage (`let noise_rng = ctx.noise("foo")`),
/// sampling is coordinate-stable and order-independent. No mutable random
/// stream exists, so worker scheduling/chunk order cannot alter terrain.
#[derive(Debug, Clone)]
struct TerrainNoiseField {
    direction: Vec3,
    body_radius_metres: f64,
    seed: u32,
}

pub(crate) fn register_rhai_api(engine: &mut Engine) {
    engine
        .register_type_with_name::<TerrainScriptContext>("TerrainContext")
        .register_get("canonical_height", terrain_canonical_height)
        .register_get("radius", terrain_radius)
        .register_get("seed", terrain_seed)
        .register_get("scale", terrain_scale)
        .register_get("x", terrain_x)
        .register_get("y", terrain_y)
        .register_get("z", terrain_z)
        .register_fn("noise", terrain_noise_default)
        .register_fn("noise", terrain_noise_named)
        .register_fn("noise", terrain_noise_numeric)
        .register_type_with_name::<TerrainNoiseField>("NoiseField")
        .register_fn("sample", noise_sample_default)
        .register_fn("sample", noise_sample_frequency)
        .register_fn("sample_wavelength", noise_sample_wavelength)
        .register_fn("fbm", noise_fbm_default)
        .register_fn("fbm", noise_fbm_full)
        .register_fn("ridged", noise_ridged_default)
        .register_fn("ridged", noise_ridged_full);
}

fn terrain_canonical_height(ctx: &mut TerrainScriptContext) -> f64 {
    ctx.canonical_height_metres
}

fn terrain_radius(ctx: &mut TerrainScriptContext) -> f64 {
    ctx.body_radius_metres
}

fn terrain_seed(ctx: &mut TerrainScriptContext) -> i64 {
    i64::from(ctx.seed)
}

fn terrain_scale(ctx: &mut TerrainScriptContext) -> i64 {
    i64::from(ctx.semantic_scale)
}

fn terrain_x(ctx: &mut TerrainScriptContext) -> f64 {
    f64::from(ctx.direction.x)
}

fn terrain_y(ctx: &mut TerrainScriptContext) -> f64 {
    f64::from(ctx.direction.y)
}

fn terrain_z(ctx: &mut TerrainScriptContext) -> f64 {
    f64::from(ctx.direction.z)
}

fn terrain_noise_default(ctx: &mut TerrainScriptContext) -> TerrainNoiseField {
    TerrainNoiseField {
        direction: ctx.direction,
        body_radius_metres: ctx.body_radius_metres,
        seed: mix_seed(ctx.seed, 0x4E4F_4953),
    }
}

fn terrain_noise_named(
    ctx: &mut TerrainScriptContext,
    channel: ImmutableString,
) -> TerrainNoiseField {
    TerrainNoiseField {
        direction: ctx.direction,
        body_radius_metres: ctx.body_radius_metres,
        seed: mix_seed(ctx.seed, hash_channel(channel.as_str())),
    }
}

fn terrain_noise_numeric(
    ctx: &mut TerrainScriptContext,
    channel: i64,
) -> TerrainNoiseField {
    TerrainNoiseField {
        direction: ctx.direction,
        body_radius_metres: ctx.body_radius_metres,
        seed: mix_seed(ctx.seed, channel as u32),
    }
}

fn noise_sample_default(noise: &mut TerrainNoiseField) -> f64 {
    noise_at_frequency(noise, 1.0)
}

fn noise_sample_frequency(noise: &mut TerrainNoiseField, frequency: f64) -> f64 {
    noise_at_frequency(noise, frequency)
}

fn noise_sample_wavelength(noise: &mut TerrainNoiseField, wavelength_metres: f64) -> f64 {
    if !wavelength_metres.is_finite() || wavelength_metres <= 0.0 {
        return 0.0;
    }
    let frequency = noise.body_radius_metres / wavelength_metres;
    noise_at_frequency(noise, frequency)
}

fn noise_fbm_default(
    noise: &mut TerrainNoiseField,
    octaves: i64,
    frequency: f64,
) -> f64 {
    noise_fbm_full(noise, octaves, frequency, 2.0, 0.5)
}

fn noise_fbm_full(
    noise: &mut TerrainNoiseField,
    octaves: i64,
    frequency: f64,
    lacunarity: f64,
    gain: f64,
) -> f64 {
    fractal_sum(noise, octaves, frequency, lacunarity, gain, false)
}

fn noise_ridged_default(
    noise: &mut TerrainNoiseField,
    octaves: i64,
    frequency: f64,
) -> f64 {
    noise_ridged_full(noise, octaves, frequency, 2.0, 0.5)
}

fn noise_ridged_full(
    noise: &mut TerrainNoiseField,
    octaves: i64,
    frequency: f64,
    lacunarity: f64,
    gain: f64,
) -> f64 {
    fractal_sum(noise, octaves, frequency, lacunarity, gain, true)
}

fn fractal_sum(
    noise: &TerrainNoiseField,
    octaves: i64,
    frequency: f64,
    lacunarity: f64,
    gain: f64,
    ridged: bool,
) -> f64 {
    if !frequency.is_finite() || !lacunarity.is_finite() || !gain.is_finite() {
        return 0.0;
    }

    let octaves = octaves.clamp(1, NOISE_MAX_OCTAVES);
    let mut frequency = frequency
        .abs()
        .clamp(NOISE_MIN_FREQUENCY, NOISE_MAX_FREQUENCY);
    let lacunarity = lacunarity.abs().clamp(1.0, 8.0);
    let gain = gain.abs().clamp(0.0, 1.0);

    let mut amplitude = 1.0;
    let mut total = 0.0;
    let mut normalization = 0.0;

    for octave in 0..octaves {
        let octave_noise = TerrainNoiseField {
            direction: noise.direction,
            body_radius_metres: noise.body_radius_metres,
            seed: mix_seed(noise.seed, octave as u32 ^ 0x9E37_79B9),
        };
        let mut value = noise_at_frequency(&octave_noise, frequency);
        if ridged {
            value = 1.0 - value.abs();
            value = value * 2.0 - 1.0;
        }

        total += value * amplitude;
        normalization += amplitude;
        amplitude *= gain;
        frequency = (frequency * lacunarity)
            .clamp(NOISE_MIN_FREQUENCY, NOISE_MAX_FREQUENCY);
    }

    if normalization > f64::EPSILON {
        (total / normalization).clamp(-1.0, 1.0)
    } else {
        0.0
    }
}

fn noise_at_frequency(noise: &TerrainNoiseField, frequency: f64) -> f64 {
    if !frequency.is_finite() {
        return 0.0;
    }
    let frequency = frequency
        .abs()
        .clamp(NOISE_MIN_FREQUENCY, NOISE_MAX_FREQUENCY) as f32;
    f64::from(script_value_noise_3d(
        noise.direction * frequency,
        noise.seed,
    ))
}

fn relief_limit_metres(field: CelestialVoxelField) -> f64 {
    (field.radius_metres() * MAX_SCRIPTED_RELIEF_RADIUS_FRACTION)
        .min(MAX_SCRIPTED_RELIEF_METRES)
        .max(0.0)
}

/// Returns body-local presentation geometry derived from a script-owned formula.
///
/// Script contract:
///
/// ```text
/// fn height(ctx: TerrainContext) -> finite number
/// ```
///
/// Return value is FINAL radial displacement from body radius in SI metres.
/// `ctx.canonical_height` exposes normal semantic terrain for augmentation.
pub(crate) fn presentation_surface_local_metres(
    field: CelestialVoxelField,
    direction: Vec3,
    policy: Option<&DeveloperScalarPolicyRuntime>,
) -> Option<DVec3> {
    let semantic = field.surface_local_metres(direction).ok()?;
    if !semantic.is_finite() {
        return None;
    }

    let semantic_radius = semantic.length();
    if !semantic_radius.is_finite() || semantic_radius <= f64::EPSILON {
        return Some(semantic);
    }

    let canonical_height = semantic_radius - field.radius_metres();
    let scripted_height = policy
        .and_then(|policy| {
            let context =
                TerrainScriptContext::new(field, direction, canonical_height);
            policy.call_f64("height", (context,)).ok()
        })
        .unwrap_or(canonical_height);

    let limit = relief_limit_metres(field);
    let lower = -limit.min(field.radius_metres() * 0.90);
    let upper = limit;
    let height = scripted_height.clamp(lower, upper);
    let radius = (field.radius_metres() + height).max(field.radius_metres() * 0.01);

    let radial_direction = semantic / semantic_radius;
    let transformed = radial_direction * radius;
    transformed.is_finite().then_some(transformed)
}


/// Conservative global radial bounds for script-owned presentation terrain.
///
/// `presentation_surface_local_metres` clamps the final scripted radial height
/// into exactly this envelope. Clipmap planning can therefore reject blocks
/// wholly outside it without evaluating canonical terrain or Rhai at all.
pub(crate) fn presentation_surface_radius_bounds_metres(
    field: CelestialVoxelField,
) -> (f64, f64) {
    let radius = field.radius_metres();
    let relief = relief_limit_metres(field);
    let lower = radius - relief.min(radius * 0.90);
    let upper = radius + relief;
    (lower.max(radius * 0.01), upper.max(radius * 0.01))
}

pub(crate) fn presentation_surface_radius_metres(
    field: CelestialVoxelField,
    direction: Vec3,
    policy: Option<&DeveloperScalarPolicyRuntime>,
) -> Option<f64> {
    presentation_surface_local_metres(field, direction, policy)
        .map(|point| point.length())
}

fn normalized_direction(direction: Vec3) -> Vec3 {
    let direction = direction.normalize_or_zero();
    if direction == Vec3::ZERO {
        Vec3::Y
    } else {
        direction
    }
}

fn hash_channel(channel: &str) -> u32 {
    let mut hash = 0x811C_9DC5_u32;
    for byte in channel.bytes() {
        hash ^= u32::from(byte);
        hash = hash.wrapping_mul(0x0100_0193);
    }
    hash
}

fn mix_seed(mut state: u32, input: u32) -> u32 {
    state ^= input.wrapping_mul(0x85EB_CA6B);
    state ^= state >> 16;
    state = state.wrapping_mul(0x7FEB_352D);
    state ^= state >> 15;
    state
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::voxel::CelestialBodyProfile;

    fn test_field() -> CelestialVoxelField {
        CelestialVoxelField::new(
            6_371_000.0,
            SpatialScale::new(6).unwrap(), SpatialScale::ZERO,
            0x4541_5254,
            CelestialBodyProfile::Rocky,
        )
    }

    #[test]
    fn no_policy_is_exactly_canonical_presentation_surface() {
        let field = test_field();
        let direction = Vec3::new(0.3, 0.8, -0.4).normalize();
        let scale = SpatialScale::new(4).unwrap();

        let canonical = field.surface_local_metres(direction).unwrap();
        let presentation =
            presentation_surface_local_metres(field, direction, None).unwrap();
        assert_eq!(presentation, canonical);
    }

    #[test]
    fn named_noise_channels_are_deterministic_and_distinct() {
        let mut ctx = TerrainScriptContext::preview(0.0);
        let mut first = terrain_noise_named(&mut ctx, "mountains".into());
        let mut again = terrain_noise_named(&mut ctx, "mountains".into());
        let mut other = terrain_noise_named(&mut ctx, "basins".into());

        assert_eq!(
            noise_sample_frequency(&mut first, 8.0).to_bits(),
            noise_sample_frequency(&mut again, 8.0).to_bits(),
        );
        assert_ne!(
            noise_sample_frequency(&mut first, 8.0).to_bits(),
            noise_sample_frequency(&mut other, 8.0).to_bits(),
        );
    }
}
