use bevy::prelude::IVec2;

use crate::usf::scale::Scale;
use crate::usf::pos::grid::types::GridPos;

use super::types::SubgridPos;

#[test]
fn subgrid_pos_zoom_out_test_1() {
    let a = GridPos::new_root(IVec2::new(0, 0));
    let a = GridPos::new(a, IVec2::new(4, 4));
    let a = GridPos::new(a, IVec2::new(3, 3));
    let a = SubgridPos::new(a, IVec2::new(2, 2));
    let mut a = a;
    a.zoom_out();
    let expected_grid = GridPos::new_root(IVec2::new(0, 0));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(4, 4));
    let expected = SubgridPos::new(expected_grid, IVec2::new(3, 3));
    assert_eq!(a, expected);
}

#[test]
fn subgrid_pos_zoom_out_test_2() {
    let a = GridPos::new_root(IVec2::new(0, 0));
    let a = GridPos::new(a, IVec2::new(4, 4));
    let a = GridPos::new(a, IVec2::new(3, 3));
    let a = GridPos::new(a, IVec2::new(2, 2));
    let a = SubgridPos::new(a, IVec2::new(1, 1));
    let mut a = a;
    a.zoom_out();
    let expected_grid = GridPos::new_root(IVec2::new(0, 0));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(4, 4));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(3, 3));
    let expected = SubgridPos::new(expected_grid, IVec2::new(2, 2));
    assert_eq!(a, expected);
}

#[test]
fn subgrid_pos_zoom_out_test_3() {
    let a = GridPos::new_root(IVec2::new(0, 0));
    let a = GridPos::new(a, IVec2::new(4, 4));
    let a = GridPos::new(a, IVec2::new(3, 3));
    let a = GridPos::new(a, IVec2::new(2, 2));
    let a = GridPos::new(a, IVec2::new(1, 1));
    let a = SubgridPos::new(a, IVec2::new(0, 0));
    let mut a = a;
    a.zoom_out();
    let expected_grid = GridPos::new_root(IVec2::new(0, 0));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(4, 4));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(3, 3));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(2, 2));
    let expected = SubgridPos::new(expected_grid, IVec2::new(1, 1));
    assert_eq!(a, expected);
}

#[test]
fn subgrid_pos_add_test_1() {
    let a = GridPos::new_root(IVec2::new(0, 0));
    let a = SubgridPos::new(a, IVec2::new(4, 4));
    let b = GridPos::new_root(IVec2::new(0, 0));
    let b = SubgridPos::new(b, IVec2::new(3, 3));
    let c = a + b;
    let expected_grid = GridPos::new_root(IVec2::new(1, 1));
    let expected = SubgridPos::new(expected_grid, IVec2::new(-3, -3));
    assert_eq!(c, expected);
}

#[test]
fn subgrid_pos_add_test_2() {
    let a = GridPos::new_root(IVec2::new(0, 0));
    let a = GridPos::new(a, IVec2::new(4, 4));
    let a = GridPos::new(a, IVec2::new(3, 3));
    let a = SubgridPos::new(a, IVec2::new(2, 2));
    let b = GridPos::new_root(IVec2::new(0, 0));
    let b = SubgridPos::new(b, IVec2::new(1, 1));
    let c = a + b;
    let expected_grid = GridPos::new_root(IVec2::new(1, 1));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(-5, -5));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(3, 3));
    let expected = SubgridPos::new(expected_grid, IVec2::new(2, 2));
    assert_eq!(c, expected);
}

#[test]
fn subgrid_pos_add_test_3() {
    let a = GridPos::new_root(IVec2::new(0, 0));
    let a = SubgridPos::new(a, IVec2::new(1, 1));
    let b = GridPos::new_root(IVec2::new(0, 0));
    let b = GridPos::new(b, IVec2::new(4, 4));
    let b = GridPos::new(b, IVec2::new(3, 3));
    let b = SubgridPos::new(b, IVec2::new(2, 2));
    let c = a + b;
    let expected_grid = GridPos::new_root(IVec2::new(1, 1));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(-5, -5));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(3, 3));
    let expected = SubgridPos::new(expected_grid, IVec2::new(2, 2));
    assert_eq!(c, expected);
}

#[test]
fn subgrid_pos_sub_test_1() {
    let a = GridPos::new_root(IVec2::new(0, 0));
    let a = SubgridPos::new(a, IVec2::new(-3, -3));
    let b = GridPos::new_root(IVec2::new(0, 0));
    let b = SubgridPos::new(b, IVec2::new(4, 4));
    let c = a - b;
    let expected_grid = GridPos::new_root(IVec2::new(-1, -1));
    let expected = SubgridPos::new(expected_grid, IVec2::new(3, 3));
    assert_eq!(c, expected);
}

#[test]
fn subgrid_pos_sub_test_2() {
    let a = GridPos::new_root(IVec2::new(0, 0));
    let a = GridPos::new(a, IVec2::new(4, 4));
    let a = GridPos::new(a, IVec2::new(3, 3));
    let a = SubgridPos::new(a, IVec2::new(2, 2));
    let b = GridPos::new_root(IVec2::new(0, 0));
    let b = SubgridPos::new(b, IVec2::new(1, 1));
    let c = a - b;
    let expected_grid = GridPos::new_root(IVec2::new(0, 0));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(3, 3));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(3, 3));
    let expected = SubgridPos::new(expected_grid, IVec2::new(2, 2));
    assert_eq!(c, expected);
}

#[test]
fn subgrid_pos_sub_test_3() {
    let a = GridPos::new_root(IVec2::new(0, 0));
    let a = SubgridPos::new(a, IVec2::new(1, 1));
    let b = GridPos::new_root(IVec2::new(0, 0));
    let b = GridPos::new(b, IVec2::new(4, 4));
    let b = GridPos::new(b, IVec2::new(3, 3));
    let b = SubgridPos::new(b, IVec2::new(2, 2));
    let c = a - b;
    let expected_grid = GridPos::new_root(IVec2::new(0, 0));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(-3, -3));
    let expected_grid = GridPos::new(expected_grid, IVec2::new(-3, -3));
    let expected = SubgridPos::new(expected_grid, IVec2::new(-2, -2));
    assert_eq!(c, expected);
}
