//! USF entity manifestation primitives.
//!
//! A [`UsfEntity`] represents one semantic entity. It need not itself be
//! spatial, rendered, physical, or otherwise directly manifested.
//!
//! Any number of concrete Bevy entities may be manifestations of it.

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