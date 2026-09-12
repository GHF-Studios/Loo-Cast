use bevy::prelude::*;

use crate::game::playground::AimRay;

pub fn ray_box_distance(
    ray: AimRay,
    transform: &Transform,
    half_extents: Vec3,
) -> Option<f32> {
    let inverse =
        transform
            .to_matrix()
            .inverse();

    let origin =
        inverse.transform_point3(
            ray.origin,
        );

    let direction =
        inverse.transform_vector3(
            ray.direction,
        );

    let min =
        -half_extents;

    let max =
        half_extents;

    let mut near =
        f32::NEG_INFINITY;

    let mut far =
        f32::INFINITY;

    for axis in 0..3 {
        let origin_axis =
            origin[axis];

        let direction_axis =
            direction[axis];

        if direction_axis.abs()
            <= f32::EPSILON
        {
            if origin_axis < min[axis]
                || origin_axis > max[axis]
            {
                return None;
            }

            continue;
        }

        let inverse_direction =
            1.0 / direction_axis;

        let mut first =
            (min[axis] - origin_axis)
                * inverse_direction;

        let mut second =
            (max[axis] - origin_axis)
                * inverse_direction;

        if first > second {
            std::mem::swap(
                &mut first,
                &mut second,
            );
        }

        near =
            near.max(first);

        far =
            far.min(second);

        if near > far {
            return None;
        }
    }

    if far < 0.0 {
        return None;
    }

    Some(
        near.max(0.0),
    )
}
