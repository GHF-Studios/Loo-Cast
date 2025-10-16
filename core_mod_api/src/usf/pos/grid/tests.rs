use bevy::prelude::IVec2;

use crate::usf::scale::Scale;
use super::types::GridPos;

#[test]
fn grid_pos_zoom_out_test_1() {
    let a = GridPos::new_root(IVec2::new(0, 0));
    let mut a = GridPos::new(a, IVec2::new(4, 4));
    a.zoom_out();
    let expected = GridPos::new_root(IVec2::new(0, 0));
    assert_eq!(a, expected);
}

#[test]
fn grid_pos_zoom_out_test_2() {
    let a = GridPos::new_root(IVec2::new(0, 0));
    let a = GridPos::new(a, IVec2::new(4, 4));
    let mut a = GridPos::new(a, IVec2::new(3, 3));
    a.zoom_out();
    let expected = GridPos::new_root(IVec2::new(0, 0));
    let expected = GridPos::new(expected, IVec2::new(4, 4));
    assert_eq!(a, expected);
}

#[test]
fn grid_pos_zoom_out_test_3() {
    let a = GridPos::new_root(IVec2::new(0, 0));
    let a = GridPos::new(a, IVec2::new(4, 4));
    let a = GridPos::new(a, IVec2::new(3, 3));
    let mut a = GridPos::new(a, IVec2::new(2, 2));
    a.zoom_out();
    let expected = GridPos::new_root(IVec2::new(0, 0));
    let expected = GridPos::new(expected, IVec2::new(4, 4));
    let expected = GridPos::new(expected, IVec2::new(3, 3));
    assert_eq!(a, expected);
}

#[test]
fn grid_pos_add_test_1() {
    let a = GridPos::new_root(IVec2::new(4, 4));
    let b = GridPos::new_root(IVec2::new(3, 3));
    let c = a + b;
    assert_eq!(c, GridPos::new_root(IVec2::new(-3, -3)));
}

#[test]
fn grid_pos_add_test_2() {
    let scale = Scale::MAX.zoomed_in();
    let a = GridPos::new_at_scale(scale, IVec2::new(4, 4));
    let b = GridPos::new_at_scale(scale, IVec2::new(3, 3));
    let c = a + b;
    let expected = GridPos::new_root(IVec2::new(1, 1));
    let expected = GridPos::new(expected, IVec2::new(-3, -3));
    assert_eq!(c, expected);
}

#[test]
fn grid_pos_add_test_3() {
    let a = GridPos::new_splat(Scale::MIN, IVec2::new(4, 4));
    let b = GridPos::new_at_scale(Scale::MIN, IVec2::new(1, 1));
    let c = a + b;
    let expected = GridPos::new_splat(Scale::MIN, IVec2::new(-5, -5));
    assert_eq!(c, expected);
}

#[test]
fn grid_pos_sub_test_1() {
    let a = GridPos::new_root(IVec2::new(3, 3));
    let b = GridPos::new_root(IVec2::new(4, 4));
    let c = a - b;
    assert_eq!(c, GridPos::new_root(IVec2::new(-1, -1)));
}

#[test]
fn grid_pos_sub_test_2() {
    let scale = Scale::MAX.zoomed_in();
    let a = GridPos::new_at_scale(scale, IVec2::new(-5, -5));
    let b = GridPos::new_at_scale(scale, IVec2::new(3, 3));
    let c = a - b;
    let expected = GridPos::new_root(IVec2::new(-1, -1));
    let expected = GridPos::new(expected, IVec2::new(2, 2));
    assert_eq!(c, expected);
}

#[test]
fn grid_pos_sub_test_3() {
    let a = GridPos::new_splat(Scale::MIN, IVec2::new(-5, -5));
    let b = GridPos::new_at_scale(Scale::MIN, IVec2::new(1, 1));
    let c = a - b;
    let expected = GridPos::new_splat(Scale::MIN, IVec2::new(4, 4));
    assert_eq!(c, expected);
}
