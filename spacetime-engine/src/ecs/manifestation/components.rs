//! Relationship components that encode the USF ownership graph.

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
/// In the generic ownership graph the target is a logical realization.
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

/// Identifies one concrete presentation/view scope.
///
/// A view owns observer-specific presentation state. It is neither semantic
/// identity nor simulation authority, and several views may independently
/// present the same logical realization.
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct UsfPresentationView;

/// Declares which presentation view owns one concrete presentation entity.
///
/// This relation is orthogonal to [`UsfPresentationProjectionOf`]: one answers
/// "what is being presented?", the other answers "for which view?".
///
/// Lifetime remains explicit for now because many existing presentations are
/// also children of simulation/render hierarchy entities. Once all consumers
/// migrate, view-owned retirement can be centralized without inventing upstream
/// authority.
#[derive(Component, Debug)]
#[relationship(relationship_target = UsfViewPresentations)]
pub struct UsfPresentationViewOf(pub Entity);

/// Presentations currently associated with one [`UsfPresentationView`].
#[derive(Component, Debug)]
#[relationship_target(relationship = UsfPresentationViewOf)]
pub struct UsfViewPresentations(Vec<Entity>);

impl UsfViewPresentations {
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
