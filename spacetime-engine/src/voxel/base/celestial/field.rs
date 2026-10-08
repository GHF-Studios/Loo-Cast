//! Canonical celestial field and its bounded runtime sampling adapter.

use super::*;

impl PreparedCelestialVoxelSampler {
    #[inline]
    pub(crate) fn sample(&self, chunk_local: Vec3) -> VoxelSample {
        // Body/Scale invariants are resolved once in `prepare_local_sampler`.
        // The inner 12^3 dense-lattice loop only converts a bounded delta and
        // enters the prepared canonical evaluator.
        let world_delta_metres = DVec3::new(
            f64::from(chunk_local.x),
            f64::from(chunk_local.y),
            f64::from(chunk_local.z),
        ) * self.metres_per_native;
        let local_point_metres =
            self.chunk_origin_local_metres + self.world_to_local * world_delta_metres;

        let Some(distance_metres) = self.body.signed_distance_local_metres(local_point_metres)
        else {
            return VoxelSample::empty(EMPTY_DISTANCE);
        };
        let distance = (distance_metres * self.native_per_metre)
            .clamp(-f64::from(EMPTY_DISTANCE), f64::from(EMPTY_DISTANCE))
            as f32;

        VoxelSample::new(
            distance,
            if distance_metres < 0.0 {
                VoxelMaterialId::ROCK
            } else {
                VoxelMaterialId::VOID
            },
        )
    }
}

impl CelestialFieldRealization {
    pub fn new(
        origin_snapshot: UsfPosition,
        frame_snapshot: UsfSemanticFrame,
        radius_metres: f64,
        realization_scale: SpatialScale,
    ) -> Self {
        assert!(radius_metres.is_finite() && radius_metres > 0.0);
        Self {
            origin_snapshot,
            frame_snapshot,
            radius_metres,
            realization_scale,
        }
    }

    pub const fn origin_snapshot(self) -> UsfPosition {
        self.origin_snapshot
    }
    pub const fn frame_snapshot(self) -> UsfSemanticFrame {
        self.frame_snapshot
    }
    pub const fn radius_metres(self) -> f64 {
        self.radius_metres
    }

    pub fn radius_native_f64(self) -> f64 {
        self.realization_scale
            .metres_to_native_f64(self.radius_metres)
    }

    /// Conservative radial interval for the canonical outer surface.
    pub(crate) fn conservative_surface_radius_bounds_metres(self) -> (f64, f64) {
        (self.radius_metres, self.radius_metres)
    }

    /// Broadphase upper bound for collision queries.
    pub(crate) fn conservative_outer_radius_metres(self) -> f64 {
        self.conservative_surface_radius_bounds_metres().1
    }

    pub(crate) fn prepare_local_sampler(
        self,
        _world_origin: VoxelQueryPosition,
        chunk_origin: VoxelQueryPosition,
    ) -> Option<PreparedCelestialVoxelSampler> {
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

        Some(PreparedCelestialVoxelSampler {
            body: self.prepare_body_sampler(),
            chunk_origin_local_metres,
            world_to_local: self.frame_snapshot.orientation().conjugate(),
            metres_per_native: self.realization_scale.metres_per_native(),
            native_per_metre: self.realization_scale.native_per_metre(),
        })
    }

    pub(crate) fn prepare_body_sampler(self) -> PreparedCelestialBodySampler {
        PreparedCelestialBodySampler::new(self)
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

    /// Body-local semantic surface point.
    pub(crate) fn surface_local_metres(self, direction: Vec3) -> Result<DVec3, UsfPositionError> {
        Ok(dvec(normalized_direction(direction)) * self.radius_metres)
    }

    /// Canonical surface point.
    pub fn surface_position(self, direction: Vec3) -> Result<UsfPosition, UsfPositionError> {
        let local_direction = normalized_direction(direction);
        let world_direction = normalized_direction(
            self.frame_snapshot
                .local_direction_to_world(local_direction),
        );
        self.origin_snapshot
            .translated_metres_f64(dvec(world_direction) * self.radius_metres)
    }

    pub(crate) fn outer_signed_distance_local_metres(
        self,
        local_point_metres: DVec3,
    ) -> Option<f64> {
        let radial = local_point_metres.length();
        radial.is_finite().then_some(radial - self.radius_metres)
    }

    pub(crate) fn signed_distance_local_metres(self, local_point_metres: DVec3) -> Option<f64> {
        self.outer_signed_distance_local_metres(local_point_metres)
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
            .relative_at_scale_bounded(&surface, self.realization_scale, max_abs_native.max(0.0))
            .ok()?;
        Some((surface, world_up, relative.dot(world_up)))
    }

    #[inline]
    pub(crate) fn field_sample_local_metres(
        self,
        local_point_metres: DVec3,
    ) -> Option<CelestialFieldSample> {
        let signed_distance_metres = self.signed_distance_local_metres(local_point_metres)?;
        let material = if signed_distance_metres < 0.0 {
            VoxelMaterialId::ROCK
        } else {
            VoxelMaterialId::VOID
        };
        Some(CelestialFieldSample::new(signed_distance_metres, material))
    }

    fn direction_to(self, point: &UsfPosition) -> Option<Vec3> {
        let relative_world = point
            .relative_at_scale_bounded_f64(&self.origin_snapshot, SpatialScale::ZERO, f64::MAX)
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
            self.frame_snapshot
                .world_direction_to_local(world_direction),
        ))
    }
}
