use avian3d::prelude::{
    Collider,
    LinearVelocity,
    SpatialQuery,
    SpatialQueryFilter,
};
use bevy::{
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
};

use crate::{
    game::portal::PortalTraveler,
    physics::character::{
        CharacterDimensions,
        CharacterGroundState,
        CharacterMotor,
        CharacterMovementConfig,
        CharacterMovementInput,
    },
};

use super::{
    Player,
    PlayerAim,
    PlayerCamera,
    PlayerController,
    PlayerNoclip,
    PlayerStance,
    cursor::CursorCapture,
};

fn gameplay_suppressed(
    keyboard: &ButtonInput<KeyCode>,
    capture: &CursorCapture,
) -> bool {
    !capture.active()
        || keyboard.just_pressed(KeyCode::Tab)
        || keyboard.just_pressed(KeyCode::Escape)
}

pub fn look(
    mouse: Res<AccumulatedMouseMotion>,
    keyboard: Res<ButtonInput<KeyCode>>,
    capture: Res<CursorCapture>,
    player: Single<
        (&PlayerController, &mut PlayerAim),
        With<Player>,
    >,
) {
    if gameplay_suppressed(&keyboard, &capture) {
        return;
    }

    let (controller, mut aim) = player.into_inner();

    aim.yaw -= mouse.delta.x * controller.look_sensitivity;
    aim.pitch -= mouse.delta.y * controller.look_sensitivity;
    aim.pitch = aim.pitch.clamp(aim.min_pitch, aim.max_pitch);
}

/// `N` is the temporary direct binding for the developer `noclip` command.
pub fn toggle_noclip(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    capture: Res<CursorCapture>,
    mut player: Single<
        (
            Entity,
            &mut PlayerNoclip,
            &mut CharacterMovementInput,
            &mut CharacterGroundState,
            &mut LinearVelocity,
        ),
        With<Player>,
    >,
) {
    if gameplay_suppressed(&keyboard, &capture)
        || !keyboard.just_pressed(KeyCode::KeyN)
    {
        return;
    }

    let (entity, mut noclip, mut input, mut ground, mut velocity) =
        player.into_inner();

    noclip.active = !noclip.active;
    input.clear();
    velocity.0 = Vec3::ZERO;
    ground.grounded = false;
    ground.ground_entity = None;

    if noclip.active {
        commands.entity(entity).remove::<CharacterMotor>();
    } else {
        commands.entity(entity).insert(CharacterMotor);
    }
}

/// Changes the physical hull while keeping the feet fixed in body-local space.
/// Standing back up is refused while the standing hull would intersect geometry.
pub fn stance(
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
    mut camera: Single<&mut PlayerCamera>,
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

        let up = (body.rotation * Vec3::Y).normalize_or_zero();
        let up = if up == Vec3::ZERO { Vec3::Y } else { up };

        body.translation -= up * center_delta;
        *collider = CharacterDimensions::crouching_collider();
        stance.crouched = true;
        camera.first_person_offset =
            Vec3::Y * CharacterDimensions::CROUCH_CENTER_TO_EYE;
        traveler.commit_position(body.translation);
        return;
    }

    let (entity, target_center, rotation) = {
        let mut player = params.p1();
        let (entity, body, _, _, _, _) = player.into_inner();
        let up = (body.rotation * Vec3::Y).normalize_or_zero();
        let up = if up == Vec3::ZERO { Vec3::Y } else { up };

        (
            entity,
            body.translation + up * center_delta,
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
    camera.first_person_offset = Vec3::Y * CharacterDimensions::CENTER_TO_EYE;
    traveler.commit_position(body.translation);
}
/// Samples local controls once per render frame immediately before the fixed
/// loop. The fixed character motor then consumes this intent deterministically.
pub fn movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    capture: Res<CursorCapture>,
    player: Single<
        (
            &Transform,
            &PlayerAim,
            &PlayerController,
            &PlayerStance,
            &PlayerNoclip,
            &mut CharacterMovementConfig,
            &mut CharacterMovementInput,
        ),
        With<Player>,
    >,
) {
    let (
        body,
        aim,
        controller,
        stance,
        noclip,
        mut config,
        mut input,
    ) = player.into_inner();

    if gameplay_suppressed(&keyboard, &capture) || noclip.active {
        input.clear();
        return;
    }

    let sprinting = !stance.crouched
        && (keyboard.pressed(KeyCode::ShiftLeft)
            || keyboard.pressed(KeyCode::ShiftRight));

    config.max_ground_speed = controller.walk_speed
        * if stance.crouched {
            controller.crouch_speed_multiplier
        } else if sprinting {
            controller.sprint_multiplier
        } else {
            1.0
        };

    let horizontal =
        keyboard.pressed(KeyCode::KeyD) as i8
            - keyboard.pressed(KeyCode::KeyA) as i8;
    let forward =
        keyboard.pressed(KeyCode::KeyW) as i8
            - keyboard.pressed(KeyCode::KeyS) as i8;

    let axis = Vec2::new(horizontal as f32, forward as f32)
        .clamp_length_max(1.0);
    let local_wish = Vec3::new(axis.x, 0.0, -axis.y);
    let world_wish = body.rotation * (aim.yaw_rotation() * local_wish);

    input.set_wish(world_wish, axis.length());
    input.jump_held = keyboard.pressed(KeyCode::Space);
    input.jump_pressed |= keyboard.just_pressed(KeyCode::Space);
}

pub fn noclip_movement(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    capture: Res<CursorCapture>,
    mut player: Single<
        (
            &mut Transform,
            &PlayerAim,
            &PlayerController,
            &PlayerNoclip,
            &mut LinearVelocity,
        ),
        With<Player>,
    >,
) {
    let (mut body, aim, controller, noclip, mut velocity) = player.into_inner();

    if !noclip.active || gameplay_suppressed(&keyboard, &capture) {
        return;
    }

    let horizontal = keyboard.pressed(KeyCode::KeyD) as i8
        - keyboard.pressed(KeyCode::KeyA) as i8;
    let forward = keyboard.pressed(KeyCode::KeyW) as i8
        - keyboard.pressed(KeyCode::KeyS) as i8;
    let vertical = keyboard.pressed(KeyCode::Space) as i8
        - keyboard.pressed(KeyCode::ControlLeft) as i8;

    let view_rotation = body.rotation * aim.local_rotation();
    let physical_up = body.rotation * Vec3::Y;
    let mut wish = view_rotation * Vec3::X * horizontal as f32
        + view_rotation * Vec3::NEG_Z * forward as f32
        + physical_up * vertical as f32;

    wish = wish.normalize_or_zero();

    let boost = if keyboard.pressed(KeyCode::ShiftLeft)
        || keyboard.pressed(KeyCode::ShiftRight)
    {
        controller.sprint_multiplier
    } else {
        1.0
    };

    body.translation += wish * controller.noclip_speed * boost * time.delta_secs();
    velocity.0 = Vec3::ZERO;
}
