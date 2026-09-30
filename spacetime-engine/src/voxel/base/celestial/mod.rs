//! Procedural voxel baselines for generic celestial bodies.
//!
//! The canonical Scale Stack carries the enormous position/range. Dense voxel
//! sampling only sees a bounded local chart around one canonical surface anchor.

use bevy::{math::DVec3, prelude::Vec3};

use crate::spatial::{SpatialScale, UsfPosition, UsfPositionError, UsfSemanticFrame};

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
    origin_snapshot: UsfPosition,
    frame_snapshot: UsfSemanticFrame,
    radius_metres: f64,
    current_scale: SpatialScale,
    coarsest_detail_scale: SpatialScale,
    seed: u32,
    profile: CelestialBodyProfile,
}

/// Prepared fine sampling keeps only canonical chunk identity.
///
/// Geometry is intentionally *not* approximated by a materialization-local
/// tangent plane. Every dense sample is reconstructed canonically and resolved
/// through the body's semantic surface query.
#[derive(Debug, Clone, Copy)]
pub(crate) struct FineSurfaceFrame {
    chunk_origin: UsfPosition,
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
                let local_delta = self.body.frame_snapshot.world_direction_to_local(chunk_local);
                self.body.sample_coarse_local(chunk_origin_from_center + local_delta)
            }
            PreparedSampling::FineSurface(frame) => self.body.sample_fine_local(frame, chunk_local),
        }
    }
}

impl ProceduralCelestialBody {
pub fn new(
        origin_snapshot: UsfPosition,
        frame_snapshot: UsfSemanticFrame,
        radius_metres: f64,
        current_scale: SpatialScale,
        coarsest_detail_scale: SpatialScale,
        seed: u32,
        profile: CelestialBodyProfile,
    ) -> Self {
        assert!(radius_metres.is_finite() && radius_metres > 0.0);
        Self {
            origin_snapshot,
            frame_snapshot,
            radius_metres,
            current_scale,
            coarsest_detail_scale,
            seed,
            profile,
        }
    }
    pub const fn origin_snapshot(self) -> UsfPosition { self.origin_snapshot }
    pub const fn frame_snapshot(self) -> UsfSemanticFrame { self.frame_snapshot }
    pub const fn radius_metres(self) -> f64 { self.radius_metres }

    pub fn radius_native_f64(self) -> f64 {
        self.current_scale.metres_to_native_f64(self.radius_metres)
    }

    pub const fn profile(self) -> CelestialBodyProfile { self.profile }

    /// Conservative maximum radial extent represented by this realization.
    ///
    /// This is broadphase metadata, not a replacement surface. It bounds the
    /// current procedural macro relief plus every detail band owned by this
    /// Scale Slice so high-speed collision queries may safely over-report a
    /// candidate without depending on a materialized mesh/collider.
    pub(crate) fn conservative_outer_radius_metres(self) -> f64 {
        let mut radius = self.radius_metres
            * (1.0 + self.maximum_outward_macro_relief_fraction());

        if self.current_scale <= self.coarsest_detail_scale {
            for raw in self.current_scale.exponent()
                ..=self.coarsest_detail_scale.exponent()
            {
                let level = SpatialScale::new(raw)
                    .expect("validated celestial detail scale");
                radius += self.detail_amplitude_native(level).abs()
                    * level.metres_per_native();
            }
        }

        radius
    }
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
            let chunk_origin_world = chunk_origin
                .usf()
                .relative_at_scale_bounded(&self.origin_snapshot, self.current_scale, bound)
                .ok()?;
            let chunk_origin_from_center = self
                .frame_snapshot
                .world_direction_to_local(chunk_origin_world);
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
        let local_direction = normalized_direction(direction);
        let world_direction = normalized_direction(
            self.frame_snapshot.local_direction_to_world(local_direction),
        );

        if self.current_scale > SpatialScale::ZERO {
            let radius_native =
                self.coarse_surface_radius_metres(local_direction, self.current_scale)
                    / self.current_scale.metres_per_native();
            return self.origin_snapshot.translated_at_scale_f64(
                self.current_scale,
                dvec(world_direction) * radius_native,
            );
        }

        let (reference, local_reference) = self.coarse_surface_references(local_direction)?;
        let mut surface = reference;
        let canonical_upper = self.coarsest_detail_scale.exponent().min(0);

