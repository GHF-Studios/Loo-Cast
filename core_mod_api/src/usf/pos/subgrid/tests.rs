#[test]
fn subgrid_extent_zoom_out_test_1() {
    use crate::subgrid_extent;

    let mut a = subgrid_extent!([(0, 0), (4, 4), (3, 3)]: (2, 2));
    a.zoom_out();
    let expected = subgrid_extent!([(0, 0), (4, 4)]: (3, 3));
    assert_eq!(a, expected);
}

#[test]
fn subgrid_extent_zoom_out_test_2() {
    use crate::subgrid_extent;

    let mut a = subgrid_extent!([(0, 0), (4, 4), (3, 3), (2, 2)]: (1, 1));
    a.zoom_out();
    let expected = subgrid_extent!([(0, 0), (4, 4), (3, 3)]: (2, 2));
    assert_eq!(a, expected);
}

#[test]
fn subgrid_extent_zoom_out_test_3() {
    use crate::subgrid_extent;

    let mut a = subgrid_extent!([(0, 0), (4, 4), (3, 3), (2, 2), (1, 1)]: (0, 0));
    a.zoom_out();
    let expected = subgrid_extent!([(0, 0), (4, 4), (3, 3), (2, 2)]: (1, 1));
    assert_eq!(a, expected);
}

#[test]
fn subgrid_extent_add_test_1() {
    use crate::subgrid_extent;

    let a = subgrid_extent!([(0, 0)]: (4, 4));
    let b = subgrid_extent!([(0, 0)]: (3, 3));
    let c = a + b;
    let expected = subgrid_extent!([(1, 1)]: (-3, -3));
    assert_eq!(c, expected);
}

#[test]
fn subgrid_extent_add_test_2() {
    use crate::subgrid_extent;

    let a = subgrid_extent!([(0, 0), (4, 4), (3, 3)]: (2, 2));
    let b = subgrid_extent!([(0, 0)]: (1, 1));
    let c = a + b;
    let expected = subgrid_extent!([(1, 1), (-5, -5), (3, 3)]: (2, 2));
    assert_eq!(c, expected);
}

#[test]
fn subgrid_extent_add_test_3() {
    use super::types::SubgridVec;

    let a = SubgridVec::build().repeat((4, 4), 70).finish((4, 4));
    let b = SubgridVec::build().repeat((0, 0), 70).finish((1, 1));
    let c = a + b;
    let expected = SubgridVec::build().repeat((-5, -5), 70).finish((-5, -5));
    assert_eq!(c, expected);
}

#[test]
fn subgrid_extent_sub_test_1() {
    use crate::subgrid_extent;

    let a = subgrid_extent!([(0, 0)]: (-3, -3));
    let b = subgrid_extent!([(0, 0)]: (4, 4));
    let c = a - b;
    let expected = subgrid_extent!([(-1, -1)]: (3, 3));
    assert_eq!(c, expected);
}

#[test]
fn subgrid_extent_sub_test_2() {
    use crate::subgrid_extent;

    let a = subgrid_extent!([(0, 0), (4, 4), (3, 3)]: (2, 2));
    let b = subgrid_extent!([(0, 0)]: (1, 1));
    let c = a - b;
    let expected = subgrid_extent!([(0, 0), (3, 3), (3, 3)]: (2, 2));
    assert_eq!(c, expected);
}

#[test]
fn subgrid_extent_sub_test_3() {
    use super::types::SubgridVec;

    let a = SubgridVec::build().repeat((-5, -5), 70).finish((-5, -5));
    let b = SubgridVec::build().repeat((0, 0), 70).finish((1, 1));
    let c = a - b;
    let expected = SubgridVec::build().repeat((4, 4), 70).finish((4, 4));
    assert_eq!(c, expected);
}
