//! Aperture-fit and center-plane crossing geometry.

use bevy::prelude::*;

use crate::{
    game::portal::{
        domain::PortalSide,
        topology::mapping::portal_plane,
    },
    physics::topology::{SpatialSplitBox, SplitPlane},
};

/// Fit tolerance is positive: a hull exactly tangent to an aperture edge is a
/// valid traversal configuration. The collision stencil itself adds a small
/// physical clearance to keep numerical contacts from forming a lip.
const APERTURE_FIT_TOLERANCE: f32 = 0.01;
pub(super) const CROSSING_EPSILON: f32 = 1.0e-5;

pub(super) fn projected_crossing_center(
    center: Vec3,
    velocity: Vec3,
    dt: f32,
    plane: SplitPlane,
) -> Vec3 {
    let distance = plane.signed_distance(center);
    let normal_speed = velocity.dot(plane.normal);
    if normal_speed.abs() > CROSSING_EPSILON {
        let time = (-distance / normal_speed).clamp(0.0, dt);
        center + velocity * time
    } else {
        center - plane.normal * distance
    }
}

pub(super) fn box_fits_aperture_at(
    split_box: SpatialSplitBox,
    body_rotation: Quat,
    center: Vec3,
    portal: &Transform,
    half_size: Vec2,
) -> bool {
    let right = portal.rotation * Vec3::X;
    let up = portal.rotation * Vec3::Y;
    let local = portal.to_matrix().inverse().transform_point3(center);
    let radius_x = split_box.projection_radius(body_rotation, right);
    let radius_y = split_box.projection_radius(body_rotation, up);

    local.x.abs() + radius_x <= half_size.x + APERTURE_FIT_TOLERANCE
        && local.y.abs() + radius_y <= half_size.y + APERTURE_FIT_TOLERANCE
}

pub(super) fn center_crossing_fraction(
    portal: &Transform,
    start: Vec3,
    end: Vec3,
) -> Option<(f32, PortalSide)> {
    let plane = portal_plane(portal)?;
    let a = plane.signed_distance(start);
    let b = plane.signed_distance(end);

    let side = if a > CROSSING_EPSILON && b <= CROSSING_EPSILON {
        PortalSide::Front
    } else if a < -CROSSING_EPSILON && b >= -CROSSING_EPSILON {
        PortalSide::Back
    } else {
        return None;
    };

    let denominator = a - b;
    if denominator.abs() <= CROSSING_EPSILON {
        return None;
    }

    let fraction = a / denominator;
    (0.0..=1.0).contains(&fraction).then_some((fraction, side))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn center_crossing_reports_exact_fraction() {
        let portal = Transform::IDENTITY;
        let (fraction, side) =
            center_crossing_fraction(&portal, Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 0.0, -3.0))
                .unwrap();

        assert!((fraction - 0.25).abs() < 1.0e-6);
        assert_eq!(side, PortalSide::Front);
    }

    #[test]
    fn character_box_fits_floor_portal() {
        let split_box = SpatialSplitBox::from_size(Vec3::new(0.8128, 1.9, 0.8128));
        let floor_portal =
            Transform::IDENTITY.with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2));

        assert!(box_fits_aperture_at(
            split_box,
            Quat::IDENTITY,
            Vec3::ZERO,
            &floor_portal,
            Vec2::new(1.25, 1.75),
        ));
    }
}
