//! Procedural voxel baselines for generic celestial bodies.
//!
//! The canonical Scale Stack carries the enormous position/range. Dense voxel
//! sampling only sees a bounded local chart around one canonical surface anchor.

use bevy::{math::DVec3, prelude::Vec3};

use crate::spatial::{SpatialScale, UsfPosition, UsfPositionError};

use super::{
    EMPTY_DISTANCE,
    noise::{scale_layer_seed, semantic_value_noise_3d, value_noise_3d},
};
use super::super::{VoxelMaterialId, VoxelQueryPosition, VoxelSample};

const LOCAL_SAMPLE_MARGIN_NATIVE: f32 = 8_192.0;
const LOCAL_SAMPLE_RELIEF_MARGIN_FRACTION: f64 = 0.05;
const FINE_SURFACE_FRAME_BOUND_NATIVE: f32 = 1_000_000_000.0;
const CANONICAL_DETAIL_CELL_NATIVE: i64 = 20;
const CANONICAL_DETAIL_FINE_CELL_NATIVE: i64 = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CelestialBodyProfile {
    Lunar,
    Rocky,
    Stellar,
}

/// Reconstructible celestial field.
///
/// Semantic radius stays in SI `f64`; it is never converted into one fine-slice
/// `f32` radius. S0 and finer instead resolve a canonical surface anchor first.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProceduralCelestialBody {
    center: UsfPosition,
    radius_metres: f64,
    current_scale: SpatialScale,
    coarsest_detail_scale: SpatialScale,
    seed: u32,
    profile: CelestialBodyProfile,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct FineSurfaceFrame {
    surface_anchor: UsfPosition,
    chunk_origin_from_surface: Vec3,
    up: Vec3,
    dynamic_levels: [Option<SpatialScale>; 2],
    anchor_noise: [f32; 2],
}

#[derive(Debug, Clone, Copy)]
enum PreparedSampling {
    CoarseRadial { chunk_origin_from_center: Vec3 },
    FineSurface(FineSurfaceFrame),
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct PreparedProceduralCelestialBody {
    body: ProceduralCelestialBody,
    sampling: PreparedSampling,
}

impl PreparedProceduralCelestialBody {
    #[inline]
    pub(crate) fn sample(self, chunk_local: Vec3) -> VoxelSample {
        match self.sampling {
            PreparedSampling::CoarseRadial { chunk_origin_from_center } => {
                self.body.sample_coarse_local(chunk_origin_from_center + chunk_local)
            }
            PreparedSampling::FineSurface(frame) => self.body.sample_fine_local(frame, chunk_local),
        }
    }
}

impl ProceduralCelestialBody {
    pub fn new(
        center: UsfPosition,
        radius_metres: f64,
        current_scale: SpatialScale,
        coarsest_detail_scale: SpatialScale,
        seed: u32,
        profile: CelestialBodyProfile,
    ) -> Self {
        assert!(radius_metres.is_finite() && radius_metres > 0.0);
        Self {
            center,
            radius_metres,
            current_scale,
            coarsest_detail_scale,
            seed,
            profile,
        }
    }

    pub const fn center(self) -> UsfPosition { self.center }
    pub const fn radius_metres(self) -> f64 { self.radius_metres }

    pub fn radius_native_f64(self) -> f64 {
        self.current_scale.metres_to_native_f64(self.radius_metres)
    }

    pub const fn profile(self) -> CelestialBodyProfile { self.profile }

    pub(crate) fn prepare_local_sampler(
        self,
        _world_origin: VoxelQueryPosition,
        chunk_origin: VoxelQueryPosition,
    ) -> Option<PreparedProceduralCelestialBody> {
        let sampling = if self.current_scale > SpatialScale::ZERO {
            let radius_native = self.radius_native_f64();
            if !radius_native.is_finite() || radius_native > f64::from(f32::MAX) {
                return None;
            }
            let bound = (
                radius_native
                    + f64::from(LOCAL_SAMPLE_MARGIN_NATIVE)
                    + radius_native * LOCAL_SAMPLE_RELIEF_MARGIN_FRACTION
            )
                .min(f64::from(f32::MAX)) as f32;
            let chunk_origin_from_center = chunk_origin
                .usf()
                .relative_at_scale_bounded(&self.center, self.current_scale, bound)
                .ok()?;
            PreparedSampling::CoarseRadial { chunk_origin_from_center }
        } else {
            PreparedSampling::FineSurface(self.prepare_fine_surface_frame(chunk_origin)?)
        };

        Some(PreparedProceduralCelestialBody { body: self, sampling })
    }

