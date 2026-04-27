#[test]
fn grid_extent_zoom_out_test_1() {
    use crate::grid_extent;

    let mut a = grid_extent!([(0, 0, 0), (4, 4, 0)]);
    a.zoom_out();
    let expected = grid_extent!([(0, 0, 0)]);
    assert_eq!(a, expected);
}

#[test]
fn grid_extent_zoom_out_test_2() {
    use crate::grid_extent;

    let mut a = grid_extent!([(0, 0, 0), (4, 4, 0), (3, 3, 0)]);
    a.zoom_out();
    let expected = grid_extent!([(0, 0, 0), (4, 4, 0)]);
    assert_eq!(a, expected);
}

#[test]
fn grid_extent_zoom_out_test_3() {
    use crate::grid_extent;

    let mut a = grid_extent!([(0, 0, 0), (4, 4, 0), (3, 3, 0), (2, 2, 0)]);
    a.zoom_out();
    let expected = grid_extent!([(0, 0, 0), (4, 4, 0), (3, 3, 0)]);
    assert_eq!(a, expected);
}

#[test]
fn grid_extent_add_test_1() {
    use crate::grid_extent;

    let a = grid_extent!([(4, 4, 0)]);
    let b = grid_extent!([(3, 3, 0)]);
    let c = a + b;
    let expected = grid_extent!([(-3, -3, 0)]);
    assert_eq!(c, expected);
}

#[test]
fn grid_extent_add_test_2() {
    use crate::grid_extent;

    let a = grid_extent!([(0, 0, 0), (4, 4, 0)]);
    let b = grid_extent!([(0, 0, 0), (3, 3, 0)]);
    let c = a + b;
    let expected = grid_extent!([(1, 1, 0), (-3, -3, 0)]);
    assert_eq!(c, expected);
}

#[test]
fn grid_extent_add_test_3() {
    use super::types::GridVec;

    let a = GridVec::build().repeat((4, 4, 0), 71).finish();
    let b = GridVec::build().repeat((0, 0, 0), 70).push((1, 1, 0)).finish();
    let c = a + b;
    let expected = GridVec::build().repeat((-5, -5, 0), 71).finish();
    assert_eq!(c, expected);
}

#[test]
fn grid_extent_sub_test_1() {
    use crate::grid_extent;

    let a = grid_extent!([(3, 3, 0)]);
    let b = grid_extent!([(4, 4, 0)]);
    let c = a - b;
    let expected = grid_extent!([(-1, -1, 0)]);
    assert_eq!(c, expected);
}

#[test]
fn grid_extent_sub_test_2() {
    use crate::grid_extent;

    let a = grid_extent!([(0, 0, 0), (-5, -5, 0)]);
    let b = grid_extent!([(0, 0, 0), (3, 3, 0)]);
    let c = a - b;
    let expected = grid_extent!([(-1, -1, 0), (2, 2, 0)]);
    assert_eq!(c, expected);
}

#[test]
fn grid_extent_sub_test_3() {
    use super::types::GridVec;

    let a = GridVec::build().repeat((-5, -5, 0), 71).finish();
    let b = GridVec::build().repeat((0, 0, 0), 70).push((1, 1, 0)).finish();
    let c = a - b;
    let expected = GridVec::build().repeat((4, 4, 0), 71).finish();
    assert_eq!(c, expected);
}

#[test]
fn grid_to_native_visual_scales_offsets_for_coarser_levels() {
    use crate::bevy::math::Vec3;
    use crate::grid_extent;

    let chunk_size = 1_000.0;
    let origin = grid_extent!([(0, 0, 0), (0, 0, 0)]);
    let coarser_neighbor = grid_extent!([(1, 0, 0)]);

    let (pos, scale) = coarser_neighbor.to_native_visual(origin);

    assert_eq!(scale, 10.0);
    assert_eq!(pos, Vec3::new(chunk_size * 9.5, chunk_size * -0.5, chunk_size * -0.5));
}

#[test]
fn grid_to_native_visual_same_scale_neighbor_uses_base_chunk_spacing() {
    use crate::bevy::math::Vec3;
    use crate::grid_extent;

    let chunk_size = 1_000.0;
    let origin = grid_extent!([(0, 0, 0), (0, 0, 0)]);
    let same_scale_neighbor = grid_extent!([(0, 0, 0), (1, 0, 0)]);

    let (pos, scale) = same_scale_neighbor.to_native_visual(origin);

    assert_eq!(scale, 1.0);
    assert_eq!(pos, Vec3::new(chunk_size, 0.0, 0.0));
}

#[test]
fn grid_to_native_visual_supports_finer_relative_to_coarser_origin() {
    use crate::grid_extent;

    let chunk_size = 1_000.0;
    let origin = grid_extent!([(0, 0, 0)]);
    let finer_neighbor = grid_extent!([(0, 0, 0), (0, 0, 0)]);

    let (pos, scale) = finer_neighbor.to_native_visual(origin);

    assert!((scale - 0.1).abs() <= 1e-6);
    assert!((pos.x - chunk_size * 0.5).abs() <= 1e-3);
    assert!((pos.y - chunk_size * 0.5).abs() <= 1e-3);
    assert!((pos.z - chunk_size * 0.5).abs() <= 1e-3);
}

#[test]
fn grid_to_native_visual_handles_wide_scale_differences_without_panicking() {
    use super::types::GridVec;

    let root = GridVec::build().push((0, 0, 0)).finish();
    let deep = GridVec::build().repeat((0, 0, 0), 20).finish();

    let (pos_a, scale_a) = root.clone().to_native_visual(deep.clone());
    let (pos_b, scale_b) = deep.to_native_visual(root);

    assert!(pos_a.is_finite() && pos_b.is_finite());
    assert!(scale_a.is_finite() && scale_b.is_finite());
}

#[test]
fn grid_to_native_logical_scales_offsets_for_coarser_levels() {
    use crate::bevy::math::Vec3;
    use crate::grid_extent;

    let chunk_size = 1_000.0;
    let origin = grid_extent!([(0, 0, 0), (0, 0, 0)]);
    let coarser_neighbor = grid_extent!([(1, 0, 0)]);

    let pos = coarser_neighbor.to_native_logical(origin);

    assert_eq!(pos, Vec3::new(chunk_size * 9.5, chunk_size * -0.5, chunk_size * -0.5));
}

#[test]
fn grid_to_native_logical_same_scale_neighbor_uses_base_chunk_spacing() {
    use crate::bevy::math::Vec3;
    use crate::grid_extent;

    let chunk_size = 1_000.0;
    let origin = grid_extent!([(0, 0, 0), (0, 0, 0)]);
    let same_scale_neighbor = grid_extent!([(0, 0, 0), (1, 0, 0)]);

    let pos = same_scale_neighbor.to_native_logical(origin);

    assert_eq!(pos, Vec3::new(chunk_size, 0.0, 0.0));
}

#[test]
fn grid_to_native_logical_z_axis_neighbor_uses_base_chunk_spacing() {
    use crate::bevy::math::Vec3;
    use crate::grid_extent;

    let chunk_size = 1_000.0;
    let origin = grid_extent!([(0, 0, 0), (0, 0, 0)]);
    let same_scale_neighbor = grid_extent!([(0, 0, 0), (0, 0, 1)]);

    let pos = same_scale_neighbor.to_native_logical(origin);

    assert_eq!(pos, Vec3::new(0.0, 0.0, chunk_size));
}
