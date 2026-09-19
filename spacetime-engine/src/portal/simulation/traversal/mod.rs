//! Instantaneous conventional portal traversal fallback.
//!
//! Entities opted into `PortalSplitTraveler` are handled by the manifestation
//! split prototype instead. This path remains useful for simple travelers that
//! do not need partial presence on both sides of a portal.

use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;

use crate::{
    portal::{
        PortalActive,
        domain::{Portal, PortalTraveler, PortalVelocity},
        topology::{
            crossing::crossed_aperture,
            mapping::{map_transform, portal_mapping},
        },
    },
    physics::character::{CharacterControlFrame, CharacterLocomotionFrame},
};

use super::{CONTROL_INPUT_BLEND_DURATION, CONTROL_SETTLE_DURATION};

pub(in super::super) fn teleport_travelers(
    portals: Query<(&Portal, &PortalActive, &Transform)>,
    mut travelers: Query<
        (
            &mut Transform,
            &mut PortalTraveler,
            Option<&CharacterLocomotionFrame>,
            Option<&mut CharacterControlFrame>,
            Option<&mut PortalVelocity>,
            Option<&mut LinearVelocity>,
        ),
        (
            Without<Portal>,
            Without<crate::portal::PortalSplitTraveler>,
        ),
    >,
) {
    for (
        mut transform,
        mut traveler,
        locomotion_frame,
        mut control_frame,
        mut portal_velocity,
        mut linear_velocity,
    ) in &mut travelers
    {
        let current = transform.translation;

        let Some(previous) = traveler.previous_position() else {
            traveler.commit_position(current);
            continue;
        };

        let mut crossing = None;

        for (portal, active, source) in &portals {
            if !active.0 {
                continue;
            }
            if crossed_aperture(
                source,
                portal.half_size,
                portal.sidedness,
                previous,
                current,
            )
            .is_none()
            {
                continue;
            }

            let Ok((_, destination_active, destination)) = portals.get(portal.destination) else {
                continue;
            };

            if !destination_active.0 {
                continue;
            }

            crossing = Some((*source, *destination));

            break;
        }

        if let Some((source, destination)) = crossing {
            let mapping = portal_mapping(&source, &destination);

            if let Some(control) = control_frame.as_deref_mut() {
                let mapped_control = map_transform(
                    &Transform::from_rotation(control.rotation()),
                    &source,
                    &destination,
                )
                .rotation;
                let target = locomotion_frame.map_or(mapped_control, |frame| {
                    frame.aligned_rotation(mapped_control)
                });
                control.begin_settle(
                    mapped_control,
                    target,
                    CONTROL_SETTLE_DURATION,
                    CONTROL_INPUT_BLEND_DURATION,
                );
            }

            *transform = map_transform(&transform, &source, &destination);
            if let Some(frame) = locomotion_frame {
                transform.rotation = frame.aligned_rotation(transform.rotation);
            }

            if let Some(velocity) = portal_velocity.as_deref_mut() {
                velocity.0 = mapping.transform_vector3(velocity.0);
            }

            if let Some(velocity) = linear_velocity.as_deref_mut() {
                velocity.0 = mapping.transform_vector3(velocity.0);
            }
        }

        // Commit after topology so arrival does not immediately retrigger the
        // destination portal.
        traveler.commit_position(transform.translation);
    }
}
