//! Ancestor-closed runtime residency over canonical USF space.
//!
//! Canonical USF topology is virtual and always addressable through
//! [`UsfChunkAddress`]. Runtime movement does not create or destroy that
//! topology. This module tracks only the sparse subset of canonical contexts
//! that currently have runtime responsibility.
//!
//! Primary spatial interest is copied from [`SpatialDemandSnapshot`].
//! Capability planners may add derived requirements through
//! [`UsfResidencyRequestBuffer`]. The resulting [`UsfContextResidency`] is
//! ancestor-closed: a resident child always retains every canonical ancestor up
//! to the Scale-Slice ceiling.
//!
//! Residency is lifecycle/context fact, not realization authority. Voxel,
//! field, physics and render systems remain free to choose their own
//! capability-local representations beneath the resident contexts.

use std::collections::{HashMap, HashSet};

use bevy::prelude::*;

use super::{
    SPATIAL_SCALE_MAX, SpatialDemandScope, SpatialDemandSet, SpatialDemandSnapshot, SpatialScale,
    UsfChunkAddress, UsfPosition, UsfPositionError,
};
use crate::usf::USF_CHUNK_NATIVE_SIZE;

/// One canonical context currently carrying runtime responsibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UsfResidentContext {
    scope: UsfChunkAddress,
    parent: Option<UsfChunkAddress>,
    direct_request_count: usize,
    child_count: usize,
    maximum_priority: Option<i32>,
}

impl UsfResidentContext {
    pub const fn scope(self) -> UsfChunkAddress {
        self.scope
    }

    pub const fn parent(self) -> Option<UsfChunkAddress> {
        self.parent
    }

    pub const fn direct_request_count(self) -> usize {
        self.direct_request_count
    }

    pub const fn child_count(self) -> usize {
        self.child_count
    }

    pub const fn maximum_priority(self) -> Option<i32> {
        self.maximum_priority
    }
}

/// Per-frame aggregation point for runtime context residency requirements.
///
/// Spatial interest is copied here automatically. Capability planners may add
/// derived scopes in [`UsfResidencySet::Collect`]. Requesting residency does not
/// prescribe which representation a subsystem must build inside that context.
#[derive(Resource, Debug, Default)]
pub struct UsfResidencyRequestBuffer {
    scopes: Vec<SpatialDemandScope>,
}

impl UsfResidencyRequestBuffer {
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

/// Sparse ancestor-closed set of canonical contexts with runtime responsibility.
#[derive(Resource, Debug, Default)]
pub struct UsfContextResidency {
    revision: u64,
    contexts: HashMap<UsfChunkAddress, UsfResidentContext>,
}

impl UsfContextResidency {
    pub const fn revision(&self) -> u64 {
        self.revision
    }

    pub fn len(&self) -> usize {
        self.contexts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.contexts.is_empty()
    }

    pub fn contains(&self, scope: UsfChunkAddress) -> bool {
        self.contexts.contains_key(&scope)
    }

    pub fn context(&self, scope: UsfChunkAddress) -> Option<UsfResidentContext> {
        self.contexts.get(&scope).copied()
    }

    pub fn iter(&self) -> impl ExactSizeIterator<Item = UsfResidentContext> + '_ {
        self.contexts.values().copied()
    }

    /// Returns root -> leaf for one resident canonical context.
    ///
    /// `None` means either the leaf is not resident or the ancestor-closure
    /// invariant has been violated.
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
        struct DirectRequest {
            sources: HashSet<Entity>,
            maximum_priority: Option<i32>,
        }

