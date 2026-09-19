use super::*;
use crate::{
    spatial::SpatialScale,
    voxel::{VoxelBrush, VoxelMaterialId, VoxelSample},
};

fn query(local: Vec3) -> VoxelQueryPosition {
    VoxelQueryPosition::from_scale0_local(local).unwrap()
}

#[test]
fn compatibility_chunk_coordinates_are_euclidean_grid_offsets() {
    assert_eq!(
        VoxelChunkCoord::new(IVec3::new(-1, 2, 0)).origin(),
        IVec3::new(
            -(MATERIALIZATION_CHUNK_SIZE as i32),
            2 * MATERIALIZATION_CHUNK_SIZE as i32,
            0,
        )
    );
    assert_eq!(
        VoxelChunkCoord::containing(Vec3::new(-0.01, 0.0, 9.99)),
        VoxelChunkCoord::new(IVec3::new(-1, 0, 0))
    );
}

#[test]
fn materialization_address_is_canonical_across_semantic_region_boundaries() {
    let world = VoxelWorld::new(VoxelBase::Empty);
    let address = world
        .chunk_address(VoxelChunkCoord::new(IVec3::new(128, 0, 0)))
        .unwrap();

    assert_eq!(address.origin().digit(SpatialScale::ZERO).x, 1);
    assert_eq!(address.origin().offset().x, 280.0);
}

#[test]
fn containing_address_crosses_usf_carry_without_flat_grid_identity() {
    let origin = UsfPosition::from_scale0_local(Vec3::new(499.0, 0.0, 0.0)).unwrap();
    let world = VoxelWorld::new_at(VoxelBase::Empty, origin);
    let point =
        VoxelQueryPosition::new(origin.translated_native(Vec3::new(12.0, 0.0, 0.0)).unwrap());
    let address = world.materialization_address_containing(point).unwrap();

    assert_eq!(
        point.relative_to(address.query_origin(), 10.0).unwrap(),
        Vec3::new(2.0, 0.0, 0.0)
    );
}

#[test]
fn canonical_addresses_do_not_collapse_through_large_f32_coordinates() {
    let world = VoxelWorld::new(VoxelBase::Empty);
    let left = world
        .chunk_address(VoxelChunkCoord::new(IVec3::new(2_000_000_000, 0, 0)))
        .unwrap();
    let right = world
        .chunk_address(VoxelChunkCoord::new(IVec3::new(2_000_000_001, 0, 0)))
        .unwrap();

    assert_eq!(20_000_000_000_i64 as f32, 20_000_000_010_i64 as f32);
    assert_ne!(left, right);
}

#[test]
fn edit_bounds_address_both_padded_chunks_at_a_seam() {
    let world = VoxelWorld::new(VoxelBase::Empty);
    let center = query(Vec3::new(MATERIALIZATION_CHUNK_SIZE as f32, 5.0, 5.0));
    let edit = VoxelEdit::Add {
        brush: VoxelBrush::sphere(center, 0.5),
        material: VoxelMaterialId::ROCK,
    };
    let addresses = world
        .materialization_addresses_intersecting(edit.influence_bounds())
        .unwrap();
    let left = world
        .chunk_address(VoxelChunkCoord::new(IVec3::ZERO))
        .unwrap();
    let right = world.chunk_address(VoxelChunkCoord::new(IVec3::X)).unwrap();

    assert!(addresses.contains(&left));
    assert!(addresses.contains(&right));
}

#[test]
fn neighbor_chunks_store_identical_overlap_after_cross_boundary_edit() {
    let world = VoxelWorld::new(VoxelBase::Empty);
    let left_address = world
        .chunk_address(VoxelChunkCoord::new(IVec3::ZERO))
        .unwrap();
    let right_address = world.chunk_address(VoxelChunkCoord::new(IVec3::X)).unwrap();
    let mut left = VoxelChunk::generate(|_| VoxelSample::empty(100.0));
    let mut right = VoxelChunk::generate(|_| VoxelSample::empty(100.0));
    let edit = VoxelEdit::Add {
        brush: VoxelBrush::sphere(
            query(Vec3::new(MATERIALIZATION_CHUNK_SIZE as f32, 5.0, 5.0)),
            3.0,
        ),
        material: VoxelMaterialId::ROCK,
    };

    assert!(
        left_address
            .sample_bounds()
            .intersects(edit.influence_bounds())
    );
    assert!(
        right_address
            .sample_bounds()
            .intersects(edit.influence_bounds())
    );
    left.apply_edit(left_address, edit);
    right.apply_edit(right_address, edit);

    for world_x in [
        MATERIALIZATION_CHUNK_SIZE as i32 - 1,
        MATERIALIZATION_CHUNK_SIZE as i32,
    ] {
        for y in 2..=8 {
            for z in 2..=8 {
                let left_point = IVec3::new(world_x, y, z);
                let right_point = IVec3::new(world_x - MATERIALIZATION_CHUNK_SIZE as i32, y, z);
                assert_eq!(left.sample(left_point), right.sample(right_point));
            }
        }
    }
}

#[test]
fn procedural_scale_layer_neighbors_share_identical_overlap_samples() {
    let scale = crate::spatial::SpatialScale::MAX;
    let origin = crate::spatial::UsfPosition::zero(scale);
    let volume = crate::voxel::ProceduralVolume::scale_layer(
        0x10_0CA57_5EED_2026,
        0x1234_5678_9ABC_DEF0,
        scale,
    );
    let world = VoxelWorld::new_at(VoxelBase::Volume(volume), origin);
    let left_address = world
        .chunk_address(VoxelChunkCoord::new(IVec3::ZERO))
        .unwrap();
    let right_address = world.chunk_address(VoxelChunkCoord::new(IVec3::X)).unwrap();
    let left = world.materialize_chunk(left_address);
    let right = world.materialize_chunk(right_address);

    for world_x in [
        MATERIALIZATION_CHUNK_SIZE as i32 - 1,
        MATERIALIZATION_CHUNK_SIZE as i32,
    ] {
        for y in -1..=MATERIALIZATION_CHUNK_SIZE as i32 {
            for z in -1..=MATERIALIZATION_CHUNK_SIZE as i32 {
                let left_local = IVec3::new(world_x, y, z);
                let right_local = IVec3::new(world_x - MATERIALIZATION_CHUNK_SIZE as i32, y, z);
                assert_eq!(left.sample(left_local), right.sample(right_local));
            }
        }
    }
}

#[test]
fn sparse_edits_survive_chunk_rematerialization() {
    let world_origin = UsfPosition::default();
    let center = query(Vec3::splat(8.0));
    let mut world = VoxelWorld::new_at(
        VoxelBase::sphere(center, 6.0, VoxelMaterialId::ROCK),
        world_origin,
    );
    let address = world
        .chunk_address(VoxelChunkCoord::new(IVec3::ZERO))
        .unwrap();
    let point = IVec3::splat(8);

    assert!(
        world
            .materialize_chunk(address)
            .sample(point)
            .unwrap()
            .distance
            .is_solid()
    );

    world
        .record_edit(VoxelEdit::Remove {
            brush: VoxelBrush::sphere(center, 2.0),
        })
        .unwrap();

    let rebuilt = world.materialize_chunk(address);
    assert!(rebuilt.sample(point).unwrap().distance.is_empty());
    assert_eq!(world.modifications().len(), 1);
}
