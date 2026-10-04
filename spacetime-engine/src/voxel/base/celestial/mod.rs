//! Procedural voxel baselines for generic celestial bodies.
//!
//! The canonical Scale Stack carries the enormous position/range. Dense voxel
//! sampling only sees a bounded local chart around one canonical surface anchor.

use bevy::{math::DVec3, prelude::Vec3};

use crate::spatial::{
    SPATIAL_SCALE_COUNT, SpatialScale, UsfPosition, UsfPositionError,
    UsfSemanticFrame,
};

use super::{
    EMPTY_DISTANCE,
    noise::{
        SemanticNoiseCornerCache, scale_layer_seed,
        semantic_value_noise_3d, semantic_value_noise_3d_cached,
        value_noise_3d,
    },
};
use super::super::{VoxelMaterialId, VoxelQueryPosition, VoxelSample};

mod bands;
mod caves;
mod rocky;

use caves::{
    CAVE_MAX_DEPTH_METRES,
    rocky_cave_void_may_intersect_aabb,
    rocky_cave_void_signed_distance_metres,
    rocky_cave_void_signed_distance_metres_with_radial,
};
use rocky::{
    rocky_maximum_outward_displacement_metres,
    rocky_surface_displacement_metres,
};

const LOCAL_SAMPLE_MARGIN_NATIVE: f32 = 8_192.0;
const LOCAL_SAMPLE_RELIEF_MARGIN_FRACTION: f64 = 0.05;
const CANONICAL_DETAIL_CELL_NATIVE: i64 = 20;
const CANONICAL_DETAIL_FINE_CELL_NATIVE: i64 = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CelestialBodyProfile {
    Lunar,
    Rocky,
    Stellar,
}

/// One canonical body-local volumetric field sample.
///
/// This is semantic terrain truth. Realization Scale, render LOD, dense cache
/// address and collision backend are all downstream adapters.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct CelestialFieldSample {
    signed_distance_metres: f64,
    material: VoxelMaterialId,
}

impl CelestialFieldSample {
    pub(crate) const fn new(
        signed_distance_metres: f64,
        material: VoxelMaterialId,
    ) -> Self {
        Self {
            signed_distance_metres,
            material,
        }
    }

    pub(crate) const fn signed_distance_metres(self) -> f64 {
        self.signed_distance_metres
    }

    pub(crate) const fn material(self) -> VoxelMaterialId {
        self.material
    }
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
    /// Numerical/cache chart only.
    current_scale: SpatialScale,
    coarsest_detail_scale: SpatialScale,
    /// Field-owned semantic surface bandwidth.
    surface_detail_scale: SpatialScale,
    seed: u32,
    profile: CelestialBodyProfile,
}

/// Prepared celestial sampling stores one exact body-local SI chunk origin.
///
/// Scale chooses only how large each `chunk_local` step is in metres. It never
/// selects a different terrain algorithm.
#[derive(Debug, Clone, Copy)]
pub(crate) struct PreparedProceduralCelestialBody {
    body: ProceduralCelestialBody,
    chunk_origin_local_metres: DVec3,
}

// presentation-central-cache-specialization-v1
// central-cache-kernel-megapass-v1
//
// Presentation samples the same body's full residual stack 729 times per
// central block. Resolve all Scale-dependent invariants once per sampler:
// SpatialScale construction, metres/native, growth.powi(), angular frequency
// and keyed seed leave the inner SDF loop.
#[derive(Debug, Clone, Copy)]
struct PreparedCelestialResidualBand {
    level: SpatialScale,
    metres_per_native: f64,
    amplitude_native: f64,
    angular_frequency: f32,
    seed: u32,
}

#[derive(Debug)]
struct PreparedCelestialResidualStack {
    coarse: [Option<PreparedCelestialResidualBand>; SPATIAL_SCALE_COUNT],
    coarse_len: usize,
    fine: [Option<PreparedCelestialResidualBand>; SPATIAL_SCALE_COUNT],
    fine_len: usize,
}

