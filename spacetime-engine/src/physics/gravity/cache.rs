//! Hierarchical gravity-field cache over the shared USF context topology.
//!
//! The first backend intentionally remains exact. The root candidate set
//! contains every canonical source. For each resident child, sources inside the
//! child remain eligible for further refinement while sources outside become an
//! inherited residual evaluated at the parent/child boundary.
//!
//! A query therefore evaluates:
//!
//! `root residual + ... + child residual + deepest local sources`
//!
//! with each source appearing exactly once. Today those residuals are exact
//! source lists. Later they can become multipoles, grids or other bounded-error
//! summaries without changing consumers or the shared context topology.

use std::collections::HashMap;

use bevy::prelude::*;

use crate::spatial::{UsfChunkAddress, UsfContextTopology};

use super::RadialGravitySource;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct CachedGravitySource {
    entity: Entity,
    source: RadialGravitySource,
}

impl CachedGravitySource {
    pub(super) const fn new(entity: Entity, source: RadialGravitySource) -> Self {
        Self { entity, source }
    }

    pub(super) const fn entity(self) -> Entity {
        self.entity
    }

    pub(super) const fn source(self) -> RadialGravitySource {
        self.source
    }
}

#[derive(Debug, Clone, Default)]
pub(super) struct GravityContextCache {
    inherited_residual: Vec<CachedGravitySource>,
    refinement_sources: Vec<CachedGravitySource>,
}

impl GravityContextCache {
    pub(super) fn inherited_residual(&self) -> &[CachedGravitySource] {
        &self.inherited_residual
    }

    pub(super) fn refinement_sources(&self) -> &[CachedGravitySource] {
        &self.refinement_sources
    }
}

/// Field-specific capability cache attached to the shared USF context topology.
#[derive(Resource, Debug, Default)]
pub(super) struct GravityFieldCache {
    revision: u64,
    topology_revision: u64,
    global_sources: Vec<CachedGravitySource>,
    contexts: HashMap<UsfChunkAddress, GravityContextCache>,
}

impl GravityFieldCache {
    pub(super) const fn revision(&self) -> u64 {
        self.revision
    }

    pub(super) const fn topology_revision(&self) -> u64 {
        self.topology_revision
    }

    pub(super) fn global_sources(&self) -> &[CachedGravitySource] {
        &self.global_sources
    }

    pub(super) fn context(
        &self,
        scope: UsfChunkAddress,
    ) -> Option<&GravityContextCache> {
        self.contexts.get(&scope)
    }

    pub(super) fn rebuild(
        &mut self,
        topology: &UsfContextTopology,
        mut sources: Vec<CachedGravitySource>,
    ) {
        sources.sort_by_key(|source| source.entity().to_bits());

        self.contexts.clear();
        self.global_sources = sources;
        self.topology_revision = topology.revision();

        let mut scopes = topology
            .iter()
            .map(|node| node.scope())
            .collect::<Vec<_>>();
        scopes.sort_by(|a, b| b.scale().cmp(&a.scale()));

        for scope in scopes {
            let Some(node) = topology.node(scope) else {
                continue;
            };

            let (inherited_residual, refinement_sources) =
                if let Some(parent) = node.parent() {
                    let Some(parent_cache) = self.contexts.get(&parent) else {
                        // Ancestor closure should make this impossible. Leave
                        // the cache incomplete so queries take exact fallback.
                        continue;
                    };

                    partition_for_child(parent_cache.refinement_sources(), scope)
                } else {
                    // A ceiling/root context starts with the complete source
                    // set. Child contexts progressively partition it.
                    (Vec::new(), self.global_sources.clone())
                };

            self.contexts.insert(
                scope,
                GravityContextCache {
                    inherited_residual,
                    refinement_sources,
                },
            );
        }

        self.revision = self.revision.wrapping_add(1).max(1);
    }
}

fn partition_for_child(
    parent_sources: &[CachedGravitySource],
    child: UsfChunkAddress,
) -> (Vec<CachedGravitySource>, Vec<CachedGravitySource>) {
    let mut residual = Vec::new();
    let mut refinement = Vec::new();

    for source in parent_sources.iter().copied() {
        let inside_child = UsfChunkAddress::containing(
            source.source().center(),
            child.scale(),
        )
        .is_ok_and(|scope| scope == child);

        if inside_child {
            refinement.push(source);
        } else {
            residual.push(source);
        }
    }

    (residual, refinement)
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
