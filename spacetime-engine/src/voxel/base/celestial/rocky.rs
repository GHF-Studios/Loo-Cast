//! Hierarchical rocky-planet morphology.
//!
//! These are broad semantic terrain bands. Dense voxel sampling, travel
//! boundary queries and later planetary presentation all consume the same
//! accumulated displacement through `ProceduralCelestialBody::surface_position`.
//!
//! The domains are keyed and deterministic. No runtime RNG, materialization
//! address, view state or chunk layout participates in terrain identity.

use bevy::prelude::Vec3;

use crate::spatial::SpatialScale;

use super::bands::PlanetaryTerrainBand;
use super::super::noise::{scale_layer_seed, value_noise_3d};

#[derive(Debug, Clone, Copy)]
enum RockyBandShape {
    Province,
    PlateauBasin,
    OrogenicRift,
    Regional,
}

#[derive(Debug, Clone, Copy)]
struct RockyBand {
    terrain: PlanetaryTerrainBand,
    shape: RockyBandShape,
}

fn scale(raw: i8) -> SpatialScale {
    SpatialScale::new(raw).expect("rocky terrain band uses a valid ordinary Scale")
}

/// Explicit semantic morphology stack.
///
/// Approximate Earth-class characteristic spans:
/// - S+6 province field: several thousand kilometres;
/// - S+5 plateau/basin field: ~1,000-3,000 km;
/// - S+5 orogenic/rift belts: hundreds to low-thousands km;
/// - S+4 regional residual: several hundred km.
///
/// Exact shape comes from keyed spherical domains, not these comments.
fn rocky_bands() -> [RockyBand; 4] {
    [
        RockyBand {
            terrain: PlanetaryTerrainBand::new(
                scale(6),
                4_200.0,
                1.35,
                0x5052_4F56, // PROV
            ),
            shape: RockyBandShape::Province,
        },
        RockyBand {
            terrain: PlanetaryTerrainBand::new(
                scale(5),
                3_200.0,
                2.60,
                0x504C_4154, // PLAT
            ),
            shape: RockyBandShape::PlateauBasin,
        },
        RockyBand {
            terrain: PlanetaryTerrainBand::new(
                scale(5),
                6_200.0,
                4.20,
                0x4F52_4F47, // OROG
            ),
            shape: RockyBandShape::OrogenicRift,
        },
        RockyBand {
            terrain: PlanetaryTerrainBand::new(
                scale(4),
                1_400.0,
                8.00,
                0x5245_474E, // REGN
            ),
            shape: RockyBandShape::Regional,
        },
    ]
}

/// Rocky terrain displacement accumulated through one semantic Scale.
///
/// Finer requests include every already-owned coarse band and add only bands
/// introduced at the newly crossed semantic Scale.
pub(super) fn rocky_surface_displacement_metres(
    direction: Vec3,
    through_scale: SpatialScale,
    seed: u32,
) -> f64 {
    let direction = normalized_direction(direction);

    rocky_bands()
        .into_iter()
        .filter(|band| band.terrain.contributes_through(through_scale))
        .map(|band| {
            sample_band(direction, band, seed) * band.terrain.amplitude_metres()
        })
        .sum()
}

/// Conservative outward bound for the broad rocky morphology stack.
///
/// Each normalized band sampler is bounded to [-1, 1], so summing absolute
/// amplitudes is intentionally conservative without depending on sampled luck.
pub(super) fn rocky_maximum_outward_displacement_metres() -> f64 {
    rocky_bands()
        .into_iter()
        .map(|band| band.terrain.amplitude_metres())
        .sum()
}

fn sample_band(direction: Vec3, band: RockyBand, seed: u32) -> f64 {
    let terrain = band.terrain;
    let keyed_seed = scale_layer_seed(
        seed ^ terrain.seed_salt(),
        terrain.introduced_at(),
    );
    let frequency = terrain.angular_frequency();

    let value = match band.shape {
        RockyBandShape::Province => {
            value_noise_3d(
                direction * frequency + Vec3::new(17.1, -9.4, 4.7),
                keyed_seed ^ 0x9E37_79B9,
            )
        }
        RockyBandShape::PlateauBasin => {
            let n = value_noise_3d(
                direction * frequency + Vec3::new(-12.7, 21.3, 8.6),
                keyed_seed ^ 0xC2B2_AE35,
            );
            // Broad low-gradient interiors with steeper province boundaries.
            n * (0.35 + 0.65 * n.abs())
        }
        RockyBandShape::OrogenicRift => {
            let carrier = value_noise_3d(
                direction * frequency + Vec3::new(5.9, 13.4, -18.1),
                keyed_seed ^ 0x27D4_EB2D,
            );
            // Zero-contours of the broad carrier form long coherent belts.
            let ridge = (1.0 - carrier.abs()).max(0.0).powi(3);
            let phase =
                keyed_seed as f32 / u32::MAX as f32 * std::f32::consts::TAU;
            let polarity = (
                direction.dot(Vec3::new(0.73, -0.31, 0.61).normalize())
                    * 5.3
                    + phase
            )
                .sin();
            (ridge * polarity * 1.35).clamp(-1.0, 1.0)
        }
        RockyBandShape::Regional => {
            value_noise_3d(
                direction * frequency + Vec3::new(-7.8, -3.1, 11.6),
                keyed_seed ^ 0xD3A2_646C,
            )
        }
    };

    f64::from(value.clamp(-1.0, 1.0))
}

