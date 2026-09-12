//! Instantaneous conventional portal traversal.
//!
//! This deliberately remains singular-entity teleportation. The visible
//! continuity problem while crossing is reserved for the later manifestation
//! experiment.

use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;

use crate::game::portal::{
    PortalActive,
    domain::{
        Portal,
        PortalTraveler,
        PortalVelocity,
    },
    topology::{
        crossing::crossed_aperture,
        mapping::{
            map_transform,
            portal_mapping,
        },
    },
};

pub(in super::super) fn teleport_travelers(
    portals: Query<(&Portal, &PortalActive, &Transform)>,
    mut travelers: Query<
        (
            &mut Transform,
            &mut PortalTraveler,
            Option<&mut PortalVelocity>,
            Option<&mut LinearVelocity>,
        ),
        Without<Portal>,
    >,
) {
    for (
        mut transform,
        mut traveler,
        mut portal_velocity,
        mut linear_velocity,
    ) in &mut travelers
    {
        let current =
            transform.translation;

        let Some(previous) =
            traveler.previous_position()
        else {
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

            let Ok((_, destination_active, destination)) =
                portals.get(portal.destination)
            else {
                continue;
            };

            if !destination_active.0 {
                continue;
            }

            crossing = Some((
                *source,
                *destination,
            ));

            break;
        }

        if let Some((source, destination)) =
            crossing
        {
            let mapping =
                portal_mapping(
                    &source,
                    &destination,
                );

            *transform =
                map_transform(
                    &transform,
                    &source,
                    &destination,
                );

            if let Some(velocity) =
                portal_velocity.as_deref_mut()
            {
                velocity.0 =
                    mapping.transform_vector3(
                        velocity.0,
                    );
            }

            if let Some(velocity) =
                linear_velocity.as_deref_mut()
            {
                velocity.0 =
                    mapping.transform_vector3(
                        velocity.0,
                    );
            }
        }

        // Commit after topology so arrival does not immediately retrigger the
        // destination portal.
        traveler.commit_position(
            transform.translation,
        );
    }
}
