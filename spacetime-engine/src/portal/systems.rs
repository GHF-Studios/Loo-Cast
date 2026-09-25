//! Portal runtime adapters for cross-cutting engine events.

use bevy::prelude::*;

use crate::spatial::{UsfOriginRebased, UsfSpatialTransitionApplied};
use super::{
    PortalSplitTraveler, PortalTraveler,
    simulation::split::retire_split_partition,
};

pub(super) fn rebase_portal_local_caches(
    mut rebases: MessageReader<UsfOriginRebased>,
    mut travelers: Query<&mut PortalTraveler>,
    mut split_travelers: Query<&mut PortalSplitTraveler>,
) {
    let shift = rebases
        .read()
        .fold(Vec3::ZERO, |total, rebase| total + rebase.local_shift);
    if shift == Vec3::ZERO {
        return;
    }

    for mut traveler in &mut travelers {
        traveler.rebase_local_origin(shift);
    }
    for mut traveler in &mut split_travelers {
        traveler.rebase_local_origin(shift);
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
