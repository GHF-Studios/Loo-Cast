//! USF entity manifestation primitives.
//!
//! A [`UsfEntity`] represents one semantic entity. It need not itself be
//! spatial, rendered, physical, or otherwise directly manifested.
//!
//! Any number of concrete Bevy entities may be manifestations of it. Systems
//! should deliberately choose whether they operate once on semantic state or
//! once per manifestation.

use bevy::prelude::*;

/// One semantic USF entity.
#[derive(Component, Debug)]
pub struct UsfEntity;

/// Declares that this concrete Bevy entity manifests a [`UsfEntity`].
///
/// Manifestations are peers. This relationship does not designate an
/// "original" manifestation.
#[derive(Component, Debug)]
#[relationship(relationship_target = UsfManifestations)]
pub struct UsfManifestationOf(pub Entity);

/// Marks the manifestation currently carrying authoritative mutable spatial
/// state for mechanics that have not yet become manifestation-aware.
///
/// Authority is intentionally separate from [`UsfManifestationOf`]: an entity
/// may have many simultaneous manifestations while legacy systems continue to
/// consume exactly one. Splitting mechanics may transfer this marker without
/// changing semantic identity.
#[derive(Component, Debug, Default)]
pub struct UsfManifestationAuthority;

/// Marks the local logical/physics projection carried by one spatial
/// manifestation.
///
/// This role is orthogonal to both semantic manifestation identity and mutable
/// manifestation authority. A semantic entity may have several simultaneous
/// spatial manifestations (portal/world-wrap topology), and each manifestation
/// may independently own a logical projection plus one or more presentation
/// projections.
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct UsfLogicalProjection;

/// All concrete manifestations of a [`UsfEntity`].
///
/// `linked_spawn` gives the semantic entity ownership of its manifestations:
/// despawning the USF entity despawns all of them.
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

/// Declares that this concrete presentation entity presents one spatial
/// manifestation.
///
/// Presentation association is deliberately explicit rather than inferred from
/// Bevy hierarchy. `ChildOf` remains useful for transform/lifetime layout, but a
/// renderer or alternate observer may later present the same manifestation
/// through a different hierarchy or several projections at once.
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
