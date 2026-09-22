//! Physical crouch/stand transitions.
//!
//! This module changes only body-facing state: transform, collider, stance and
//! portal-traveler history. Camera presentation derives its eye offset from
//! [`PlayerStance`](super::PlayerStance) instead of being mutated here.

use avian3d::prelude::{Collider, SpatialQuery, SpatialQueryFilter};
use bevy::prelude::*;

use crate::{
    physics::{
        character::CharacterDimensions,
        topology::{KinematicQueryExclusions, SpatialSplitBox},
    },
    portal::PortalTraveler,
    spatial::UsfScaleLayer,
};

use super::{
    ControlledSubjectLocomotion, Player, PlayerDead, PlayerDetailedPhysicsScale,
    PlayerMotionKernel, PlayerStance,
    controls::gameplay_suppressed,
    cursor::CursorCapture,
};

/// Changes the physical hull while keeping the feet fixed in body-local space.
/// Standing back up is refused while the standing hull would intersect geometry.
pub fn update_stance(
    keyboard: Res<ButtonInput<KeyCode>>,
    capture: Res<CursorCapture>,
    mut params: ParamSet<(
        SpatialQuery,
        Single<
            (
                Entity,
                &mut Transform,
                Option<&mut Collider>,
                &UsfScaleLayer,
                &PlayerDetailedPhysicsScale,
                &mut PlayerStance,
                &ControlledSubjectLocomotion,
                Option<&PlayerDead>,
                &mut PortalTraveler,
                &mut SpatialSplitBox,
                Option<&KinematicQueryExclusions>,
            ),
            With<Player>,
        >,
    )>,
) {
    if gameplay_suppressed(&keyboard, &capture) {
        return;
    }

    // C belongs exclusively to Cruise. Crouch remains Ctrl.
    let wants_crouch = keyboard.pressed(KeyCode::ControlLeft);

    let (character_kernel, dead, crouched, detailed_slice) = {
        let player = params.p1();
        let (_, _, _, layer, detailed, stance, locomotion, dead, _, _, _) =
            player.into_inner();
        (
            locomotion.kernel() == PlayerMotionKernel::Character
                && layer.scale() == detailed.0,
            dead.is_some(),
            stance.crouched,
            layer.scale() == detailed.0,
        )
    };

    if dead
        || !character_kernel
        || !detailed_slice
        || wants_crouch == crouched
    {
        return;
    }

    let center_delta = CharacterDimensions::HALF_HEIGHT - CharacterDimensions::CROUCH_HALF_HEIGHT;

    if wants_crouch {
        let player = params.p1();
        let (
            _,
            mut body,
            collider,
            _,
            _,
            mut stance,
            _,
            _,
            mut traveler,
            mut split_box,
            _,
        ) = player.into_inner();
        let Some(mut collider) = collider else {
            return;
        };

        let up = physical_up(&body);
        body.translation -= up * center_delta;
        *collider = CharacterDimensions::crouching_collider();
        stance.crouched = true;
        split_box.half_extents = Vec3::new(
            CharacterDimensions::HULL_WIDTH * 0.5,
            CharacterDimensions::CROUCH_HALF_HEIGHT,
            CharacterDimensions::HULL_WIDTH * 0.5,
        );
        traveler.commit_position(body.translation);
        return;
    }

    let (entity, target_center, rotation, excluded) = {
        let player = params.p1();
        let (entity, body, _, _, _, _, _, _, _, _, exclusions) = player.into_inner();
        (
            entity,
            body.translation + physical_up(&body) * center_delta,
            body.rotation,
            exclusions
                .map(|exclusions| exclusions.iter().collect::<Vec<_>>())
                .unwrap_or_default(),
        )
    };

    let standing = CharacterDimensions::standing_collider();
    let filter =
        SpatialQueryFilter::from_excluded_entities(std::iter::once(entity).chain(excluded));

    if !params
        .p0()
        .shape_intersections(&standing, target_center, rotation, &filter)
        .is_empty()
    {
        return;
    }

    let player = params.p1();
    let (
        _,
        mut body,
        collider,
        _,
        _,
        mut stance,
        _,
        _,
        mut traveler,
        mut split_box,
        _,
    ) = player.into_inner();
    let Some(mut collider) = collider else {
        return;
    };

    body.translation = target_center;
    *collider = standing;
    stance.crouched = false;
    split_box.half_extents = Vec3::new(
        CharacterDimensions::HULL_WIDTH * 0.5,
        CharacterDimensions::HALF_HEIGHT,
        CharacterDimensions::HULL_WIDTH * 0.5,
    );
    traveler.commit_position(body.translation);
}

fn physical_up(body: &Transform) -> Vec3 {
    let up = (body.rotation * Vec3::Y).normalize_or_zero();
    if up == Vec3::ZERO { Vec3::Y } else { up }
}
