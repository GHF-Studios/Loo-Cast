//! One accepted character center crossing.

use avian3d::{
    character_controller::move_and_slide::MoveAndSlide,
    prelude::*,
};
use bevy::prelude::*;

use crate::{
    game::portal::{
        PortalSplitTraveler,
        domain::ActivePortalSplit,
        topology::mapping::{map_transform, portal_mapping},
    },
    physics::{
        character::{CharacterControlFrame, CharacterGroundState, CharacterLocomotionFrame},
        topology::{KinematicQueryExclusions, SpatialSplitBox},
    },
};

use super::remainder::simulate_destination_remainder;
use crate::game::portal::simulation::{
    CONTROL_INPUT_BLEND_DURATION, CONTROL_SETTLE_DURATION,
};

pub(super) struct CrossingContext<'a> {
    pub entity: Entity,
    pub peer: Entity,
    pub split_box: SpatialSplitBox,
    pub active: ActivePortalSplit,
    pub source: &'a Transform,
    pub destination: &'a Transform,
    pub fraction: f32,
    pub dt: f32,
    pub move_and_slide: &'a MoveAndSlide,
}

pub(super) struct CrossingState<'a> {
    pub body: &'a mut Transform,
    pub locomotion_frame: Option<&'a CharacterLocomotionFrame>,
    pub control_frame: Option<&'a mut CharacterControlFrame>,
    pub velocity: &'a mut LinearVelocity,
    pub ground: &'a mut CharacterGroundState,
    pub split: &'a mut PortalSplitTraveler,
    pub exclusions: &'a mut KinematicQueryExclusions,
}

pub(super) fn resolve_crossing(
    context: CrossingContext<'_>,
    state: CrossingState<'_>,
) {
    let start = state.split.tick_start.translation;
    let end = state.body.translation;
    let crossing = Transform {
        translation: start.lerp(end, context.fraction),
        rotation: state
            .split
            .tick_start
            .rotation
            .slerp(state.body.rotation, context.fraction),
        scale: Vec3::ONE,
    };

    let mapping = portal_mapping(context.source, context.destination);
    let mapped_crossing = map_transform(&crossing, context.source, context.destination);
    let mapped_velocity = mapping.transform_vector3(state.velocity.0);

    if let Some(control) = state.control_frame {
        let mapped_control = map_transform(
            &Transform::from_rotation(control.rotation()),
            context.source,
            context.destination,
        )
        .rotation;
        let target = state
            .locomotion_frame
            .map_or(mapped_control, |frame| frame.aligned_rotation(mapped_control));
        control.begin_settle(
            mapped_control,
            target,
            CONTROL_SETTLE_DURATION,
            CONTROL_INPUT_BLEND_DURATION,
        );
    }

    let filter = SpatialQueryFilter::from_excluded_entities([context.entity, context.peer]);
    let remaining = context.dt * (1.0 - context.fraction).clamp(0.0, 1.0);
    let (resolved_body, resolved_velocity) = simulate_destination_remainder(
        context.move_and_slide,
        context.split_box,
        mapped_crossing,
        mapped_velocity,
        context.destination,
        remaining,
        &filter,
    );

    *state.body = resolved_body;
    state.velocity.0 = resolved_velocity;

    state.ground.grounded = false;
    state.ground.ground_entity = None;
    state.ground.just_landed = false;
    state.ground.just_left_ground = true;

    // The authority now lives in the former destination space. Reversing the
    // active pair lets a reversal while still straddling cross back naturally.
    state.split.active = Some(ActivePortalSplit {
        source: context.active.destination,
        destination: context.active.source,
    });
    state.exclusions.replace([context.peer]);
}
