use crate::unit_extent;

use super::types::UnitVec;

#[test]
fn unit_extent_zoom_out_test_1() {
    let mut a = unit_extent!([(0, 0), (0, 0), (0, 0)]: (200.0, 200.0));
    a.zoom_out();
    let expected = unit_extent!([(0, 0), (0, 0)]: (20.0, 20.0));
    assert_eq!(a, expected);
}

#[test]
fn unit_extent_zoom_out_test_2() {
    let mut a = unit_extent!([(0, 0), (4, 4), (3, 3), (2, 2), (1, 1)]: (0.0, 0.0));
    a.zoom_out();
    let expected = unit_extent!([(0, 0), (4, 4), (3, 3), (2, 2)]: (100.0, 100.0));
    assert_eq!(a, expected);
}

#[test]
fn unit_extent_zoom_out_test_3() {
    let mut a = unit_extent!([(0, 0), (4, 4), (3, 3), (2, 2), (1, 1)]: (499.9, 499.9));
    a.zoom_out();
    let expected = unit_extent!([(0, 0), (4, 4), (3, 3), (2, 2)]: (149.98999, 149.98999)); // Not 149.99 due to floating point precision
    assert_eq!(a, expected);
}

#[test]
fn unit_extent_zoom_in_test_1() {
    let mut a = unit_extent!([(0, 0), (0, 0)]: (20.0, 20.0));
    a.zoom_in();
    let expected = unit_extent!([(0, 0), (0, 0), (0, 0)]: (200.0, 200.0));
    assert_eq!(a, expected);
}

#[test]
fn unit_extent_zoom_in_test_2() {
    let mut a = unit_extent!([(0, 0)]: (200.0, 200.0));
    a.zoom_in();
    let expected = unit_extent!([(0, 0), (2, 2)]: (0.0, 0.0));
    assert_eq!(a, expected);
}

#[test]
fn unit_extent_zoom_in_test_3() {
    let mut a = unit_extent!([(0, 0)]: (149.99, 149.99));
    a.zoom_in();
    let expected = unit_extent!([(0, 0), (1, 1)]: (499.90002, 499.90002)); // Not 499.9 due to floating point precision limitations
    assert_eq!(a, expected);
}

#[test]
fn unit_extent_zoom_in_multi_test_1() {
    let mut a = unit_extent!([(1, 1)]: (437.0, 437.0));
    let scale_b = a.grid_offset.scale.zoomed_in().zoomed_in();
    a.zoom_in_multi(scale_b);
    let expected = unit_extent!([(1, 1), (4, 4), (4, 4)]: (-300.0, -300.0));
    assert_eq!(a, expected);
}

#[test]
fn unit_extent_zoom_in_multi_test_2() {
    let mut a = unit_extent!([(0, 0)]: (0.0, 0.0));
    let scale_b = a.grid_offset.scale.zoomed_in().zoomed_in().zoomed_in();
    a.zoom_in_multi(scale_b);
    let expected = unit_extent!([(0, 0), (0, 0), (0, 0), (0, 0)]: (0.0, 0.0));
    assert_eq!(a, expected);
}

#[test]
fn unit_extent_zoom_in_multi_test_3() {
    let mut a = unit_extent!([(1, 1), (1, 1)]: (499.99, 499.99));
    let scale_b = a.grid_offset.scale.zoomed_in().zoomed_in().zoomed_in();
    a.zoom_in_multi(scale_b);
    // This wraps because the float error in the intermediate steps happened to push the final value over the wrap threshold
    // after subtraction, scaling, and remapping — and did so deterministically, but in a way that's opaque unless you replay all ops step-by-step.
    let expected = unit_extent!([(1, 1), (2, 2), (-5, -5), (0, 0), (0, 0)]: (-10.009766, -10.009766));
    assert_eq!(a, expected);
}

#[test]
fn unit_extent_add_test_1() {
    let a = unit_extent!([(0, 0)]: (0.0, 0.0));
    let b = unit_extent!([(0, 0), (0, 0)]: (200.0, 200.0));
    let c = a + b;
    let expected = unit_extent!([(0, 0), (0, 0)]: (200.0, 200.0));
    assert_eq!(c, expected);
}

#[test]
fn unit_extent_add_test_2() {
    let a = unit_extent!([(0, 0)]: (400.0, 400.0));
    let b = unit_extent!([(0, 0)]: (200.0, 200.0));
    let c = a + b;
    let expected = unit_extent!([(1, 1)]: (-400.0, -400.0));
    assert_eq!(c, expected);
}

#[test]
fn unit_extent_add_test_3() {
    let a = unit_extent!([(1, 1)]: (437.0, 437.0));
    let b = unit_extent!([(1, 1), (1, 1), (1, 1)]: (200.0, 200.0));
    let c = a + b;
    let expected = unit_extent!([(3, 3), (-4, -4), (-5, -5)]: (-100.0, -100.0));
    assert_eq!(c, expected);
}

#[test]
fn unit_extent_add_test_4() {
    let a = unit_extent!([(1, 1), (1, 1), (1, 1)]: (499.99, 499.99));
    let b = unit_extent!([(1, 1), (1, 1), (1, 1)]: (0.02, 0.02));
    let c = a + b;
    let expected = unit_extent!([(2, 2), (2, 2), (3, 3)]: (-499.99, -499.99));
    assert_eq!(c, expected);
}

#[test]
fn unit_extent_sub_test_1() {
    let a = unit_extent!([(1, 1)]: (0.0, 0.0));
    let b = unit_extent!([(0, 0), (0, 0)]: (200.0, 200.0));
    let c = a - b;
    let expected = unit_extent!([(1, 1), (0, 0)]: (-200.0, -200.0));
    assert_eq!(c, expected);
}

#[test]
fn unit_extent_sub_test_2() {
    let a = unit_extent!([(0, 0)]: (-200.0, -200.0));
    let b = unit_extent!([(0, 0)]: (400.0, 400.0));
    let c = a - b;
    let expected = unit_extent!([(-1, -1)]: (400.0, 400.0));
    assert_eq!(c, expected);
}

#[test]
fn unit_extent_sub_test_3() {
    let a = unit_extent!([(1, 1)]: (100.0, 100.0));
    let b = unit_extent!([(1, 1), (0, 0), (0, 0)]: (200.0, 200.0));
    let c = a - b;
    let expected = unit_extent!([(0, 0), (1, 1), (0, 0)]: (-200.0, -200.0));
    assert_eq!(c, expected);
}

#[test]
fn unit_extent_sub_test_4() {
    let a = unit_extent!([(1, 1)]: (437.0, 437.0));
    let b = unit_extent!([(1, 1), (1, 1), (1, 1)]: (200.0, 200.0));
    let c = a - b;
    let expected = unit_extent!([(0, 0), (3, 3), (3, 3)]: (-500.0, -500.0));
    assert_eq!(c, expected);
}

#[test]
fn unit_complex_test_1() {
    let mut a = unit_extent!([(0, 0)]: (0.0, 0.0));
    let b = unit_extent!([(0, 0), (0, 0), (0, 0)]: (0.0, 0.0));
    a.zoom_in_multi(b.grid_offset.scale.zoomed_in().zoomed_in().zoomed_in());
    let expected = unit_extent!([(1, 1), (4, 4), (4, 4)]: (-300.0, -300.0));
    assert_eq!(a, expected);
}