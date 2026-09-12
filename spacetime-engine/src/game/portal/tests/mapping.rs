use bevy::prelude::*;

use super::super::topology::mapping::portal_mapping;

#[test]
fn opposite_mappings_are_inverse_with_arbitrary_rotations() {
    let first = Transform::from_xyz(-3.0, 2.0, 4.0).with_rotation(Quat::from_euler(
        EulerRot::XYZ,
        0.4,
        -0.7,
        1.1,
    ));

    let second = Transform::from_xyz(7.0, -2.0, -1.0).with_rotation(Quat::from_euler(
        EulerRot::XYZ,
        -0.8,
        0.2,
        -0.5,
    ));

    let round_trip = portal_mapping(&second, &first) * portal_mapping(&first, &second);

    assert!(round_trip.abs_diff_eq(Mat4::IDENTITY, 0.0001,));
}
