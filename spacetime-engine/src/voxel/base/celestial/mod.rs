//! Procedural voxel baselines for generic celestial bodies.
//!
//! A celestial body is one semantic field realized repeatedly at different USF
//! scales. Coarse whole-body views and fine local terrain therefore use the same
//! VoxelWorld -> dense materialization -> Surface Nets -> manifestation path.

use bevy::prelude::Vec3;

use crate::spatial::{SpatialScale, UsfPosition};

use super::{
    EMPTY_DISTANCE,
    noise::{scale_layer_seed, value_noise_3d},
};
use super::super::{VoxelMaterialId, VoxelQueryPosition, VoxelSample};

const LOCAL_SAMPLE_MARGIN_NATIVE: f32 = 8192.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CelestialBodyProfile {
    Lunar,
    Rocky,
    Stellar,
}

/// Reconstructible spherical celestial-body field.
///
/// `center` remains canonical and independent of the owning voxel world's grid
/// origin. `current_scale` only selects the bounded realization chart and which
/// detail bands are allowed to appear there.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProceduralCelestialBody {
    center: UsfPosition,
    radius_native: f32,
    current_scale: SpatialScale,
    coarsest_detail_scale: SpatialScale,
    seed: u32,
    profile: CelestialBodyProfile,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct PreparedProceduralCelestialBody {
    body: ProceduralCelestialBody,
    chunk_origin_from_center: Vec3,
}

impl PreparedProceduralCelestialBody {
    #[inline]
    pub(crate) fn sample(self, chunk_local: Vec3) -> VoxelSample {
        self.body.sample_local(self.chunk_origin_from_center + chunk_local)
    }
}

impl ProceduralCelestialBody {
    pub fn new(
        center: UsfPosition,
        radius_scale0: f64,
        current_scale: SpatialScale,
        coarsest_detail_scale: SpatialScale,
        seed: u32,
        profile: CelestialBodyProfile,
    ) -> Self {
        assert!(
            current_scale <= coarsest_detail_scale,
            "celestial realization must not be coarser than its ladder root"
        );
        let radius_native = current_scale.scale0_to_native_f64(radius_scale0) as f32;
        assert!(
            radius_native.is_finite() && radius_native > 0.0,
            "celestial radius must remain finite and positive in its realization chart"
        );
        Self {
            center,
            radius_native,
            current_scale,
            coarsest_detail_scale,
            seed,
            profile,
        }
    }

    pub const fn center(self) -> UsfPosition {
        self.center
    }

    pub const fn radius_native(self) -> f32 {
        self.radius_native
    }

    pub const fn profile(self) -> CelestialBodyProfile {
        self.profile
    }

    pub(crate) fn prepare_local_sampler(
        self,
        _world_origin: VoxelQueryPosition,
        chunk_origin: VoxelQueryPosition,
    ) -> Option<PreparedProceduralCelestialBody> {
        let bound = self.radius_native + LOCAL_SAMPLE_MARGIN_NATIVE;
        let chunk_origin_from_center = chunk_origin
            .usf()
            .relative_at_scale_bounded(&self.center, self.current_scale, bound)
            .ok()?;
        Some(PreparedProceduralCelestialBody {
            body: self,
            chunk_origin_from_center,
        })
    }

