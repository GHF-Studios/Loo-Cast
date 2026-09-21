//! Spherical procedural voxel baselines for celestial bodies.
//!
//! This is deliberately a `VoxelBase`: streaming, dense materialization,
//! Surface Nets, rendering, collision and edits remain the ordinary voxel
//! pipeline. The only body-specific concern here is the reconstructible scalar
//! field sampled by that pipeline.

use bevy::prelude::Vec3;

use crate::spatial::SpatialScale;

use super::{
    EMPTY_DISTANCE,
    noise::{scale_layer_seed, value_noise_3d},
};
use super::super::{VoxelMaterialId, VoxelQueryPosition, VoxelSample};

const LOCAL_SAMPLE_MARGIN_NATIVE: f32 = 8192.0;

/// Reconstructible spherical rocky-body field.
///
/// `current_scale` is the native chart of the owning voxel world. Coarser
/// terrain bands are reproduced in every finer realization and each newly
/// entered scale adds only another deterministic detail band.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProceduralCelestialBody {
    radius_native: f32,
    current_scale: SpatialScale,
    coarsest_detail_scale: SpatialScale,
    seed: u32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct PreparedProceduralCelestialBody {
    body: ProceduralCelestialBody,
    chunk_origin_from_center: Vec3,
}

impl PreparedProceduralCelestialBody {
    #[inline]
    pub(crate) fn sample(self, chunk_local: Vec3) -> VoxelSample {
        self.body
            .sample_local(self.chunk_origin_from_center + chunk_local)
    }
}

impl ProceduralCelestialBody {
    /// First lunar profile used by the Moon vertical slice.
    ///
    /// Scale +5 is intentionally the first voxelized scale: one native unit is
    /// 100 km there, giving the 1737-km Moon a useful ~35-cell diameter while
    /// leaving farther views to the cheap whole-body proxy.
    pub fn lunar(radius_scale0: f64, current_scale: SpatialScale, seed: u32) -> Self {
        let coarsest_detail_scale =
            SpatialScale::new(5).expect("Scale +5 is a valid lunar refinement scale");
        assert!(
            current_scale <= coarsest_detail_scale,
            "lunar voxel realization is only intended for S+5 and finer"
        );

        let radius_native = current_scale.scale0_to_native_f64(radius_scale0) as f32;
        assert!(
            radius_native.is_finite() && radius_native > 0.0,
            "lunar radius must remain finite and positive in the realization chart"
        );

        Self {
            radius_native,
            current_scale,
            coarsest_detail_scale,
            seed,
        }
    }

    pub const fn radius_native(self) -> f32 {
        self.radius_native
    }

    pub(crate) fn prepare_local_sampler(
        self,
        world_origin: VoxelQueryPosition,
        chunk_origin: VoxelQueryPosition,
    ) -> Option<PreparedProceduralCelestialBody> {
        let bound = self.radius_native + LOCAL_SAMPLE_MARGIN_NATIVE;
        let chunk_origin_from_center = chunk_origin.relative_to(world_origin, bound).ok()?;
        Some(PreparedProceduralCelestialBody {
            body: self,
            chunk_origin_from_center,
        })
    }

    pub(crate) fn sample_at(
        self,
        world_origin: VoxelQueryPosition,
        point: VoxelQueryPosition,
    ) -> VoxelSample {
        let bound = self.radius_native + LOCAL_SAMPLE_MARGIN_NATIVE;
        let Ok(local) = point.relative_to(world_origin, bound) else {
            return VoxelSample::empty(EMPTY_DISTANCE);
        };
        self.sample_local(local)
    }

    #[inline]
    fn sample_local(self, local: Vec3) -> VoxelSample {
        let radial = local.length();
        if !radial.is_finite() {
            return VoxelSample::empty(EMPTY_DISTANCE);
        }

        let direction = if radial > f32::EPSILON {
            local / radial
        } else {
            Vec3::Y
        };

        let surface_radius = self.surface_radius_native(direction);
        let distance = (radial - surface_radius).clamp(-EMPTY_DISTANCE, EMPTY_DISTANCE);

        VoxelSample::new(
            distance,
            if distance < 0.0 {
                VoxelMaterialId::ROCK
            } else {
                VoxelMaterialId::VOID
            },
        )
    }

    fn surface_radius_native(self, direction: Vec3) -> f32 {
        let macro_relief = lunar_macro_relative_relief(direction) * self.radius_native;
        self.radius_native + macro_relief + self.hierarchical_detail_native(direction)
    }

