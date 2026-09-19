//! Authored kinematic motion state and fixed-step realization.

use super::*;

pub(super) fn authored_motion(base: Transform, motion: CompiledMotion) -> AuthoredMotion {
    AuthoredMotion {
        base,
        travel: motion.travel,
        period_seconds: motion.period_seconds,
        phase: motion.phase,
        elapsed_seconds: 0.0,
    }
}

pub(super) fn motion_state(motion: &AuthoredMotion) -> (Vec3, Vec3) {
    let phase = (motion.phase + motion.elapsed_seconds / motion.period_seconds).rem_euclid(1.0);
    let angle = std::f32::consts::TAU * phase;

    // Smooth endpoint-to-endpoint oscillation. Position starts at base when phase=0,
    // reaches base+travel at phase=0.5, then returns.
    let factor = 0.5 - 0.5 * angle.cos();
    let factor_velocity = 0.5 * std::f32::consts::TAU / motion.period_seconds * angle.sin();

    (
        motion.base.translation + motion.travel * factor,
        motion.travel * factor_velocity,
    )
}

pub(in crate::geometry) fn animate_authored_movers(
    time: Res<Time<Fixed>>,
    mut movers: Query<(&mut Transform, &mut LinearVelocity, &mut AuthoredMotion)>,
) {
    for (mut transform, mut velocity, mut motion) in &mut movers {
        motion.elapsed_seconds += time.delta_secs();

        let (translation, linear_velocity) = motion_state(&motion);
        transform.translation = translation;
        transform.rotation = motion.base.rotation;
        velocity.0 = linear_velocity;
    }
}