    pub(crate) fn sample_at(
        self,
        _world_origin: VoxelQueryPosition,
        point: VoxelQueryPosition,
    ) -> VoxelSample {
        let bound = self.radius_native + LOCAL_SAMPLE_MARGIN_NATIVE;
        let Ok(local) = point
            .usf()
            .relative_at_scale_bounded(&self.center, self.current_scale, bound)
        else {
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
        let macro_relief = match self.profile {
            CelestialBodyProfile::Lunar => lunar_macro_relative_relief(direction),
            CelestialBodyProfile::Rocky => rocky_macro_relative_relief(direction, self.seed),
            CelestialBodyProfile::Stellar => stellar_macro_relative_relief(direction, self.seed),
        } * self.radius_native;

        self.radius_native + macro_relief + self.hierarchical_detail_native(direction)
    }

    /// Every finer realization reproduces all already-entered coarser bands and
    /// adds only detail belonging to newly entered scales.
    fn hierarchical_detail_native(self, direction: Vec3) -> f32 {
        let mut result = 0.0_f64;

        for raw in (self.current_scale.exponent()..=self.coarsest_detail_scale.exponent()).rev() {
            let level = SpatialScale::new(raw).expect("validated celestial detail scale");
            let exponent_delta = i32::from(level.exponent() - self.current_scale.exponent());
            let current_units_per_level_unit = 10.0_f64.powi(exponent_delta);
            let radius_at_level = f64::from(self.radius_native) / current_units_per_level_unit;

            let (frequency_factor, amplitude, growth, salt): (f64, f64, f64, u32) =
                match self.profile {
                CelestialBodyProfile::Lunar => (0.82, 0.040, 1.70, 0x4C55_4E41),
                CelestialBodyProfile::Rocky => (0.63, 0.025, 1.55, 0x524F_434B),
                CelestialBodyProfile::Stellar => (0.48, 0.008, 1.30, 0x5354_4152),
            };

            let angular_frequency = (radius_at_level * frequency_factor).max(4.0) as f32;
            let depth = i32::from(self.coarsest_detail_scale.exponent() - level.exponent());
            let amplitude_level_native = amplitude * growth.powi(depth);
            let seed = scale_layer_seed(self.seed ^ salt, level);

            let p = direction * angular_frequency;
            let broad = value_noise_3d(p + Vec3::new(13.7, -7.1, 3.9), seed ^ 0xA341_316C);
            let fine = value_noise_3d(
                p * 2.31 + Vec3::new(-5.3, 11.9, 17.2),
                seed ^ 0xC801_3EA4,
            );
            let band = f64::from(broad * 0.72 + fine * 0.28);
            result += band * amplitude_level_native * current_units_per_level_unit;
        }

        result as f32
    }
}

fn rocky_macro_relative_relief(direction: Vec3, seed: u32) -> f32 {
    let phase = (seed as f32 / u32::MAX as f32) * std::f32::consts::TAU;
    (direction.dot(Vec3::new(1.1, -1.7, 0.6)) * 4.0 + phase).sin() * 0.0012
        + (direction.dot(Vec3::new(-2.2, 0.4, 1.8)) * 7.0 - phase).sin() * 0.0007
}

fn stellar_macro_relative_relief(direction: Vec3, seed: u32) -> f32 {
    let phase = (seed as f32 / u32::MAX as f32) * std::f32::consts::TAU;
    (direction.dot(Vec3::new(0.7, 1.3, -1.1)) * 5.0 + phase).sin() * 0.00035
}

/// Large lunar features shared identically by every lunar refinement level.
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

    fn center() -> UsfPosition {
        UsfPosition::zero(SpatialScale::ZERO)
            .translated_whole_native([384_400_000, 18_000_000, 22_000_000])
            .unwrap()
    }

    #[test]
    fn lunar_field_changes_sign_across_surface() {
        let body = ProceduralCelestialBody::new(
            center(),
            1_737_000.0,
            SpatialScale::ZERO,
            SpatialScale::new(5).unwrap(),
            0x4D4F_4F4E,
            CelestialBodyProfile::Lunar,
        );
        let direction = Vec3::new(0.3, 0.4, 0.8660254).normalize();
        let surface = body.surface_radius_native(direction);
        assert!(body.sample_local(direction * (surface - 4.0)).distance.is_solid());
        assert!(body.sample_local(direction * (surface + 4.0)).distance.is_empty());
    }

    #[test]
    fn body_radius_projects_consistently_across_scales() {
        let radius_scale0 = 1_737_000.0;
        let coarsest = SpatialScale::new(5).unwrap();
        for raw in 0..=5 {
            let scale = SpatialScale::new(raw).unwrap();
            let body = ProceduralCelestialBody::new(
                center(),
                radius_scale0,
                scale,
                coarsest,
                7,
                CelestialBodyProfile::Lunar,
            );
            let reconstructed = f64::from(body.radius_native()) * scale.scale0_units_per_native();
            assert!((reconstructed - radius_scale0).abs() < 1.0);
        }
    }
}
