//! Canonical celestial field and its bounded runtime sampling adapter.

use super::*;

impl PreparedCelestialVoxelSampler {
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
        ) * self.body.realization_scale.metres_per_native();
        let local_delta_metres =
            self.body.frame_snapshot.orientation().conjugate() * world_delta_metres;
        let local_point_metres = self.chunk_origin_local_metres + local_delta_metres;

        self.body.sample_body_local_metres(local_point_metres)
    }
}

impl CelestialFieldRealization {
    pub fn new(
        origin_snapshot: UsfPosition,
        frame_snapshot: UsfSemanticFrame,
        radius_metres: f64,
        realization_scale: SpatialScale,
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
            realization_scale,
            coarsest_detail_scale,
            surface_detail_scale,
            seed,
            profile,
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

    pub const fn profile(self) -> CelestialBodyProfile {
        self.profile
    }

    /// Conservative radial interval for the canonical outer surface.
    ///
    /// Bounds come from the same authored morphology and residual bands as
    /// field evaluation. Caves can add internal boundaries below this interval;
    /// callers must extend the lower bound by their inward support.
    pub(crate) fn conservative_surface_radius_bounds_metres(self) -> (f64, f64) {
        let mut residual_bound = 0.0;
        if self.surface_detail_scale <= self.coarsest_detail_scale {
            for raw in self.surface_detail_scale.exponent()..=self.coarsest_detail_scale.exponent()
            {
                let level =
                    SpatialScale::new(raw).expect("validated celestial semantic detail scale");
                residual_bound +=
                    self.detail_amplitude_native(level).abs() * level.metres_per_native();
            }
        }
        (
            (self.radius_metres - self.maximum_inward_macro_relief_metres() - residual_bound)
                .max(0.0),
            self.radius_metres + self.maximum_outward_macro_relief_metres() + residual_bound,
        )
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
            body: self,
            chunk_origin_local_metres,
        })
    }

    pub(crate) fn prepare_presentation_sampler(self) -> PreparedCelestialPresentationBody {
        PreparedCelestialPresentationBody::new(self)
    }

    pub(crate) fn cave_void_may_intersect_local_aabb(
        self,
        center_local_metres: DVec3,
        half_extent_metres: DVec3,
    ) -> bool {
        match self.profile {
            CelestialBodyProfile::Rocky => rocky_cave_void_may_intersect_aabb(
                center_local_metres,
                half_extent_metres,
                self.seed,
            ),
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

    /// Body-local semantic surface point at full canonical terrain bandwidth.
    pub(crate) fn surface_local_metres(self, direction: Vec3) -> Result<DVec3, UsfPositionError> {
        self.surface_local_metres_through(direction, self.surface_detail_scale)
    }

    /// Same semantic field, accumulated only through one authored residual band.
    pub(crate) fn surface_local_metres_through(
        self,
        direction: Vec3,
        through_scale: SpatialScale,
    ) -> Result<DVec3, UsfPositionError> {
        let local_direction = normalized_direction(direction);
        let radius = self.semantic_surface_radius_metres_through(local_direction, through_scale)?;
        Ok(dvec(local_direction) * radius)
    }

    /// Canonical surface point including every semantic detail band.
    pub fn surface_position(self, direction: Vec3) -> Result<UsfPosition, UsfPositionError> {
        let local_direction = normalized_direction(direction);
        let world_direction = normalized_direction(
            self.frame_snapshot
                .local_direction_to_world(local_direction),
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

    pub(crate) fn signed_distance_local_metres(self, local_point_metres: DVec3) -> Option<f64> {
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
        let void_sdf = rocky_cave_void_signed_distance_metres_with_radial(
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
        // Outer evaluation already paid for radial length. Preserve it through
        // cave composition instead of repeating the f64 square root.
        let (outer_sdf, radial) = self.outer_signed_distance_and_radial_local_metres_through(
            local_point_metres,
            through_scale,
        )?;

        if !include_caves || self.profile != CelestialBodyProfile::Rocky {
            return Some(outer_sdf);
        }

        let outer_surface_radius_metres = radial - outer_sdf;
        let void_sdf = rocky_cave_void_signed_distance_metres_with_radial(
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

    #[inline]
    fn sample_body_local_metres(self, local_point_metres: DVec3) -> VoxelSample {
        let Some(sample) = self.field_sample_local_metres(local_point_metres) else {
            return VoxelSample::empty(EMPTY_DISTANCE);
        };

        let distance_native =
            sample.signed_distance_metres() / self.realization_scale.metres_per_native();
        let distance =
            distance_native.clamp(-f64::from(EMPTY_DISTANCE), f64::from(EMPTY_DISTANCE)) as f32;

        VoxelSample::new(distance, sample.material())
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

    /// S1+ remains a conventional radial evaluation. S0 and finer do not.
    fn semantic_surface_radius_metres(self, direction: Vec3) -> Result<f64, UsfPositionError> {
        self.semantic_surface_radius_metres_through(direction, self.surface_detail_scale)
    }

    fn semantic_surface_radius_metres_through(
        self,
        direction: Vec3,
        through_scale: SpatialScale,
    ) -> Result<f64, UsfPositionError> {
        self.semantic_surface_radius_metres_through_with_cache(direction, through_scale, None)
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

        let mut radius =
            self.radius_metres + self.macro_surface_displacement_metres(direction, through_scale);

        let coarse_lower = floor.max(1);
        if coarse_lower <= root {
            for raw in (coarse_lower..=root).rev() {
                let level = SpatialScale::new(raw)
                    .expect("validated coarse celestial semantic detail scale");
                radius +=
                    self.coarse_detail_band_native(direction, level) * level.metres_per_native();
            }
        }

        let fine_upper = root.min(0);
        if floor <= fine_upper {
            let local_reference_metres = dvec(direction) * radius;
            for raw in (floor..=fine_upper).rev() {
                let level =
                    SpatialScale::new(raw).expect("validated fine celestial semantic detail scale");
                let noise = match canonical_noise_cache {
                    Some(cache) => self.canonical_detail_noise_at_local_metres_cached(
                        local_reference_metres,
                        level,
                        cache,
                    )?,
                    None => {
                        self.canonical_detail_noise_at_local_metres(local_reference_metres, level)?
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
        let angular_frequency = (radius_at_level * frequency_factor)
            .max(4.0)
            .min(f64::from(f32::MAX)) as f32;
        let seed = scale_layer_seed(self.seed ^ salt, level);

        f64::from(coarse_residual_sample(direction, angular_frequency, seed))
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
        let broad =
            semantic_value_noise_3d(point, CANONICAL_DETAIL_CELL_NATIVE, seed ^ 0xA341_316C);
        let fine =
            semantic_value_noise_3d(point, CANONICAL_DETAIL_FINE_CELL_NATIVE, seed ^ 0xC801_3EA4);
        Ok(blend_residual_noise(broad, fine))
    }

    #[inline]
    fn canonical_detail_noise_at_local_metres(
        self,
        local_position_metres: DVec3,
        level: SpatialScale,
    ) -> Result<f32, UsfPositionError> {
        let native = local_position_metres / level.metres_per_native();
        let canonical = UsfPosition::from_scale_native_f64(native, level, level)?;
        let point = VoxelQueryPosition::new(canonical);
        let (_, _, _, salt) = self.detail_parameters();
        let seed = scale_layer_seed(self.seed ^ salt, level);
        let broad =
            semantic_value_noise_3d(point, CANONICAL_DETAIL_CELL_NATIVE, seed ^ 0xA341_316C);
        let fine =
            semantic_value_noise_3d(point, CANONICAL_DETAIL_FINE_CELL_NATIVE, seed ^ 0xC801_3EA4);
        Ok(blend_residual_noise(broad, fine))
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
    pub(super) fn canonical_detail_noise_at_local_metres_cached_prepared(
        self,
        local_position_metres: DVec3,
        level: SpatialScale,
        metres_per_native: f64,
        seed: u32,
        cache: &SemanticNoiseCornerCache,
    ) -> Result<f32, UsfPositionError> {
        let native = local_position_metres / metres_per_native;

        //
        // PERFORMANCE INVARIANT:
        // This path intentionally bypasses generic UsfPosition construction and
        // profiling spans for ordinary presentation samples. It is called for
        // every fine residual of every density point; seemingly "cleaner"
        // abstraction here multiplies directly into 9^3 clipmap generation.
        //
        // Keep this path bit-equivalent to semantic_value_noise_3d(...).
        if let Some(prepared) = PreparedSemanticNoisePoint::from_native_f64(native, level)
            && let Some(broad) = semantic_value_noise_3d_cached_prepared(
                prepared,
                CANONICAL_DETAIL_CELL_NATIVE,
                seed ^ 0xA341_316C,
                cache,
            )
            && let Some(fine) = semantic_value_noise_3d_cached_prepared(
                prepared,
                CANONICAL_DETAIL_FINE_CELL_NATIVE,
                seed ^ 0xC801_3EA4,
                cache,
            )
        {
            record_fine_residual_fast_path_completion();
            return Ok(blend_residual_noise(broad, fine));
        }

        record_fine_residual_generic_fallback();

        let canonical = UsfPosition::from_scale_native_f64(native, level, level)?;
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
        Ok(blend_residual_noise(broad, fine))
    }

    pub(super) fn detail_amplitude_native(self, level: SpatialScale) -> f64 {
        let (_, amplitude, growth, _) = self.detail_parameters();
        let depth = i32::from(self.coarsest_detail_scale.exponent() - level.exponent()).max(0);
        amplitude * growth.powi(depth)
    }

    pub(super) fn detail_parameters(self) -> (f64, f64, f64, u32) {
        match self.profile {
            CelestialBodyProfile::Lunar => (0.82, 0.040, 1.70, 0x4C55_4E41),
            // Rocky planetary morphology is now owned by explicit
            // semantic bands. This generic stack is residual texture beneath
            // those bands instead of a 25 km S+6 pseudo-macro surface.
            CelestialBodyProfile::Rocky => (0.72, 0.0025, 1.65, 0x524F_434B),
            CelestialBodyProfile::Stellar => (0.48, 0.008, 1.30, 0x5354_4152),
        }
    }

    pub(super) fn macro_surface_displacement_metres(
        self,
        direction: Vec3,
        through_scale: SpatialScale,
    ) -> f64 {
        match self.profile {
            CelestialBodyProfile::Lunar => {
                self.radius_metres * f64::from(lunar_macro_relative_relief(direction))
            }
            CelestialBodyProfile::Rocky => {
                rocky_macro_displacement_metres(direction, through_scale, self.seed)
            }
            CelestialBodyProfile::Stellar => {
                self.radius_metres * f64::from(stellar_macro_relative_relief(direction, self.seed))
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
                    0.0100 + 0.0070 + 0.0060 + 0.0048 + 0.0040 + 0.0034 + 0.0028 + 0.0024;
                self.radius_metres * (0.0014 + 0.0008 + crater_depth_sum * 0.28)
            }
            CelestialBodyProfile::Rocky => rocky_macro_relief_bounds_metres().1,
            CelestialBodyProfile::Stellar => self.radius_metres * 0.00035,
        }
    }

    fn maximum_inward_macro_relief_metres(self) -> f64 {
        match self.profile {
            CelestialBodyProfile::Lunar => {
                let crater_depth_sum =
                    0.0100 + 0.0070 + 0.0060 + 0.0048 + 0.0040 + 0.0034 + 0.0028 + 0.0024;
                self.radius_metres * (0.0014 + 0.0008 + crater_depth_sum)
            }
            CelestialBodyProfile::Rocky => rocky_macro_relief_bounds_metres().0,
            CelestialBodyProfile::Stellar => self.radius_metres * 0.00035,
        }
    }
}
