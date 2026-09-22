//! Motion-kernel input adapters.
//!
//! Every writer is gated by [`ControlledSubjectLocomotion::kernel`]. Exactly one
//! controlled-subject motion implementation may therefore own a frame.

use super::*;

use avian3d::character_controller::move_and_slide::{
    MoveAndSlide, MoveAndSlideConfig, MoveAndSlideHitResponse,
};
use crate::physics::{
    chart::UsfPhysicsCharts,
    topology::KinematicQueryExclusions,
};

/// Samples local controls once per render frame immediately before the fixed
/// loop. The fixed character motor consumes this intent deterministically.
pub(in crate::game::player) fn movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    capture: Res<CursorCapture>,
    player: Single<
        (
            &CharacterLocomotionFrame,
            &CharacterControlFrame,
            Option<&PlayerDead>,
            &PlayerAim,
            &PlayerController,
            &PlayerStance,
            &PlayerTravelSpeed,
            &ControlledSubjectLocomotion,
            &CharacterMovementConfig,
            &mut CharacterMovementInput,
        ),
        With<Player>,
    >,
) {
    let (
        frame,
        control,
        dead,
        aim,
        controller,
        stance,
        travel_speed,
        locomotion,
        movement_config,
        mut input,
    ) = player.into_inner();

    if dead.is_some()
        || gameplay_suppressed(&keyboard, &capture)
        || locomotion.kernel() != PlayerMotionKernel::Character
    {
        input.clear();
        return;
    }

    let sprinting = !stance.crouched
        && (keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight));

    let stance_multiplier = if stance.crouched {
        controller.crouch_speed_multiplier
    } else if sprinting {
        controller.sprint_multiplier
    } else {
        1.0
    };
    let base_speed =
        travel_speed.character_units_per_second(movement_config.max_ground_speed);
    let speed_multiplier = if movement_config.max_ground_speed > f32::EPSILON {
        stance_multiplier * base_speed / movement_config.max_ground_speed
    } else {
        0.0
    };

    let horizontal = keyboard.pressed(KeyCode::KeyD) as i8 - keyboard.pressed(KeyCode::KeyA) as i8;
    let forward = keyboard.pressed(KeyCode::KeyW) as i8 - keyboard.pressed(KeyCode::KeyS) as i8;

    let axis = Vec2::new(horizontal as f32, forward as f32).clamp_length_max(1.0);
    let up = frame.up();
    let view_forward = control.rotation() * (aim.yaw_rotation() * Vec3::NEG_Z);
    let planar_forward = view_forward - up * view_forward.dot(up);
    let forward = if planar_forward.length_squared() > 1.0e-8 {
        planar_forward.normalize()
    } else {
        frame.aligned_rotation(control.rotation()) * Vec3::NEG_Z
    };
    let right = forward.cross(up).normalize_or_zero();
    let world_wish = right * axis.x + forward * axis.y;

    let input_scale = control.movement_input_scale();
    input.set_wish(world_wish, axis.length() * input_scale);
    input.set_speed_multiplier(speed_multiplier);
    let jump_enabled = input_scale >= 0.5;
    input.jump_held = jump_enabled && keyboard.pressed(KeyCode::Space);
    input.jump_pressed |= jump_enabled && keyboard.just_pressed(KeyCode::Space);
}

fn free_flight_wish(
    control: &CharacterControlFrame,
    aim: &PlayerAim,
    physical_up: Vec3,
    horizontal: f32,
    forward: f32,
    vertical: f32,
) -> Vec3 {
    let view_rotation = control.rotation() * aim.local_rotation();
    (view_rotation * Vec3::X * horizontal
        + view_rotation * Vec3::NEG_Z * forward
        + physical_up * vertical)
        .normalize_or_zero()
}

fn canonical_speed_to_native(scale: SpatialScale, metres_per_second: f64) -> f32 {
    scale
        .scale0_to_native_f64(metres_per_second.max(0.0))
        .clamp(0.0, f64::from(f32::MAX)) as f32
}