impl PreparedCelestialResidualStack {
    fn new(body: ProceduralCelestialBody) -> Self {
        let mut stack = Self {
            coarse: [None; SPATIAL_SCALE_COUNT],
            coarse_len: 0,
            fine: [None; SPATIAL_SCALE_COUNT],
            fine_len: 0,
        };

        let floor = body.surface_detail_scale.exponent();
        let root = body.coarsest_detail_scale.exponent();
        let (frequency_factor, _, _, salt) = body.detail_parameters();

        let coarse_lower = floor.max(1);
        if coarse_lower <= root {
            for raw in (coarse_lower..=root).rev() {
                let level = SpatialScale::new(raw)
                    .expect("validated coarse celestial semantic detail scale");
                let metres_per_native = level.metres_per_native();
                let amplitude_native = body.detail_amplitude_native(level);
                let radius_at_level =
                    body.radius_metres / metres_per_native;
                let angular_frequency = (
                    radius_at_level * frequency_factor
                )
                    .max(4.0)
                    .min(f64::from(f32::MAX))
                    as f32;
                let seed = scale_layer_seed(
                    body.seed ^ salt,
                    level,
                );

                stack.coarse[stack.coarse_len] =
                    Some(PreparedCelestialResidualBand {
                        level,
                        metres_per_native,
                        amplitude_native,
                        angular_frequency,
                        seed,
                    });
                stack.coarse_len += 1;
            }
        }

        let fine_upper = root.min(0);
        if floor <= fine_upper {
            for raw in (floor..=fine_upper).rev() {
                let level = SpatialScale::new(raw)
                    .expect("validated fine celestial semantic detail scale");
                stack.fine[stack.fine_len] =
                    Some(PreparedCelestialResidualBand {
                        level,
                        metres_per_native: level.metres_per_native(),
                        amplitude_native:
                            body.detail_amplitude_native(level),
                        angular_frequency: 0.0,
                        seed: scale_layer_seed(
                            body.seed ^ salt,
                            level,
                        ),
                    });
                stack.fine_len += 1;
            }
        }

        stack
    }
}

#[derive(Debug)]
pub(crate) struct PreparedCelestialPresentationBody {
    body: ProceduralCelestialBody,
    canonical_noise_cache: SemanticNoiseCornerCache,
    residual: PreparedCelestialResidualStack,
}

impl PreparedCelestialPresentationBody {
    pub(crate) fn new(body: ProceduralCelestialBody) -> Self {
        Self {
            body,
            canonical_noise_cache: SemanticNoiseCornerCache::new(),
            residual: PreparedCelestialResidualStack::new(body),
        }
    }

    #[inline]
    fn semantic_surface_radius_metres(
        &self,
        direction: Vec3,
    ) -> Result<f64, UsfPositionError> {
        // Preserve the reference normalization sequence exactly:
        // callers normalize once before this function and the reference
        // semantic surface function normalizes again here.
        let direction = normalized_direction(direction);
        let mut radius = self.body.radius_metres
            + self.body.macro_surface_displacement_metres(
                direction,
                self.body.surface_detail_scale,
            );

        for band in self
            .residual
            .coarse
            .iter()
            .take(self.residual.coarse_len)
            .flatten()
        {
            let p = direction * band.angular_frequency;
            let broad = value_noise_3d(
                p + Vec3::new(13.7, -7.1, 3.9),
                band.seed ^ 0xA341_316C,
            );
            let fine = value_noise_3d(
                p * 2.31 + Vec3::new(-5.3, 11.9, 17.2),
                band.seed ^ 0xC801_3EA4,
            );
            let detail_native =
                f64::from(broad * 0.72 + fine * 0.28)
                    * band.amplitude_native;
            radius += detail_native * band.metres_per_native;
        }

        if self.residual.fine_len > 0 {
            let local_reference_metres = dvec(direction) * radius;
            for band in self
                .residual
                .fine
                .iter()
                .take(self.residual.fine_len)
                .flatten()
            {
                let noise = self
                    .body
                    .canonical_detail_noise_at_local_metres_cached_prepared(
                        local_reference_metres,
                        band.level,
                        band.metres_per_native,
                        band.seed,
                        &self.canonical_noise_cache,
                    )?;
                radius += f64::from(noise)
                    * band.amplitude_native
                    * band.metres_per_native;
            }
        }

        Ok(radius)
    }

    #[inline]
    fn outer_signed_distance_and_radial_local_metres(
        &self,
        local_point_metres: DVec3,
    ) -> Option<(f64, f64)> {
        let radial = local_point_metres.length();
        if !radial.is_finite() || radial <= f64::EPSILON {
            return None;
        }

        let direction = Vec3::new(
            (local_point_metres.x / radial) as f32,
            (local_point_metres.y / radial) as f32,
            (local_point_metres.z / radial) as f32,
        )
        .normalize_or_zero();
        if direction == Vec3::ZERO {
            return None;
        }

        let surface_radius =
            self.semantic_surface_radius_metres(direction).ok()?;
        Some((radial - surface_radius, radial))
    }

    #[inline]
    pub(crate) fn signed_distance_local_metres(
        &self,
        local_point_metres: DVec3,
    ) -> Option<f64> {
        let (outer_sdf, radial) =
            self.outer_signed_distance_and_radial_local_metres(
                local_point_metres,
            )?;

        if self.body.profile != CelestialBodyProfile::Rocky {
            return Some(outer_sdf);
        }

        let outer_surface_radius_metres = radial - outer_sdf;
        let void_sdf =
            rocky_cave_void_signed_distance_metres_with_radial(
                local_point_metres,
                radial,
                outer_surface_radius_metres,
                self.body.seed,
            );

        Some(outer_sdf.max(-void_sdf))
    }

