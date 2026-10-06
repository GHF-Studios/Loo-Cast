//! Local freecam movement consumes bindings without claiming gameplay authority.

use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

use crate::{
    devtools::DeveloperScriptWorkbench,
    game::{
        control::LocalViewTarget,
        player::{
            Player, PlayerController,
            cursor::CursorCapture,
            input::{PlayerAction, PlayerInputBindings},
        },
    },
    input_focus::InputFocus,
    spatial::UsfScaleLayer,
};

use super::super::PlayerCamera;
use super::DebugFreecam;

pub(in crate::game::player) fn update_freecam(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    bindings: Res<PlayerInputBindings>,
    focus: Res<InputFocus>,
    capture: Res<CursorCapture>,
    controller: Single<&PlayerController, With<Player>>,
    view_target: Single<&UsfScaleLayer, With<LocalViewTarget>>,
    settings: Res<DebugFreecam>,
    script_workbench: Res<DeveloperScriptWorkbench>,
    mut camera: Single<&mut Transform, With<PlayerCamera>>,
) {
    if !settings.enabled()
        || !capture.active()
        || focus.pointer_claimed()
        || focus.gameplay_claimed()
    {
        return;
    }

    let controller = controller.into_inner();
    let layer = view_target.into_inner();

    let look = mouse_motion.delta;
    if look != Vec2::ZERO {
        let yaw = -look.x * controller.look_sensitivity;
        let pitch = -look.y * controller.look_sensitivity;
        camera.rotation =
            (Quat::from_rotation_y(yaw) * camera.rotation * Quat::from_rotation_x(pitch))
                .normalize();
    }

    let local_direction = movement_direction(&bindings, &keyboard, &mouse);
    if local_direction == Vec3::ZERO {
        return;
    }

    let distance_metres = movement_distance_metres(
        &settings,
        &script_workbench,
        &bindings,
        &keyboard,
        &mouse,
        time.delta_secs(),
    );
    let distance_native = layer.scale().metres_to_native_f32(distance_metres);
    let rotation = camera.rotation;
    camera.translation += rotation * local_direction * distance_native;
}

fn movement_direction(
    bindings: &PlayerInputBindings,
    keyboard: &ButtonInput<KeyCode>,
    mouse: &ButtonInput<MouseButton>,
) -> Vec3 {
    let pressed = |action| bindings.pressed_raw(action, keyboard, mouse);
    let axis = |positive, negative| (pressed(positive) as i8 - pressed(negative) as i8) as f32;
    Vec3::new(
        axis(PlayerAction::MoveRight, PlayerAction::MoveLeft),
        axis(PlayerAction::Ascend, PlayerAction::Descend),
        -axis(PlayerAction::MoveForward, PlayerAction::MoveBackward),
    )
    .normalize_or_zero()
}

fn movement_distance_metres(
    settings: &DebugFreecam,
    script_workbench: &DeveloperScriptWorkbench,
    bindings: &PlayerInputBindings,
    keyboard: &ButtonInput<KeyCode>,
    mouse: &ButtonInput<MouseButton>,
    delta_seconds: f32,
) -> f32 {
    let boost = if bindings.pressed_raw(PlayerAction::FastModifier, keyboard, mouse) {
        settings.boost_multiplier()
    } else {
        1.0
    };
    let scripted_speed = script_workbench
        .apply_live_scalar(f64::from(settings.translation_speed_mps()))
        .clamp(0.0, 1.0e9) as f32;
    scripted_speed * boost * delta_seconds.max(0.0)
}