/// Detailed-slice Local Flight kernel.
///
/// The travel policy is canonical. This kernel performs the one required
/// conversion into the current Scale Slice immediately before physics.
pub(in crate::game::player) fn local_flight_movement(
    time: Res<Time<Fixed>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    capture: Res<CursorCapture>,
    move_and_slide: MoveAndSlide,
    physics_charts: UsfPhysicsCharts,
    player: Single<
        (
            Entity,
            &mut Transform,
            &CharacterLocomotionFrame,
            &CharacterControlFrame,
            Option<&PlayerDead>,
            &PlayerAim,
            &PlayerController,
            &ControlledSubjectLocomotion,
            &UsfScaleLayer,
            &Collider,
            Option<&KinematicQueryExclusions>,
            &PlayerTravelSpeed,
            &PlayerTravelEnvelope,
            &mut LinearVelocity,
        ),
        With<Player>,
    >,
) {
    let (
        entity,
        mut body,
        frame,
        control,
        dead,
        aim,
        controller,
        locomotion,
        layer,
        collider,
        exclusions,
        travel_speed,
        envelope,
        mut velocity,
    ) = player.into_inner();

    if dead.is_some()
        || locomotion.kernel() != PlayerMotionKernel::ThrusterFlight
        || gameplay_suppressed(&keyboard, &capture)
    {
        return;
    }

    let horizontal =
        keyboard.pressed(KeyCode::KeyD) as i8 - keyboard.pressed(KeyCode::KeyA) as i8;
    let forward =
        keyboard.pressed(KeyCode::KeyW) as i8 - keyboard.pressed(KeyCode::KeyS) as i8;
    let vertical =
        keyboard.pressed(KeyCode::Space) as i8 - keyboard.pressed(KeyCode::ControlLeft) as i8;

    let wish = free_flight_wish(
        control,
        aim,
        frame.up(),
        horizontal as f32,
        forward as f32,
        vertical as f32,
    );

    let boost = if keyboard.pressed(KeyCode::ShiftLeft)
        || keyboard.pressed(KeyCode::ShiftRight)
    {
        controller.sprint_multiplier
    } else {
        1.0
    };

    let canonical_speed =
        envelope.manual_speed_metres_per_second * f64::from(travel_speed.multiplier.max(0.0))
            * f64::from(boost.max(0.0));
    let desired_velocity =
        wish * canonical_speed_to_native(layer.scale(), canonical_speed);

    let excluded = std::iter::once(entity)
        .chain(exclusions.into_iter().flat_map(|exclusions| exclusions.iter()));
    let filter = physics_charts.filter_for_scale(layer.scale(), excluded);
    let move_config = MoveAndSlideConfig::default();
    let moved = move_and_slide.move_and_slide(
        collider,
        body.translation,
        body.rotation,
        desired_velocity,
        time.delta(),
        &move_config,
        &filter,
        |_| MoveAndSlideHitResponse::Accept,
    );

    body.translation = moved.position;
    velocity.0 = moved.projected_velocity;
}

/// Coarse manual navigation through the physics kernel of the current Scale
/// Slice. The movement policy remains canonical and chart-independent.
pub(in crate::game::player) fn scale_navigation_movement(
    time: Res<Time<Fixed>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    capture: Res<CursorCapture>,
    move_and_slide: MoveAndSlide,
    physics_charts: UsfPhysicsCharts,
    player: Single<
        (
            Entity,
            &mut Transform,
            &CharacterLocomotionFrame,
            &CharacterControlFrame,
            Option<&PlayerDead>,
            &PlayerAim,
            &ControlledSubjectLocomotion,
            &UsfScaleLayer,
            &Collider,
            Option<&KinematicQueryExclusions>,
            &PlayerTravelSpeed,
            &PlayerTravelEnvelope,
            &PlayerTravelState,
            &mut LinearVelocity,
        ),
        With<Player>,
    >,
) {
    let (
        entity,
        mut body,
        frame,
        control,
        dead,
        aim,
        locomotion,
        layer,
        collider,
        exclusions,
        travel_speed,
        envelope,
        travel,
        mut velocity,
    ) = player.into_inner();

    if dead.is_some()
        || locomotion.kernel() != PlayerMotionKernel::ScaleNavigation
        || gameplay_suppressed(&keyboard, &capture)
    {
        return;
    }

    let horizontal =
        keyboard.pressed(KeyCode::KeyD) as i8 - keyboard.pressed(KeyCode::KeyA) as i8;
    let forward =
        keyboard.pressed(KeyCode::KeyW) as i8 - keyboard.pressed(KeyCode::KeyS) as i8;
    let vertical =
        keyboard.pressed(KeyCode::Space) as i8 - keyboard.pressed(KeyCode::ControlLeft) as i8;

    let wish = free_flight_wish(
        control,
        aim,
        frame.up(),
        horizontal as f32,
        forward as f32,
        vertical as f32,
    );

    let boost = if keyboard.pressed(KeyCode::ShiftLeft)
        || keyboard.pressed(KeyCode::ShiftRight)
    {
        4.0_f64
    } else {
        1.0_f64
    };

    let canonical_speed =
        envelope.manual_speed_metres_per_second * f64::from(travel_speed.multiplier.max(0.0))
            * boost;
    let desired_thrust =
        wish * canonical_speed_to_native(layer.scale(), canonical_speed);

    let gravity_native = layer.scale().metres_to_native_f32(travel.local_gravity);
    let free_fall = if gravity_native > 0.0 {
        frame.up() * velocity.0.dot(frame.up())
            - frame.up() * gravity_native * time.delta_secs()
    } else {
        Vec3::ZERO
    };
    let desired_velocity = desired_thrust + free_fall;

    let excluded = std::iter::once(entity)
        .chain(exclusions.into_iter().flat_map(|exclusions| exclusions.iter()));
    let filter = physics_charts.filter_for_scale(layer.scale(), excluded);
    let move_config = MoveAndSlideConfig::default();
    let moved = move_and_slide.move_and_slide(
        collider,
        body.translation,
        body.rotation,
        desired_velocity,
        time.delta(),
        &move_config,
        &filter,
        |_| MoveAndSlideHitResponse::Accept,
    );

    body.translation = moved.position;
    velocity.0 = moved.projected_velocity;
}
