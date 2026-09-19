use super::*;

fn position(local: Vec3) -> VoxelQueryPosition {
    VoxelQueryPosition::from_scale0_local(local).unwrap()
}

#[test]
fn canonical_bounds_cross_a_usf_digit_boundary_without_flat_coordinates() {
    let anchor = position(Vec3::new(499.0, 0.0, 0.0));
    let bounds = VoxelBounds::new(anchor, Vec3::splat(-3.0), Vec3::splat(3.0));
    let point = anchor.translated(Vec3::new(2.5, 0.0, 0.0)).unwrap();

    assert!(bounds.contains(point));
}

#[test]
fn sphere_add_uses_sdf_union_near_the_brush() {
    let center = position(Vec3::ZERO);
    let edit = VoxelEdit::Add {
        brush: VoxelBrush::sphere(center, 2.0),
        material: VoxelMaterialId::ROCK,
    };
    let sample = edit.apply_to_sample(center, VoxelSample::empty(100.0));

    assert_eq!(sample.distance.0, -2.0);
    assert_eq!(sample.material, VoxelMaterialId::ROCK);
}

#[test]
fn sphere_remove_uses_sdf_difference_near_the_brush() {
    let center = position(Vec3::ZERO);
    let edit = VoxelEdit::Remove {
        brush: VoxelBrush::sphere(center, 2.0),
    };
    let sample = edit.apply_to_sample(center, VoxelSample::new(-10.0, VoxelMaterialId::ROCK));

    assert_eq!(sample.distance.0, 2.0);
    assert_eq!(sample.material, VoxelMaterialId::VOID);
}

#[test]
fn edits_do_not_rewrite_the_field_outside_their_influence_band() {
    let center = position(Vec3::ZERO);
    let before = VoxelSample::empty(100.0);
    let edit = VoxelEdit::Add {
        brush: VoxelBrush::sphere(center, 2.0),
        material: VoxelMaterialId::ROCK,
    };
    let far = center.translated(Vec3::new(20.0, 0.0, 0.0)).unwrap();

    assert_eq!(edit.apply_to_sample(far, before), before);
}