    #[inline]
    pub(crate) fn outer_signed_distance_local_metres(
        &self,
        local_point_metres: DVec3,
    ) -> Option<f64> {
        self.outer_signed_distance_and_radial_local_metres(
            local_point_metres,
        )
        .map(|(distance, _)| distance)
    }

    #[inline]
    pub(crate) fn surface_local_metres(
        &self,
        direction: Vec3,
    ) -> Result<DVec3, UsfPositionError> {
        let direction = normalized_direction(direction);
        let radius = self.semantic_surface_radius_metres(direction)?;
        Ok(dvec(direction) * radius)
    }

    pub(crate) fn noise_cache_stats(&self) -> (u64, u64) {
        self.canonical_noise_cache.stats()
    }
}

impl PreparedProceduralCelestialBody {
    #[inline]
    pub(crate) fn sample(self, chunk_local: Vec3) -> VoxelSample {
        // `chunk_local` is a bounded vector in the realization chart. Convert
        // only that small delta to SI and rotate it into the semantic body
        // frame; the large planet position was resolved once when this sampler
        // was prepared.
        let world_delta_metres = DVec3::new(
            f64::from(chunk_local.x),
            f64::from(chunk_local.y),
            f64::from(chunk_local.z),
        ) * self.body.current_scale.metres_per_native();
        let local_delta_metres =
            self.body.frame_snapshot.orientation().conjugate() * world_delta_metres;
        let local_point_metres =
            self.chunk_origin_local_metres + local_delta_metres;

        self.body.sample_body_local_metres(local_point_metres)
    }
}

