//! Gravity capability cache over the generic hierarchical USF field substrate.

use bevy::prelude::*;

use crate::spatial::{
    UsfContextTopology, UsfFieldContext, UsfFieldSource, UsfHierarchicalFieldCache,
};

use super::RadialGravitySource;

pub(super) type CachedGravitySource = UsfFieldSource<RadialGravitySource>;
pub(super) type GravityContextCache = UsfFieldContext<RadialGravitySource>;

#[derive(Resource, Debug, Default)]
pub(super) struct GravityFieldCache {
    hierarchy: UsfHierarchicalFieldCache<RadialGravitySource>,
}

impl GravityFieldCache {
    pub(super) const fn revision(&self) -> u64 {
        self.hierarchy.revision()
    }

    pub(super) const fn topology_revision(&self) -> u64 {
        self.hierarchy.topology_revision()
    }

    pub(super) fn global_sources(&self) -> &[CachedGravitySource] {
        self.hierarchy.global_sources()
    }

    pub(super) fn context(
        &self,
        scope: crate::spatial::UsfChunkAddress,
    ) -> Option<&GravityContextCache> {
        self.hierarchy.context(scope)
    }

    pub(super) fn rebuild(
        &mut self,
        topology: &UsfContextTopology,
        sources: Vec<CachedGravitySource>,
    ) {
        self.hierarchy.rebuild(topology, sources);
    }
}

pub(super) fn prepare_gravity_field_cache(
    topology: Res<UsfContextTopology>,
    sources: Query<(Entity, &RadialGravitySource)>,
    mut cache: ResMut<GravityFieldCache>,
) {
    let mut snapshot = sources
        .iter()
        .map(|(entity, source)| CachedGravitySource::new(entity, *source))
        .collect::<Vec<_>>();
    snapshot.sort_by_key(|source| source.entity().to_bits());

    if cache.topology_revision() == topology.revision()
        && cache.global_sources() == snapshot.as_slice()
    {
        return;
    }

    cache.rebuild(&topology, snapshot);
}
