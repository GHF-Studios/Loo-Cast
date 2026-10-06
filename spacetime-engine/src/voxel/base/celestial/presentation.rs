//! Prepared, cache-backed celestial presentation sampling.

use super::*;

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
    fn new(body: CelestialFieldRealization) -> Self {
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
            let (_, base_amplitude, growth, _) = body.detail_parameters();
            let mut amplitude_native = base_amplitude;
            for raw in (coarse_lower..=root).rev() {
                let level = SpatialScale::new(raw)
                    .expect("validated coarse celestial semantic detail scale");
                let metres_per_native = level.metres_per_native();
                let radius_at_level = body.radius_metres * level.native_per_metre();
                let angular_frequency = (radius_at_level * frequency_factor)
                    .max(4.0)
                    .min(f64::from(f32::MAX)) as f32;
                let seed = scale_layer_seed(body.seed ^ salt, level);

                stack.coarse[stack.coarse_len] = Some(PreparedCelestialResidualBand {
                    level,
                    metres_per_native,
                    amplitude_native,
                    angular_frequency,
                    seed,
                });
                stack.coarse_len += 1;
                amplitude_native *= growth;
            }
        }

        let fine_upper = root.min(0);
        if floor <= fine_upper {
            let (_, base_amplitude, growth, _) = body.detail_parameters();
            let mut amplitude_native = base_amplitude;
            for _ in fine_upper..root {
                amplitude_native *= growth;
            }
            for raw in (floor..=fine_upper).rev() {
                let level =
                    SpatialScale::new(raw).expect("validated fine celestial semantic detail scale");
                stack.fine[stack.fine_len] = Some(PreparedCelestialResidualBand {
                    level,
                    metres_per_native: level.metres_per_native(),
                    amplitude_native,
                    angular_frequency: 0.0,
                    seed: scale_layer_seed(body.seed ^ salt, level),
                });
                stack.fine_len += 1;
                amplitude_native *= growth;
            }
        }

        stack
    }
}

//
// Keep the 9^3 first-touch lattice itself as the profiling boundary. Per-sample
// nested Tracy zones and sampled Instant accounting distorted the kernel being
// measured. Semantic arithmetic and per-sample accumulation order are unchanged.
#[derive(Debug)]
pub(crate) struct PreparedCelestialPresentationBody {
    body: CelestialFieldRealization,
    canonical_noise_cache: SemanticNoiseCornerCache,
    residual: PreparedCelestialResidualStack,
}

impl PreparedCelestialPresentationBody {
    pub(crate) fn new(body: CelestialFieldRealization) -> Self {
        Self {
            body,
            canonical_noise_cache: SemanticNoiseCornerCache::new(),
            residual: PreparedCelestialResidualStack::new(body),
        }
    }

    #[inline]
    fn pre_fine_surface_radius_metres(&self, direction: Vec3) -> Result<f64, UsfPositionError> {
        let direction = normalized_direction(direction);

        let macro_displacement = self
            .body
            .macro_surface_displacement_metres(direction, self.body.surface_detail_scale);
        let mut radius = self.body.radius_metres + macro_displacement;

        for band in self
            .residual
            .coarse
            .iter()
            .take(self.residual.coarse_len)
            .flatten()
        {
            let detail_native = f64::from(coarse_residual_sample(
                direction,
                band.angular_frequency,
                band.seed,
            )) * band.amplitude_native;
            radius += detail_native * band.metres_per_native;
        }

        Ok(radius)
    }

    #[inline]
    pub(crate) fn pre_fine_surface_local_metres(
        &self,
        direction: Vec3,
    ) -> Result<DVec3, UsfPositionError> {
        let direction = normalized_direction(direction);
        let radius = self.pre_fine_surface_radius_metres(direction)?;
        Ok(dvec(direction) * radius)
    }
    #[inline]
    fn semantic_surface_radius_metres(&self, direction: Vec3) -> Result<f64, UsfPositionError> {
        let direction = normalized_direction(direction);
        let mut radius = self.pre_fine_surface_radius_metres(direction)?;

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
                radius += f64::from(noise) * band.amplitude_native * band.metres_per_native;
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

        let surface_radius = self.semantic_surface_radius_metres(direction).ok()?;

        Some((radial - surface_radius, radial))
    }

    #[inline]
    pub(crate) fn signed_distance_local_metres(&self, local_point_metres: DVec3) -> Option<f64> {
        let (outer_sdf, radial) =
            self.outer_signed_distance_and_radial_local_metres(local_point_metres)?;

        if self.body.profile != CelestialBodyProfile::Rocky {
            return Some(outer_sdf);
        }

        let outer_surface_radius_metres = radial - outer_sdf;
        let void_sdf = rocky_cave_void_signed_distance_metres_with_radial(
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
        self.outer_signed_distance_and_radial_local_metres(local_point_metres)
            .map(|(distance, _)| distance)
    }

    #[inline]
    pub(crate) fn surface_local_metres(&self, direction: Vec3) -> Result<DVec3, UsfPositionError> {
        let direction = normalized_direction(direction);
        let radius = self.semantic_surface_radius_metres(direction)?;
        Ok(dvec(direction) * radius)
    }

    pub(crate) fn noise_cache_stats(&self) -> (u64, u64) {
        self.canonical_noise_cache.stats()
    }

    pub(crate) fn noise_cell_cache_stats(&self) -> (u64, u64) {
        self.canonical_noise_cache.cell_stats()
    }
}
