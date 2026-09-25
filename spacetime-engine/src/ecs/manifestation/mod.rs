//! USF semantic-entity manifestation primitives.
//!
//! # Ownership graph
//!
//! Semantic identity, topological authority, scale-local simulation, and
//! presentation are separate runtime dimensions:
//!
//! ```text
//! UsfEntity
//!   -> authority partition(s)
//!       -> logical realization(s)
//!           -> presentation projection(s)
//! ```
//!
//! [`UsfEntity`] is the single semantic identity for an object.
//!
//! An entity carrying [`UsfAuthorityPartitionOf`] is one authoritative
//! topological portion of exactly one semantic entity. Partitions are peers;
//! there is no permanent "original" partition.
//!
//! An entity carrying [`UsfLogicalRealizationOf`] is one scale/backend-local
//! simulation realization of exactly one authority partition. Realizations are
//! disposable runtime representations: rebuilding one must not change semantic
//! or partition identity.
//!
//! An entity carrying [`UsfPresentationProjectionOf`] is presentation-only.
//! New generic-graph users target a logical realization. Presentation lifetime
//! and viewer count never manufacture semantic or simulation authority.
//!
//! The relationship components themselves identify authority partitions and
//! logical realizations. Separate marker components would duplicate role state
//! and could drift out of sync with the ownership relationship.
//!
//! # Lifetime ownership
//!
//! - A semantic entity owns its authority partitions.
//! - An authority partition owns its logical realizations.
//! - Destroying a realization does not destroy its partition.
//! - Destroying a partition does not destroy its semantic entity or siblings.
//! - Presentation is not linked-spawn-owned by the logical realization.
//!
//! # Transitional compatibility
//!
//! [`UsfManifestationOf`] / [`UsfManifestations`] remain as the existing flat
//! semantic-to-runtime relation while current systems migrate.
//! [`UsfManifestationAuthority`] remains the legacy single-authority marker and
//! [`UsfLogicalProjection`] remains the legacy flattened logical marker.
//!
//! New systems should use the generic ownership graph instead of extending the
//! flattened manifestation ontology.

use bevy::prelude::*;

/// One semantic USF entity.
#[derive(Component, Debug)]
pub struct UsfEntity;

/// Declares this entity to be one authority partition of a semantic
/// [`UsfEntity`].
///
/// The relationship is the partition's role identity: no separate partition
/// marker is required.
#[derive(Component, Debug)]
#[relationship(relationship_target = UsfAuthorityPartitions)]
pub struct UsfAuthorityPartitionOf(pub Entity);

/// Authority partitions currently owned by one semantic [`UsfEntity`].
///
/// `linked_spawn` makes semantic lifetime flow downstream into every partition.
#[derive(Component, Debug)]
#[relationship_target(
    relationship = UsfAuthorityPartitionOf,
    linked_spawn
)]
pub struct UsfAuthorityPartitions(Vec<Entity>);

impl UsfAuthorityPartitions {
    /// Iterate every currently linked authority partition.
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

/// Declares this entity to be one logical realization of an authority
/// partition.
///
/// The relationship is the realization's role identity: no separate logical
/// realization marker is required.
#[derive(Component, Debug)]
#[relationship(relationship_target = UsfLogicalRealizations)]
pub struct UsfLogicalRealizationOf(pub Entity);

/// Logical realizations currently owned by one authority partition.
///
/// `linked_spawn` makes partition lifetime flow downstream into every logical
/// realization without making realization lifetime flow back upstream.
#[derive(Component, Debug)]
#[relationship_target(
    relationship = UsfLogicalRealizationOf,
    linked_spawn
)]
pub struct UsfLogicalRealizations(Vec<Entity>);

impl UsfLogicalRealizations {
    /// Iterate every currently linked logical realization.
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

/// Declares that this concrete presentation entity presents one runtime target.
///
/// In the generic ownership graph the target is a logical realization. Existing
/// legacy call sites may temporarily still target a flattened manifestation
/// while their owning subsystem migrates.
///
/// Presentation association is explicit rather than inferred from Bevy
/// hierarchy, and deliberately does not impose linked-spawn lifetime ownership.
#[derive(Component, Debug)]
#[relationship(relationship_target = UsfPresentationProjections)]
pub struct UsfPresentationProjectionOf(pub Entity);

/// Presentation projections currently associated with one runtime target.
///
/// Presentation is downstream and non-authoritative. This relationship does not
/// use `linked_spawn`: presentation lifetime is owned by the relevant
/// view/render hierarchy rather than by simulation authority.
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

/// Transitional flat relation from a concrete runtime manifestation to a
/// [`UsfEntity`].
///
/// This relation collapses authority partition and logical realization. It
/// remains for compatibility while existing systems migrate; new systems
/// should use [`UsfAuthorityPartitionOf`] and [`UsfLogicalRealizationOf`].
#[derive(Component, Debug)]
#[relationship(relationship_target = UsfManifestations)]
pub struct UsfManifestationOf(pub Entity);

/// Legacy marker for the single flattened manifestation currently carrying
/// mutable spatial authority for systems that have not yet migrated.
///
/// This is not an authority-partition component.
#[derive(Component, Debug, Default)]
pub struct UsfManifestationAuthority;

/// Legacy marker identifying a flattened manifestation as a local
/// logical/physics projection.
///
/// New generic-graph code should identify a logical realization through
/// [`UsfLogicalRealizationOf`] instead.
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct UsfLogicalProjection;

/// Transitional collection for the flat [`UsfManifestationOf`] relation.
///
/// `linked_spawn` preserves the existing semantic-to-flat-manifestation
/// lifetime behavior until those consumers migrate onto authority partitions.
#[derive(Component, Debug)]
#[relationship_target(
    relationship = UsfManifestationOf,
    linked_spawn
)]
pub struct UsfManifestations(Vec<Entity>);

