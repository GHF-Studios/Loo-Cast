//! Collision-pipeline integration for USF charts and split manifestations.
//!
//! Registering [`SpatialTopologyCollisionHooks`] with Avian does not execute the
//! filter for every collider. A collider must explicitly opt into
//! `ActiveCollisionHooks::FILTER_PAIRS`; split-peer producers own that opt-in.

use avian3d::collision::hooks::CollisionHooks;
use bevy::{ecs::system::SystemParam, prelude::*};

use crate::physics::chart::UsfPhysicsCharts;

/// Reserved proxy manifestation used while one authoritative spatial body is
/// partitioned across topology.
///
/// This relation is generic topology state rather than portal state: a semantic
/// entity may have many ordinary manifestations, while a split peer exists only
/// to represent the complementary spatial portion of one authority.
#[derive(Component, Debug, Clone, Copy)]
pub struct SpatialSplitPeer {
    pub authority: Entity,
}

/// Marks a reserved [`SpatialSplitPeer`] that is currently manifested in
/// collision space. A peer entity can exist permanently while remaining
/// spatially dormant outside a split.
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct SpatialSplitPeerActive;

/// Avian pair filter for generic split peers. A reserved proxy represents part
/// of its authority in another topological location; those two solver entities
/// must therefore never generate contacts against each other.
#[derive(SystemParam)]
pub(crate) struct SpatialTopologyCollisionHooks<'w, 's> {
    peers: Query<'w, 's, &'static SpatialSplitPeer>,
    charts: UsfPhysicsCharts<'w, 's>,
}

impl CollisionHooks for SpatialTopologyCollisionHooks<'_, '_> {
    fn filter_pairs(&self, collider1: Entity, collider2: Entity, _commands: &mut Commands) -> bool {
        if let (Some(first), Some(second)) = (
            self.charts.collider_scale(collider1),
            self.charts.collider_scale(collider2),
        ) && first != second
        {
            return false;
        }

        let first_is_peer_of_second = self
            .peers
            .get(collider1)
            .is_ok_and(|peer| peer.authority == collider2);
        let second_is_peer_of_first = self
            .peers
            .get(collider2)
            .is_ok_and(|peer| peer.authority == collider1);

        !first_is_peer_of_second && !second_is_peer_of_first
    }
}
