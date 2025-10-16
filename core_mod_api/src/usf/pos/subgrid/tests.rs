use bevy::prelude::IVec2;

use crate::usf::scale::Scale;
use crate::usf::pos::grid::types::GridPos;
use crate::subgrid_pos;

use super::types::SubgridPos;

#[test]
fn subgrid_pos_zoom_out_test_1() {
    let mut a = subgrid_pos!([(0, 0), (4, 4), (3, 3)]: (2, 2));
    a.zoom_out();
    let expected = subgrid_pos!([(0, 0), (4, 4)]: (3, 3));
    assert_eq!(a, expected);
}

#[test]
fn subgrid_pos_zoom_out_test_2() {
    let mut a = subgrid_pos!([(0, 0), (4, 4), (3, 3), (2, 2)]: (1, 1));
    a.zoom_out();
    let expected = subgrid_pos!([(0, 0), (4, 4), (3, 3)]: (2, 2));
    assert_eq!(a, expected);
}

#[test]
fn subgrid_pos_zoom_out_test_3() {
    let mut a = subgrid_pos!([(0, 0), (4, 4), (3, 3), (2, 2), (1, 1)]: (0, 0));
    a.zoom_out();
    let expected = subgrid_pos!([(0, 0), (4, 4), (3, 3), (2, 2)]: (1, 1));
    assert_eq!(a, expected);
}

#[test]
fn subgrid_pos_add_test_1() {
    let a = subgrid_pos!([(0, 0)]: (4, 4));
    let b = subgrid_pos!([(0, 0)]: (3, 3));
    let c = a + b;
    let expected = subgrid_pos!([(1, 1)]: (-3, -3));
    assert_eq!(c, expected);
}

#[test]
fn subgrid_pos_add_test_2() {
    let a = subgrid_pos!([(0, 0), (4, 4), (3, 3)]: (2, 2));
    let b = subgrid_pos!([(0, 0)]: (1, 1));
    let c = a + b;
    let expected = subgrid_pos!([(1, 1), (-5, -5), (3, 3)]: (2, 2));
    assert_eq!(c, expected);
}

#[test]
fn subgrid_pos_add_test_3() {
    let a = subgrid_pos!([(0, 0)]: (1, 1));
    let b = subgrid_pos!([(0, 0), (4, 4), (3, 3)]: (2, 2));
    let c = a + b;
    let expected = subgrid_pos!([(1, 1), (-5, -5), (3, 3)]: (2, 2));
    assert_eq!(c, expected);
}

#[test]
fn subgrid_pos_sub_test_1() {
    let a = subgrid_pos!([(0, 0)]: (-3, -3));
    let b = subgrid_pos!([(0, 0)]: (4, 4));
    let c = a - b;
    let expected = subgrid_pos!([(-1, -1)]: (3, 3));
    assert_eq!(c, expected);
}

#[test]
fn subgrid_pos_sub_test_2() {
    let a = subgrid_pos!([(0, 0), (4, 4), (3, 3)]: (2, 2));
    let b = subgrid_pos!([(0, 0)]: (1, 1));
    let c = a - b;
    let expected = subgrid_pos!([(0, 0), (3, 3), (3, 3)]: (2, 2));
    assert_eq!(c, expected);
}

#[test]
fn subgrid_pos_sub_test_3() {
    let a = subgrid_pos!([(0, 0)]: (1, 1));
    let b = subgrid_pos!([(0, 0), (4, 4), (3, 3)]: (2, 2));
    let c = a - b;
    let expected = subgrid_pos!([(0, 0), (-3, -3), (-3, -3)]: (-2, -2));
    assert_eq!(c, expected);
}