impl UsfManifestations {
    /// Iterate every currently linked legacy manifestation.
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
    fn generic_graph_supports_peer_partitions_and_multiple_realizations() {
        let mut world = World::new();
        let semantic = world.spawn(UsfEntity).id();

        let partition_a = world.spawn(UsfAuthorityPartitionOf(semantic)).id();
        let partition_b = world.spawn(UsfAuthorityPartitionOf(semantic)).id();

        let realization_a0 = world.spawn(UsfLogicalRealizationOf(partition_a)).id();
        let realization_a1 = world.spawn(UsfLogicalRealizationOf(partition_a)).id();
        let realization_b0 = world.spawn(UsfLogicalRealizationOf(partition_b)).id();

        let presentation = world
            .spawn(UsfPresentationProjectionOf(realization_a0))
            .id();

        let partitions = world.get::<UsfAuthorityPartitions>(semantic).unwrap();
        assert_eq!(partitions.len(), 2);
        assert!(partitions.iter().any(|entity| entity == partition_a));
        assert!(partitions.iter().any(|entity| entity == partition_b));

        let realizations_a = world.get::<UsfLogicalRealizations>(partition_a).unwrap();
        assert_eq!(realizations_a.len(), 2);
        assert!(realizations_a.iter().any(|entity| entity == realization_a0));
        assert!(realizations_a.iter().any(|entity| entity == realization_a1));

        let realizations_b = world.get::<UsfLogicalRealizations>(partition_b).unwrap();
        assert_eq!(
            realizations_b.iter().collect::<Vec<_>>(),
            vec![realization_b0]
        );

        let presentations = world
            .get::<UsfPresentationProjections>(realization_a0)
            .unwrap();
        assert_eq!(presentations.iter().collect::<Vec<_>>(), vec![presentation]);

        assert!(world.get::<UsfAuthorityPartitionOf>(presentation).is_none());
        assert!(world.get::<UsfLogicalRealizationOf>(presentation).is_none());
        assert!(
            world
                .get::<UsfManifestationAuthority>(presentation)
                .is_none()
        );
    }

    #[test]
    fn generic_graph_lifetime_ownership_flows_downstream_only() {
        let mut world = World::new();
        let semantic = world.spawn(UsfEntity).id();
        let partition_a = world.spawn(UsfAuthorityPartitionOf(semantic)).id();
        let partition_b = world.spawn(UsfAuthorityPartitionOf(semantic)).id();
        let realization_a = world.spawn(UsfLogicalRealizationOf(partition_a)).id();
        let realization_b = world.spawn(UsfLogicalRealizationOf(partition_b)).id();

        world.despawn(realization_a);

        assert!(world.entities().contains(semantic));
        assert!(world.entities().contains(partition_a));
        assert!(world.entities().contains(partition_b));
        assert!(!world.entities().contains(realization_a));
        assert!(world.entities().contains(realization_b));

        let replacement_a = world.spawn(UsfLogicalRealizationOf(partition_a)).id();

        world.despawn(partition_a);

        assert!(world.entities().contains(semantic));
        assert!(!world.entities().contains(partition_a));
        assert!(!world.entities().contains(replacement_a));
        assert!(world.entities().contains(partition_b));
        assert!(world.entities().contains(realization_b));

        let partitions = world.get::<UsfAuthorityPartitions>(semantic).unwrap();
        assert_eq!(partitions.iter().collect::<Vec<_>>(), vec![partition_b]);

        world.despawn(semantic);

        assert!(!world.entities().contains(semantic));
        assert!(!world.entities().contains(partition_b));
        assert!(!world.entities().contains(realization_b));
    }

    #[test]
    fn presentation_lifetime_does_not_flow_authority_upstream() {
        let mut world = World::new();
        let semantic = world.spawn(UsfEntity).id();
        let partition = world.spawn(UsfAuthorityPartitionOf(semantic)).id();
        let realization = world.spawn(UsfLogicalRealizationOf(partition)).id();
        let presentation = world.spawn(UsfPresentationProjectionOf(realization)).id();

        world.despawn(presentation);

        assert!(world.entities().contains(semantic));
        assert!(world.entities().contains(partition));
        assert!(world.entities().contains(realization));
        assert!(!world.entities().contains(presentation));
    }

    #[test]
    fn legacy_flat_manifestation_remains_compatible_during_migration() {
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
