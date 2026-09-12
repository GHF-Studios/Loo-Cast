//! Physical crouch/stand transitions.
//!
//! This module changes only body-facing state: transform, collider, stance and
//! portal-traveler history. Camera presentation derives its eye offset from
//! [`PlayerStance`](super::PlayerStance) instead of being mutated here.

use avian3d::prelude::{
    Collider,
    SpatialQuery,
    SpatialQueryFilter,
};
use bevy::prelude::*;

use crate::{
    game::portal::PortalTraveler,
    physics::character::CharacterDimensions,
};

use super::{
    Player,
    PlayerNoclip,
    PlayerStance,
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
                &mut Collider,
                &mut PlayerStance,
                &PlayerNoclip,
                &mut PortalTraveler,
            ),
            With<Player>,
        >,
    )>,
) {
    if gameplay_suppressed(&keyboard, &capture) {
        return;
    }

    let wants_crouch = keyboard.pressed(KeyCode::ControlLeft)
        || keyboard.pressed(KeyCode::KeyC);

    let (noclip_active, crouched) = {
        let mut player = params.p1();
        let (_, _, _, stance, noclip, _) = player.into_inner();
        (noclip.active, stance.crouched)
    };

    if noclip_active || wants_crouch == crouched {
        return;
    }

    let center_delta = CharacterDimensions::HALF_HEIGHT
        - CharacterDimensions::CROUCH_HALF_HEIGHT;

    if wants_crouch {
        let mut player = params.p1();
        let (_, mut body, mut collider, mut stance, _, mut traveler) =
            player.into_inner();

        let up = physical_up(&body);
        body.translation -= up * center_delta;
        *collider = CharacterDimensions::crouching_collider();
        stance.crouched = true;
        traveler.commit_position(body.translation);
        return;
    }

    let (entity, target_center, rotation) = {
        let mut player = params.p1();
        let (entity, body, _, _, _, _) = player.into_inner();
        (
            entity,
            body.translation + physical_up(&body) * center_delta,
            body.rotation,
        )
    };

    let standing = CharacterDimensions::standing_collider();
    let filter = SpatialQueryFilter::from_excluded_entities([entity]);

    if !params
        .p0()
        .shape_intersections(&standing, target_center, rotation, &filter)
        .is_empty()
    {
        return;
    }

    let mut player = params.p1();
    let (_, mut body, mut collider, mut stance, _, mut traveler) =
        player.into_inner();

    body.translation = target_center;
    *collider = standing;
    stance.crouched = false;
    traveler.commit_position(body.translation);
}

fn physical_up(body: &Transform) -> Vec3 {
    let up = (body.rotation * Vec3::Y).normalize_or_zero();
    if up == Vec3::ZERO { Vec3::Y } else { up }
}
