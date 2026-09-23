//! Ancestor-closed runtime context topology for canonical USF space.
//!
//! [`SpatialDemandSnapshot`] contains primary spatial intent. Capability
//! adapters may derive additional runtime context demand (for example a nearby
//! observer asking a celestial voxel mechanism for a surface patch somewhere
//! other than the observer's own chunk). All such demand is collected into
//! [`UsfContextDemandBuffer`] and reconciled into one shared sparse topology.
//!
//! The topology is *ancestor closed*: a resident child can never exist without
//! every canonical parent up to its Scale-Slice ceiling.
//!
//! Capability-specific state (voxel caches, physical fields, simulation state,
//! etc.) keys itself beneath these context addresses rather than inventing an
//! unrelated spatial tree. Coverage remains capability-specific realized fact;
//! this topology answers only which contextual nodes currently have runtime
//! residency responsibility.

use std::collections::{HashMap, HashSet};

use bevy::prelude::*;

use super::{
    SPATIAL_SCALE_MAX, SpatialDemandScope, SpatialDemandSet, SpatialDemandSnapshot, SpatialScale,
    UsfChunkAddress, UsfPosition, UsfPositionError,
};
use super::position::USF_CHUNK_NATIVE_SIZE;

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

/// Per-frame aggregation point for runtime context demand.
///
/// Spatial demand is copied here automatically. Domain/capability adapters may
/// add derived scopes in [`UsfContextSet::Collect`].
#[derive(Resource, Debug, Default)]
pub struct UsfContextDemandBuffer {
    scopes: Vec<SpatialDemandScope>,
}

impl UsfContextDemandBuffer {
    pub fn request(&mut self, scope: SpatialDemandScope) {
        self.scopes.push(scope);
    }

    fn clear(&mut self) {
        self.scopes.clear();
    }