    pub(crate) fn sample_at(
        self,
        world_origin: VoxelQueryPosition,
        point: VoxelQueryPosition,
    ) -> VoxelSample {
        self.prepare_local_sampler(world_origin, point)
            .map(|prepared| prepared.sample(Vec3::ZERO))
            .unwrap_or_else(|| VoxelSample::empty(EMPTY_DISTANCE))
    }

    /// Canonical surface point including every detail band owned by this scale.
    pub fn surface_position(self, direction: Vec3) -> Result<UsfPosition, UsfPositionError> {
        let direction = normalized_direction(direction);

        if self.current_scale > SpatialScale::ZERO {
            let radius_native =
                self.coarse_surface_radius_metres(direction, self.current_scale)
                    / self.current_scale.metres_per_native();
            return self.center.translated_at_scale_f64(
                self.current_scale,
                dvec(direction) * radius_native,
            );
        }

        let reference = self.coarse_surface_reference(direction)?;
        let mut surface = reference;
        let canonical_upper = self.coarsest_detail_scale.exponent().min(0);

        if self.current_scale.exponent() <= canonical_upper {
            for raw in (self.current_scale.exponent()..=canonical_upper).rev() {
                let level = SpatialScale::new(raw)
                    .expect("validated canonical celestial detail scale");
                let noise = self.canonical_detail_noise_at(reference, level)?;
                let displacement_native =
                    noise * self.detail_amplitude_native(level) as f32;
                surface = surface.translated_at_scale(
                    level,
                    direction * displacement_native,
                )?;
            }
        }

        Ok(surface)
    }

    /// Resolve a canonical surface anchor first, then measure only tiny local
    /// clearance in the target chart.
    pub(crate) fn surface_near(
        self,
        point: &UsfPosition,
        max_abs_native: f32,
    ) -> Option<(UsfPosition, Vec3, f32)> {
        let up = self.direction_to(point)?;
        let surface = self.surface_position(up).ok()?;
        let relative = point
            .relative_at_scale_bounded(&surface, self.current_scale, max_abs_native.max(0.0))
            .ok()?;
        Some((surface, up, relative.dot(up)))
    }

    fn prepare_fine_surface_frame(
        self,
        chunk_origin: VoxelQueryPosition,
    ) -> Option<FineSurfaceFrame> {
        let up = self.direction_to(&chunk_origin.usf())?;
        let surface_anchor = self.surface_position(up).ok()?;
        let chunk_origin_from_surface = chunk_origin
            .usf()
            .relative_at_scale_bounded(
                &surface_anchor,
                self.current_scale,
                FINE_SURFACE_FRAME_BOUND_NATIVE,
            )
            .ok()?;

        let mut dynamic_levels = [None, None];
        dynamic_levels[0] = Some(self.current_scale);
        if self.current_scale < SpatialScale::ZERO {
            let parent = SpatialScale::new(self.current_scale.exponent() + 1)
                .expect("fine scale has an adjacent parent");
            if parent <= SpatialScale::ZERO && parent <= self.coarsest_detail_scale {
                dynamic_levels[1] = Some(parent);
            }
        }

        let mut anchor_noise = [0.0; 2];
        for (index, level) in dynamic_levels.into_iter().enumerate() {
            if let Some(level) = level {
                anchor_noise[index] =
                    self.canonical_detail_noise_at(surface_anchor, level).ok()?;
            }
        }

        Some(FineSurfaceFrame {
            surface_anchor,
            chunk_origin_from_surface,
            up,
            dynamic_levels,
            anchor_noise,
        })
    }

    #[inline]
    fn sample_coarse_local(self, local: Vec3) -> VoxelSample {
        let x = f64::from(local.x);
        let y = f64::from(local.y);
        let z = f64::from(local.z);
        let radial = (x * x + y * y + z * z).sqrt();
        if !radial.is_finite() {
            return VoxelSample::empty(EMPTY_DISTANCE);
        }

        let direction = if radial > f64::EPSILON {
            Vec3::new(
                (x / radial) as f32,
                (y / radial) as f32,
                (z / radial) as f32,
            )
            .normalize_or_zero()
        } else {
            Vec3::Y
        };

        let surface_radius =
            self.coarse_surface_radius_metres(direction, self.current_scale)
                / self.current_scale.metres_per_native();
        self.sample_from_signed_distance(radial - surface_radius)
    }

