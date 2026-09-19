use super::*;

#[test]
fn floor_aligned_rectangle_can_touch_cuboid_face_edge() {
    let source = CollisionClipSource::cuboid(Vec3::new(5.2, 5.4, 0.4));
    let host = Transform::from_xyz(0.0, 2.7, 0.0);
    let stencil = Transform::from_xyz(0.0, 1.75, -0.2)
        .with_rotation(Quat::from_rotation_y(std::f32::consts::PI));

    assert!(supports_rectangular_stencil(
        source,
        &host,
        &stencil,
        Vec2::new(1.25, 1.75),
        1.0e-3,
    ));
}

#[test]
fn nearby_rectangle_snaps_to_face_edge() {
    let source = CollisionClipSource::cuboid(Vec3::new(5.2, 5.4, 0.4));
    let host = Transform::from_xyz(0.0, 2.7, 0.0);
    let requested = Transform::from_xyz(0.0, 1.68, -0.2)
        .with_rotation(Quat::from_rotation_y(std::f32::consts::PI));

    let fit = fit_rectangular_stencil(
        source,
        &host,
        &requested,
        Vec2::new(1.25, 1.75),
        1.0e-3,
        0.5,
        0.15,
    )
    .unwrap();

    assert!((fit.transform.translation.y - 1.75).abs() < 1.0e-5);
    assert_eq!(fit.snapped_edges, 1);
}

#[test]
fn fitted_roll_is_exact_quarter_turn_on_face() {
    let source = CollisionClipSource::cuboid(Vec3::new(8.0, 0.4, 8.0));
    let host = Transform::from_xyz(0.0, -0.2, 0.0);
    let base = Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2);
    let requested =
        Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_rotation_y(0.04) * base);

    let fit = fit_rectangular_stencil(
        source,
        &host,
        &requested,
        Vec2::new(1.25, 1.75),
        1.0e-3,
        0.5,
        0.0,
    )
    .unwrap();

    let right = fit.transform.rotation * Vec3::X;
    let up = fit.transform.rotation * Vec3::Y;
    let normal = fit.transform.rotation * Vec3::Z;
    assert!(normal.dot(Vec3::Y) > 0.99999);
    assert!(right.abs().max_element() > 0.99999);
    assert!(up.abs().max_element() > 0.99999);
}
