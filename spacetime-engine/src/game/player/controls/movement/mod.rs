//! Character-motor and noclip movement input adapters.

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
            &PlayerNoclip,
            &PlayerThrusters,
            &UsfScaleLayer,
            &PlayerTravelSpeed,
            &PlayerAdaptiveCruise,
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
        noclip,
        thrusters,
        layer,
        travel_speed,
        cruise,
        movement_config,
        mut input,
    ) = player.into_inner();

    if dead.is_some()
        || gameplay_suppressed(&keyboard, &capture)
        || (noclip.active && thrusters.enabled)
        || cruise.active
        || layer.scale() != SpatialScale::ZERO
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

pub(in crate::game::player) fn noclip_movement(
    time: Res<Time>,
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
            &PlayerNoclip,
            &PlayerThrusters,
            &UsfScaleLayer,
            &Collider,
            Option<&KinematicQueryExclusions>,
            &PlayerTravelSpeed,
            &PlayerAdaptiveCruise,
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
        noclip,
        thrusters,
        layer,
        collider,
        exclusions,
        travel_speed,
        cruise,
        mut velocity,
    ) = player.into_inner();

    if dead.is_some()
        || !noclip.active
        || !thrusters.enabled
        || cruise.active
        || gameplay_suppressed(&keyboard, &capture)
    {
        return;
    }

    let horizontal = keyboard.pressed(KeyCode::KeyD) as i8 - keyboard.pressed(KeyCode::KeyA) as i8;
    let forward = keyboard.pressed(KeyCode::KeyW) as i8 - keyboard.pressed(KeyCode::KeyS) as i8;
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

    let boost = if keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight) {
        controller.sprint_multiplier
    } else {
        1.0
    };

    let desired_velocity =
        wish * travel_speed.free_flight_native_units_per_second() * boost;
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


/// Collisionless manual navigation outside the canonical human-physics chart.
///
/// Speed still comes from [`PlayerTravelSpeed`] in canonical S0 units/s and is
/// projected into the active chart only at the final runtime adapter.
pub(in crate::game::player) fn scale_navigation_movement(
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
            &UsfScaleLayer,
            &PlayerTravelSpeed,
            &UsfNavigationContext,
            &PlayerAdaptiveCruise,
            Option<&mut LinearVelocity>,
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
        layer,
        travel_speed,
        navigation,
        cruise,
        velocity,
    ) = player.into_inner();

    if dead.is_some()
        || cruise.active
        || layer.scale() == SpatialScale::ZERO
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
        10.0
    } else {
        1.0
    };

    let context_speed = navigation.manual_native_units_per_second(layer.scale());
    body.translation += wish
        * context_speed
        * travel_speed.multiplier
        * boost
        * time.delta_secs();
    if let Some(mut velocity) = velocity {
        velocity.0 = Vec3::ZERO;
    }
}
