//! Completion criterion for an active rigid split.

use bevy::prelude::*;

use crate::{
    portal::{
        Portal, PortalActive, PortalSplitTraveler,
        topology::mapping::portal_plane,
    },
    physics::topology::SpatialSplitBox,
};

use super::super::super::split::{active_pair_is_valid, CLEAR_MARGIN};

pub(super) fn split_should_finish(
    split_box: SpatialSplitBox,
    body: &Transform,
    split: &PortalSplitTraveler,
    portals: &Query<(Entity, &Portal, &PortalActive, &Transform), With<Portal>>,
) -> bool {
    let Some(active) = split.active else {
        return false;
    };
    if !active_pair_is_valid(active, portals) {
        return true;
    }

    portals
        .get(active.source)
        .ok()
        .and_then(|(_, _, _, source)| {
            let plane = portal_plane(source)?;
            let radius = split_box.projection_radius(body.rotation, plane.normal);
            Some(plane.signed_distance(body.translation).abs() > radius + CLEAR_MARGIN)
        })
        .unwrap_or(true)
}
