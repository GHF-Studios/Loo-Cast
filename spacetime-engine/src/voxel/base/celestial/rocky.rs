//! Hierarchical rocky-planet morphology.
//!
//! These are broad semantic terrain bands. Dense voxel sampling, travel
//! boundary queries and later planetary presentation all consume the same
//! accumulated displacement through `CelestialFieldRealization::surface_position`.
//!
//! The domains are keyed and deterministic. No runtime RNG, materialization
//! address, view state or chunk layout participates in terrain identity.

use bevy::prelude::Vec3;

use crate::spatial::SpatialScale;

use super::super::noise::{scale_layer_seed, value_noise_3d};
use super::bands::PlanetaryTerrainBand;

#[derive(Debug, Clone, Copy)]
enum RockyBandShape {
    Province,
    PlateauBasin,
    OrogenicRift,
    Regional,
    AlpineRidge,
    CanyonNetwork,
    FineRidge,
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
fn rocky_bands() -> [RockyBand; 7] {
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
        RockyBand {
            terrain: PlanetaryTerrainBand::new(
                scale(3),
                11_000.0,
                15.0,
                0x414C_504E, // ALPN
            ),
            shape: RockyBandShape::AlpineRidge,
        },
        RockyBand {
            terrain: PlanetaryTerrainBand::new(
                scale(3),
                8_000.0,
                21.0,
                0x4341_4E59, // CANY
            ),
            shape: RockyBandShape::CanyonNetwork,
        },
        RockyBand {
            terrain: PlanetaryTerrainBand::new(
                scale(2),
                2_600.0,
                42.0,
                0x5249_4447, // RIDG
            ),
            shape: RockyBandShape::FineRidge,
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
        .map(|band| sample_band(direction, band, seed) * band.terrain.amplitude_metres())
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
    let keyed_seed = scale_layer_seed(seed ^ terrain.seed_salt(), terrain.introduced_at());
    let frequency = terrain.angular_frequency();

    let value = match band.shape {
        RockyBandShape::Province => value_noise_3d(
            direction * frequency + Vec3::new(17.1, -9.4, 4.7),
            keyed_seed ^ 0x9E37_79B9,
        ),
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
            let phase = keyed_seed as f32 / u32::MAX as f32 * std::f32::consts::TAU;
            let polarity =
                (direction.dot(Vec3::new(0.73, -0.31, 0.61).normalize()) * 5.3 + phase).sin();
            (ridge * polarity * 1.35).clamp(-1.0, 1.0)
        }
        RockyBandShape::Regional => value_noise_3d(
            direction * frequency + Vec3::new(-7.8, -3.1, 11.6),
            keyed_seed ^ 0xD3A2_646C,
        ),
        RockyBandShape::AlpineRidge => {
            let carrier = value_noise_3d(
                direction * frequency + Vec3::new(19.7, -4.3, 12.8),
                keyed_seed ^ 0xA24B_AED4,
            );
            let ridge = (1.0 - carrier.abs()).max(0.0).powi(6);
            let envelope = value_noise_3d(
                direction * (frequency * 0.37) + Vec3::new(-8.1, 15.6, 2.9),
                keyed_seed ^ 0x9FB2_1C65,
            );
            let envelope01 = (f64::from(envelope) * 0.5 + 0.5).clamp(0.0, 1.0);
            (f64::from(ridge) * (0.20 + envelope01 * 0.80)).clamp(0.0, 1.0) as f32
        }
        RockyBandShape::CanyonNetwork => {
            let carrier = value_noise_3d(
                direction * frequency + Vec3::new(-14.2, 6.7, 22.1),
                keyed_seed ^ 0x1656_67B1,
            );
            let line = (1.0 - carrier.abs()).max(0.0).powi(7);
            let envelope = value_noise_3d(
                direction * (frequency * 0.29) + Vec3::new(3.4, -18.9, 7.2),
                keyed_seed ^ 0xD4EB_2F6A,
            );
            let envelope01 = (f64::from(envelope) * 0.5 + 0.5).clamp(0.0, 1.0);
            -(f64::from(line) * (0.25 + envelope01 * 0.75)).clamp(0.0, 1.0) as f32
        }
        RockyBandShape::FineRidge => {
            let carrier = value_noise_3d(
                direction * frequency + Vec3::new(4.8, 9.2, -16.7),
                keyed_seed ^ 0x85EB_CA77,
            );
            let ridge = (1.0 - carrier.abs()).max(0.0).powi(4);
            let polarity = value_noise_3d(
                direction * (frequency * 0.41) + Vec3::new(-11.5, 1.8, 5.6),
                keyed_seed ^ 0xC2B2_AE3D,
            );
            (ridge * polarity).clamp(-1.0, 1.0)
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

mod development;
use development::{
    ROCKY_EXAGGERATED_INWARD_BOUND_METRES, ROCKY_EXAGGERATED_OUTWARD_BOUND_METRES,
    rocky_exaggerated_relief_metres_through,
};

/// Complete authored rocky macro relief, including the deliberately prominent
/// development morphology. Both layers participate in canonical field truth.
pub(super) fn rocky_macro_displacement_metres(
    direction: Vec3,
    through_scale: SpatialScale,
    seed: u32,
) -> f64 {
    rocky_surface_displacement_metres(direction, through_scale, seed)
        + rocky_exaggerated_relief_metres_through(direction, seed, through_scale)
}

/// Conservative inward and outward bounds for that same recipe.
pub(super) fn rocky_macro_relief_bounds_metres() -> (f64, f64) {
    let authored = rocky_maximum_outward_displacement_metres();
    (
        authored + ROCKY_EXAGGERATED_INWARD_BOUND_METRES,
        authored + ROCKY_EXAGGERATED_OUTWARD_BOUND_METRES,
    )
}
