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

#[test]
fn body_local_edit_follows_semantic_frame_translation() {
    use bevy::math::DVec3;
    use crate::{
        spatial::{SpatialScale, UsfPosition, UsfSemanticFrame},
        voxel::{VoxelFrameBrush, VoxelFrameEdit, VoxelFramePosition, VoxelFrameSnapshot},
    };

    let frame = UsfSemanticFrame::identity();
    let origin_a = UsfPosition::zero(SpatialScale::ZERO);
    let origin_b = origin_a.translated_at_scale(SpatialScale::ZERO, Vec3::X * 100.0).unwrap();
    let local_center = VoxelFramePosition::from_scale_native(
        DVec3::new(10.0, 0.0, 0.0),
        SpatialScale::ZERO,
    ).unwrap();
    let edit = VoxelFrameEdit::Remove {
        brush: VoxelFrameBrush::sphere(local_center, 2.0),
    };
    let snapshot_a = VoxelFrameSnapshot::new(origin_a, frame, SpatialScale::ZERO);
    let snapshot_b = VoxelFrameSnapshot::new(origin_b, frame, SpatialScale::ZERO);
    let world_a = snapshot_a.frame_to_world(local_center).unwrap();
    let world_b = snapshot_b.frame_to_world(local_center).unwrap();
    let projected_a = edit.projected_world(snapshot_a).unwrap();
    let projected_b = edit.projected_world(snapshot_b).unwrap();
    let solid = VoxelSample::new(-10.0, VoxelMaterialId::ROCK);

    assert!(projected_a.apply_to_sample(world_a, solid).distance.is_empty());
    assert!(projected_b.apply_to_sample(world_b, solid).distance.is_empty());
    assert_eq!(projected_b.apply_to_sample(world_a, solid), solid);
}
