use bevy::prelude::{IVec2, Vec2};

use crate::usf::scale::Scale;
use crate::usf::pos::grid::types::GridPos;

use super::types::UnitPos;

// TODO: Impl properly
#[test]
fn unit_pos_zoom_out_test_1() {
    let a = GridPos::new_root(IVec2::new(0, 0));
    let a = GridPos::new(a, IVec2::new(0, 0));
    let a = GridPos::new(a, IVec2::new(0, 0));
    let a = UnitPos::new(a, Vec2::new(200.0, 200.0));
    let mut a = a;
    a.zoom_out();
    let expected_grid = GridPos::new_root(IVec2::new(0, 0));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(0, 0));
    let expected = UnitPos::new(expected_grid, Vec2::new(20.0, 20.0));
    assert_eq!(a, expected);
}

// TODO: Impl properly
#[test]
fn unit_pos_zoom_out_test_2() {
}

// TODO: Impl properly
#[test]
fn unit_pos_zoom_out_test_3() {
}

// TODO: Impl properly
#[test]
fn unit_pos_zoom_in_test_1() {
}

// TODO: Impl properly
#[test]
fn unit_pos_zoom_in_test_2() {
}

// TODO: Impl properly
#[test]
fn unit_pos_zoom_in_test_3() {
}

// TODO: Impl properly
#[test]
fn unit_pos_zoom_in_multi_test_1() {
}

// TODO: Impl properly
#[test]
fn unit_pos_zoom_in_multi_test_2() {
}

// TODO: Impl properly
#[test]
fn unit_pos_zoom_in_multi_test_3() {
}

#[test]
fn unit_pos_add_test_1() {
    let a_grid = GridPos::new_root(IVec2::new(0, 0));
    let a = UnitPos::new(a_grid, Vec2::new(0.0, 0.0));
    let b_grid = GridPos::new_root(IVec2::new(0, 0));
    let b_grid = GridPos::new(b_grid, IVec2::new(0, 0));
    let b = UnitPos::new(b_grid, Vec2::new(200.0, 200.0));
    let c = a + b;
    let expected_grid = GridPos::new_root(IVec2::new(0, 0));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(0, 0));
    let expected = UnitPos::new(expected_grid, Vec2::new(200.0, 200.0));
    assert_eq!(c, expected);
}

#[test]
fn unit_pos_add_test_2() {
    let a_grid = GridPos::new_root(IVec2::new(0, 0));
    let a = UnitPos::new(a_grid, Vec2::new(400.0, 400.0));
    let b_grid = GridPos::new_root(IVec2::new(0, 0));
    let b = UnitPos::new(b_grid, Vec2::new(200.0, 200.0));
    let c = a + b;
    let expected_grid = GridPos::new_root(IVec2::new(1, 1));
    let expected = UnitPos::new(expected_grid, Vec2::new(-400.0, -400.0));
    assert_eq!(c, expected);
}

#[test]
fn unit_pos_add_test_3() {
    let a_grid = GridPos::new_root(IVec2::new(1, 1));
    let a = UnitPos::new(a_grid, Vec2::new(437.0, 437.0));
    let b_grid = GridPos::new_root(IVec2::new(1, 1));
    let b_grid = GridPos::new(b_grid, IVec2::new(1, 1));
    let b_grid = GridPos::new(b_grid, IVec2::new(1, 1));
    let b = UnitPos::new(b_grid, Vec2::new(200.0, 200.0));
    let c = a + b;
    let expected_grid = GridPos::new_root(IVec2::new(3, 3));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(-4, -4));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(-5, -5));
    let expected = UnitPos::new(expected_grid, Vec2::new(-100.0, -100.0));
    assert_eq!(c, expected);
}

#[test]
fn unit_pos_add_test_4() {
    // Three-level deep grid
    let grid = GridPos::new_root(IVec2::new(1, 1));
    let grid = GridPos::new(grid, IVec2::new(1, 1));
    let grid = GridPos::new(grid, IVec2::new(1, 1));

    // Two UnitPos, their Vec3 adds up to cause a wrap on one level
    let a = UnitPos::new(grid.clone(), Vec2::new(499.99, 499.99));
    let b = UnitPos::new(grid.clone(), Vec2::new(0.02, 0.02));

    let c = a + b;

    // Carry of (1,1) applied to the lowest grid level
    let expected_grid = GridPos::new_root(IVec2::new(2, 2));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(2, 2));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(3, 3));

    let expected = UnitPos::new(expected_grid, Vec2::new(-499.99, -499.99));

    assert_eq!(c, expected);
}

#[test]
fn unit_pos_sub_test_1() {
    let a_grid = GridPos::new_root(IVec2::new(1, 1));
    let a = UnitPos::new(a_grid, Vec2::new(0.0, 0.0));
    let b_grid = GridPos::new_root(IVec2::new(0, 0));
    let b_grid = GridPos::new(b_grid, IVec2::new(0, 0));
    let b = UnitPos::new(b_grid, Vec2::new(200.0, 200.0));
    let c = a - b;
    let expected_grid = GridPos::new_root(IVec2::new(1, 1));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(0, 0));
    let expected = UnitPos::new(expected_grid, Vec2::new(-200.0, -200.0));
    assert_eq!(c, expected);
}

#[test]
fn unit_pos_sub_test_2() {
    let a_grid = GridPos::new_root(IVec2::new(0, 0));
    let a = UnitPos::new(a_grid, Vec2::new(-200.0, -200.0));
    let b_grid = GridPos::new_root(IVec2::new(0, 0));
    let b = UnitPos::new(b_grid, Vec2::new(400.0, 400.0));
    let c = a - b;
    let expected_grid = GridPos::new_root(IVec2::new(-1, -1));
    let expected = UnitPos::new(expected_grid, Vec2::new(400.0, 400.0));
    assert_eq!(c, expected);
}

#[test]
fn unit_pos_sub_test_3() {
    let a_grid = GridPos::new_root(IVec2::new(1, 1));
    let a = UnitPos::new(a_grid, Vec2::new(100.0, 100.0));
    let b_grid = GridPos::new_root(IVec2::new(1, 1));
    let b_grid = GridPos::new(b_grid, IVec2::new(0, 0));
    let b_grid = GridPos::new(b_grid, IVec2::new(0, 0));
    let b = UnitPos::new(b_grid, Vec2::new(200.0, 200.0));
    let c = a - b;
    let expected_grid = GridPos::new_root(IVec2::new(0, 0));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(1, 1));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(0, 0));
    let expected = UnitPos::new(expected_grid, Vec2::new(-200.0, -200.0));
    assert_eq!(c, expected);
}

// TODO: Impl properly
#[test]
fn unit_pos_sub_test_4() {
    let a_grid = GridPos::new_root(IVec2::new(1, 1));
    let a = UnitPos::new(a_grid, Vec2::new(437.0, 437.0));
    let b_grid = GridPos::new_root(IVec2::new(1, 1));
    let b_grid = GridPos::new(b_grid, IVec2::new(1, 1));
    let b_grid = GridPos::new(b_grid, IVec2::new(1, 1));
    let b = UnitPos::new(b_grid, Vec2::new(200.0, 200.0));
    let c = a - b;
    let expected_grid = GridPos::new_root(IVec2::new(-4, -4));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(-1, -1));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(-1, -1));
    let expected = UnitPos::new(expected_grid, Vec2::new(-500.0, -500.0));
    assert_eq!(c, expected);
}