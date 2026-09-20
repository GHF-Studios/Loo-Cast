//! Character-motor and noclip movement input adapters.

use super::*;

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
            &PlayerNoclip,
            &UsfScaleLayer,
            &PlayerTravelSpeed,
            &PlayerAdaptiveCruise,
            &CharacterMovementConfig,
            Option<&PlayerScaleNavigation>,
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
        noclip,
        layer,
        travel_speed,
        cruise,
        movement_config,
        scale_navigation,
        mut input,
    ) = player.into_inner();

    if dead.is_some()
        || gameplay_suppressed(&keyboard, &capture)
        || noclip.active
        || cruise.active
        || scale_navigation.is_some()
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
    let base_speed = travel_speed.native_units_per_second(layer.scale());
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

pub(in crate::game::player) fn noclip_movement(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    capture: Res<CursorCapture>,
    player: Single<
        (
            &mut Transform,
            &CharacterLocomotionFrame,
            &CharacterControlFrame,
            Option<&PlayerDead>,
            &PlayerAim,
            &PlayerController,
            &PlayerNoclip,
            &UsfScaleLayer,
            &PlayerTravelSpeed,
            &PlayerAdaptiveCruise,
            Option<&PlayerScaleNavigation>,
            &mut LinearVelocity,
        ),
        With<Player>,
    >,
) {
    let (
        mut body,
        frame,
        control,
        dead,
        aim,
        controller,
        noclip,
        layer,
        travel_speed,
        cruise,
        scale_navigation,
        mut velocity,
    ) = player.into_inner();

    if dead.is_some()
        || !noclip.active
        || cruise.active
        || scale_navigation.is_some()
        || gameplay_suppressed(&keyboard, &capture)
    {
        return;
    }

    let horizontal = keyboard.pressed(KeyCode::KeyD) as i8 - keyboard.pressed(KeyCode::KeyA) as i8;
    let forward = keyboard.pressed(KeyCode::KeyW) as i8 - keyboard.pressed(KeyCode::KeyS) as i8;
    let vertical =
        keyboard.pressed(KeyCode::Space) as i8 - keyboard.pressed(KeyCode::ControlLeft) as i8;

    let view_rotation = control.rotation() * aim.local_rotation();
    let physical_up = frame.up();
    let mut wish = view_rotation * Vec3::X * horizontal as f32
        + view_rotation * Vec3::NEG_Z * forward as f32
        + physical_up * vertical as f32;

    wish = wish.normalize_or_zero();

    let boost = if keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight) {
        controller.sprint_multiplier
    } else {
        1.0
    };

    body.translation += wish
        * travel_speed.native_units_per_second(layer.scale())
        * boost
        * time.delta_secs();
    velocity.0 = Vec3::ZERO;
}


/// Free-flight through a non-character-scale USF chart.
///
/// Movement is view-relative and collisionless. Speed is chart-native rather
/// than metre-authored; scroll changes it logarithmically so the same mechanic
/// remains useful from satellite inspection through galactic navigation.
pub(in crate::game::player) fn scale_navigation_movement(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    capture: Res<CursorCapture>,
    player: Single<
        (
            &mut Transform,
            &CharacterControlFrame,
            Option<&PlayerDead>,
            &PlayerAim,
            &PlayerScaleNavigation,
            &UsfScaleLayer,
            &PlayerTravelSpeed,
            &PlayerAdaptiveCruise,
            Option<&mut LinearVelocity>,
        ),
        With<Player>,
    >,
) {
    let (
        mut body,
        control,
        dead,
        aim,
        _navigation,
        layer,
        travel_speed,
        cruise,
        velocity,
    ) = player.into_inner();

    if dead.is_some() || cruise.active || gameplay_suppressed(&keyboard, &capture) {
        return;
    }

    let horizontal =
        keyboard.pressed(KeyCode::KeyD) as i8 - keyboard.pressed(KeyCode::KeyA) as i8;
    let forward =
        keyboard.pressed(KeyCode::KeyW) as i8 - keyboard.pressed(KeyCode::KeyS) as i8;
    let vertical =
        keyboard.pressed(KeyCode::Space) as i8 - keyboard.pressed(KeyCode::ControlLeft) as i8;

    let view_rotation = control.rotation() * aim.local_rotation();
    let mut wish = view_rotation * Vec3::X * horizontal as f32
        + view_rotation * Vec3::NEG_Z * forward as f32
        + view_rotation * Vec3::Y * vertical as f32;
    wish = wish.normalize_or_zero();

    let boost = if keyboard.pressed(KeyCode::ShiftLeft)
        || keyboard.pressed(KeyCode::ShiftRight)
    {
        10.0
    } else {
        1.0
    };

    body.translation += wish
        * travel_speed.native_units_per_second(layer.scale())
        * boost
        * time.delta_secs();
    if let Some(mut velocity) = velocity {
        velocity.0 = Vec3::ZERO;
    }
}
