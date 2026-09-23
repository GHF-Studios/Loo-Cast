//! Ancestor-closed runtime context topology for canonical USF space.
//!
//! [`SpatialDemandSnapshot`] describes what runtime mechanisms are currently
//! requested. This module turns that demand into one shared sparse topology of
//! canonical [`UsfChunkAddress`] nodes. The topology is *ancestor closed*: a
//! resident child can never exist without every canonical parent up to its
//! Scale-Slice ceiling.
//!
//! Capability-specific state (voxel caches, physical fields, simulation state,
//! etc.) should key itself by these context addresses instead of inventing an
//! unrelated spatial tree. Coverage remains capability-specific realized fact;
//! this topology answers only which contextual nodes currently have runtime
//! demand/residency responsibility.

use std::collections::{HashMap, HashSet};

use bevy::prelude::*;

use super::{
    SPATIAL_SCALE_MAX, SpatialDemandScope, SpatialDemandSet, SpatialDemandSnapshot, SpatialScale,
    UsfChunkAddress, UsfPosition,
};

/// One resident node in the shared runtime USF context topology.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UsfContextNode {
    scope: UsfChunkAddress,
    parent: Option<UsfChunkAddress>,
    direct_demand_count: usize,
    child_count: usize,
    maximum_priority: Option<i32>,
}

impl UsfContextNode {
    pub const fn scope(self) -> UsfChunkAddress {
        self.scope
    }

    pub const fn parent(self) -> Option<UsfChunkAddress> {
        self.parent
    }

    /// Number of distinct demand sources that explicitly requested this node.
    ///
    /// Ancestors inserted only to preserve contextual closure have zero direct
    /// demand until some mechanism independently requests them.
    pub const fn direct_demand_count(self) -> usize {
        self.direct_demand_count
    }

    pub const fn child_count(self) -> usize {
        self.child_count
    }

    pub const fn maximum_priority(self) -> Option<i32> {
        self.maximum_priority
    }
}

/// Shared sparse runtime topology over canonical USF chunk addresses.
///
/// The resource is rebuilt from the current demand snapshot. Rebuilding instead
/// of incrementally mutating is intentional for now: it makes pruning exact and
/// guarantees that stale descendants cannot survive a vanished ancestor demand.
/// Capability caches can use [`Self::revision`] to update only when topology
/// actually changes.
#[derive(Resource, Debug, Default)]
pub struct UsfContextTopology {
    revision: u64,
    nodes: HashMap<UsfChunkAddress, UsfContextNode>,
}

impl UsfContextTopology {
    pub const fn revision(&self) -> u64 {
        self.revision
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn contains(&self, scope: UsfChunkAddress) -> bool {
        self.nodes.contains_key(&scope)
    }

    pub fn node(&self, scope: UsfChunkAddress) -> Option<UsfContextNode> {
        self.nodes.get(&scope).copied()
    }

    pub fn iter(&self) -> impl ExactSizeIterator<Item = UsfContextNode> + '_ {
        self.nodes.values().copied()
    }

    /// Returns root -> leaf for one resident scope.
    ///
    /// `None` is an invariant failure: either the leaf is not resident or some
    /// ancestor is missing from what is required to be an ancestor-closed set.
    pub fn path_from_root(
        &self,
        leaf: UsfChunkAddress,
    ) -> Option<Vec<UsfChunkAddress>> {
        if !self.contains(leaf) {
            return None;
        }

        let mut path = Vec::new();
        let mut current = leaf;
        loop {
            path.push(current);
            let Some(parent) = current.parent() else {
                break;
            };
            if !self.contains(parent) {
                return None;
            }
            current = parent;
        }
        path.reverse();
        Some(path)
    }

    /// Finds the deepest resident context at or above `requested_finest` that
    /// contains `position`.
    ///
    /// Queries never manufacture finer semantic precision than the position
    /// already carries. If the requested node is not resident, evaluation
    /// walks upward until an available ancestor is found.
    pub fn deepest_resident_containing(
        &self,
        position: &UsfPosition,
        requested_finest: SpatialScale,
    ) -> Option<UsfChunkAddress> {
        let finest = requested_finest.max(position.leaf_scale());

        for raw in finest.exponent()..=SPATIAL_SCALE_MAX {
            let scale = SpatialScale::new(raw).expect("validated USF context scale");
            let Ok(scope) = UsfChunkAddress::containing(*position, scale) else {
                continue;
            };
            if self.contains(scope) {
                return Some(scope);
            }
        }

        None
    }

    /// Returns the currently resident root -> deepest-usable context path for a
    /// canonical position and requested evaluation depth.
    pub fn path_for_position(
        &self,
        position: &UsfPosition,
        requested_finest: SpatialScale,
    ) -> Option<Vec<UsfChunkAddress>> {
        let leaf = self.deepest_resident_containing(position, requested_finest)?;
        self.path_from_root(leaf)
    }