        let mut direct = HashMap::<UsfChunkAddress, DirectRequest>::new();
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
            let direct_request = direct.get(&scope);
            next.insert(
                scope,
                UsfResidentContext {
                    scope,
                    parent: scope.parent(),
                    direct_request_count: direct_request.map_or(0, |request| request.sources.len()),
                    child_count: child_counts.get(&scope).copied().unwrap_or(0),
                    maximum_priority: direct_request.and_then(|request| request.maximum_priority),
                },
            );
        }

        if self.contexts != next {
            self.contexts = next;
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
pub enum UsfResidencySet {
    Reset,
    Collect,
    Reconcile,
}

fn clear_residency_requests(mut demand: ResMut<UsfResidencyRequestBuffer>) {
    demand.clear();
}

fn collect_spatial_residency_requests(
    spatial: Res<SpatialDemandSnapshot>,
    mut demand: ResMut<UsfResidencyRequestBuffer>,
) {
    for scope in spatial.iter() {
        demand.request(scope);
    }
}

fn reconcile_context_residency(
    demand: Res<UsfResidencyRequestBuffer>,
    mut residency: ResMut<UsfContextResidency>,
) {
    if let Err(error) = residency.reconcile_from_scopes(demand.iter()) {
        error!(
            ?error,
            "USF residency demand could not be represented canonically; retaining previous residency"
        );
    }
}

pub(in crate::spatial) fn configure(app: &mut App) {
    app.init_resource::<UsfResidencyRequestBuffer>()
        .init_resource::<UsfContextResidency>()
        .configure_sets(
            Update,
            UsfResidencySet::Reset.after(SpatialDemandSet::Collect),
        )
        .configure_sets(
            Update,
            UsfResidencySet::Collect.after(UsfResidencySet::Reset),
        )
        .configure_sets(
            Update,
            UsfResidencySet::Reconcile.after(UsfResidencySet::Collect),
        )
        .add_systems(
            Update,
            clear_residency_requests.in_set(UsfResidencySet::Reset),
        )
        .add_systems(
            Update,
            collect_spatial_residency_requests.in_set(UsfResidencySet::Collect),
        )
        .add_systems(
            Update,
            reconcile_context_residency.in_set(UsfResidencySet::Reconcile),
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
    fn fine_residency_retains_the_complete_ancestor_path() {
        let mut world = World::new();
        let source = world.spawn_empty().id();
        let position = at_metres(0.0);
        let leaf = UsfChunkAddress::containing(position, SpatialScale::ZERO).unwrap();

        let mut residency = UsfContextResidency::default();
        residency
            .reconcile_from_scopes([demand(source, position, SpatialScale::ZERO)])
            .unwrap();

        assert_eq!(residency.len(), 36);
        let path = residency.path_from_root(leaf).unwrap();
        assert_eq!(path.len(), 36);
        assert_eq!(path.first().unwrap().scale(), SpatialScale::MAX);
        assert_eq!(path.last().copied(), Some(leaf));

        for context in residency.iter() {
            if let Some(parent) = context.parent() {
                assert!(residency.contains(parent));
            }
        }
    }

    #[test]
    fn finite_extent_marks_every_intersected_context_resident() {
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

        let mut residency = UsfContextResidency::default();
        residency.reconcile_from_scopes([scope]).unwrap();

        assert!(residency.contains(left));
        assert!(residency.contains(right));
        assert_eq!(residency.context(left).unwrap().direct_request_count(), 1);
        assert_eq!(residency.context(right).unwrap().direct_request_count(), 1);
        assert_eq!(residency.context(right).unwrap().maximum_priority(), Some(9));
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

        let mut residency = UsfContextResidency::default();
        residency
            .reconcile_from_scopes([
                demand(a, origin, SpatialScale::ZERO),
                demand(b, neighbor, SpatialScale::ZERO),
            ])
            .unwrap();

        assert_eq!(residency.len(), 37);
        assert_eq!(residency.context(shared_parent).unwrap().child_count(), 2);

        residency
            .reconcile_from_scopes([demand(a, origin, SpatialScale::ZERO)])
            .unwrap();

        assert_eq!(residency.len(), 36);
        assert!(residency.contains(origin_leaf));
        assert!(!residency.contains(neighbor_leaf));
        assert!(residency.contains(shared_parent));
    }
}