fn normalized_direction(direction: Vec3) -> Vec3 {
    let direction = direction.normalize_or_zero();
    if direction == Vec3::ZERO {
        Vec3::Y
    } else {
        direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EARTH_RADIUS_METRES: f64 = 6_371_000.0;
    const EARTH_SEED: u32 = 0x4541_5254;

    fn great_circle_direction(angle: f32) -> Vec3 {
        Vec3::new(angle.cos(), 0.0, angle.sin())
    }

    fn fibonacci_direction(index: usize, count: usize) -> Vec3 {
        let i = index as f32 + 0.5;
        let n = count as f32;
        let y = 1.0 - 2.0 * i / n;
        let radius = (1.0 - y * y).max(0.0).sqrt();
        let golden_ratio = (1.0 + 5.0_f32.sqrt()) * 0.5;
        let theta = std::f32::consts::TAU * index as f32 / golden_ratio;
        Vec3::new(theta.cos() * radius, y, theta.sin() * radius)
            .normalize()
    }

    #[test]
    fn rocky_bands_are_deterministic_and_additive_across_scales() {
        let direction = Vec3::new(0.31, 0.72, -0.61).normalize();

        let s6 = rocky_surface_displacement_metres(direction, scale(6), EARTH_SEED);
        let s5 = rocky_surface_displacement_metres(direction, scale(5), EARTH_SEED);
        let s4 = rocky_surface_displacement_metres(direction, scale(4), EARTH_SEED);

        assert_eq!(
            s4.to_bits(),
            rocky_surface_displacement_metres(direction, scale(4), EARTH_SEED)
                .to_bits()
        );

        let bands = rocky_bands();
        let s5_residual: f64 = bands
            .into_iter()
            .filter(|band| band.terrain.introduced_at() == scale(5))
            .map(|band| sample_band(direction, band, EARTH_SEED)
                * band.terrain.amplitude_metres())
            .sum();
        let s4_residual: f64 = rocky_bands()
            .into_iter()
            .filter(|band| band.terrain.introduced_at() == scale(4))
            .map(|band| sample_band(direction, band, EARTH_SEED)
                * band.terrain.amplitude_metres())
            .sum();

        assert!(((s5 - s6) - s5_residual).abs() < 1.0e-9);
        assert!(((s4 - s5) - s4_residual).abs() < 1.0e-9);
    }

    #[test]
    fn earth_class_rocky_relief_has_multi_kilometre_positive_and_negative_range() {
        let mut minimum = f64::INFINITY;
        let mut maximum = f64::NEG_INFINITY;

        for index in 0..2_048 {
            let direction = fibonacci_direction(index, 2_048);
            let displacement =
                rocky_surface_displacement_metres(direction, scale(4), EARTH_SEED);
            minimum = minimum.min(displacement);
            maximum = maximum.max(displacement);
        }

        assert!(
            maximum >= 5_000.0,
            "expected >=5 km positive rocky relief class, got {maximum:.1} m"
        );
        assert!(
            minimum <= -5_000.0,
            "expected >=5 km negative rocky relief class, got {minimum:.1} m"
        );
    }

    #[test]
    fn broad_rocky_morphology_is_coherent_over_hundreds_of_kilometres() {
        let near_angle = (250_000.0 / EARTH_RADIUS_METRES) as f32;
        let far_angle = (2_500_000.0 / EARTH_RADIUS_METRES) as f32;

        let mut near_delta = 0.0;
        let mut far_delta = 0.0;
        let samples = 96;

        for index in 0..samples {
            let angle = std::f32::consts::TAU * index as f32 / samples as f32;
            let base = rocky_surface_displacement_metres(
                great_circle_direction(angle),
                scale(4),
                EARTH_SEED,
            );
            near_delta += (
                rocky_surface_displacement_metres(
                    great_circle_direction(angle + near_angle),
                    scale(4),
                    EARTH_SEED,
                ) - base
            )
                .abs();
            far_delta += (
                rocky_surface_displacement_metres(
                    great_circle_direction(angle + far_angle),
                    scale(4),
                    EARTH_SEED,
                ) - base
            )
                .abs();
        }

        let near_mean = near_delta / samples as f64;
        let far_mean = far_delta / samples as f64;
        assert!(
            near_mean < far_mean * 0.5,
            "250 km mean relief delta ({near_mean:.1} m) should be much smaller \
             than 2500 km delta ({far_mean:.1} m)"
        );
    }
}
