#[test]
fn subgrid_extent_zoom_out_test_1() {
    use crate::subgrid_extent;

    let mut a = subgrid_extent!([(0, 0, 0), (4, 4, 0), (3, 3, 0)]: (2, 2, 0));
    a.zoom_out();
    let expected = subgrid_extent!([(0, 0, 0), (4, 4, 0)]: (3, 3, 0));
    assert_eq!(a, expected);
}

#[test]
fn subgrid_extent_zoom_out_test_2() {
    use crate::subgrid_extent;

    let mut a = subgrid_extent!([(0, 0, 0), (4, 4, 0), (3, 3, 0), (2, 2, 0)]: (1, 1, 0));
    a.zoom_out();
    let expected = subgrid_extent!([(0, 0, 0), (4, 4, 0), (3, 3, 0)]: (2, 2, 0));
    assert_eq!(a, expected);
}

#[test]
fn subgrid_extent_zoom_out_test_3() {
    use crate::subgrid_extent;

    let mut a = subgrid_extent!([(0, 0, 0), (4, 4, 0), (3, 3, 0), (2, 2, 0), (1, 1, 0)]: (0, 0, 0));
    a.zoom_out();
    let expected = subgrid_extent!([(0, 0, 0), (4, 4, 0), (3, 3, 0), (2, 2, 0)]: (1, 1, 0));
    assert_eq!(a, expected);
}

#[test]
fn subgrid_extent_add_test_1() {
    use crate::subgrid_extent;

    let a = subgrid_extent!([(0, 0, 0)]: (4, 4, 0));
    let b = subgrid_extent!([(0, 0, 0)]: (3, 3, 0));
    let c = a + b;
    let expected = subgrid_extent!([(1, 1, 0)]: (-3, -3, 0));
    assert_eq!(c, expected);
}

#[test]
fn subgrid_extent_add_test_2() {
    use crate::subgrid_extent;

    let a = subgrid_extent!([(0, 0, 0), (4, 4, 0), (3, 3, 0)]: (2, 2, 0));
    let b = subgrid_extent!([(0, 0, 0)]: (1, 1, 0));
    let c = a + b;
    let expected = subgrid_extent!([(1, 1, 0), (-5, -5, 0), (3, 3, 0)]: (2, 2, 0));
    assert_eq!(c, expected);
}

#[test]
fn subgrid_extent_add_test_3() {
    use super::types::SubgridVec;

    let a = SubgridVec::build().repeat((4, 4, 0), 70).finish((4, 4, 0));
    let b = SubgridVec::build().repeat((0, 0, 0), 70).finish((1, 1, 0));
    let c = a + b;
    let expected = SubgridVec::build().repeat((-5, -5, 0), 70).finish((-5, -5, 0));
    assert_eq!(c, expected);
}

#[test]
fn subgrid_extent_sub_test_1() {
    use crate::subgrid_extent;

    let a = subgrid_extent!([(0, 0, 0)]: (-3, -3, 0));
    let b = subgrid_extent!([(0, 0, 0)]: (4, 4, 0));
    let c = a - b;
    let expected = subgrid_extent!([(-1, -1, 0)]: (3, 3, 0));
    assert_eq!(c, expected);
}

#[test]
fn subgrid_extent_sub_test_2() {
    use crate::subgrid_extent;

    let a = subgrid_extent!([(0, 0, 0), (4, 4, 0), (3, 3, 0)]: (2, 2, 0));
    let b = subgrid_extent!([(0, 0, 0)]: (1, 1, 0));
    let c = a - b;
    let expected = subgrid_extent!([(0, 0, 0), (3, 3, 0), (3, 3, 0)]: (2, 2, 0));
    assert_eq!(c, expected);
}

#[test]
fn subgrid_extent_sub_test_3() {
    use super::types::SubgridVec;

    let a = SubgridVec::build().repeat((-5, -5, 0), 70).finish((-5, -5, 0));
    let b = SubgridVec::build().repeat((0, 0, 0), 70).finish((1, 1, 0));
    let c = a - b;
    let expected = SubgridVec::build().repeat((4, 4, 0), 70).finish((4, 4, 0));
    assert_eq!(c, expected);
}

#[test]
fn subgrid_extent_add_test_z_axis() {
    use crate::subgrid_extent;

    let a = subgrid_extent!([(0, 0, 0)]: (0, 0, 4));
    let b = subgrid_extent!([(0, 0, 0)]: (0, 0, 3));
    let c = a + b;
    let expected = subgrid_extent!([(0, 0, 1)]: (0, 0, -3));
    assert_eq!(c, expected);
}
