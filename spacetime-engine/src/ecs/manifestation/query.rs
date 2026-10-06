//! Read-only traversal of the generic USF ownership graph.
//!
//! This SystemParam owns no state and caches nothing. It only traverses the
//! authoritative semantic -> authority partition -> logical realization graph.

use bevy::{ecs::system::SystemParam, prelude::*};

use super::{
    UsfAuthorityPartitionOf, UsfAuthorityPartitions, UsfLogicalRealizationOf,
    UsfLogicalRealizations,
};

#[derive(SystemParam)]
pub struct UsfOwnershipQuery<'w, 's> {
    realization_of: Query<'w, 's, &'static UsfLogicalRealizationOf>,
    partition_of: Query<'w, 's, &'static UsfAuthorityPartitionOf>,
    partitions: Query<'w, 's, &'static UsfAuthorityPartitions>,
    realizations: Query<'w, 's, &'static UsfLogicalRealizations>,
}

impl UsfOwnershipQuery<'_, '_> {
    pub fn semantic_of(&self, realization: Entity) -> Option<Entity> {
        let logical = self.realization_of.get(realization).ok()?;
        self.semantic_for(logical)
    }

    pub fn semantic_for(&self, logical: &UsfLogicalRealizationOf) -> Option<Entity> {
        self.partition_of
            .get(logical.0)
            .ok()
            .map(|partition| partition.0)
    }

    pub fn is_realization_of(&self, realization: Entity, semantic: Entity) -> bool {
        self.semantic_of(realization) == Some(semantic)
    }

    pub fn realizations_of(&self, semantic: Entity) -> impl Iterator<Item = Entity> + '_ {
        self.partitions
            .get(semantic)
            .into_iter()
            .flat_map(|partitions| partitions.iter())
            .flat_map(|partition| {
                self.realizations
                    .get(partition)
                    .into_iter()
                    .flat_map(|realizations| realizations.iter())
            })
    }
}
