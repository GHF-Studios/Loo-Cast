//! Manual spatial-query exclusions used by topology-aware manifestations.

use avian3d::prelude::SpatialQueryFilter;
use bevy::prelude::*;

/// Extra collider entities that a kinematically controlled manifestation must
/// ignore in manual spatial queries.
///
/// The component is generic topology state. Portal traversal is merely its
/// first producer; character movement and camera collision are consumers.
#[derive(Component, Debug, Default, Clone)]
pub struct KinematicQueryExclusions {
    entities: Vec<Entity>,
}

impl KinematicQueryExclusions {
    pub fn from_entities(entities: impl IntoIterator<Item = Entity>) -> Self {
        let mut exclusions = Self::default();
        exclusions.replace(entities);
        exclusions
    }

    pub fn replace(&mut self, entities: impl IntoIterator<Item = Entity>) {
        self.entities.clear();
        for entity in entities {
            if !self.entities.contains(&entity) {
                self.entities.push(entity);
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Entity> + '_ {
        self.entities.iter().copied()
    }

    pub fn filter_for(&self, owner: Entity) -> SpatialQueryFilter {
        SpatialQueryFilter::from_excluded_entities(std::iter::once(owner).chain(self.iter()))
    }
}
