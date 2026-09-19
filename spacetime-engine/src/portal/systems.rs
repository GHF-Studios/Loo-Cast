//! Portal runtime adapters for cross-cutting engine events.

use bevy::prelude::*;

use crate::spatial::UsfOriginRebased;
use super::{PortalSplitTraveler, PortalTraveler};

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
