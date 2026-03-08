#[test]
fn grid_extent_zoom_out_test_1() {
    use crate::grid_extent;

    let mut a = grid_extent!([(0, 0), (4, 4)]);
    a.zoom_out();
    let expected = grid_extent!([(0, 0)]);
    assert_eq!(a, expected);
}

#[test]
fn grid_extent_zoom_out_test_2() {
    use crate::grid_extent;

    let mut a = grid_extent!([(0, 0), (4, 4), (3, 3)]);
    a.zoom_out();
    let expected = grid_extent!([(0, 0), (4, 4)]);
    assert_eq!(a, expected);
}

#[test]
fn grid_extent_zoom_out_test_3() {
    use crate::grid_extent;

    let mut a = grid_extent!([(0, 0), (4, 4), (3, 3), (2, 2)]);
    a.zoom_out();
    let expected = grid_extent!([(0, 0), (4, 4), (3, 3)]);
    assert_eq!(a, expected);
}

#[test]
fn grid_extent_add_test_1() {
    use crate::grid_extent;

    let a = grid_extent!([(4, 4)]);
    let b = grid_extent!([(3, 3)]);
    let c = a + b;
    let expected = grid_extent!([(-3, -3)]);
    assert_eq!(c, expected);
}

#[test]
fn grid_extent_add_test_2() {
    use crate::grid_extent;

    let a = grid_extent!([(0, 0), (4, 4)]);
    let b = grid_extent!([(0, 0), (3, 3)]);
    let c = a + b;
    let expected = grid_extent!([(1, 1), (-3, -3)]);
    assert_eq!(c, expected);
}

#[test]
fn grid_extent_add_test_3() {
    use super::types::GridVec;

    let a = GridVec::build().repeat((4, 4), 71).finish();
    let b = GridVec::build().repeat((0, 0), 70).push((1, 1)).finish();
    let c = a + b;
    let expected = GridVec::build().repeat((-5, -5), 71).finish();
    assert_eq!(c, expected);
}

#[test]
fn grid_extent_sub_test_1() {
    use crate::grid_extent;

    let a = grid_extent!([(3, 3)]);
    let b = grid_extent!([(4, 4)]);
    let c = a - b;
    let expected = grid_extent!([(-1, -1)]);
    assert_eq!(c, expected);
}

#[test]
fn grid_extent_sub_test_2() {
    use crate::grid_extent;

    let a = grid_extent!([(0, 0), (-5, -5)]);
    let b = grid_extent!([(0, 0), (3, 3)]);
    let c = a - b;
    let expected = grid_extent!([(-1, -1), (2, 2)]);
    assert_eq!(c, expected);
}

#[test]
fn grid_extent_sub_test_3() {
    use super::types::GridVec;

    let a = GridVec::build().repeat((-5, -5), 71).finish();
    let b = GridVec::build().repeat((0, 0), 70).push((1, 1)).finish();
    let c = a - b;
    let expected = GridVec::build().repeat((4, 4), 71).finish();
    assert_eq!(c, expected);
}

#[test]
fn grid_to_native_visual_scales_offsets_for_coarser_levels() {
    use crate::bevy::math::Vec2;
    use crate::grid_extent;

    let chunk_size = 1_000.0;
    let origin = grid_extent!([(0, 0), (0, 0)]);
    let coarser_neighbor = grid_extent!([(1, 0)]);

    let (pos, scale) = coarser_neighbor.to_native_visual(origin);

    assert_eq!(scale, 10.0);
    assert_eq!(pos, Vec2::new(chunk_size * 9.5, chunk_size * -0.5));
}

#[test]
fn grid_to_native_visual_same_scale_neighbor_uses_base_chunk_spacing() {
    use crate::bevy::math::Vec2;
    use crate::grid_extent;

    let chunk_size = 1_000.0;
    let origin = grid_extent!([(0, 0), (0, 0)]);
    let same_scale_neighbor = grid_extent!([(0, 0), (1, 0)]);

    let (pos, scale) = same_scale_neighbor.to_native_visual(origin);

    assert_eq!(scale, 1.0);
    assert_eq!(pos, Vec2::new(chunk_size, 0.0));
}

#[test]
fn grid_to_native_logical_scales_offsets_for_coarser_levels() {
    use crate::bevy::math::Vec2;
    use crate::grid_extent;

    let chunk_size = 1_000.0;
    let origin = grid_extent!([(0, 0), (0, 0)]);
    let coarser_neighbor = grid_extent!([(1, 0)]);

    let pos = coarser_neighbor.to_native_logical(origin);

    assert_eq!(pos, Vec2::new(chunk_size * 9.5, chunk_size * -0.5));
}

#[test]
fn grid_to_native_logical_same_scale_neighbor_uses_base_chunk_spacing() {
    use crate::bevy::math::Vec2;
    use crate::grid_extent;

    let chunk_size = 1_000.0;
    let origin = grid_extent!([(0, 0), (0, 0)]);
    let same_scale_neighbor = grid_extent!([(0, 0), (1, 0)]);

    let pos = same_scale_neighbor.to_native_logical(origin);

    assert_eq!(pos, Vec2::new(chunk_size, 0.0));
}
