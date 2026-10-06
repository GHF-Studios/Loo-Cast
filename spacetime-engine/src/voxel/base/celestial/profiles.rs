//! Lunar and stellar macro relief profiles.

use super::*;

pub(super) fn stellar_macro_relative_relief(direction: Vec3, seed: u32) -> f32 {
    let phase = (seed as f32 / u32::MAX as f32) * std::f32::consts::TAU;
    (direction.dot(Vec3::new(0.7, 1.3, -1.1)) * 5.0 + phase).sin() * 0.00035
}

pub(super) fn lunar_macro_relative_relief(direction: Vec3) -> f32 {
    let mut height = (direction.dot(Vec3::new(1.7, -2.3, 0.9)) * 5.0).sin() * 0.0014
        + (direction.dot(Vec3::new(-3.1, 0.7, 2.4)) * 8.0).sin() * 0.0008;

    let craters = [
        (Vec3::new(0.82, 0.21, 0.53), 0.36_f32, 0.0100_f32),
        (Vec3::new(-0.51, 0.70, 0.49), 0.27, 0.0070),
        (Vec3::new(0.18, -0.88, 0.44), 0.23, 0.0060),
        (Vec3::new(-0.77, -0.23, -0.59), 0.19, 0.0048),
        (Vec3::new(0.39, 0.48, -0.79), 0.16, 0.0040),
        (Vec3::new(-0.08, -0.35, 0.93), 0.13, 0.0034),
        (Vec3::new(0.63, -0.66, -0.40), 0.11, 0.0028),
        (Vec3::new(-0.33, 0.14, -0.93), 0.095, 0.0024),
    ];

    for (raw_center, crater_radius, depth) in craters {
        let center = raw_center.normalize();
        let distance = direction.distance(center);
        let q = distance / crater_radius;

        if q < 1.0 {
            let bowl = 1.0 - q * q;
            height -= depth * bowl * bowl;
        }

        let rim_distance = ((q - 1.0) / 0.22).abs();
        if rim_distance < 1.0 {
            let rim = 1.0 - rim_distance;
            height += depth * 0.28 * rim * rim;
        }
    }

    height
}
