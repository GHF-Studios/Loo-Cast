//! Manual spatial-query exclusions used by topology-aware manifestations.

use avian3d::prelude::SpatialQueryFilter;
use bevy::{ecs::system::SystemParam, prelude::*};

use crate::ecs::{UsfAuthorityPartitionOf, UsfLogicalRealizationOf, UsfOwnershipQuery};
use crate::spatial::{SpatialScale, UsfScaleLayer};
use super::hooks::SpatialSplitPeer;

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

/// Read-only adapter from runtime topology entities to generic USF ownership.
///
/// Ordinary logical realizations resolve directly through `UsfOwnershipQuery`.
/// Reserved split peers resolve through their authoritative realization first.
/// This adapter owns no identity and stores no duplicate relationship.
#[derive(SystemParam)]
pub struct UsfRuntimeOwnershipQuery<'w, 's> {
    ownership: UsfOwnershipQuery<'w, 's>,
    split_peers: Query<'w, 's, (Entity, &'static SpatialSplitPeer)>,
}

impl UsfRuntimeOwnershipQuery<'_, '_> {
    pub fn authority_realization(&self, runtime: Entity) -> Entity {
        self.split_peers
            .get(runtime)
            .map_or(runtime, |(_, peer)| peer.authority)
    }

    pub fn semantic_of(&self, runtime: Entity) -> Option<Entity> {
        self.ownership.semantic_of(self.authority_realization(runtime))
    }

    /// Resolve a runtime sample's Scale Slice, including split peers that
    /// inherit their authoritative realization's chart membership. Unlayered
    /// samples use the current anchor slice, matching runtime rebase policy.
    pub fn scale_of(
        &self,
        runtime: Entity,
        layers: &Query<&UsfScaleLayer>,
        fallback: SpatialScale,
    ) -> SpatialScale {
        layers
            .get(runtime)
            .or_else(|_| layers.get(self.authority_realization(runtime)))
            .copied()
            .map_or(fallback, UsfScaleLayer::scale)
    }

    pub fn runtime_entities_of(&self, semantic: Entity) -> Vec<Entity> {
        let authorities = self.ownership.realizations_of(semantic).collect::<Vec<_>>();
        let mut entities = authorities.clone();

        for (entity, peer) in &self.split_peers {
            if authorities.contains(&peer.authority) && !entities.contains(&entity) {
                entities.push(entity);
            }
        }

        entities
    }

    pub fn runtime_entities_for(&self, runtime: Entity) -> Vec<Entity> {
        self.semantic_of(runtime)
            .map(|semantic| self.runtime_entities_of(semantic))
            .filter(|entities| !entities.is_empty())
            .unwrap_or_else(|| vec![runtime])
    }

    pub fn filter_excluding_subject(&self, runtime: Entity) -> SpatialQueryFilter {
        SpatialQueryFilter::from_excluded_entities(self.runtime_entities_for(runtime))
    }
}


/// Resolve a runtime entity to semantic ownership from an exclusive `World`.
///
/// This is the exclusive-world counterpart to [`UsfRuntimeOwnershipQuery`].
/// It stores no state and does not create another ownership relation.
pub fn runtime_semantic_of_world(world: &World, runtime: Entity) -> Option<Entity> {
    let authority = world
        .get::<SpatialSplitPeer>(runtime)
        .map_or(runtime, |peer| peer.authority);
    let realization = world.get::<UsfLogicalRealizationOf>(authority)?;
    world
        .get::<UsfAuthorityPartitionOf>(realization.0)
        .map(|partition| partition.0)
}
