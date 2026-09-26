//! Portal runtime adapters for cross-cutting engine events.

use bevy::prelude::*;

use crate::spatial::{UsfOriginRebased, UsfSpatialTransitionApplied};
use super::{
    PortalSplitTraveler, PortalTraveler,
    simulation::split::retire_split_partition,
};

pub(super) fn refresh_portal_local_caches_after_rebase(
    mut rebases: MessageReader<UsfOriginRebased>,
    mut travelers: Query<(&Transform, &mut PortalTraveler)>,
    mut split_travelers: Query<(&Transform, &mut PortalSplitTraveler)>,
) {
    // A rebase is representation-only. Portal crossing history is disposable
    // runtime cache, so rebuild it from the already-rebased runtime projection
    // instead of applying USF chart deltas inside portal state.
    if rebases.read().next().is_none() {
        return;
    }

    for (transform, mut traveler) in &mut travelers {
        traveler.refresh_runtime_cache_after_rebase(transform.translation);
    }

    for (transform, mut traveler) in &mut split_travelers {
        traveler.refresh_runtime_cache_after_rebase(*transform);
    }
}

/// A canonical relocation/rechart invalidates local-space portal crossing history.
pub(super) fn reset_portal_spatial_transition_caches(
    mut commands: Commands,
    mut transitions: MessageReader<UsfSpatialTransitionApplied>,
    mut travelers: Query<(&Transform, &mut PortalTraveler)>,
    mut split_travelers: Query<(&Transform, &mut PortalSplitTraveler)>,
) {
    for transition in transitions.read() {
        if let Ok((transform, mut traveler)) = travelers.get_mut(transition.anchor) {
            traveler.reset_spatial_transition(transform.translation);
        }
        if let Ok((transform, mut traveler)) = split_travelers.get_mut(transition.anchor) {
            retire_split_partition(&mut commands, &mut traveler);
            traveler.reset_spatial_transition(*transform);
        }
    }
}