    pub(crate) fn reconcile_from_scopes(
        &mut self,
        scopes: impl IntoIterator<Item = SpatialDemandScope>,
    ) {
        #[derive(Default)]
        struct DirectDemand {
            sources: HashSet<Entity>,
            maximum_priority: Option<i32>,
        }

        let mut direct = HashMap::<UsfChunkAddress, DirectDemand>::new();
        let mut resident = HashSet::<UsfChunkAddress>::new();

        for demand in scopes {
            let Ok(scope) = UsfChunkAddress::containing(demand.center(), demand.scale()) else {
                continue;
            };

            let entry = direct.entry(scope).or_default();
            entry.sources.insert(demand.source());
            entry.maximum_priority = Some(
                entry
                    .maximum_priority
                    .map_or(demand.priority(), |current| current.max(demand.priority())),
            );

            let mut current = Some(scope);
            while let Some(address) = current {
                resident.insert(address);
                current = address.parent();
            }
        }

        let mut child_counts = HashMap::<UsfChunkAddress, usize>::new();
        for scope in resident.iter().copied() {
            if let Some(parent) = scope.parent()
                && resident.contains(&parent)
            {
                *child_counts.entry(parent).or_default() += 1;
            }
        }

        let mut next = HashMap::with_capacity(resident.len());
        for scope in resident {
            let direct_demand = direct.get(&scope);
            next.insert(
                scope,
                UsfContextNode {
                    scope,
                    parent: scope.parent(),
                    direct_demand_count: direct_demand.map_or(0, |demand| demand.sources.len()),
                    child_count: child_counts.get(&scope).copied().unwrap_or(0),
                    maximum_priority: direct_demand.and_then(|demand| demand.maximum_priority),
                },
            );
        }

        if self.nodes != next {
            self.nodes = next;
            self.revision = self.revision.wrapping_add(1).max(1);
        }
    }
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UsfContextSet {
    Reconcile,
}

fn reconcile_context_topology(
    demand: Res<SpatialDemandSnapshot>,
    mut topology: ResMut<UsfContextTopology>,
) {
    topology.reconcile_from_scopes(demand.iter());
}

pub(in crate::spatial) fn configure(app: &mut App) {
    app.init_resource::<UsfContextTopology>().add_systems(
        Update,
        reconcile_context_topology
            .in_set(UsfContextSet::Reconcile)
            .after(SpatialDemandSet::Collect),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at_metres(x: f32) -> UsfPosition {
        UsfPosition::zero(SpatialScale::ZERO)
            .translated_native(Vec3::new(x, 0.0, 0.0))
            .unwrap()
    }

    fn demand(
        source: Entity,
        position: UsfPosition,
        scale: SpatialScale,
    ) -> SpatialDemandScope {
        SpatialDemandScope::at_scale(source, scale, position, Vec3::ONE, 7)
    }

    #[test]
    fn one_fine_scope_materializes_the_complete_ancestor_path() {
        let mut world = World::new();
        let source = world.spawn_empty().id();
        let position = at_metres(0.0);
        let leaf = UsfChunkAddress::containing(position, SpatialScale::ZERO).unwrap();

        let mut topology = UsfContextTopology::default();
        topology.reconcile_from_scopes([demand(source, position, SpatialScale::ZERO)]);

        assert_eq!(topology.len(), 36);
        let path = topology.path_from_root(leaf).unwrap();
        assert_eq!(path.len(), 36);
        assert_eq!(path.first().unwrap().scale(), SpatialScale::MAX);
        assert_eq!(path.last().copied(), Some(leaf));

        for node in topology.iter() {
            if let Some(parent) = node.parent() {
                assert!(topology.contains(parent));
            }
        }
    }

    #[test]
    fn sibling_branches_share_ancestors_and_prune_without_orphans() {
        let mut world = World::new();
        let a = world.spawn_empty().id();
        let b = world.spawn_empty().id();
        let origin = at_metres(0.0);
        let neighbor = at_metres(1_500.0);
        let origin_leaf = UsfChunkAddress::containing(origin, SpatialScale::ZERO).unwrap();
        let neighbor_leaf =
            UsfChunkAddress::containing(neighbor, SpatialScale::ZERO).unwrap();
        let shared_parent =
            UsfChunkAddress::containing(origin, SpatialScale::new(1).unwrap()).unwrap();

        let mut topology = UsfContextTopology::default();
        topology.reconcile_from_scopes([
            demand(a, origin, SpatialScale::ZERO),
            demand(b, neighbor, SpatialScale::ZERO),
        ]);

        assert_eq!(topology.len(), 37);
        assert_eq!(topology.node(shared_parent).unwrap().child_count(), 2);
        assert_eq!(
            topology.node(shared_parent).unwrap().direct_demand_count(),
            0
        );

        topology.reconcile_from_scopes([demand(a, origin, SpatialScale::ZERO)]);

        assert_eq!(topology.len(), 36);
        assert!(topology.contains(origin_leaf));
        assert!(!topology.contains(neighbor_leaf));
        assert!(topology.contains(shared_parent));
        assert_eq!(topology.node(shared_parent).unwrap().child_count(), 1);
    }

    #[test]
    fn query_depth_stops_at_requested_context_even_when_finer_nodes_exist() {
        let mut world = World::new();
        let source = world.spawn_empty().id();
        let position = at_metres(0.0);
        let mut topology = UsfContextTopology::default();
        topology.reconcile_from_scopes([demand(source, position, SpatialScale::ZERO)]);

        let requested = SpatialScale::new(4).unwrap();
        let path = topology.path_for_position(&position, requested).unwrap();

        assert_eq!(path.last().unwrap().scale(), requested);
    }
}
