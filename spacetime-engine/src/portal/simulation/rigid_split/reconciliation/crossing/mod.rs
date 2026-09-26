//! Mapping of a rigid authority when its center crosses the active portal.

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    portal::{
        Portal, PortalActive, PortalSplitTraveler,
        domain::ActivePortalSplit,
        topology::mapping::{map_transform, portal_mapping},
    },
    physics::topology::SpatialSplitBox,
};

use super::super::super::split::{box_fits_aperture_at, center_crossing_fraction};

pub(super) fn resolve_authority_crossing(
    split_box: SpatialSplitBox,
    body: &mut Transform,
    velocity: &mut LinearVelocity,
    angular_velocity: &mut AngularVelocity,
    split: &mut PortalSplitTraveler,
    portals: &Query<(Entity, &Portal, &PortalActive, &Transform), With<Portal>>,
) {
    let Some(active) = split.active else {
        return;
    };

    let pair = match (
        portals.get(active.source),
        portals.get(active.destination),
    ) {
        (
            Ok((_, source_portal, source_active, source)),
            Ok((_, _, destination_active, destination)),
        ) if source_active.0 && destination_active.0 => {
            Some((source_portal, source, destination))
        }
        _ => None,
    };
    let Some((source_portal, source, destination)) = pair else {
        return;
    };

    let start = split.tick_start.translation;
    let end = body.translation;
    let Some((fraction, side)) = center_crossing_fraction(source, start, end) else {
        return;
    };

    if !source_portal.sidedness.allows(side)
        || !box_fits_aperture_at(
            split_box,
            body.rotation,
            start.lerp(end, fraction),
            source,
            source_portal.half_size,
        )
    {
        return;
    }

    let mapping = portal_mapping(source, destination);
    *body = map_transform(body, source, destination);
    velocity.0 = mapping.transform_vector3(velocity.0);
    angular_velocity.0 = mapping.transform_vector3(angular_velocity.0);
    split.active = Some(ActivePortalSplit {
        source: active.destination,
        destination: active.source,
        partition: active.partition,
        realization: active.realization,
    });
}
