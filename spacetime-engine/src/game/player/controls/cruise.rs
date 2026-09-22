//! Adaptive long-distance travel kernel.
//!
//! Cruise consumes the same canonical travel envelope as ordinary flight.
//! Scale Slice conversion happens only at the final displacement boundary.

use super::*;

const THROTTLE_RATE_PER_SECOND: f32 = 0.45;
const SPEED_RESPONSE: f64 = 1.4;

pub(in crate::game::player) fn adaptive_cruise_movement(
    time: Res<Time<Fixed>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    capture: Res<CursorCapture>,
    presentation: Res<PrimaryViewPresentation>,
    mut was_active: Local<bool>,
    mut was_explicit: Local<bool>,
    player: Single<
        (
            &mut Transform,
            &UsfScaleLayer,
            &CharacterControlFrame,
            Option<&PlayerDead>,
            &PlayerAim,
            &ControlledSubjectLocomotion,
            &mut PlayerAdaptiveCruise,
            &PlayerTravelEnvelope,
            &PlayerTravelState,
            Option<&mut LinearVelocity>,
        ),
        With<Player>,
    >,
) {
    let (
        mut body,
        layer,
        control,
        dead,
        aim,
        locomotion,
        mut cruise,
        envelope,
        travel,
        velocity,
    ) = player.into_inner();

    if locomotion.kernel() != PlayerMotionKernel::Cruise {
        *was_active = false;
        *was_explicit = false;
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

    let explicit =
        locomotion.request() == PlayerLocomotionRequest::Regime(PlayerLocomotionRegime::Cruise);
    let just_engaged = !*was_active;
    let just_explicitly_engaged = explicit && !*was_explicit;
    *was_active = true;
    *was_explicit = explicit;

    cruise.speed_cap_scale0 = envelope.cruise_max_speed_metres_per_second;
    cruise.default_speed_scale0 = envelope.cruise_default_speed_metres_per_second;
    cruise.nearest_hard_clearance_scale0 = travel.nearest_body_clearance_scale0;
    cruise.medium_speed_cap_scale0 = envelope.medium_speed_cap_metres_per_second;

    if just_explicitly_engaged {
        cruise.throttle = throttle_for_speed(
            envelope.cruise_default_speed_metres_per_second,
            envelope.cruise_max_speed_metres_per_second,
        );
        cruise.speed_scale0 = envelope.cruise_default_speed_metres_per_second;
    } else if just_engaged {
        // Automatic deep-space Cruise is a semantic domain, not an instruction
        // to launch forward. It begins idle until the player supplies throttle.
        cruise.throttle = 0.0;
        cruise.speed_scale0 = 0.0;
    }

    let throttle_delta =
        keyboard.pressed(KeyCode::KeyW) as i8 - keyboard.pressed(KeyCode::KeyS) as i8;
    cruise.throttle =
        (cruise.throttle + throttle_delta as f32 * THROTTLE_RATE_PER_SECOND * dt)
            .clamp(0.0, 1.0);

    let requested = envelope.cruise_max_speed_metres_per_second
        * f64::from(cruise.throttle.powf(2.0));
    cruise.speed_scale0 =
        smooth_log_value(cruise.speed_scale0, requested, dt, SPEED_RESPONSE);

    let native_speed = layer
        .scale()
        .scale0_to_native_f64(cruise.speed_scale0.max(0.0))
        .clamp(0.0, f64::from(f32::MAX)) as f32;
    let direction =
        (control.rotation() * aim.local_rotation() * Vec3::NEG_Z).normalize_or_zero();

    body.translation += direction * native_speed * dt;

    if let Some(mut velocity) = velocity {
        velocity.0 = direction * native_speed;
    }
}

fn throttle_for_speed(speed_metres_per_second: f64, max_metres_per_second: f64) -> f32 {
    if !speed_metres_per_second.is_finite()
        || !max_metres_per_second.is_finite()
        || max_metres_per_second <= 0.0
    {
        return 0.0;
    }
    (speed_metres_per_second / max_metres_per_second)
        .clamp(0.0, 1.0)
        .sqrt() as f32
}

fn smooth_log_value(current: f64, target: f64, dt: f32, response: f64) -> f64 {
    let current_log = (1.0 + current.max(0.0)).log10();
    let target_log = (1.0 + target.max(0.0)).log10();
    let alpha = 1.0 - (-response * f64::from(dt)).exp();
    10.0_f64.powf(current_log + (target_log - current_log) * alpha) - 1.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engagement_throttle_reconstructs_default_speed() {
        let maximum = 80_000.0;
        let default = 20_000.0;
        let throttle = throttle_for_speed(default, maximum);
        let requested = maximum * f64::from(throttle.powf(2.0));
        assert!((requested - default).abs() < 1.0);
    }
}
