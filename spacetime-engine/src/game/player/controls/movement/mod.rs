//! Human device input -> generic controlled-subject intent.

use super::*;

/// Ship wheel routing: Alt changes view scale, Ctrl changes camera distance,
/// Shift changes characteristic pace by octaves, plain wheel trims throttle.
/// In debug traversal plain wheel controls pace directly.
pub(in crate::game::player) fn adjust_ship_scroll_controls(
    input: Res<PlayerInputFrame>,
    mut pace: Single<&mut TravelPace, With<Player>>,
    mut controlled_vehicle: Query<
        (
            &mut crate::game::locomotion::FlightThrottle,
            Option<&DeveloperMotionOverride>,
        ),
        (With<LocalControlSubject>, Without<Player>),
    >,
) {
    if !input.gameplay_active() || input.scroll_y() == 0.0 {
        return;
    }
    if let Some((mut throttle, debug)) = controlled_vehicle.iter_mut().next() {
        if input.pressed(PlayerAction::ViewScaleModifier) || input.pressed(PlayerAction::Descend) {
            return;
        }
        if input.pressed(PlayerAction::FastModifier)
            || debug.is_some_and(|state| state.characteristic_traversal())
        {
            pace.add_log2_steps(input.scroll_y().signum());
        } else {
            throttle.nudge(input.scroll_y().signum() * 0.05);
        }
    }
}

/// Samples human flight controls into device-agnostic subject intent.
///
/// AI/autopilot/network/replay controllers can write the same component without
/// impersonating keyboard or mouse input.
pub(in crate::game::player) fn sample_flight_control_intent(
    time: Res<Time>,
    input: Res<PlayerInputFrame>,
    camera: Single<&PlayerCamera>,
    controller: Single<(&TravelPace, &PlayerController, Option<&PlayerDead>), With<Player>>,
    subject: Single<
        (
            &MotionExecution,
            &crate::game::navigation::TravelProfile,
            &PilotAttitudeLaw,
            &FlightActuation,
            &mut FlightControlIntent,
        ),
        With<LocalControlSubject>,
    >,
) {
    let (pace, controller, dead) = controller.into_inner();
    let (execution, profile, law, _actuation, mut intent) = subject.into_inner();

    if dead.is_some()
        || !input.gameplay_active()
        || !execution.kernel().consumes_flight_control_intent()
    {
        intent.clear();
        return;
    }

    let horizontal = input.digital_axis(PlayerAction::MoveLeft, PlayerAction::MoveRight);
    let forward = input.digital_axis(PlayerAction::MoveBackward, PlayerAction::MoveForward);
    let vertical = if input.scroll_y() != 0.0 && input.pressed(PlayerAction::Descend) {
        0.0
    } else {
        input.digital_axis(PlayerAction::Descend, PlayerAction::Ascend)
    };
    let boost = input.pressed(PlayerAction::Boost) && input.scroll_y() == 0.0;

    // Pilot angular rate is distinct from observer orientation. Orbit mode
    // routes the mouse to the camera; keyboard axes remain available to steer.
    let attitude = match law {
        PilotAttitudeLaw::Hold => FlightAttitudeCommand::Hold,
        PilotAttitudeLaw::ManualRate => {
            let dt = time.delta_secs().max(1.0 / 240.0);
            let mouse = if camera.mode == CameraMode::Orbit {
                Vec2::ZERO
            } else {
                input.look_delta() * controller.look_sensitivity.max(0.0) / dt
            };
            let limits = Vec3::new(
                profile.flight.pitch_rate_radians_per_second,
                profile.flight.yaw_rate_radians_per_second,
                profile.flight.roll_rate_radians_per_second,
            );
            let rate = Vec3::new(
                input.digital_axis(PlayerAction::PitchDown, PlayerAction::PitchUp) * limits.x
                    - mouse.y,
                input.digital_axis(PlayerAction::YawRight, PlayerAction::YawLeft) * limits.y
                    - mouse.x,
                input.digital_axis(PlayerAction::RollRight, PlayerAction::RollLeft) * limits.z,
            )
            .clamp(-limits, limits);
            if rate.length_squared() <= 1.0e-8 {
                FlightAttitudeCommand::Hold
            } else {
                FlightAttitudeCommand::AngularVelocityLocal(rate)
            }
        }
    };

    intent.set(
        Vec3::new(horizontal, vertical, 0.0),
        attitude,
        pace.multiplier.max(0.0),
        boost,
    );
    intent.set_throttle_axis(forward);
}

/// Samples local human controls once per render frame immediately before the
/// fixed character motor loop.
pub(in crate::game::player) fn write_character_movement_intent(
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
            &MotionExecution,
            &CharacterMovementConfig,
            &mut CharacterMovementIntent,
        ),
        With<LocalControlSubject>,
    >,
) {
    let (controller, aim, travel_speed, dead) = controller.into_inner();
    let (frame, control, stance, execution, movement_config, mut input) = subject.into_inner();
    let crouched = stance.is_some_and(|stance| stance.crouched);

    if dead.is_some()
        || !controls.gameplay_active()
        || execution.kernel() != MotionKernel::Character
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
    let base_speed = travel_speed.character_units_per_second(movement_config.max_ground_speed);
    let speed_multiplier = if movement_config.max_ground_speed > f32::EPSILON {
        stance_multiplier * base_speed / movement_config.max_ground_speed
    } else {
        0.0
    };

    let horizontal = controls.digital_axis(PlayerAction::MoveLeft, PlayerAction::MoveRight);
    let forward = controls.digital_axis(PlayerAction::MoveBackward, PlayerAction::MoveForward);

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
