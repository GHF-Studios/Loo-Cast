//! Typed gravity-field query over hierarchical USF context caches.

use bevy::{
    ecs::system::SystemParam,
    math::DVec3,
    prelude::*,
};

use crate::spatial::{SpatialScale, UsfChunkAddress, UsfContextTopology, UsfPosition};

use super::cache::{CachedGravitySource, GravityFieldCache};

/// Provenance/quality of one gravity sample.
///
/// Both current variants are exact. The distinction tells diagnostics whether
/// the sample used the ancestor-closed context stack or had to fall back to the
/// global source set because no resident context path was available yet.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum GravitySampleQuality {
    ExactHierarchical,
    #[default]
    ExactGlobalFallback,
}

/// Fixed-tick gravity sample attached to a runtime subject.
///
/// Acceleration is expressed in canonical USF axes and SI m/s². It is
/// independent from the subject's current numerical Scale Slice.
#[derive(Component, Debug, Clone, Copy)]
pub struct GravitySample {
    acceleration_metres_per_second2: DVec3,
    strongest_source: Option<Entity>,
    deepest_context: Option<UsfChunkAddress>,
    context_depth: u8,
    cache_revision: u64,
    quality: GravitySampleQuality,
}

impl Default for GravitySample {
    fn default() -> Self {
        Self {
            acceleration_metres_per_second2: DVec3::ZERO,
            strongest_source: None,
            deepest_context: None,
            context_depth: 0,
            cache_revision: 0,
            quality: GravitySampleQuality::ExactGlobalFallback,
        }
    }
}

impl GravitySample {
    pub const fn acceleration_metres_per_second2(self) -> DVec3 {
        self.acceleration_metres_per_second2
    }

    pub const fn strongest_source(self) -> Option<Entity> {
        self.strongest_source
    }

    pub const fn deepest_context(self) -> Option<UsfChunkAddress> {
        self.deepest_context
    }

    pub const fn context_depth(self) -> u8 {
        self.context_depth
    }

    pub const fn cache_revision(self) -> u64 {
        self.cache_revision
    }

    pub const fn quality(self) -> GravitySampleQuality {
        self.quality
    }

    pub fn magnitude_metres_per_second2(self) -> f32 {
        self.acceleration_metres_per_second2
            .length()
            .clamp(0.0, f64::from(f32::MAX)) as f32
    }
}

#[derive(Default)]
struct GravityAccumulator {
    acceleration: DVec3,
    strongest_source: Option<Entity>,
    strongest_magnitude2: f64,
}

impl GravityAccumulator {
    fn add_sources(&mut self, position: &UsfPosition, sources: &[CachedGravitySource]) {
        for cached in sources.iter().copied() {
            let Some(contribution) = cached.source().acceleration_at(position) else {
                continue;
            };

            let magnitude2 = contribution.length_squared();
            self.acceleration += contribution;
            if magnitude2 > self.strongest_magnitude2 {
                self.strongest_magnitude2 = magnitude2;
                self.strongest_source = Some(cached.entity());
            }
        }
    }

    fn finish(
        self,
        deepest_context: Option<UsfChunkAddress>,
        context_depth: usize,
        cache_revision: u64,
        quality: GravitySampleQuality,
    ) -> GravitySample {
        GravitySample {
            acceleration_metres_per_second2: self.acceleration,
            strongest_source: self.strongest_source,
            deepest_context,
            context_depth: context_depth.min(u8::MAX as usize) as u8,
            cache_revision,
            quality,
        }
    }
}

