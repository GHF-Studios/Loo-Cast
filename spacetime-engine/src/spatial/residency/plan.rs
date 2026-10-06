//! Turns canonical demand ranges into an ancestor-closed resident graph.

use super::{
    SpatialDemandScope, UsfChunkAddress, UsfContextResidency, UsfPositionError,
    UsfResidencyDemandRange, UsfResidentContext, address_range_intersecting_demand,
};
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

type DemandPlan = HashMap<UsfResidencyDemandRange, i32>;

#[derive(Default)]
struct DirectRequest {
    count: usize,
    maximum_priority: Option<i32>,
}

impl UsfContextResidency {
    pub(crate) fn reconcile_from_scopes(
        &mut self,
        scopes: impl IntoIterator<Item = SpatialDemandScope>,
    ) -> Result<(), UsfPositionError> {
        let normalize_span = bevy::log::info_span!("usf_residency.normalize_plan").entered();
        let demand_plan = canonical_demand_plan(scopes)?;
        drop(normalize_span);
        if self.demand_plan == demand_plan {
            return Ok(());
        }

        let rebuild_span = bevy::log::info_span!("usf_residency.rebuild_graph").entered();
        let next = build_resident_contexts(&demand_plan)?;

        if self.contexts != next {
            self.contexts = next;
            self.revision = self.revision.wrapping_add(1).max(1);
        }
        self.demand_plan = demand_plan;
        drop(rebuild_span);
        Ok(())
    }
}

/// Motion within one canonical range does not change the graph. The source is
/// part of the key so overlapping observers count as distinct direct demand.
fn canonical_demand_plan(
    scopes: impl IntoIterator<Item = SpatialDemandScope>,
) -> Result<DemandPlan, UsfPositionError> {
    let mut plan = DemandPlan::new();
    for demand in scopes {
        let (anchor, minimum, maximum) = address_range_intersecting_demand(demand)?;
        let key = UsfResidencyDemandRange {
            source: demand.source(),
            anchor,
            minimum,
            maximum,
        };
        plan.entry(key)
            .and_modify(|priority| *priority = (*priority).max(demand.priority()))
            .or_insert(demand.priority());
    }
    Ok(plan)
}

fn build_resident_contexts(
    plan: &DemandPlan,
) -> Result<HashMap<UsfChunkAddress, UsfResidentContext>, UsfPositionError> {
    let mut direct = HashMap::<UsfChunkAddress, DirectRequest>::new();
    let mut direct_sources = HashSet::<(UsfChunkAddress, Entity)>::new();
    let mut resident = HashSet::<UsfChunkAddress>::new();

    for (range, &priority) in plan {
        for z in range.minimum.z..=range.maximum.z {
            for y in range.minimum.y..=range.maximum.y {
                for x in range.minimum.x..=range.maximum.x {
                    let scope = range.anchor.translated_chunks(IVec3::new(x, y, z))?;
                    let entry = direct.entry(scope).or_default();
                    if direct_sources.insert((scope, range.source)) {
                        entry.count += 1;
                    }
                    entry.maximum_priority = Some(
                        entry
                            .maximum_priority
                            .map_or(priority, |current| current.max(priority)),
                    );
                    retain_ancestors(scope, &mut resident);
                }
            }
        }
    }
    Ok(materialize_contexts(resident, direct))
}

fn retain_ancestors(scope: UsfChunkAddress, resident: &mut HashSet<UsfChunkAddress>) {
    let mut current = Some(scope);
    while let Some(address) = current {
        resident.insert(address);
        current = address.parent();
    }
}

fn materialize_contexts(
    resident: HashSet<UsfChunkAddress>,
    direct: HashMap<UsfChunkAddress, DirectRequest>,
) -> HashMap<UsfChunkAddress, UsfResidentContext> {
    let mut child_counts = HashMap::<UsfChunkAddress, usize>::with_capacity(resident.len());
    for scope in resident.iter().copied() {
        if let Some(parent) = scope.parent()
            && resident.contains(&parent)
        {
            *child_counts.entry(parent).or_default() += 1;
        }
    }

    let mut contexts = HashMap::with_capacity(resident.len());
    for scope in resident {
        let request = direct.get(&scope);
        contexts.insert(
            scope,
            UsfResidentContext {
                scope,
                parent: scope.parent(),
                direct_request_count: request.map_or(0, |request| request.count),
                child_count: child_counts.get(&scope).copied().unwrap_or(0),
                maximum_priority: request.and_then(|request| request.maximum_priority),
            },
        );
    }
    contexts
}
