//! USF semantic-entity manifestation primitives.
//!
//! # Target ownership model
//!
//! The engine treats semantic identity, topological authority, scale-local
//! simulation, and presentation as separate dimensions:
//!
//! ```text
//! UsfEntity
//!   -> authority partition(s)
//!       -> logical realization(s)
//!           -> presentation projection(s)
//! ```
//!
//! [`UsfEntity`] is the single semantic identity for an object. It owns state
//! whose meaning is independent of topology, scale-local runtime representation,
//! and viewer count.
//!
//! An authority partition is a distinct ECS entity representing one authoritative
//! topological portion of that semantic object. The ordinary unsplit case still
//! has one partition; the semantic entity itself does not double as that
//! partition. Partitions are peers: there is no permanent "original" partition.
//!
//! A logical realization is a distinct ECS entity owned by exactly one authority
//! partition. It carries one scale/backend-local simulation realization of that
//! partition. A partition may temporarily have no materialized realization, or
//! may own several simultaneous realizations when different Scale Slices or
//! simulation backends are required.
//!
//! Presentation is downstream of logical realization and never carries semantic
//! or simulation authority. View/client ownership is intentionally deferred to
//! the presentation work tracked separately from the generic ownership graph.
//!
//! # State ownership invariants
//!
//! - Semantic state belongs on [`UsfEntity`] only when it remains meaningful and
//!   singular regardless of partition count or realization count.
//! - Mutable state that may legitimately differ across a topological split
//!   belongs to the authority partition.
//! - Scale/backend-specific runtime state belongs to a logical realization and
//!   must be rebuildable without changing semantic identity.
//! - View-specific state belongs only to presentation.
//! - Viewer count may multiply presentation entities, never semantic identities,
//!   authority partitions, or physical authority.
//! - Destroying a semantic entity destroys its authority partitions; destroying
//!   a partition does not destroy the semantic entity or sibling partitions.
//! - Destroying/rebuilding a logical realization does not destroy its partition.
//! - Split/merge operations must explicitly reconcile partition-local mutable
//!   state before retiring partitions.
//!
//! # Transitional compatibility
//!
//! The types currently defined in this module predate the target hierarchy.
//! [`UsfManifestationOf`] / [`UsfManifestations`] flatten semantic ownership and
//! runtime manifestation into one relation. [`UsfManifestationAuthority`] encodes
//! a legacy single-authoritative-manifestation assumption. [`UsfLogicalProjection`]
//! is a marker on those flattened manifestations rather than an ownership
//! relation from a logical realization to an authority partition.
//!
//! These compatibility types remain until the generic graph is implemented.
//! They must not be treated as the target ontology by new systems.

use bevy::prelude::*;

/// One semantic USF entity.
#[derive(Component, Debug)]
pub struct UsfEntity;

/// Transitional flat relation from a concrete runtime manifestation to a
/// [`UsfEntity`].
///
/// This relation currently collapses the target authority-partition and
/// logical-realization layers. It remains for compatibility while existing
/// systems migrate; it is not the target ownership model.
#[derive(Component, Debug)]
#[relationship(relationship_target = UsfManifestations)]
pub struct UsfManifestationOf(pub Entity);

/// Legacy marker for the single manifestation currently carrying mutable
/// spatial authority for systems that have not yet migrated.
///
/// This is not an authority-partition component. The target model permits
/// `1..N` peer authority partitions, with no permanent original and no
/// requirement that exactly one runtime manifestation own all mutable state.
#[derive(Component, Debug, Default)]
pub struct UsfManifestationAuthority;

/// Transitional marker identifying a flattened manifestation as a local
/// logical/physics projection.
///
/// In the target model, a logical realization is its own ECS entity owned by
/// exactly one authority partition. Several realizations may coexist for one
/// partition when distinct Scale Slices or simulation backends are required.
/// Rebuilding a realization must not change semantic or partition identity.
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct UsfLogicalProjection;

/// Transitional collection for the flat [`UsfManifestationOf`] relation.
///
/// `linked_spawn` currently gives the semantic entity lifetime ownership of
/// these flattened manifestations. The target graph instead gives the semantic
/// entity linked ownership of authority partitions, and each partition linked
/// ownership of its logical realizations.
#[derive(Component, Debug)]
#[relationship_target(
    relationship = UsfManifestationOf,
    linked_spawn
)]
pub struct UsfManifestations(Vec<Entity>);

impl UsfManifestations {
    /// Iterate every currently linked manifestation.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = Entity> + '_ {
        self.0.iter().copied()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// Declares that this concrete presentation entity presents one current runtime
/// manifestation.
///
/// This association is transitional because current manifestations flatten the
/// partition/realization hierarchy. In the target graph, presentation is
/// downstream of a logical realization. View/client ownership remains a
/// separate concern and must never manufacture semantic or simulation authority.
#[derive(Component, Debug)]
#[relationship(relationship_target = UsfPresentationProjections)]
pub struct UsfPresentationProjectionOf(pub Entity);

/// Presentation projections currently associated with one spatial
/// manifestation.
///
/// This relationship does not use `linked_spawn`: current presentation
/// entities are already owned by their ordinary Bevy hierarchy, while the USF
/// relation records semantic association rather than imposing storage/lifetime
/// policy.
#[derive(Component, Debug)]
#[relationship_target(relationship = UsfPresentationProjectionOf)]
pub struct UsfPresentationProjections(Vec<Entity>);

impl UsfPresentationProjections {
    pub fn iter(&self) -> impl ExactSizeIterator<Item = Entity> + '_ {
        self.0.iter().copied()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn presentation_projection_is_orthogonal_to_spatial_manifestation() {
        let mut world = World::new();
        let semantic = world.spawn(UsfEntity).id();
        let manifestation = world
            .spawn((UsfManifestationOf(semantic), UsfLogicalProjection))
            .id();
        let presentation = world.spawn(UsfPresentationProjectionOf(manifestation)).id();

        let manifestations = world.get::<UsfManifestations>(semantic).unwrap();
        assert_eq!(
            manifestations.iter().collect::<Vec<_>>(),
            vec![manifestation]
        );

        let presentations = world
            .get::<UsfPresentationProjections>(manifestation)
            .unwrap();
        assert_eq!(presentations.iter().collect::<Vec<_>>(), vec![presentation]);
        assert!(world.get::<UsfManifestationOf>(presentation).is_none());
    }
}
