//! Deliberately pronounced rocky morphology for the current authored world.
//!
//! This is part of the semantic field, never a presentation-only displacement.

use super::{normalized_direction, value_noise_3d};
use crate::spatial::SpatialScale;
use bevy::prelude::Vec3;

pub(super) const ROCKY_EXAGGERATED_OUTWARD_BOUND_METRES: f64 = 55_000.0;
pub(super) const ROCKY_EXAGGERATED_INWARD_BOUND_METRES: f64 = 38_000.0;

/// Deliberately unmistakable development morphology layered onto the ordinary
/// rocky semantic bands.
///
/// This is canonical physical terrain, not presentation displacement. Every
/// consumer of `semantic_surface_radius_metres()` therefore sees the same
/// mountains/valleys: dense voxels, travel boundaries, clipmap and regional
/// presentation.
///
/// The amplitudes are intentionally obvious while the terrain pipeline is
/// being exercised. This authored recipe can change without changing the
/// scale-local realization machinery.
pub(super) fn rocky_exaggerated_relief_metres_through(
    direction: Vec3,
    seed: u32,
    through_scale: SpatialScale,
) -> f64 {
    let direction = normalized_direction(direction);

    let province = value_noise_3d(
        direction * 2.4 + Vec3::new(7.3, -11.8, 4.1),
        seed ^ 0x5052_4F56,
    );
    let mut relief = f64::from(province) * 8_000.0;

    if through_scale <= SpatialScale::new(3).expect("S+3 is valid") {
        relief += alpine_canyon_relief_metres(direction, seed);
    }

    if through_scale <= SpatialScale::new(2).expect("S+2 is valid") {
        relief += serration_relief_metres(direction, seed);
    }

    relief.clamp(-38_000.0, 48_000.0)
}

/// One regional feature group introduced at S+3.
fn alpine_canyon_relief_metres(direction: Vec3, seed: u32) -> f64 {
    let alpine_carrier = value_noise_3d(
        direction * 10.0 + Vec3::new(-17.2, 6.9, 12.4),
        seed ^ 0x414C_504E,
    );
    let alpine_ridge = (1.0 - alpine_carrier.abs()).max(0.0).powi(9);
    let alpine_envelope = (value_noise_3d(
        direction * 3.7 + Vec3::new(3.1, 19.6, -8.8),
        seed ^ 0xA1F1_4E55,
    ) * 0.5
        + 0.5)
        .clamp(0.0, 1.0);

    let canyon_carrier = value_noise_3d(
        direction * 15.0 + Vec3::new(14.2, -4.7, -16.5),
        seed ^ 0x4341_4E59,
    );
    let canyon_line = (1.0 - canyon_carrier.abs()).max(0.0).powi(9);
    let canyon_envelope = (value_noise_3d(
        direction * 4.3 + Vec3::new(-9.9, 5.4, 21.1),
        seed ^ 0x5249_4654,
    ) * 0.5
        + 0.5)
        .clamp(0.0, 1.0);

    let massif_carrier = value_noise_3d(
        direction * 14.5 + Vec3::new(22.4, 7.7, -3.6),
        seed ^ 0x4D41_5353,
    );
    let massif_cross = (1.0 - massif_carrier.abs()).max(0.0).powi(8);
    let massif = alpine_ridge * massif_cross;

    f64::from(alpine_ridge * alpine_envelope) * 30_000.0
        + f64::from(massif * alpine_envelope) * 16_000.0
        - f64::from(canyon_line * canyon_envelope) * 24_000.0
}

/// Fine serration introduced at S+2.
fn serration_relief_metres(direction: Vec3, seed: u32) -> f64 {
    let serration = value_noise_3d(
        direction * 32.0 + Vec3::new(1.7, -13.3, 9.2),
        seed ^ 0x5345_5252,
    );
    let sharp_serration = serration.signum() * serration.abs().powi(2);
    f64::from(sharp_serration) * 5_000.0
}
