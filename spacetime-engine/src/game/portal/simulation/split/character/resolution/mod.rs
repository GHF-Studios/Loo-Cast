//! Character split resolution after ordinary motor movement.

use avian3d::{
    character_controller::move_and_slide::MoveAndSlide,
    prelude::*,
};
use bevy::prelude::*;

use crate::{
    ecs::UsfLogicalProjection,
    game::portal::{
        Portal, PortalActive, PortalSplitTraveler, PortalTraveler,
        topology::mapping::portal_plane,
    },
    physics::{
        character::{CharacterControlFrame, CharacterGroundState, CharacterLocomotionFrame},
        topology::{KinematicQueryExclusions, SpatialSplitBox, SpatialSplitPeer},
    },
};

use super::finish_character_split;
use super::super::{box_fits_aperture_at, center_crossing_fraction, CLEAR_MARGIN};

mod crossing;
mod remainder;

use crossing::{CrossingContext, CrossingState};

pub(crate) fn resolve_portal_splits(
    time: Res<Time<Fixed>>,
    move_and_slide: MoveAndSlide,
    portals: Query<(Entity, &Portal, &PortalActive, &Transform), With<Portal>>,
    mut travelers: Query<
        (
            Entity,
            &SpatialSplitBox,
            &mut Transform,
            Option<&CharacterLocomotionFrame>,
            Option<&mut CharacterControlFrame>,
            &mut LinearVelocity,
            &mut CharacterGroundState,
            &mut PortalTraveler,
            &mut PortalSplitTraveler,
            &mut KinematicQueryExclusions,
        ),
        (
            With<UsfLogicalProjection>,
            Without<SpatialSplitPeer>,
            Without<Portal>,
        ),
    >,
) {
    let dt = time.delta_secs().max(0.0);
    if dt <= 0.0 {
        return;
    }

    for (
        entity,
        split_box,
        mut body,
        locomotion_frame,
        mut control_frame,
        mut velocity,
        mut ground,
        mut traveler,
        mut split,
        mut exclusions,
    ) in &mut travelers
    {
        let peer = split.peer();

        let Some(active) = split.active else {
            traveler.commit_position(body.translation);
            continue;
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
            finish_character_split(&mut split, &mut body, locomotion_frame);
            exclusions.replace([peer]);
            traveler.commit_position(body.translation);
            continue;
        };

        let start = split.tick_start.translation;
        let end = body.translation;

        if let Some((fraction, side)) = center_crossing_fraction(source, start, end) {
            if source_portal.sidedness.allows(side)
                && box_fits_aperture_at(
                    *split_box,
                    body.rotation,
                    start.lerp(end, fraction),
                    source,
                    source_portal.half_size,
                )
            {
                crossing::resolve_crossing(
                    CrossingContext {
                        entity,
                        peer,
                        split_box: *split_box,
                        active,
                        source,
                        destination,
                        fraction,
                        dt,
                        move_and_slide: &move_and_slide,
                    },
                    CrossingState {
                        body: &mut body,
                        locomotion_frame,
                        control_frame: control_frame.as_deref_mut(),
                        velocity: &mut velocity,
                        ground: &mut ground,
                        split: &mut split,
                        exclusions: &mut exclusions,
                    },
                );
            }
        }

        if split_cleared_portal(*split_box, &body, &split, &portals) {
            finish_character_split(&mut split, &mut body, locomotion_frame);
            exclusions.replace([peer]);
        }

        traveler.commit_position(body.translation);
    }
}

fn split_cleared_portal(
    split_box: SpatialSplitBox,
    body: &Transform,
    split: &PortalSplitTraveler,
    portals: &Query<(Entity, &Portal, &PortalActive, &Transform), With<Portal>>,
) -> bool {
    let Some(active) = split.active else {
        return false;
    };
    let Ok((_, _, _, current_source)) = portals.get(active.source) else {
        return false;
    };
    let Some(plane) = portal_plane(current_source) else {
        return false;
    };

    let radius = split_box.projection_radius(body.rotation, plane.normal);
    plane.signed_distance(body.translation).abs() > radius + CLEAR_MARGIN
}