    #[inline]
    fn sample_fine_local(self, frame: FineSurfaceFrame, chunk_local: Vec3) -> VoxelSample {
        let delta = frame.chunk_origin_from_surface + chunk_local;
        let normal_distance = f64::from(delta.dot(frame.up));
        let tangential = delta - frame.up * delta.dot(frame.up);

        let Ok(noise_position) = frame
            .surface_anchor
            .translated_at_scale(self.current_scale, tangential)
        else {
            return VoxelSample::empty(EMPTY_DISTANCE);
        };

        let mut detail_delta_native = 0.0_f64;
        for (index, level) in frame.dynamic_levels.into_iter().enumerate() {
            let Some(level) = level else { continue };
            let Ok(noise) = self.canonical_detail_noise_at(noise_position, level) else {
                continue;
            };
            let native_scale = 10.0_f64.powi(
                i32::from(level.exponent()) - i32::from(self.current_scale.exponent()),
            );
            detail_delta_native +=
                f64::from(noise - frame.anchor_noise[index])
                    * self.detail_amplitude_native(level)
                    * native_scale;
        }

        self.sample_from_signed_distance(normal_distance - detail_delta_native)
    }

    #[inline]
    fn sample_from_signed_distance(self, distance_native: f64) -> VoxelSample {
        let distance = distance_native
            .clamp(-f64::from(EMPTY_DISTANCE), f64::from(EMPTY_DISTANCE))
            as f32;
        VoxelSample::new(
            distance,
            if distance < 0.0 {
                VoxelMaterialId::ROCK
            } else {
                VoxelMaterialId::VOID
            },
        )
    }

    fn direction_to(self, point: &UsfPosition) -> Option<Vec3> {
        let relative = point
            .relative_at_scale_bounded_f64(
                &self.center,
                SpatialScale::ZERO,
                f64::MAX,
            )
            .ok()?;
        let radial = relative.length();
        if !radial.is_finite() || radial <= f64::EPSILON {
            return None;
        }

        Some(
            Vec3::new(
                (relative.x / radial) as f32,
                (relative.y / radial) as f32,
                (relative.z / radial) as f32,
            )
            .normalize_or_zero(),
        )
    }

    /// S1+ remains a conventional radial evaluation. S0 and finer do not.
    fn coarse_surface_radius_metres(
        self,
        direction: Vec3,
        through_scale: SpatialScale,
    ) -> f64 {
        let direction = normalized_direction(direction);
        let mut radius =
            self.radius_metres * (1.0 + f64::from(self.macro_relative_relief(direction)));

        let lower = through_scale.exponent().max(1);
        let upper = self.coarsest_detail_scale.exponent();
        if lower <= upper {
            for raw in (lower..=upper).rev() {
                let level =
                    SpatialScale::new(raw).expect("validated coarse celestial detail scale");
                radius += self.coarse_detail_band_native(direction, level)
                    * level.metres_per_native();
            }
        }

        radius
    }

    fn coarse_surface_reference(
        self,
        direction: Vec3,
    ) -> Result<UsfPosition, UsfPositionError> {
        let direction = normalized_direction(direction);
        let s1 = SpatialScale::new(1).expect("S1 is a valid USF scale");
        let radius_metres = self.coarse_surface_radius_metres(direction, s1);
        self.center
            .translated_metres_f64(dvec(direction) * radius_metres)
    }

    fn coarse_detail_band_native(self, direction: Vec3, level: SpatialScale) -> f64 {
        let (frequency_factor, _, _, salt) = self.detail_parameters();
        let radius_at_level = self.radius_metres / level.metres_per_native();
        let angular_frequency =
            (radius_at_level * frequency_factor).max(4.0).min(f64::from(f32::MAX)) as f32;
        let seed = scale_layer_seed(self.seed ^ salt, level);

        let p = direction * angular_frequency;
        let broad =
            value_noise_3d(p + Vec3::new(13.7, -7.1, 3.9), seed ^ 0xA341_316C);
        let fine = value_noise_3d(
            p * 2.31 + Vec3::new(-5.3, 11.9, 17.2),
            seed ^ 0xC801_3EA4,
        );

        f64::from(broad * 0.72 + fine * 0.28)
            * self.detail_amplitude_native(level)
    }

