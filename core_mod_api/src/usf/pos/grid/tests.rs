use bevy::prelude::IVec2;

use crate::usf::scale::Scale;
use crate::grid_pos; // ← macro import

use super::types::GridPos;

#[test]
fn grid_pos_zoom_out_test_1() {
    let mut a = grid_pos!([(0, 0), (4, 4)]);
    a.zoom_out();
    let expected = grid_pos!([(0, 0)]);
    assert_eq!(a, expected);
}

#[test]
fn grid_pos_zoom_out_test_2() {
    let mut a = grid_pos!([(0, 0), (4, 4), (3, 3)]);
    a.zoom_out();
    let expected = grid_pos!([(0, 0), (4, 4)]);
    assert_eq!(a, expected);
}

#[test]
fn grid_pos_zoom_out_test_3() {
    let mut a = grid_pos!([(0, 0), (4, 4), (3, 3), (2, 2)]);
    a.zoom_out();
    let expected = grid_pos!([(0, 0), (4, 4), (3, 3)]);
    assert_eq!(a, expected);
}

#[test]
fn grid_pos_add_test_1() {
    let a = grid_pos!([(4, 4)]);
    let b = grid_pos!([(3, 3)]);
    let c = a + b;
    let expected = grid_pos!([(-3, -3)]);
    assert_eq!(c, expected);
}

#[test]
fn grid_pos_add_test_2() {
    let scale = Scale::MAX.zoomed_in();
    let a = GridPos::new_at_scale(scale, IVec2::new(4, 4));
    let b = GridPos::new_at_scale(scale, IVec2::new(3, 3));
    let c = a + b;
    let expected = grid_pos!([(1, 1), (-3, -3)]);
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
    let a = grid_pos!([(3, 3)]);
    let b = grid_pos!([(4, 4)]);
    let c = a - b;
    let expected = grid_pos!([(-1, -1)]);
    assert_eq!(c, expected);
}

#[test]
fn grid_pos_sub_test_2() {
    let scale = Scale::MAX.zoomed_in();
    let a = GridPos::new_at_scale(scale, IVec2::new(-5, -5));
    let b = GridPos::new_at_scale(scale, IVec2::new(3, 3));
    let c = a - b;
    let expected = grid_pos!([(-1, -1), (2, 2)]);
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