impl GravityFieldCache {
    pub(super) fn sample(
        &self,
        topology: &UsfContextTopology,
        position: &UsfPosition,
        requested_finest: SpatialScale,
    ) -> GravitySample {
        let fallback = || {
            let mut accumulator = GravityAccumulator::default();
            accumulator.add_sources(position, self.global_sources());
            accumulator.finish(
                None,
                0,
                self.revision(),
                GravitySampleQuality::ExactGlobalFallback,
            )
        };

        if self.topology_revision() != topology.revision() {
            return fallback();
        }

        let Some(path) = topology.path_for_position(position, requested_finest) else {
            return fallback();
        };

        let contexts = path
            .iter()
            .map(|scope| self.context(*scope))
            .collect::<Option<Vec<_>>>();
        let Some(contexts) = contexts else {
            return fallback();
        };
        let Some(leaf) = contexts.last().copied() else {
            return fallback();
        };

        let mut accumulator = GravityAccumulator::default();

        // Each child residual is exactly the portion of its parent's candidate
        // set that the child did not inherit for further refinement.
        for context in contexts.iter().skip(1) {
            accumulator.add_sources(position, context.inherited_residual());
        }

        // Whatever remains at the requested/deepest context is evaluated here.
        accumulator.add_sources(position, leaf.refinement_sources());

        accumulator.finish(
            path.last().copied(),
            path.len(),
            self.revision(),
            GravitySampleQuality::ExactHierarchical,
        )
    }
}

/// Typed gravity query. Consumers know only that they can ask for gravity at a
/// canonical location and requested USF context depth.
#[derive(SystemParam)]
pub struct GravityFieldQuery<'w> {
    topology: Res<'w, UsfContextTopology>,
    cache: Res<'w, GravityFieldCache>,
}

impl GravityFieldQuery<'_> {
    pub fn sample(
        &self,
        position: &UsfPosition,
        requested_finest: SpatialScale,
    ) -> GravitySample {
        self.cache
            .sample(&self.topology, position, requested_finest)
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use crate::{
        physics::gravity::{
            cache::{CachedGravitySource, GravityFieldCache},
            GravitySampleQuality, RadialGravitySource,
        },
        spatial::{
            SpatialDemandScope, SpatialScale, UsfChunkAddress, UsfContextTopology, UsfPosition,
        },
    };

    fn at_metres(value: Vec3) -> UsfPosition {
        UsfPosition::zero(SpatialScale::ZERO)
            .translated_native(value)
            .unwrap()
    }

    #[test]
    fn hierarchical_partition_is_exact_across_query_depths() {
        let mut world = World::new();
        let demand_source = world.spawn_empty().id();
        let local_source_entity = world.spawn_empty().id();
        let neighboring_source_entity = world.spawn_empty().id();

        let sample_position = at_metres(Vec3::Y * 10.0);
        let mut topology = UsfContextTopology::default();
        topology.reconcile_from_scopes([SpatialDemandScope::at_scale(
            demand_source,
            SpatialScale::ZERO,
            sample_position,
            Vec3::ONE,
            0,
        )]);

        let local = RadialGravitySource::new(
            at_metres(Vec3::ZERO),
            10.0,
            SpatialScale::ZERO,
            9.0,
        );
        let neighboring = RadialGravitySource::new(
            at_metres(Vec3::X * 1_500.0),
            10.0,
            SpatialScale::ZERO,
            2.0,
        );

        let mut cache = GravityFieldCache::default();
        cache.rebuild(
            &topology,
            vec![
                CachedGravitySource::new(local_source_entity, local),
                CachedGravitySource::new(neighboring_source_entity, neighboring),
            ],
        );

        let coarse = cache.sample(
            &topology,
            &sample_position,
            SpatialScale::new(1).unwrap(),
        );
        let detailed = cache.sample(
            &topology,
            &sample_position,
            SpatialScale::ZERO,
        );

        assert_eq!(coarse.quality(), GravitySampleQuality::ExactHierarchical);
        assert_eq!(detailed.quality(), GravitySampleQuality::ExactHierarchical);
        assert!(
            (coarse.acceleration_metres_per_second2()
                - detailed.acceleration_metres_per_second2())
            .length()
                < 1.0e-12
        );

        let leaf =
            UsfChunkAddress::containing(sample_position, SpatialScale::ZERO).unwrap();
        let leaf_cache = cache.context(leaf).unwrap();

        assert_eq!(leaf_cache.inherited_residual().len(), 1);
        assert_eq!(leaf_cache.refinement_sources().len(), 1);
    }
}
