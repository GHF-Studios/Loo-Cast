//! Applies public portal-domain commands to persistent physical endpoints.
//!
//! Presentation is intentionally absent here. Placement resolves immutable
//! support geometry once, then publishes a generic collision stencil. The
//! physics layer owns actual host-collider reconstruction.

use bevy::prelude::*;

use crate::{
    game::portal::{
        Portal, PortalActive, PortalCommand, PortalEndpoint, PortalPair,
        domain::PortalSupport,
        simulation::placement::{coplanar_apertures_overlap, resolve_portal_placement},
    },
    physics::collision_topology::{CollisionClipSource, CollisionStencil},
};

pub fn apply_portal_commands(
    pair: Option<Res<PortalPair>>,
    mut messages: MessageReader<PortalCommand>,
    supports: Query<(Entity, &CollisionClipSource, &Transform), Without<Portal>>,
    mut portals: ParamSet<(
        Query<(Entity, &Portal, &PortalActive, &Transform, &PortalSupport), With<Portal>>,
        Query<(
            &Portal,
            &mut Transform,
            &mut PortalActive,
            &mut PortalSupport,
            &mut CollisionStencil,
        ), With<Portal>>,
    )>,
) {
    let Some(pair) = pair else {
        // Consume commands even if startup has not produced the persistent pair.
        for _ in messages.read() {}
        return;
    };

    for command in messages.read() {
        match command {
            PortalCommand::Place {
                endpoint,
                transform,
            } => {
                // Physical portals are rigid transforms. Invalid scale is a
                // malformed command, not presentation state to compensate for.
                if (transform.scale - Vec3::ONE).length_squared() > 1e-6 {
                    continue;
                }

                let target = pair.entity(*endpoint);
                let half_size = {
                    let read = portals.p0();
                    let Ok((_, portal, _, _, _)) = read.get(target) else {
                        continue;
                    };
                    portal.half_size
                };

                let Some(placement) =
                    resolve_portal_placement(*transform, half_size, &supports)
                else {
                    continue;
                };

                let overlaps_other = {
                    let read = portals.p0();
                    let other_entity = pair.entity(endpoint.other());
                    read.get(other_entity).is_ok_and(
                        |(_, other, other_active, other_transform, _)| {
                            other_active.0
                                && coplanar_apertures_overlap(
                                    &placement.transform,
                                    half_size,
                                    other_transform,
                                    other.half_size,
                                )
                        },
                    )
                };
                if overlaps_other {
                    continue;
                }

                {
                    let mut write = portals.p1();
                    if let Ok((_, mut current, mut active, mut support, mut stencil)) =
                        write.get_mut(target)
                    {
                        *current = placement.transform;
                        active.0 = true;
                        support.0 = Some(placement.support);
                        stencil.transform = placement.transform;
                        stencil.target = Some(placement.support);
                    }
                }
            }
            PortalCommand::Remove { endpoint } => {
                {
                    let mut write = portals.p1();
                    if let Ok((_, _, mut active, mut support, mut stencil)) =
                        write.get_mut(pair.entity(*endpoint))
                    {
                        active.0 = false;
                        support.0 = None;
                        stencil.enabled = false;
                        stencil.target = None;
                    }
                }
            }
            PortalCommand::RemovePair => {
                for endpoint in [PortalEndpoint::First, PortalEndpoint::Second] {
                    let mut write = portals.p1();
                    if let Ok((_, _, mut active, mut support, mut stencil)) =
                        write.get_mut(pair.entity(endpoint))
                    {
                        active.0 = false;
                        support.0 = None;
                        stencil.enabled = false;
                        stencil.target = None;
                    }
                }
            }
        }
    }

    // An isolated endpoint is not a hole to nowhere. Collision subtraction is
    // enabled only while both endpoints are active and supported.
    let pair_connected = {
        let read = portals.p0();
        [pair.first, pair.second].into_iter().all(|entity| {
            read.get(entity).is_ok_and(|(_, _, active, _, support)| {
                active.0 && support.0.is_some_and(|host| supports.get(host).is_ok())
            })
        })
    };

    let mut write = portals.p1();
    for endpoint in [PortalEndpoint::First, PortalEndpoint::Second] {
        if let Ok((portal, transform, active, support, mut stencil)) =
            write.get_mut(pair.entity(endpoint))
        {
            stencil.enabled = pair_connected && active.0 && support.0.is_some();
            stencil.transform = *transform;
            stencil.half_size = portal.half_size;
            stencil.target = support.0;
        }
    }
}
