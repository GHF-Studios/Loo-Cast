//! Human device input -> generic controlled-subject intent.

use super::*;

/// Samples human flight controls into device-agnostic subject intent.
///
/// AI/autopilot/network/replay controllers can write the same component without
/// impersonating keyboard or mouse input.
pub(in crate::game::player) fn sample_flight_control_intent(
    time: Res<Time>,
    input: Res<PlayerInputFrame>,
    controller: Single<(&PlayerController, &TravelPace, Option<&PlayerDead>), With<Player>>,
    subject: Single<&mut FlightControlIntent, With<LocalControlSubject>>,
) {
    let (controller, pace, dead) = controller.into_inner();
    let mut intent = subject.into_inner();

    if dead.is_some() || !input.gameplay_active() {
        intent.clear();
        return;
    }

    let horizontal = input.digital_axis(PlayerAction::MoveLeft, PlayerAction::MoveRight);
    let forward = input.digital_axis(PlayerAction::MoveBackward, PlayerAction::MoveForward);
    let vertical = input.digital_axis(PlayerAction::Descend, PlayerAction::Ascend);
    let boost = input.pressed(PlayerAction::Boost);

    // Current default manual-flight adapter: mouse delta requests local
    // angular rate. Alternative Elite/KSP-style policies can replace this
    // mapping without changing generic locomotion execution.
    let dt = time.delta_secs().max(1.0e-6);
    let look = input.look_delta();
    let angular_velocity = Vec3::new(
        -look.y * controller.look_sensitivity / dt,
        -look.x * controller.look_sensitivity / dt,
        0.0,
    );

    intent.set(
        Vec3::new(horizontal as f32, vertical as f32, forward as f32),
        FlightAttitudeCommand::AngularVelocityLocal(angular_velocity),
        pace.multiplier.max(0.0),
        boost,
    );
}

/// Samples local human controls once per render frame immediately before the
/// fixed character motor loop.
pub(in crate::game::player) fn movement(
    controls: Res<PlayerInputFrame>,
    controller: Single<
        (
            &PlayerController,
            &PlayerAim,
            &TravelPace,
            Option<&PlayerDead>,
        ),
        With<Player>,
    >,
    subject: Single<
        (
            &CharacterLocomotionFrame,
            &CharacterControlFrame,
            Option<&CharacterStance>,
            &ControlledSubjectLocomotion,
            &CharacterMovementConfig,
            &mut CharacterMovementInput,
        ),
        With<LocalControlSubject>,
    >,
) {
    let (controller, aim, travel_speed, dead) = controller.into_inner();
    let (frame, control, stance, locomotion, movement_config, mut input) =
        subject.into_inner();
    let crouched = stance.is_some_and(|stance| stance.crouched);

    if dead.is_some()
        || !controls.gameplay_active()
        || locomotion.kernel() != MotionKernel::Character
    {
        input.clear();
        return;
    }

    let sprinting = !crouched && controls.pressed(PlayerAction::Sprint);

    let stance_multiplier = if crouched {
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

    let horizontal =
        controls.digital_axis(PlayerAction::MoveLeft, PlayerAction::MoveRight);
    let forward =
        controls.digital_axis(PlayerAction::MoveBackward, PlayerAction::MoveForward);

    let axis = Vec2::new(horizontal, forward).clamp_length_max(1.0);
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
    input.jump_held = jump_enabled && controls.pressed(PlayerAction::Jump);
    input.jump_pressed |= jump_enabled && controls.just_pressed(PlayerAction::Jump);
}