    /// Adds scale-native detail bands while preserving every coarser band.
    ///
    /// At S0 the approximate physical amplitudes are:
    /// S+5 ~4 km, S+4 ~0.7 km, S+3 ~0.12 km, S+2 ~20 m,
    /// S+1 ~3 m, S0 ~0.6 m.
    fn hierarchical_detail_native(self, direction: Vec3) -> f32 {
        let mut result = 0.0_f64;

        for raw in (self.current_scale.exponent()..=self.coarsest_detail_scale.exponent()).rev() {
            let level = SpatialScale::new(raw).expect("validated lunar detail scale");
            let exponent_delta = i32::from(level.exponent() - self.current_scale.exponent());
            let current_units_per_level_unit = 10.0_f64.powi(exponent_delta);

            let radius_at_level = f64::from(self.radius_native) / current_units_per_level_unit;
            let angular_frequency = (radius_at_level * 0.82).max(4.0) as f32;

            let depth = i32::from(self.coarsest_detail_scale.exponent() - level.exponent());
            let amplitude_level_native = 0.04_f64 * 1.70_f64.powi(depth);
            let seed = scale_layer_seed(self.seed, level);

            let p = direction * angular_frequency;
            let broad = value_noise_3d(
                p + Vec3::new(13.7, -7.1, 3.9),
                seed ^ 0x4C55_4E41,
            );
            let fine = value_noise_3d(
                p * 2.31 + Vec3::new(-5.3, 11.9, 17.2),
                seed ^ 0x5245_474F,
            );
            let band = f64::from(broad * 0.72 + fine * 0.28);

            result += band * amplitude_level_native * current_units_per_level_unit;
        }

        result as f32
    }
}

/// Large lunar features shared identically by every voxel refinement level.
fn lunar_macro_relative_relief(direction: Vec3) -> f32 {
    let mut height =
        (direction.dot(Vec3::new(1.7, -2.3, 0.9)) * 5.0).sin() * 0.0014
            + (direction.dot(Vec3::new(-3.1, 0.7, 2.4)) * 8.0).sin() * 0.0008;

    let craters = [
        (Vec3::new(0.82, 0.21, 0.53), 0.36_f32, 0.0100_f32),
        (Vec3::new(-0.51, 0.70, 0.49), 0.27, 0.0070),
        (Vec3::new(0.18, -0.88, 0.44), 0.23, 0.0060),
        (Vec3::new(-0.77, -0.23, -0.59), 0.19, 0.0048),
        (Vec3::new(0.39, 0.48, -0.79), 0.16, 0.0040),
        (Vec3::new(-0.08, -0.35, 0.93), 0.13, 0.0034),
        (Vec3::new(0.63, -0.66, -0.40), 0.11, 0.0028),
        (Vec3::new(-0.33, 0.14, -0.93), 0.095, 0.0024),
    ];

    for (raw_center, crater_radius, depth) in craters {
        let center = raw_center.normalize();
        let distance = direction.distance(center);
        let q = distance / crater_radius;

        if q < 1.0 {
            let bowl = 1.0 - q * q;
            height -= depth * bowl * bowl;
        }

        let rim_distance = ((q - 1.0) / 0.22).abs();
        if rim_distance < 1.0 {
            let rim = 1.0 - rim_distance;
            height += depth * 0.28 * rim * rim;
        }
    }

    height
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lunar_field_changes_sign_across_surface() {
        let body = ProceduralCelestialBody::lunar(1_737_000.0, SpatialScale::ZERO, 0x4D4F_4F4E);
        let direction = Vec3::new(0.3, 0.4, 0.8660254).normalize();
        let surface = body.surface_radius_native(direction);

        assert!(body.sample_local(direction * (surface - 4.0)).distance.is_solid());
        assert!(body.sample_local(direction * (surface + 4.0)).distance.is_empty());
    }

    #[test]
    fn lunar_radius_projects_consistently_across_scales() {
        let radius_scale0 = 1_737_000.0;
        for raw in 0..=5 {
            let scale = SpatialScale::new(raw).unwrap();
            let body = ProceduralCelestialBody::lunar(radius_scale0, scale, 7);
            let reconstructed = f64::from(body.radius_native()) * scale.scale0_units_per_native();
            assert!((reconstructed - radius_scale0).abs() < 1.0);
        }
    }

    #[test]
    fn finer_realizations_keep_the_same_macro_moon() {
        let direction = Vec3::new(-0.42, 0.81, 0.40).normalize();
        let s5 = SpatialScale::new(5).unwrap();
        let coarse = ProceduralCelestialBody::lunar(1_737_000.0, s5, 0x4D4F_4F4E);
        let fine = ProceduralCelestialBody::lunar(1_737_000.0, SpatialScale::ZERO, 0x4D4F_4F4E);

        let coarse_scale0 = f64::from(coarse.surface_radius_native(direction)) * s5.scale0_units_per_native();
        let fine_scale0 = f64::from(fine.surface_radius_native(direction));
        assert!((coarse_scale0 - fine_scale0).abs() < 20_000.0);
    }
}
