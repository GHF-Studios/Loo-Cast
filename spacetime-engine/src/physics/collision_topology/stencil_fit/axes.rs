//! Face-axis projections shared by fit and strict support checks.

use bevy::prelude::*;

pub(super) fn dominant_axis(vector: Vec3) -> (usize, f32) {
    let absolute = vector.abs();
    if absolute.x >= absolute.y && absolute.x >= absolute.z {
        (0, absolute.x)
    } else if absolute.y >= absolute.z {
        (1, absolute.y)
    } else {
        (2, absolute.z)
    }
}

pub(super) fn axis_vector(axis: usize) -> Vec3 {
    match axis {
        0 => Vec3::X,
        1 => Vec3::Y,
        _ => Vec3::Z,
    }
}

pub(super) fn component(vector: Vec3, axis: usize) -> f32 {
    match axis {
        0 => vector.x,
        1 => vector.y,
        _ => vector.z,
    }
}

pub(super) fn set_component(vector: &mut Vec3, axis: usize, value: f32) {
    match axis {
        0 => vector.x = value,
        1 => vector.y = value,
        _ => vector.z = value,
    }
}
