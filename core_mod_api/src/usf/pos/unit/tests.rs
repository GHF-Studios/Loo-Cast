#[test]
fn unit_extent_zoom_out_test_1() {
    use crate::unit_extent;

    let mut a = unit_extent!([(0, 0, 0), (0, 0, 0), (0, 0, 0)]: (200.0, 200.0, 0.0));
    a.zoom_out();
    let expected = unit_extent!([(0, 0, 0), (0, 0, 0)]: (20.0, 20.0, 0.0));
    assert_eq!(a, expected);
}

#[test]
fn unit_extent_zoom_out_test_2() {
    use crate::unit_extent;

    let mut a = unit_extent!([(0, 0, 0), (4, 4, 0), (3, 3, 0), (2, 2, 0), (1, 1, 0)]: (0.0, 0.0, 0.0));
    a.zoom_out();
    let expected = unit_extent!([(0, 0, 0), (4, 4, 0), (3, 3, 0), (2, 2, 0)]: (100.0, 100.0, 0.0));
    assert_eq!(a, expected);
}

#[test]
fn unit_extent_zoom_out_test_3() {
    use crate::unit_extent;

    let mut a = unit_extent!([(0, 0, 0), (4, 4, 0), (3, 3, 0), (2, 2, 0), (1, 1, 0)]: (499.9, 499.9, 0.0));
    a.zoom_out();
    let expected = unit_extent!([(0, 0, 0), (4, 4, 0), (3, 3, 0), (2, 2, 0)]: (149.98999, 149.98999, 0.0));
    assert_eq!(a, expected);
}

#[test]
fn unit_extent_zoom_in_test_1() {
    use crate::unit_extent;

    let mut a = unit_extent!([(0, 0, 0), (0, 0, 0)]: (20.0, 20.0, 0.0));
    a.zoom_in();
    let expected = unit_extent!([(0, 0, 0), (0, 0, 0), (0, 0, 0)]: (200.0, 200.0, 0.0));
    assert_eq!(a, expected);
}

#[test]
fn unit_extent_zoom_in_test_2() {
    use crate::unit_extent;

    let mut a = unit_extent!([(0, 0, 0)]: (200.0, 200.0, 0.0));
    a.zoom_in();
    let expected = unit_extent!([(0, 0, 0), (2, 2, 0)]: (0.0, 0.0, 0.0));
    assert_eq!(a, expected);
}

#[test]
fn unit_extent_zoom_in_test_3() {
    use crate::unit_extent;

    let mut a = unit_extent!([(0, 0, 0)]: (149.99, 149.99, 0.0));
    a.zoom_in();
    let expected = unit_extent!([(0, 0, 0), (1, 1, 0)]: (499.90002, 499.90002, 0.0));
    assert_eq!(a, expected);
}

#[test]
fn unit_extent_zoom_in_multi_test_1() {
    use crate::unit_extent;

    let mut a = unit_extent!([(1, 1, 0)]: (437.0, 437.0, 0.0));
    let scale_b = a.grid_offset.scale.zoomed_in().zoomed_in();
    a.zoom_in_multi(scale_b).unwrap();
    let expected = unit_extent!([(1, 1, 0), (4, 4, 0), (4, 4, 0)]: (-300.0, -300.0, 0.0));
    assert_eq!(a, expected);
}

#[test]
fn unit_extent_zoom_in_multi_test_2() {
    use crate::unit_extent;

    let mut a = unit_extent!([(0, 0, 0)]: (0.0, 0.0, 0.0));
    let scale_b = a.grid_offset.scale.zoomed_in().zoomed_in().zoomed_in();
    a.zoom_in_multi(scale_b).unwrap();
    let expected = unit_extent!([(0, 0, 0), (0, 0, 0), (0, 0, 0), (0, 0, 0)]: (0.0, 0.0, 0.0));
    assert_eq!(a, expected);
}

#[test]
fn unit_extent_zoom_in_multi_test_3() {
    use crate::unit_extent;

    let mut a = unit_extent!([(1, 1, 0), (1, 1, 0)]: (499.99, 499.99, 0.0));
    let scale_b = a.grid_offset.scale.zoomed_in().zoomed_in().zoomed_in();
    a.zoom_in_multi(scale_b).unwrap();
    let expected = unit_extent!([(1, 1, 0), (2, 2, 0), (-5, -5, 0), (0, 0, 0), (0, 0, 0)]: (-10.009766, -10.009766, 0.0));
    assert_eq!(a, expected);
}

#[test]
fn unit_extent_add_test_1() {
    use crate::unit_extent;

    let a = unit_extent!([(0, 0, 0)]: (0.0, 0.0, 0.0));
    let b = unit_extent!([(0, 0, 0), (0, 0, 0)]: (200.0, 200.0, 0.0));
    let c = a + b;
    let expected = unit_extent!([(0, 0, 0), (0, 0, 0)]: (200.0, 200.0, 0.0));
    assert_eq!(c, expected);
}

#[test]
fn unit_extent_add_test_2() {
    use crate::unit_extent;

    let a = unit_extent!([(0, 0, 0)]: (400.0, 400.0, 0.0));
    let b = unit_extent!([(0, 0, 0)]: (200.0, 200.0, 0.0));
    let c = a + b;
    let expected = unit_extent!([(1, 1, 0)]: (-400.0, -400.0, 0.0));
    assert_eq!(c, expected);
}

