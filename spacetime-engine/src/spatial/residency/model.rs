//! Resident context facts and request-buffer API.

use super::{
    SPATIAL_SCALE_MAX, SpatialDemandScope, SpatialScale, UsfChunkAddress, UsfPosition,
    UsfPositionError, address_range_intersecting_demand,
};
use bevy::prelude::*;
use std::collections::HashMap;

/// One canonical context currently carrying runtime responsibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UsfResidentContext {
    pub(super) scope: UsfChunkAddress,
    pub(super) parent: Option<UsfChunkAddress>,
    pub(super) direct_request_count: usize,
    pub(super) child_count: usize,
    pub(super) maximum_priority: Option<i32>,
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

    pub(super) fn clear(&mut self) {
        self.scopes.clear();
    }

    pub(super) fn iter(&self) -> impl ExactSizeIterator<Item = SpatialDemandScope> + '_ {
        self.scopes.iter().copied()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct UsfResidencyDemandRange {
    pub(super) source: Entity,
    pub(super) anchor: UsfChunkAddress,
    pub(super) minimum: IVec3,
    pub(super) maximum: IVec3,
}

/// Sparse ancestor-closed set of canonical contexts with runtime responsibility.
#[derive(Resource, Debug, Default)]
pub struct UsfContextResidency {
    pub(super) revision: u64,
    pub(super) contexts: HashMap<UsfChunkAddress, UsfResidentContext>,
    /// Canonicalized direct-demand footprint that produced `contexts`.
    ///
    /// Runtime demand centers may move every frame while still intersecting
    /// exactly the same canonical USF contexts. Caching this range-level plan
    /// makes that common case O(number of demand scopes) instead of rebuilding
    /// the entire ancestor-closed graph.
    pub(super) demand_plan: HashMap<UsfResidencyDemandRange, i32>,
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

    /// Returns the first context intersecting `demand` that is not resident.
    ///
    /// This validates context responsibility at native USF-context granularity
    /// rather than forcing capability-local subcells to reconstruct and hash the
    /// same canonical context repeatedly.
    pub fn first_missing_intersecting(
        &self,
        demand: SpatialDemandScope,
    ) -> Result<Option<UsfChunkAddress>, UsfPositionError> {
        let (anchor, minimum, maximum) = address_range_intersecting_demand(demand)?;

        for z in minimum.z..=maximum.z {
            for y in minimum.y..=maximum.y {
                for x in minimum.x..=maximum.x {
                    let scope = anchor.translated_chunks(IVec3::new(x, y, z))?;
                    if !self.contains(scope) {
                        return Ok(Some(scope));
                    }
                }
            }
        }

        Ok(None)
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
}
