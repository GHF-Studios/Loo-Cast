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