        if self.current_scale.exponent() <= canonical_upper {
            for raw in (self.current_scale.exponent()..=canonical_upper).rev() {
                let level = SpatialScale::new(raw)
                    .expect("validated canonical celestial detail scale");
                let noise = self.canonical_detail_noise_at(local_reference, level)?;
                let displacement_native = noise * self.detail_amplitude_native(level) as f32;
                surface = surface.translated_at_scale(
                    level,
                    world_direction * displacement_native,
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
        let local_up = self.direction_to(point)?;
        let world_up = normalized_direction(self.frame_snapshot.local_direction_to_world(local_up));
        let surface = self.surface_position(local_up).ok()?;
        let relative = point
            .relative_at_scale_bounded(&surface, self.current_scale, max_abs_native.max(0.0))
            .ok()?;
        Some((surface, world_up, relative.dot(world_up)))
    }

    fn prepare_fine_surface_frame(
        self,
        chunk_origin: VoxelQueryPosition,
    ) -> Option<FineSurfaceFrame> {
        // Validate that this chunk has a meaningful radial relation to the body,
        // but keep the canonical origin itself as the prepared state. The old
        // implementation captured a chunk-local tangent plane here, making the
        // cache boundary part of planetary geometry.
        self.direction_to(&chunk_origin.usf())?;
        Some(FineSurfaceFrame {
            chunk_origin: chunk_origin.usf(),
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
        let Ok(point) = frame
            .chunk_origin
            .translated_at_scale(self.current_scale, chunk_local)
        else {
            return VoxelSample::empty(EMPTY_DISTANCE);
        };

        // One semantic resolver now owns both travel clearance and dense fine
        // voxel geometry. Neighboring materializations that sample the same
        // canonical border point therefore receive the same signed distance,
        // regardless of which cache produced the query.
        let Some((_, _, clearance_native)) =
            self.surface_near(&point, FINE_SURFACE_FRAME_BOUND_NATIVE)
        else {
            return VoxelSample::empty(EMPTY_DISTANCE);
        };

        self.sample_from_signed_distance(f64::from(clearance_native))
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
        let relative_world = point
            .relative_at_scale_bounded_f64(
                &self.origin_snapshot,
                SpatialScale::ZERO,
                f64::MAX,
            )
            .ok()?;
        let radial = relative_world.length();
        if !radial.is_finite() || radial <= f64::EPSILON {
            return None;
        }

        let world_direction = Vec3::new(
            (relative_world.x / radial) as f32,
            (relative_world.y / radial) as f32,
            (relative_world.z / radial) as f32,
        )
        .normalize_or_zero();
        Some(normalized_direction(
            self.frame_snapshot.world_direction_to_local(world_direction),
        ))
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
fn coarse_surface_references(
        self,
        direction: Vec3,
    ) -> Result<(UsfPosition, UsfPosition), UsfPositionError> {
        let local_direction = normalized_direction(direction);
        let world_direction = normalized_direction(
            self.frame_snapshot.local_direction_to_world(local_direction),
        );
        let s1 = SpatialScale::new(1).expect("S1 is a valid USF scale");
        let radius_metres = self.coarse_surface_radius_metres(local_direction, s1);
        let world = self
            .origin_snapshot
            .translated_metres_f64(dvec(world_direction) * radius_metres)?;
        let local = UsfPosition::zero(self.origin_snapshot.leaf_scale())
            .translated_metres_f64(dvec(local_direction) * radius_metres)?;
        Ok((world, local))
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
        local_position: UsfPosition,
        level: SpatialScale,
    ) -> Result<f32, UsfPositionError> {
        let point = VoxelQueryPosition::new(local_position.reexpressed_at(level)?);
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

    fn maximum_outward_macro_relief_fraction(self) -> f64 {
        match self.profile {
            // Two broad waves plus the deliberately conservative assumption
            // that every crater rim can contribute at once. Bowl depth is
            // inward and therefore irrelevant to an outer bound.
            CelestialBodyProfile::Lunar => {
                let crater_depth_sum =
                    0.0100 + 0.0070 + 0.0060 + 0.0048
                    + 0.0040 + 0.0034 + 0.0028 + 0.0024;
                0.0014 + 0.0008 + crater_depth_sum * 0.28
            }
            CelestialBodyProfile::Rocky => 0.0012 + 0.0007,
            CelestialBodyProfile::Stellar => 0.00035,
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
                UsfSemanticFrame::identity(),
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
    fn neighboring_fine_materializations_share_identical_canonical_border_samples() {
        let scale = SpatialScale::ZERO;
        let body = ProceduralCelestialBody::new(
            earth_center(),
            UsfSemanticFrame::identity(),
            6_371_000.0,
            scale,
            SpatialScale::new(6).unwrap(),
            0x4541_5254,
            CelestialBodyProfile::Rocky,
        );

        let surface = body.surface_position(Vec3::Y).unwrap();
        let left_origin = surface
            .translated_at_scale(scale, Vec3::new(-5.0, -4.0, -5.0))
            .unwrap();
        let right_origin = left_origin
            .translated_at_scale(scale, Vec3::new(10.0, 0.0, 0.0))
            .unwrap();

        let left = body
            .prepare_local_sampler(
                VoxelQueryPosition::new(surface),
                VoxelQueryPosition::new(left_origin),
            )
            .unwrap();
        let right = body
            .prepare_local_sampler(
                VoxelQueryPosition::new(surface),
                VoxelQueryPosition::new(right_origin),
            )
            .unwrap();

        // Surface Nets stores one copied sample of padding. These two x pairs
        // address the exact same canonical planes from adjacent 10-unit
        // materializations: left 9 == right -1, left 10 == right 0.
        for (left_x, right_x) in [(9.0, -1.0), (10.0, 0.0)] {
            for z in -1..=10 {
                for y in -1..=10 {
                    let left_sample =
                        left.sample(Vec3::new(left_x, y as f32, z as f32));
                    let right_sample =
                        right.sample(Vec3::new(right_x, y as f32, z as f32));

                    assert_eq!(left_sample.material, right_sample.material);
                    assert_eq!(
                        left_sample.distance.0.to_bits(),
                        right_sample.distance.0.to_bits(),
                        "shared canonical border sample diverged at y={y}, z={z}",
                    );
                }
            }
        }
    }

    #[test]
    fn planck_scale_surface_patch_never_requires_planet_radius_in_f32() {
        let scale = SpatialScale::MIN;
        let body = ProceduralCelestialBody::new(
            earth_center(),
            UsfSemanticFrame::identity(),
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