    fn canonical_detail_noise_at(
        self,
        position: UsfPosition,
        level: SpatialScale,
    ) -> Result<f32, UsfPositionError> {
        let point = VoxelQueryPosition::new(position.reexpressed_at(level)?);
        let (_, _, _, salt) = self.detail_parameters();
        let seed = scale_layer_seed(self.seed ^ salt, level);
        let broad = semantic_value_noise_3d(
            point,
            CANONICAL_DETAIL_CELL_NATIVE,
            seed ^ 0xA341_316C,
        );
        let fine = semantic_value_noise_3d(
            point,
            CANONICAL_DETAIL_FINE_CELL_NATIVE,
            seed ^ 0xC801_3EA4,
        );
        Ok(broad * 0.72 + fine * 0.28)
    }

    fn detail_amplitude_native(self, level: SpatialScale) -> f64 {
        let (_, amplitude, growth, _) = self.detail_parameters();
        let depth =
            i32::from(self.coarsest_detail_scale.exponent() - level.exponent()).max(0);
        amplitude * growth.powi(depth)
    }

    fn detail_parameters(self) -> (f64, f64, f64, u32) {
        match self.profile {
            CelestialBodyProfile::Lunar => (0.82, 0.040, 1.70, 0x4C55_4E41),
            CelestialBodyProfile::Rocky => (0.63, 0.025, 1.55, 0x524F_434B),
            CelestialBodyProfile::Stellar => (0.48, 0.008, 1.30, 0x5354_4152),
        }
    }

    fn macro_relative_relief(self, direction: Vec3) -> f32 {
        match self.profile {
            CelestialBodyProfile::Lunar => lunar_macro_relative_relief(direction),
            CelestialBodyProfile::Rocky => rocky_macro_relative_relief(direction, self.seed),
            CelestialBodyProfile::Stellar => stellar_macro_relative_relief(direction, self.seed),
        }
    }
}

fn normalized_direction(direction: Vec3) -> Vec3 {
    let direction = direction.normalize_or_zero();
    if direction == Vec3::ZERO { Vec3::Y } else { direction }
}

fn dvec(value: Vec3) -> DVec3 {
    DVec3::new(
        f64::from(value.x),
        f64::from(value.y),
        f64::from(value.z),
    )
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

    fn earth_center() -> UsfPosition {
        UsfPosition::from_scale_native_f64(
            DVec3::new(0.0, -6_371_000.0, 0.0),
            SpatialScale::ZERO,
            SpatialScale::MIN,
        )
        .unwrap()
    }

    #[test]
    fn coarse_body_radius_projects_consistently() {
        let radius_metres = 6_371_000.0;
        let coarsest = SpatialScale::new(6).unwrap();

        for raw in 1..=6 {
            let scale = SpatialScale::new(raw).unwrap();
            let body = ProceduralCelestialBody::new(
                earth_center(),
                radius_metres,
                scale,
                coarsest,
                0x4541_5254,
                CelestialBodyProfile::Rocky,
            );
            let reconstructed =
                body.radius_native_f64() * scale.metres_per_native();
            assert!((reconstructed - radius_metres).abs() < 1.0e-6);
        }
    }

    #[test]
    fn planck_scale_surface_patch_never_requires_planet_radius_in_f32() {
        let scale = SpatialScale::MIN;
        let body = ProceduralCelestialBody::new(
            earth_center(),
            6_371_000.0,
            scale,
            SpatialScale::new(6).unwrap(),
            0x4541_5254,
            CelestialBodyProfile::Rocky,
        );

        assert!(body.radius_native_f64().is_finite());
        assert!(body.radius_native_f64() > f64::from(f32::MAX));

        let surface = body.surface_position(Vec3::Y).unwrap();
        let inside = surface
            .translated_at_scale(scale, Vec3::Y * -4.0)
            .unwrap();
        let outside = surface
            .translated_at_scale(scale, Vec3::Y * 4.0)
            .unwrap();

        assert!(
            body.sample_at(
                VoxelQueryPosition::new(surface),
                VoxelQueryPosition::new(inside),
            )
            .distance
            .is_solid()
        );
        assert!(
            body.sample_at(
                VoxelQueryPosition::new(surface),
                VoxelQueryPosition::new(outside),
            )
            .distance
            .is_empty()
        );
    }
}
