//! Instantaneous conventional portal traversal.
//!
//! This deliberately remains singular-entity teleportation. The visible
//! continuity problem while crossing is reserved for the later manifestation
//! experiment.

use bevy::prelude::*;

use crate::game::portal::{
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
    portals: Query<(&Portal, &Transform)>,
    mut travelers: Query<
        (
            &mut Transform,
            &mut PortalTraveler,
            Option<&mut PortalVelocity>,
        ),
        Without<Portal>,
    >,
) {
    for (
        mut transform,
        mut traveler,
        mut velocity,
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

        for (portal, source) in &portals {
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

            let Ok((_, destination)) =
                portals.get(portal.destination)
            else {
                continue;
            };

            crossing = Some((
                *source,
                *destination,
            ));

            break;
        }

        if let Some((source, destination)) =
            crossing
        {
            *transform =
                map_transform(
                    &transform,
                    &source,
                    &destination,
                );

            if let Some(velocity) =
                velocity.as_deref_mut()
            {
                velocity.0 =
                    portal_mapping(
                        &source,
                        &destination,
                    )
                    .transform_vector3(
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
