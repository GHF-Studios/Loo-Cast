//! Adaptive long-distance free-flight.
//!
//! Cruise speed is canonical (S0 units/s), while runtime displacement is
//! projected into whichever USF chart currently owns interaction.

use super::*;

const THROTTLE_RATE_PER_SECOND: f32 = 0.45;
const SPEED_RESPONSE: f64 = 1.4;
const VIEW_SCALE_RESPONSE: f32 = 1.6;
const APPROACH_HORIZON_SECONDS: f64 = 4.0;
const BODY_RADIUS_SPEED_FLOOR: f64 = 0.001;
const ABSOLUTE_MAX_SPEED_SCALE0: f64 = 1.0e30;

pub(in crate::game::player) fn adaptive_cruise_movement(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    capture: Res<CursorCapture>,
    presentation: Res<PrimaryViewPresentation>,
    frames: Res<UsfScaleLayerFrames>,
    influences: Query<&UsfTravelInfluence>,
    mut view: ResMut<UsfViewFrame>,
    player: Single<
        (
            &mut Transform,
            &UsfScaleLayer,
            &CharacterControlFrame,
            Option<&PlayerDead>,
            &PlayerAim,
            &mut PlayerAdaptiveCruise,
            Option<&mut LinearVelocity>,
        ),
        With<Player>,
    >,
) {
    let (mut body, layer, control, dead, aim, mut cruise, velocity) = player.into_inner();

    if !cruise.active {
        return;
    }
    if dead.is_some() || presentation.is_embedded() || gameplay_suppressed(&keyboard, &capture) {
        if let Some(mut velocity) = velocity {
            velocity.0 = Vec3::ZERO;
        }
        return;
    }

    let dt = time.delta_secs().max(0.0);
    if dt <= 0.0 {
        return;
    }

    let throttle_delta =
        keyboard.pressed(KeyCode::KeyW) as i8 - keyboard.pressed(KeyCode::KeyS) as i8;
    cruise.throttle =
        (cruise.throttle + throttle_delta as f32 * THROTTLE_RATE_PER_SECOND * dt)
            .clamp(0.0, 1.0);

    let player_absolute = frames.absolute(layer.scale(), body.translation);
    let mut speed_cap = ABSOLUTE_MAX_SPEED_SCALE0;
    let mut nearest_clearance = None::<f64>;

    for influence in &influences {
        let player_in_scale =
            frames.convert_absolute(player_absolute, layer.scale(), influence.scale());
        let center_distance = (influence.absolute() - player_in_scale).length();
        if !center_distance.is_finite() {
            continue;
        }

        let clearance_native = (center_distance - influence.radius_native()).max(0.0);
        let to_scale0 = 10.0_f64.powi(influence.scale().exponent() as i32);
        let clearance_scale0 = clearance_native * to_scale0;
        let radius_scale0 = influence.radius_native() * to_scale0;

        nearest_clearance = Some(
            nearest_clearance.map_or(clearance_scale0, |current| current.min(clearance_scale0)),
        );
        let local_cap = (clearance_scale0 / APPROACH_HORIZON_SECONDS)
            .max(radius_scale0 * BODY_RADIUS_SPEED_FLOOR);
        speed_cap = speed_cap.min(local_cap);
    }

    cruise.speed_cap_scale0 = speed_cap;
    cruise.nearest_clearance_scale0 = nearest_clearance;

    let requested = speed_cap * f64::from(cruise.throttle.powf(2.0));
    cruise.speed_scale0 = smooth_log_value(cruise.speed_scale0, requested, dt, SPEED_RESPONSE);

    let native_denominator = 10.0_f64.powi(layer.scale().exponent() as i32);
    let native_speed = (cruise.speed_scale0 / native_denominator)
        .clamp(0.0, f32::MAX as f64) as f32;
    let direction = (control.rotation() * aim.local_rotation() * Vec3::NEG_Z).normalize_or_zero();
    body.translation += direction * native_speed * dt;

    if let Some(mut velocity) = velocity {
        velocity.0 = Vec3::ZERO;
    }

    if cruise.speed_scale0 > 0.01 {
        let target_exponent = cruise.speed_scale0.log10().clamp(0.0, 35.0) as f32;
        let alpha = 1.0 - (-VIEW_SCALE_RESPONSE * dt).exp();
        let current = view.continuous_exponent();
        view.set_continuous_exponent(current + (target_exponent - current) * alpha);
    }
}

fn smooth_log_value(current: f64, target: f64, dt: f32, response: f64) -> f64 {
    let current_log = (1.0 + current.max(0.0)).log10();
    let target_log = (1.0 + target.max(0.0)).log10();
    let alpha = 1.0 - (-response * f64::from(dt)).exp();
    10.0_f64.powf(current_log + (target_log - current_log) * alpha) - 1.0
}
