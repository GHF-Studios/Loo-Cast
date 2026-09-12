use bevy::prelude::*;

use super::super::{
    domain::{PortalSide, PortalSidedness},
    topology::crossing::crossed_aperture,
};

#[test]
fn crossing_is_independent_of_world_orientation() {
    let portal = Transform::from_xyz(3.0, 4.0, -7.0).with_rotation(Quat::from_euler(
        EulerRot::XYZ,
        0.9,
        -0.4,
        0.7,
    ));

    let previous = portal
        .to_matrix()
        .transform_point3(Vec3::new(0.2, 0.3, 1.0));

    let current = portal
        .to_matrix()
        .transform_point3(Vec3::new(0.2, 0.3, -1.0));

    assert_eq!(
        crossed_aperture(
            &portal,
            Vec2::ONE,
            PortalSidedness::TwoSided,
            previous,
            current,
        ),
        Some(PortalSide::Front),
    );
}

#[test]
fn one_sided_rejects_back_to_front() {
    let portal = Transform::IDENTITY;

    assert_eq!(
        crossed_aperture(
            &portal,
            Vec2::ONE,
            PortalSidedness::OneSided,
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 0.0, 1.0),
        ),
        None,
    );
}