#[test]
fn unit_extent_add_test_3() {
    use crate::unit_extent;

    let a = unit_extent!([(1, 1, 0)]: (437.0, 437.0, 0.0));
    let b = unit_extent!([(1, 1, 0), (1, 1, 0), (1, 1, 0)]: (200.0, 200.0, 0.0));
    let c = a + b;
    let expected = unit_extent!([(3, 3, 0), (-4, -4, 0), (-5, -5, 0)]: (-100.0, -100.0, 0.0));
    assert_eq!(c, expected);
}

#[test]
fn unit_extent_add_test_4() {
    use crate::unit_extent;

    let a = unit_extent!([(1, 1, 0), (1, 1, 0), (1, 1, 0)]: (499.99, 499.99, 0.0));
    let b = unit_extent!([(1, 1, 0), (1, 1, 0), (1, 1, 0)]: (0.02, 0.02, 0.0));
    let c = a + b;
    let expected = unit_extent!([(2, 2, 0), (2, 2, 0), (3, 3, 0)]: (-499.99, -499.99, 0.0));
    assert_eq!(c, expected);
}

#[test]
fn unit_extent_sub_test_1() {
    use crate::unit_extent;

    let a = unit_extent!([(1, 1, 0)]: (0.0, 0.0, 0.0));
    let b = unit_extent!([(0, 0, 0), (0, 0, 0)]: (200.0, 200.0, 0.0));
    let c = a - b;
    let expected = unit_extent!([(1, 1, 0), (0, 0, 0)]: (-200.0, -200.0, 0.0));
    assert_eq!(c, expected);
}

#[test]
fn unit_extent_sub_test_2() {
    use crate::unit_extent;

    let a = unit_extent!([(0, 0, 0)]: (-200.0, -200.0, 0.0));
    let b = unit_extent!([(0, 0, 0)]: (400.0, 400.0, 0.0));
    let c = a - b;
    let expected = unit_extent!([(-1, -1, 0)]: (400.0, 400.0, 0.0));
    assert_eq!(c, expected);
}

#[test]
fn unit_extent_sub_test_3() {
    use crate::unit_extent;

    let a = unit_extent!([(1, 1, 0)]: (100.0, 100.0, 0.0));
    let b = unit_extent!([(1, 1, 0), (0, 0, 0), (0, 0, 0)]: (200.0, 200.0, 0.0));
    let c = a - b;
    let expected = unit_extent!([(0, 0, 0), (1, 1, 0), (0, 0, 0)]: (-200.0, -200.0, 0.0));
    assert_eq!(c, expected);
}

#[test]
fn unit_extent_sub_test_4() {
    use crate::unit_extent;

    let a = unit_extent!([(1, 1, 0)]: (437.0, 437.0, 0.0));
    let b = unit_extent!([(1, 1, 0), (1, 1, 0), (1, 1, 0)]: (200.0, 200.0, 0.0));
    let c = a - b;
    let expected = unit_extent!([(0, 0, 0), (3, 3, 0), (3, 3, 0)]: (-500.0, -500.0, 0.0));
    assert_eq!(c, expected);
}

#[test]
fn unit_extent_add_test_z_axis_carry() {
    use crate::unit_extent;

    let a = unit_extent!([(0, 0, 0)]: (0.0, 0.0, 490.0));
    let b = unit_extent!([(0, 0, 0)]: (0.0, 0.0, 20.0));
    let c = a + b;
    let expected = unit_extent!([(0, 0, 1)]: (0.0, 0.0, -490.0));
    assert_eq!(c, expected);
}

#[test]
fn unit_add_vec3_crossing_500_carries_into_grid() {
    use crate::bevy::math::Vec3;
    use crate::unit_extent;

    let a = unit_extent!([(0, 0, 0)]: (490.0, 0.0, 0.0));
    let c = a + Vec3::new(20.0, 0.0, 0.0);
    let expected = unit_extent!([(1, 0, 0)]: (-490.0, 0.0, 0.0));
    assert_eq!(c, expected);
}

#[test]
fn unit_add_vec3_crossing_negative_500_carries_into_grid() {
    use crate::bevy::math::Vec3;
    use crate::unit_extent;

    let a = unit_extent!([(0, 0, 0)]: (-490.0, 0.0, 0.0));
    let c = a + Vec3::new(-20.0, 0.0, 0.0);
    let expected = unit_extent!([(-1, 0, 0)]: (490.0, 0.0, 0.0));
    assert_eq!(c, expected);
}

#[test]
fn unit_add_vec3_crossing_1500_carries_multiple_steps() {
    use crate::bevy::math::Vec3;
    use crate::unit_extent;

    let a = unit_extent!([(0, 0, 0)]: (100.0, 0.0, 0.0));
    let c = a + Vec3::new(1500.0, 0.0, 0.0);
    let expected = unit_extent!([(2, 0, 0)]: (-400.0, 0.0, 0.0));
    assert_eq!(c, expected);
}

#[test]
fn unit_sub_vec3_crossing_1500_carries_multiple_steps() {
    use crate::bevy::math::Vec3;
    use crate::unit_extent;

    let a = unit_extent!([(0, 0, 0)]: (-100.0, 0.0, 0.0));
    let c = a - Vec3::new(1500.0, 0.0, 0.0);
    let expected = unit_extent!([(-2, 0, 0)]: (400.0, 0.0, 0.0));
    assert_eq!(c, expected);
}
