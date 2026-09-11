use bevy::prelude::*;

/// Reject a vector's component along `axis`.
#[inline]
pub fn reject(vector: Vec3, axis: Vec3) -> Vec3 {
    vector - axis * vector.dot(axis)
}

/// Source/Quake-style acceleration.
#[inline]
pub fn accelerate(
    velocity: Vec3,
    wish_dir: Vec3,
    wish_speed: f32,
    acceleration: f32,
    surface_friction: f32,
    dt: f32,
) -> Vec3 {
    if wish_speed <= 0.0 || wish_dir == Vec3::ZERO {
        return velocity;
    }

    let current_speed = velocity.dot(wish_dir);
    let add_speed = wish_speed - current_speed;
    if add_speed <= 0.0 {
        return velocity;
    }

    let accel_speed =
        (acceleration * dt * wish_speed * surface_friction).min(add_speed);

    velocity + wish_dir * accel_speed
}

/// Source-style air acceleration.
///
/// Source caps wish speed for `add_speed`, but still uses uncapped wish speed
/// in the acceleration term. That asymmetry is intentional.
#[inline]
pub fn air_accelerate(
    velocity: Vec3,
    wish_dir: Vec3,
    wish_speed: f32,
    wish_speed_cap: Option<f32>,
    acceleration: f32,
    surface_friction: f32,
    dt: f32,
) -> Vec3 {
    if wish_speed <= 0.0 || wish_dir == Vec3::ZERO {
        return velocity;
    }

    let capped_wish_speed =
        wish_speed_cap.map_or(wish_speed, |cap| wish_speed.min(cap));
    let current_speed = velocity.dot(wish_dir);
    let add_speed = capped_wish_speed - current_speed;
    if add_speed <= 0.0 {
        return velocity;
    }

    let accel_speed =
        (acceleration * dt * wish_speed * surface_friction).min(add_speed);

    velocity + wish_dir * accel_speed
}

/// Source-style ground friction for a velocity already projected into the
/// locomotion plane.
#[inline]
pub fn apply_friction(
    planar_velocity: Vec3,
    friction: f32,
    stop_speed: f32,
    surface_friction: f32,
    dt: f32,
) -> Vec3 {
    let speed = planar_velocity.length();
    if speed <= f32::EPSILON {
        return Vec3::ZERO;
    }

    let control = speed.max(stop_speed);
    let drop = control * friction * surface_friction * dt;
    let new_speed = (speed - drop).max(0.0);

    if new_speed == speed {
        planar_velocity
    } else {
        planar_velocity * (new_speed / speed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acceleration_never_overshoots_wish_speed() {
        let velocity = accelerate(Vec3::ZERO, Vec3::X, 10.0, 1000.0, 1.0, 1.0);
        assert!((velocity.x - 10.0).abs() < 1e-5);
    }

    #[test]
    fn friction_cannot_reverse_velocity() {
        assert_eq!(
            apply_friction(Vec3::X, 100.0, 100.0, 1.0, 1.0),
            Vec3::ZERO,
        );
    }

    #[test]
    fn source_air_cap_limits_parallel_speed_gain() {
        let velocity =
            air_accelerate(Vec3::ZERO, Vec3::X, 10.0, Some(1.0), 1000.0, 1.0, 1.0);
        assert!((velocity.x - 1.0).abs() < 1e-5);
    }

    #[test]
    fn reject_removes_axis_component() {
        assert_eq!(
            reject(Vec3::new(1.0, 2.0, 3.0), Vec3::Y),
            Vec3::new(1.0, 0.0, 3.0),
        );
    }
}