impl ProceduralCelestialBody {
pub fn new(
        origin_snapshot: UsfPosition,
        frame_snapshot: UsfSemanticFrame,
        radius_metres: f64,
        current_scale: SpatialScale,
        coarsest_detail_scale: SpatialScale,
        surface_detail_scale: SpatialScale,
        seed: u32,
        profile: CelestialBodyProfile,
    ) -> Self {
        assert!(radius_metres.is_finite() && radius_metres > 0.0);
        assert!(surface_detail_scale <= coarsest_detail_scale);
        Self {
            origin_snapshot,
            frame_snapshot,
            radius_metres,
            current_scale,
            coarsest_detail_scale,
            surface_detail_scale,
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
        let mut radius = self.radius_metres + self.maximum_outward_macro_relief_metres();
        if self.surface_detail_scale <= self.coarsest_detail_scale {
            for raw in self.surface_detail_scale.exponent()..=self.coarsest_detail_scale.exponent() {
                let level = SpatialScale::new(raw).expect("validated celestial semantic detail scale");
                radius += self.detail_amplitude_native(level).abs() * level.metres_per_native();
            }
        }
        radius
    }

pub(crate) fn prepare_local_sampler(
        self,
        _world_origin: VoxelQueryPosition,
        chunk_origin: VoxelQueryPosition,
    ) -> Option<PreparedProceduralCelestialBody> {
        // Resolve the large canonical position once per chunk into body-local
        // physical metres. S0 is only a numerically convenient measurement
        // chart here; it does not own terrain detail.
        let chunk_origin_local_metres = self
            .frame_snapshot
            .world_to_local_metres(
                &self.origin_snapshot,
                &chunk_origin.usf(),
                SpatialScale::ZERO,
                f64::MAX,
            )
            .ok()?;

        Some(PreparedProceduralCelestialBody {
            body: self,
            chunk_origin_local_metres,
        })
    }

    pub(crate) fn prepare_presentation_sampler(
        self,
    ) -> PreparedCelestialPresentationBody {
        PreparedCelestialPresentationBody::new(self)
    }

    pub(crate) fn cave_void_may_intersect_local_aabb(
        self,
        center_local_metres: DVec3,
        half_extent_metres: DVec3,
    ) -> bool {
        match self.profile {
            CelestialBodyProfile::Rocky => {
                rocky_cave_void_may_intersect_aabb(
                    center_local_metres,
                    half_extent_metres,
                    self.seed,
                )
            }
            CelestialBodyProfile::Lunar | CelestialBodyProfile::Stellar => false,
        }
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

    // resolution-filtered-celestial-surface-v1
    /// Body-local semantic surface point at full canonical terrain bandwidth.
    pub(crate) fn surface_local_metres(
        self,
        direction: Vec3,
    ) -> Result<DVec3, UsfPositionError> {
        self.surface_local_metres_through(direction, self.surface_detail_scale)
    }

    /// Same semantic field, accumulated only through one authored residual band.
    pub(crate) fn surface_local_metres_through(
        self,
        direction: Vec3,
        through_scale: SpatialScale,
    ) -> Result<DVec3, UsfPositionError> {
        let local_direction = normalized_direction(direction);
        let radius =
            self.semantic_surface_radius_metres_through(local_direction, through_scale)?;
        Ok(dvec(local_direction) * radius)
    }

    /// Canonical surface point including every semantic detail band.
    pub fn surface_position(self, direction: Vec3) -> Result<UsfPosition, UsfPositionError> {
        let local_direction = normalized_direction(direction);
        let world_direction = normalized_direction(
            self.frame_snapshot.local_direction_to_world(local_direction),
        );
        let radius = self.semantic_surface_radius_metres(local_direction)?;
        self.origin_snapshot
            .translated_metres_f64(dvec(world_direction) * radius)
    }

    /// Conservative inward support for non-radial volumetric surfaces.
    pub(crate) fn volumetric_surface_inward_support_metres(self) -> f64 {
        match self.profile {
            CelestialBodyProfile::Rocky => CAVE_MAX_DEPTH_METRES,
            CelestialBodyProfile::Lunar | CelestialBodyProfile::Stellar => 0.0,
        }
    }

    pub(crate) fn outer_signed_distance_local_metres(
        self,
        local_point_metres: DVec3,
    ) -> Option<f64> {
        self.outer_signed_distance_local_metres_through(
            local_point_metres,
            self.surface_detail_scale,
        )
    }

    #[inline]
    fn outer_signed_distance_and_radial_local_metres_through(
        self,
        local_point_metres: DVec3,
        through_scale: SpatialScale,
    ) -> Option<(f64, f64)> {
        self.outer_signed_distance_and_radial_local_metres_through_with_cache(
            local_point_metres,
            through_scale,
            None,
        )
    }

    #[inline]
    fn outer_signed_distance_and_radial_local_metres_through_with_cache(
        self,
        local_point_metres: DVec3,
        through_scale: SpatialScale,
        canonical_noise_cache: Option<&SemanticNoiseCornerCache>,
    ) -> Option<(f64, f64)> {
        let radial = local_point_metres.length();
        if !radial.is_finite() || radial <= f64::EPSILON {
            return None;
        }

        let direction = Vec3::new(
            (local_point_metres.x / radial) as f32,
            (local_point_metres.y / radial) as f32,
            (local_point_metres.z / radial) as f32,
        )
        .normalize_or_zero();
        if direction == Vec3::ZERO {
            return None;
        }

        let surface_radius = self
            .semantic_surface_radius_metres_through_with_cache(
                direction,
                through_scale,
                canonical_noise_cache,
            )
            .ok()?;
        Some((radial - surface_radius, radial))
    }

    pub(crate) fn outer_signed_distance_local_metres_through(
        self,
        local_point_metres: DVec3,
        through_scale: SpatialScale,
    ) -> Option<f64> {
        self.outer_signed_distance_and_radial_local_metres_through(
            local_point_metres,
            through_scale,
        )
        .map(|(signed_distance, _)| signed_distance)
    }

    pub(crate) fn signed_distance_local_metres(
        self,
        local_point_metres: DVec3,
    ) -> Option<f64> {
        self.signed_distance_local_metres_through(
            local_point_metres,
            self.surface_detail_scale,
            true,
        )
    }

    fn signed_distance_local_metres_with_detail_cache(
        self,
        local_point_metres: DVec3,
        cache: &SemanticNoiseCornerCache,
        include_caves: bool,
    ) -> Option<f64> {
        let (outer_sdf, radial) = self
            .outer_signed_distance_and_radial_local_metres_through_with_cache(
                local_point_metres,
                self.surface_detail_scale,
                Some(cache),
            )?;

        if !include_caves || self.profile != CelestialBodyProfile::Rocky {
            return Some(outer_sdf);
        }

        let outer_surface_radius_metres = radial - outer_sdf;
        let void_sdf =
            rocky_cave_void_signed_distance_metres_with_radial(
                local_point_metres,
                radial,
                outer_surface_radius_metres,
                self.seed,
            );

        Some(outer_sdf.max(-void_sdf))
    }


    pub(crate) fn signed_distance_local_metres_through(
        self,
        local_point_metres: DVec3,
        through_scale: SpatialScale,
        include_caves: bool,
    ) -> Option<f64> {
        // presentation-extract-hotpath-v1
        // Outer evaluation already paid for radial length. Preserve it through
        // cave composition instead of repeating the f64 square root.
        let (outer_sdf, radial) =
            self.outer_signed_distance_and_radial_local_metres_through(
                local_point_metres,
                through_scale,
            )?;

        if !include_caves || self.profile != CelestialBodyProfile::Rocky {
            return Some(outer_sdf);
        }

        let outer_surface_radius_metres = radial - outer_sdf;
        let void_sdf =
            rocky_cave_void_signed_distance_metres_with_radial(
                local_point_metres,
                radial,
                outer_surface_radius_metres,
                self.seed,
            );

        Some(outer_sdf.max(-void_sdf))
    }

    pub(crate) fn volumetric_void_signed_distance_local_metres(
        self,
        local_point_metres: DVec3,
    ) -> Option<f64> {
        if self.profile != CelestialBodyProfile::Rocky {
            return None;
        }
        let radial = local_point_metres.length();
        if !radial.is_finite() || radial <= f64::EPSILON {
            return None;
        }
        let direction = Vec3::new(
            (local_point_metres.x / radial) as f32,
            (local_point_metres.y / radial) as f32,
            (local_point_metres.z / radial) as f32,
        )
        .normalize_or_zero();
        let surface_radius = self.semantic_surface_radius_metres(direction).ok()?;
        Some(rocky_cave_void_signed_distance_metres(
            local_point_metres,
            surface_radius,
            self.seed,
        ))
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


    #[inline]
pub(crate) fn field_sample_local_metres(
        self,
        local_point_metres: DVec3,
    ) -> Option<CelestialFieldSample> {
        let signed_distance_metres =
            self.signed_distance_local_metres(local_point_metres)?;
        let material = if signed_distance_metres < 0.0 {
            VoxelMaterialId::ROCK
        } else {
            VoxelMaterialId::VOID
        };
        Some(CelestialFieldSample::new(
            signed_distance_metres,
            material,
        ))
    }

    #[inline]
    fn sample_body_local_metres(
        self,
        local_point_metres: DVec3,
    ) -> VoxelSample {
        let Some(sample) = self.field_sample_local_metres(local_point_metres)
        else {
            return VoxelSample::empty(EMPTY_DISTANCE);
        };

        let distance_native =
            sample.signed_distance_metres() / self.current_scale.metres_per_native();
        let distance = distance_native
            .clamp(-f64::from(EMPTY_DISTANCE), f64::from(EMPTY_DISTANCE))
            as f32;

        VoxelSample::new(distance, sample.material())
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
fn semantic_surface_radius_metres(
        self,
        direction: Vec3,
    ) -> Result<f64, UsfPositionError> {
        self.semantic_surface_radius_metres_through(
            direction,
            self.surface_detail_scale,
        )
    }

    fn semantic_surface_radius_metres_through(
        self,
        direction: Vec3,
        through_scale: SpatialScale,
    ) -> Result<f64, UsfPositionError> {
        self.semantic_surface_radius_metres_through_with_cache(
            direction,
            through_scale,
            None,
        )
    }

    fn semantic_surface_radius_metres_through_with_cache(
        self,
        direction: Vec3,
        through_scale: SpatialScale,
        canonical_noise_cache: Option<&SemanticNoiseCornerCache>,
    ) -> Result<f64, UsfPositionError> {
        let direction = normalized_direction(direction);
        let requested = through_scale.exponent().clamp(
            self.surface_detail_scale.exponent(),
            self.coarsest_detail_scale.exponent(),
        );
        let through_scale =
            SpatialScale::new(requested).expect("clamped semantic detail scale is valid");
        let floor = through_scale.exponent();
        let root = self.coarsest_detail_scale.exponent();

        let mut radius = self.radius_metres
            + self.macro_surface_displacement_metres(direction, through_scale);

        let coarse_lower = floor.max(1);
        if coarse_lower <= root {
            for raw in (coarse_lower..=root).rev() {
                let level = SpatialScale::new(raw)
                    .expect("validated coarse celestial semantic detail scale");
                radius += self.coarse_detail_band_native(direction, level)
                    * level.metres_per_native();
            }
        }

        let fine_upper = root.min(0);
        if floor <= fine_upper {
            let local_reference_metres = dvec(direction) * radius;
            for raw in (floor..=fine_upper).rev() {
                let level = SpatialScale::new(raw)
                    .expect("validated fine celestial semantic detail scale");
                let noise = match canonical_noise_cache {
                    Some(cache) => {
                        self.canonical_detail_noise_at_local_metres_cached(
                            local_reference_metres,
                            level,
                            cache,
                        )?
                    }
                    None => {
                        self.canonical_detail_noise_at_local_metres(
                            local_reference_metres,
                            level,
                        )?
                    }
                };
                radius += f64::from(noise)
                    * self.detail_amplitude_native(level)
                    * level.metres_per_native();
            }
        }
        Ok(radius)
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

    #[inline]
    fn canonical_detail_noise_at_local_metres(
        self,
        local_position_metres: DVec3,
        level: SpatialScale,
    ) -> Result<f32, UsfPositionError> {
        let native =
            local_position_metres / level.metres_per_native();
        let canonical = UsfPosition::from_scale_native_f64(
            native,
            level,
            level,
        )?;
        let point = VoxelQueryPosition::new(canonical);
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

    #[inline]
    fn canonical_detail_noise_at_local_metres_cached(
        self,
        local_position_metres: DVec3,
        level: SpatialScale,
        cache: &SemanticNoiseCornerCache,
    ) -> Result<f32, UsfPositionError> {
        let metres_per_native = level.metres_per_native();
        let (_, _, _, salt) = self.detail_parameters();
        let seed = scale_layer_seed(self.seed ^ salt, level);
        self.canonical_detail_noise_at_local_metres_cached_prepared(
            local_position_metres,
            level,
            metres_per_native,
            seed,
            cache,
        )
    }

    #[inline]
    fn canonical_detail_noise_at_local_metres_cached_prepared(
        self,
        local_position_metres: DVec3,
        level: SpatialScale,
        metres_per_native: f64,
        seed: u32,
        cache: &SemanticNoiseCornerCache,
    ) -> Result<f32, UsfPositionError> {
        let native =
            local_position_metres / metres_per_native;
        let canonical = UsfPosition::from_scale_native_f64(
            native,
            level,
            level,
        )?;
        let point = VoxelQueryPosition::new(canonical);
        let broad = semantic_value_noise_3d_cached(
            point,
            CANONICAL_DETAIL_CELL_NATIVE,
            seed ^ 0xA341_316C,
            cache,
        );
        let fine = semantic_value_noise_3d_cached(
            point,
            CANONICAL_DETAIL_FINE_CELL_NATIVE,
            seed ^ 0xC801_3EA4,
            cache,
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
            // Rocky planetary morphology is now owned by explicit
            // semantic bands. This generic stack is residual texture beneath
            // those bands instead of a 25 km S+6 pseudo-macro surface.
            CelestialBodyProfile::Rocky => (0.72, 0.0025, 1.65, 0x524F_434B),
            CelestialBodyProfile::Stellar => (0.48, 0.008, 1.30, 0x5354_4152),
        }
    }

    fn macro_surface_displacement_metres(
        self,
        direction: Vec3,
        through_scale: SpatialScale,
    ) -> f64 {
        match self.profile {
            CelestialBodyProfile::Lunar => {
                self.radius_metres * f64::from(lunar_macro_relative_relief(direction))
            }
            CelestialBodyProfile::Rocky => {
                rocky_surface_displacement_metres(
                    direction,
                    through_scale,
                    self.seed,
                ) + rocky_exaggerated_relief_metres_through(
                    direction,
                    self.seed,
                    through_scale,
                )
            }
            CelestialBodyProfile::Stellar => {
                self.radius_metres
                    * f64::from(stellar_macro_relative_relief(direction, self.seed))
            }
        }
    }

    fn maximum_outward_macro_relief_metres(self) -> f64 {
        match self.profile {
            // Two broad waves plus the deliberately conservative assumption
            // that every crater rim can contribute at once. Bowl depth is
            // inward and therefore irrelevant to an outer bound.
            CelestialBodyProfile::Lunar => {
                let crater_depth_sum =
                    0.0100 + 0.0070 + 0.0060 + 0.0048
                    + 0.0040 + 0.0034 + 0.0028 + 0.0024;
                self.radius_metres
                    * (0.0014 + 0.0008 + crater_depth_sum * 0.28)
            }
            CelestialBodyProfile::Rocky => {
                rocky_maximum_outward_displacement_metres()
                    + ROCKY_EXAGGERATED_OUTWARD_BOUND_METRES
            }
            CelestialBodyProfile::Stellar => self.radius_metres * 0.00035,
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

const ROCKY_EXAGGERATED_OUTWARD_BOUND_METRES: f64 = 55_000.0;

/// Deliberately unmistakable development morphology layered onto the ordinary
/// rocky semantic bands.
///
/// This is canonical physical terrain, not presentation displacement. Every
/// consumer of `semantic_surface_radius_metres()` therefore sees the same
/// mountains/valleys: dense voxels, travel boundaries, clipmap and regional
/// presentation.
///
/// The amplitudes are intentionally obvious while the world-generation
/// stack is being exercised, but remain within a sane planetary test range. Once end-to-end terrain realization is healthy,
/// this can become an authored profile parameter instead of a hardcoded dev
/// morphology layer.
fn rocky_exaggerated_relief_metres_through(
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
        let alpine_carrier = value_noise_3d(
            direction * 10.0 + Vec3::new(-17.2, 6.9, 12.4),
            seed ^ 0x414C_504E,
        );
        let alpine_ridge =
            (1.0 - alpine_carrier.abs()).max(0.0).powi(9);
        let alpine_envelope = (
            value_noise_3d(
                direction * 3.7 + Vec3::new(3.1, 19.6, -8.8),
                seed ^ 0xA1F1_4E55,
            ) * 0.5
                + 0.5
        )
            .clamp(0.0, 1.0);

        let canyon_carrier = value_noise_3d(
            direction * 15.0 + Vec3::new(14.2, -4.7, -16.5),
            seed ^ 0x4341_4E59,
        );
        let canyon_line =
            (1.0 - canyon_carrier.abs()).max(0.0).powi(9);
        let canyon_envelope = (
            value_noise_3d(
                direction * 4.3 + Vec3::new(-9.9, 5.4, 21.1),
                seed ^ 0x5249_4654,
            ) * 0.5
                + 0.5
        )
            .clamp(0.0, 1.0);

        let massif_carrier = value_noise_3d(
            direction * 14.5 + Vec3::new(22.4, 7.7, -3.6),
            seed ^ 0x4D41_5353,
        );
        let massif_cross =
            (1.0 - massif_carrier.abs()).max(0.0).powi(8);
        let massif = alpine_ridge * massif_cross;

        relief += f64::from(alpine_ridge * alpine_envelope) * 30_000.0
            + f64::from(massif * alpine_envelope) * 16_000.0
            - f64::from(canyon_line * canyon_envelope) * 24_000.0;
    }

    if through_scale <= SpatialScale::new(2).expect("S+2 is valid") {
        let serration = value_noise_3d(
            direction * 32.0 + Vec3::new(1.7, -13.3, 9.2),
            seed ^ 0x5345_5252,
        );
        let sharp_serration =
            serration.signum() * serration.abs().powi(2);
        relief += f64::from(sharp_serration) * 5_000.0;
    }

    relief.clamp(-38_000.0, 48_000.0)
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
    fn prepared_presentation_sampler_matches_reference_sdf_bits() {
        let body = ProceduralCelestialBody::new(
            earth_center(),
            UsfSemanticFrame::identity(),
            6_371_000.0,
            SpatialScale::ZERO,
            SpatialScale::new(6).unwrap(),
            SpatialScale::ZERO,
            0x4541_5254,
            CelestialBodyProfile::Rocky,
        );
        let prepared = body.prepare_presentation_sampler();

        for point in [
            DVec3::new(0.0, 6_371_025.0, 0.0),
            DVec3::new(37.0, 6_370_900.0, -83.0),
            DVec3::new(1_234.5, 6_371_111.25, -777.75),
            DVec3::new(-9_876.25, 6_369_500.0, 4_321.5),
        ] {
            let reference =
                body.signed_distance_local_metres(point).unwrap();
            let optimized =
                prepared.signed_distance_local_metres(point).unwrap();
            assert_eq!(
                reference.to_bits(),
                optimized.to_bits(),
                "prepared presentation SDF changed canonical field at {point:?}",
            );
        }
    }

    #[test]
    fn direct_body_local_detail_noise_matches_hierarchical_reference() {
        let body = ProceduralCelestialBody::new(
            earth_center(),
            UsfSemanticFrame::identity(),
            6_371_000.0,
            SpatialScale::ZERO,
            SpatialScale::new(6).unwrap(),
            SpatialScale::ZERO,
            0x4541_5254,
            CelestialBodyProfile::Rocky,
        );
        let level = SpatialScale::ZERO;

        for point in [
            DVec3::new(0.0, 6_371_000.0, 0.0),
            DVec3::new(
                1_234_567.0,
                5_432_100.0,
                -2_345_678.0,
            ),
            DVec3::new(
                -4_321_000.25,
                2_111_000.5,
                3_777_000.75,
            ),
        ] {
            let hierarchical = UsfPosition::zero(
                body.origin_snapshot.leaf_scale(),
            )
            .translated_metres_f64(point)
            .unwrap();
            let reference = body
                .canonical_detail_noise_at(
                    hierarchical,
                    level,
                )
                .unwrap();
            let direct = body
                .canonical_detail_noise_at_local_metres(
                    point,
                    level,
                )
                .unwrap();

            assert_eq!(
                reference.to_bits(),
                direct.to_bits(),
                "direct body-local noise changed canonical result at {point:?}",
            );
        }
    }

    #[test]
    fn outer_sdf_is_zero_on_canonical_surface() {
        let body = ProceduralCelestialBody::new(
            earth_center(),
            UsfSemanticFrame::identity(),
            6_371_000.0,
            SpatialScale::ZERO,
            SpatialScale::new(6).unwrap(),
            SpatialScale::ZERO,
            0x4541_5254,
            CelestialBodyProfile::Rocky,
        );

        for direction in [
            Vec3::Y,
            Vec3::new(0.31, 0.83, -0.46).normalize(),
            Vec3::new(-0.72, 0.22, 0.66).normalize(),
        ] {
            let surface = body.surface_local_metres(direction).unwrap();
            let sdf = body
                .outer_signed_distance_local_metres(surface)
                .unwrap();
            assert!(
                sdf.abs() < 1.0e-5,
                "outer SDF disagrees with canonical radial surface: {sdf}"
            );
        }
    }

    #[test]
    fn canonical_field_sample_is_independent_of_realization_scale() {
        let direction = Vec3::new(0.37, 0.81, -0.45).normalize();
        let s0 = ProceduralCelestialBody::new(
            earth_center(),
            UsfSemanticFrame::identity(),
            6_371_000.0,
            SpatialScale::ZERO,
            SpatialScale::new(6).unwrap(),
            SpatialScale::ZERO,
            0x4541_5254,
            CelestialBodyProfile::Rocky,
        );
        let s6 = ProceduralCelestialBody::new(
            earth_center(),
            UsfSemanticFrame::identity(),
            6_371_000.0,
            SpatialScale::new(6).unwrap(),
            SpatialScale::new(6).unwrap(),
            SpatialScale::ZERO,
            0x4541_5254,
            CelestialBodyProfile::Rocky,
        );

        let surface = s0.surface_local_metres(direction).unwrap();
        let local_point = surface
            - DVec3::new(
                f64::from(direction.x),
                f64::from(direction.y),
                f64::from(direction.z),
            ) * 25.0;

        let a = s0.field_sample_local_metres(local_point).unwrap();
        let b = s6.field_sample_local_metres(local_point).unwrap();

        assert_eq!(a.material(), b.material());
        assert!(
            (a.signed_distance_metres() - b.signed_distance_metres()).abs()
                < 1.0e-9,
            "realization Scale changed canonical field truth: a={a:?}, b={b:?}"
        );
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
                coarsest, SpatialScale::ZERO,
                0x4541_5254,
                CelestialBodyProfile::Rocky,
            );
            let reconstructed =
                body.radius_native_f64() * scale.metres_per_native();
            assert!((reconstructed - radius_metres).abs() < 1.0e-6);
        }
    }

    #[test]
    fn rocky_canonical_surface_is_unmistakably_non_spherical() {
        let body = ProceduralCelestialBody::new(
            earth_center(),
            UsfSemanticFrame::identity(),
            6_371_000.0,
            SpatialScale::ZERO,
            SpatialScale::new(6).unwrap(),
            SpatialScale::ZERO,
            0x4541_5254,
            CelestialBodyProfile::Rocky,
        );

        let mut minimum = f64::INFINITY;
        let mut maximum = f64::NEG_INFINITY;
        for index in 0..512 {
            let i = index as f32 + 0.5;
            let n = 512.0_f32;
            let y = 1.0 - 2.0 * i / n;
            let horizontal = (1.0 - y * y).max(0.0).sqrt();
            let golden_ratio = (1.0 + 5.0_f32.sqrt()) * 0.5;
            let theta = std::f32::consts::TAU * index as f32 / golden_ratio;
            let direction =
                Vec3::new(theta.cos() * horizontal, y, theta.sin() * horizontal)
                    .normalize();
            let radius = body.surface_local_metres(direction).unwrap().length();
            let relief = radius - body.radius_metres();
            minimum = minimum.min(relief);
            maximum = maximum.max(relief);
        }

        assert!(
            maximum >= 15_000.0,
            "expected high canonical mountains, max={maximum:.1} m"
        );
        assert!(
            minimum <= -10_000.0,
            "expected deep canonical valleys/canyons, min={minimum:.1} m"
        );
        assert!(
            maximum - minimum >= 30_000.0,
            "expected dramatic canonical relief span, got {:.1} m",
            maximum - minimum,
        );
    }

    #[test]
    fn realization_scale_never_changes_semantic_surface_geometry() {
        let radius_metres = 6_371_000.0;
        let coarsest = SpatialScale::new(6).unwrap();
        let surface_floor = SpatialScale::ZERO;
        let direction = Vec3::new(0.37, 0.81, -0.45).normalize();
        let reference = ProceduralCelestialBody::new(
            earth_center(),
            UsfSemanticFrame::identity(),
            radius_metres,
            SpatialScale::ZERO,
            coarsest,
            surface_floor,
            0x4541_5254,
            CelestialBodyProfile::Rocky,
        )
        .surface_position(direction)
        .unwrap();

        for raw in 0..=6 {
            let realization_scale = SpatialScale::new(raw).unwrap();
            let actual = ProceduralCelestialBody::new(
                earth_center(),
                UsfSemanticFrame::identity(),
                radius_metres,
                realization_scale,
                coarsest,
                surface_floor,
                0x4541_5254,
                CelestialBodyProfile::Rocky,
            )
            .surface_position(direction)
            .unwrap();
            let delta = actual
                .relative_at_scale_bounded_f64(&reference, SpatialScale::ZERO, 0.01)
                .unwrap();
            assert!(
                delta.length() < 1.0e-6,
                "S{raw:+} realization changed semantic surface by {delta:?}",
            );
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
            SpatialScale::new(6).unwrap(), SpatialScale::ZERO,
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
            SpatialScale::new(6).unwrap(), SpatialScale::ZERO,
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