    fn iter(&self) -> impl ExactSizeIterator<Item = SpatialDemandScope> + '_ {
        self.scopes.iter().copied()
    }
}

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

    pub fn path_from_root(&self, leaf: UsfChunkAddress) -> Option<Vec<UsfChunkAddress>> {
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
    ) -> Result<(), UsfPositionError> {
        #[derive(Default)]
        struct DirectDemand {
            sources: HashSet<Entity>,
            maximum_priority: Option<i32>,
        }

        let mut direct = HashMap::<UsfChunkAddress, DirectDemand>::new();
        let mut resident = HashSet::<UsfChunkAddress>::new();

        for demand in scopes {
            for scope in addresses_intersecting_demand(demand)? {
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
        Ok(())
    }
}

fn addresses_intersecting_demand(
    demand: SpatialDemandScope,
) -> Result<Vec<UsfChunkAddress>, UsfPositionError> {
    let scale = demand.scale();
    let center = demand.center().reexpressed_at(scale)?;
    let anchor = UsfChunkAddress::containing(center, scale)?;
    let anchor_center = anchor.center();

    let local_center =
        center.relative_at_scale_bounded(&anchor_center, scale, USF_CHUNK_NATIVE_SIZE)?;
    let half = demand.half_extent_native().abs();
    let chunk_size = USF_CHUNK_NATIVE_SIZE;
    let half_chunk = chunk_size * 0.5;

    let minimum = checked_ivec3(
        ((local_center - half + Vec3::splat(half_chunk)) / chunk_size).floor(),
    )?;
    let maximum = checked_ivec3(
        ((local_center + half + Vec3::splat(half_chunk)) / chunk_size).floor(),
    )?;

    let mut result = Vec::new();
    for z in minimum.z..=maximum.z {
        for y in minimum.y..=maximum.y {
            for x in minimum.x..=maximum.x {
                result.push(anchor.translated_chunks(IVec3::new(x, y, z))?);
            }
        }
    }
    Ok(result)
}

fn checked_ivec3(value: Vec3) -> Result<IVec3, UsfPositionError> {
    fn component(value: f32) -> Result<i32, UsfPositionError> {
        let value64 = f64::from(value);
        if !value.is_finite() || value64 < i32::MIN as f64 || value64 > i32::MAX as f64 {
            Err(UsfPositionError::TranslationTooLarge)
        } else {
            Ok(value as i32)
        }
    }

    Ok(IVec3::new(
        component(value.x)?,
        component(value.y)?,
        component(value.z)?,
    ))
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UsfContextSet {
    Reset,
    Collect,
    Reconcile,
}

fn clear_context_demand(mut demand: ResMut<UsfContextDemandBuffer>) {
    demand.clear();
}

fn collect_spatial_context_demand(
    spatial: Res<SpatialDemandSnapshot>,
    mut demand: ResMut<UsfContextDemandBuffer>,
) {
    for scope in spatial.iter() {
        demand.request(scope);
    }
}

fn reconcile_context_topology(
    demand: Res<UsfContextDemandBuffer>,
    mut topology: ResMut<UsfContextTopology>,
) {
    if let Err(error) = topology.reconcile_from_scopes(demand.iter()) {
        error!(
            ?error,
            "USF context demand could not be represented canonically; retaining previous topology"
        );
    }
}

pub(in crate::spatial) fn configure(app: &mut App) {
    app.init_resource::<UsfContextDemandBuffer>()
        .init_resource::<UsfContextTopology>()
        .configure_sets(
            Update,
            UsfContextSet::Reset.after(SpatialDemandSet::Collect),
        )
        .configure_sets(
            Update,
            UsfContextSet::Collect.after(UsfContextSet::Reset),
        )
        .configure_sets(
            Update,
            UsfContextSet::Reconcile.after(UsfContextSet::Collect),
        )
        .add_systems(
            Update,
            clear_context_demand.in_set(UsfContextSet::Reset),
        )
        .add_systems(
            Update,
            collect_spatial_context_demand.in_set(UsfContextSet::Collect),
        )
        .add_systems(
            Update,
            reconcile_context_topology.in_set(UsfContextSet::Reconcile),
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
        topology
            .reconcile_from_scopes([demand(source, position, SpatialScale::ZERO)])
            .unwrap();

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
    fn finite_extent_materializes_every_intersected_leaf_context() {
        let mut world = World::new();
        let source = world.spawn_empty().id();
        let center = at_metres(490.0);
        let scope = SpatialDemandScope::at_scale(
            source,
            SpatialScale::ZERO,
            center,
            Vec3::new(20.0, 1.0, 1.0),
            9,
        );

        let left = UsfChunkAddress::containing(center, SpatialScale::ZERO).unwrap();
        let right = left.translated_chunks(IVec3::X).unwrap();

        let mut topology = UsfContextTopology::default();
        topology.reconcile_from_scopes([scope]).unwrap();

        assert!(topology.contains(left));
        assert!(topology.contains(right));
        assert_eq!(topology.node(left).unwrap().direct_demand_count(), 1);
        assert_eq!(topology.node(right).unwrap().direct_demand_count(), 1);
        assert_eq!(topology.node(right).unwrap().maximum_priority(), Some(9));
    }

    #[test]
    fn sibling_branches_share_ancestors_and_prune_without_orphans() {
        let mut world = World::new();
        let a = world.spawn_empty().id();
        let b = world.spawn_empty().id();
        let origin = at_metres(0.0);
        let neighbor = at_metres(1_250.0);
        let origin_leaf = UsfChunkAddress::containing(origin, SpatialScale::ZERO).unwrap();
        let neighbor_leaf =
            UsfChunkAddress::containing(neighbor, SpatialScale::ZERO).unwrap();
        let shared_parent =
            UsfChunkAddress::containing(origin, SpatialScale::new(1).unwrap()).unwrap();

        let mut topology = UsfContextTopology::default();
        topology
            .reconcile_from_scopes([
                demand(a, origin, SpatialScale::ZERO),
                demand(b, neighbor, SpatialScale::ZERO),
            ])
            .unwrap();

        assert_eq!(topology.len(), 37);
        assert_eq!(topology.node(shared_parent).unwrap().child_count(), 2);

        topology
            .reconcile_from_scopes([demand(a, origin, SpatialScale::ZERO)])
            .unwrap();

        assert_eq!(topology.len(), 36);
        assert!(topology.contains(origin_leaf));
        assert!(!topology.contains(neighbor_leaf));
        assert!(topology.contains(shared_parent));
    }

    #[test]
    fn query_depth_stops_at_requested_context_even_when_finer_nodes_exist() {
        let mut world = World::new();
        let source = world.spawn_empty().id();
        let position = at_metres(0.0);
        let mut topology = UsfContextTopology::default();
        topology
            .reconcile_from_scopes([demand(source, position, SpatialScale::ZERO)])
            .unwrap();

        let requested = SpatialScale::new(4).unwrap();
        let path = topology.path_for_position(&position, requested).unwrap();

        assert_eq!(path.last().unwrap().scale(), requested);
    }
}
