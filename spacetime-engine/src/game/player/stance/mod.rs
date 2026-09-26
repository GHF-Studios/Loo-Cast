//! Physical crouch/stand transitions.
//!
//! This module mutates the authoritative [`PhysicalBoxHull`] and immediately
//! refreshes its detailed backend collider while keeping the feet fixed.
//! Camera presentation derives eye offset from [`CharacterStance`] separately.

use avian3d::prelude::{Collider, SpatialQuery};
use bevy::prelude::*;

use crate::{
    physics::{
        PhysicalBoxHull,
        slice::UsfPhysicsSlices,
        character::CharacterDimensions,
        topology::KinematicQueryExclusions,
    },
    portal::PortalTraveler,
    spatial::UsfScaleLayer,
};

use crate::game::control::LocalControlSubject;

use crate::game::locomotion::{
    CharacterStance, ControlledSubjectLocomotion, DetailedBodyScale, MotionKernel,
};

use super::{
    Player, PlayerDead,
    input::{PlayerAction, PlayerInputFrame},
};

/// Changes the detailed physical hull while keeping the feet fixed in body-local
/// space. Standing back up is refused while the standing hull would intersect
/// geometry.
pub fn update_stance(
    input: Res<PlayerInputFrame>,
    mut params: ParamSet<(
        SpatialQuery,
        UsfPhysicsSlices,
        Single<
            (
                Entity,
                &mut Transform,
                Option<&mut Collider>,
                &UsfScaleLayer,
                &DetailedBodyScale,
                &mut CharacterStance,
                &ControlledSubjectLocomotion,
                Option<&PlayerDead>,
                &mut PortalTraveler,
                &mut PhysicalBoxHull,
                Option<&KinematicQueryExclusions>,
            ),
            (With<Player>, With<LocalControlSubject>),
        >,
    )>,
) {
    if !input.gameplay_active() {
        return;
    }

    let wants_crouch = input.pressed(PlayerAction::Crouch);

    let (character_kernel, dead, crouched, detailed_slice) = {
        let player = params.p2();
        let (_, _, _, layer, detailed, stance, locomotion, dead, _, _, _) =
            player.into_inner();
        (
            locomotion.kernel() == MotionKernel::Character
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

    let center_delta_metres =
        CharacterDimensions::HALF_HEIGHT - CharacterDimensions::CROUCH_HALF_HEIGHT;

    if wants_crouch {
        let player = params.p2();
        let (
            _,
            mut body,
            collider,
            layer,
            _,
            mut stance,
            _,
            _,
            mut traveler,
            mut hull,
            _,
        ) = player.into_inner();
        let Some(mut collider) = collider else {
            return;
        };

        let up = physical_up(&body);
        let center_delta_native =
            layer.scale().metres_to_native_f32(center_delta_metres);
        body.translation -= up * center_delta_native;

        let crouching = CharacterDimensions::crouching_hull();
        *collider = crouching.collider(layer.scale());
        *hull = crouching;
        stance.crouched = true;
        traveler.commit_position(body.translation);
        return;
    }

    let (entity, target_center, rotation, scale, excluded) = {
        let player = params.p2();
        let (entity, body, _, layer, _, _, _, _, _, _, exclusions) = player.into_inner();
        let center_delta_native =
            layer.scale().metres_to_native_f32(center_delta_metres);
        (
            entity,
            body.translation + physical_up(&body) * center_delta_native,
            body.rotation,
            layer.scale(),
            exclusions
                .map(|exclusions| exclusions.iter().collect::<Vec<_>>())
                .unwrap_or_default(),
        )
    };

    let standing_hull = CharacterDimensions::standing_hull();
    let standing = standing_hull.collider(scale);
    let filter = params
        .p1()
        .filter_for_scale(scale, std::iter::once(entity).chain(excluded));

    if !params
        .p0()
        .shape_intersections(&standing, target_center, rotation, &filter)
        .is_empty()
    {
        return;
    }

    let player = params.p2();
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
        mut hull,
        _,
    ) = player.into_inner();
    let Some(mut collider) = collider else {
        return;
    };

    body.translation = target_center;
    *collider = standing;
    *hull = standing_hull;
    stance.crouched = false;
    traveler.commit_position(body.translation);
}

fn physical_up(body: &Transform) -> Vec3 {
    let up = (body.rotation * Vec3::Y).normalize_or_zero();
    if up == Vec3::ZERO { Vec3::Y } else { up }
}
