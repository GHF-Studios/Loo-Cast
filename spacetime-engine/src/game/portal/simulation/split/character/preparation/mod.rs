//! Predictive character split activation before motor simulation.

use bevy::prelude::*;
use avian3d::prelude::LinearVelocity;

use crate::{
    ecs::UsfLogicalProjection,
    game::portal::{Portal, PortalActive, PortalSplitTraveler},
    physics::{
        character::CharacterLocomotionFrame,
        topology::{KinematicQueryExclusions, SpatialSplitBox, SpatialSplitPeer},
    },
};

use super::finish_character_split;
use super::super::{
    active_pair_is_valid, box_reaches_portal_this_tick, find_split_candidate,
};

/// Predictively opens portal-host collision before the character motor runs.
///
/// We open as soon as the rigid box overlaps the plane *or can reach it during
/// this fixed tick*. That prevents a floor portal from contributing a one-frame
/// ground contact before traversal gets a chance to happen.
pub(crate) fn prepare_portal_splits(
    time: Res<Time<Fixed>>,
    portals: Query<(Entity, &Portal, &PortalActive, &Transform), With<Portal>>,
    mut travelers: Query<
        (
            &mut Transform,
            Option<&CharacterLocomotionFrame>,
            &LinearVelocity,
            &SpatialSplitBox,
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

    for (mut body, locomotion_frame, velocity, split_box, mut split, mut exclusions) in
        &mut travelers
    {
        let peer = split.peer();

        if let Some(active) = split.active {
            if !active_pair_is_valid(active, &portals)
                || !box_reaches_portal_this_tick(
                    *split_box,
                    &body,
                    velocity.0,
                    dt,
                    active.source,
                    &portals,
                )
            {
                finish_character_split(&mut split, &mut body, locomotion_frame);
            }
        }

        split.tick_start = *body;

        if split.active.is_none() {
            split.active = find_split_candidate(*split_box, &body, velocity.0, dt, &portals);
        }

        // Whole-entity exclusions are reserved for the peer manifestation.
        // Portal hosts expose an actual clipped collider instead of disappearing
        // from the character controller's query world.
        exclusions.replace([peer]);
    }
}
