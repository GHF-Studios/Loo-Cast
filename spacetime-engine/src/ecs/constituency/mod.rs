//! Recursive semantic entity constituency.
//!
//! Constituency is intentionally orthogonal to control and manifestation.
//! A player may be contained by a spacecraft, a spacecraft by a hangar, and
//! the same semantic entities may have independent scale-local manifestations.

use bevy::prelude::*;

#[derive(Component, Debug)]
#[relationship(relationship_target = UsfConstituents)]
pub struct UsfConstituentOf(pub Entity);

#[derive(Component, Debug)]
#[relationship_target(relationship = UsfConstituentOf)]
pub struct UsfConstituents(Vec<Entity>);

impl UsfConstituents {
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
