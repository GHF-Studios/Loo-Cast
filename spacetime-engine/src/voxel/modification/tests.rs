use bevy::prelude::{IVec3, Vec3};

use super::*;
use crate::voxel::{
    MATERIALIZATION_CHUNK_SIZE, VoxelBase, VoxelBrush, VoxelChunkCoord, VoxelMaterialId,
    VoxelQueryPosition, VoxelWorld,
};

fn query(local: Vec3) -> VoxelQueryPosition {
    VoxelQueryPosition::from_scale0_local(local).unwrap()
}

#[test]
fn canonical_chunk_index_returns_only_local_edits_in_global_order() {
    let mut world = VoxelWorld::new(VoxelBase::Empty);
    let local_address = world
        .chunk_address(VoxelChunkCoord::new(IVec3::ZERO))
        .unwrap();
    let first = VoxelEdit::Add {
        brush: VoxelBrush::sphere(query(Vec3::splat(8.0)), 2.0),
        material: VoxelMaterialId::ROCK,
    };
    let distant = VoxelEdit::Add {
        brush: VoxelBrush::sphere(query(Vec3::splat(1000.0)), 2.0),
        material: VoxelMaterialId::ROCK,
    };
    let second = VoxelEdit::Remove {
        brush: VoxelBrush::sphere(query(Vec3::splat(12.0)), 1.0),
    };

    world.record_edit(first).unwrap();
    world.record_edit(distant).unwrap();
    world.record_edit(second).unwrap();

    assert_eq!(
        world
            .modifications()
            .for_chunk(local_address)
            .collect::<Vec<_>>(),
        vec![first, second]
    );
    assert_eq!(world.modifications().edits(), &[first, distant, second]);
}

#[test]
fn canonical_chunk_index_can_replay_only_edits_after_a_snapshot() {
    let mut world = VoxelWorld::new(VoxelBase::Empty);
    let address = world
        .chunk_address(VoxelChunkCoord::new(IVec3::ZERO))
        .unwrap();

    world
        .record_edit(VoxelEdit::Add {
            brush: VoxelBrush::sphere(query(Vec3::splat(8.0)), 2.0),
            material: VoxelMaterialId::ROCK,
        })
        .unwrap();
    let snapshot_count = world.modifications().len();

    world
        .record_edit(VoxelEdit::Add {
            brush: VoxelBrush::sphere(query(Vec3::splat(1000.0)), 2.0),
            material: VoxelMaterialId::ROCK,
        })
        .unwrap();
    let local_after_snapshot = VoxelEdit::Remove {
        brush: VoxelBrush::sphere(query(Vec3::splat(8.0)), 1.0),
    };
    world.record_edit(local_after_snapshot).unwrap();

    assert_eq!(
        world
            .modifications()
            .for_chunk_since(address, snapshot_count)
            .collect::<Vec<_>>(),
        vec![local_after_snapshot]
    );
}

#[test]
fn seam_edit_is_indexed_for_both_canonical_padded_chunk_domains() {
    let mut world = VoxelWorld::new(VoxelBase::Empty);
    let edit = VoxelEdit::Add {
        brush: VoxelBrush::sphere(
            query(Vec3::new(MATERIALIZATION_CHUNK_SIZE as f32, 8.0, 8.0)),
            1.0,
        ),
        material: VoxelMaterialId::ROCK,
    };
    world.record_edit(edit).unwrap();

    let left = world
        .chunk_address(VoxelChunkCoord::new(IVec3::ZERO))
        .unwrap();
    let right = world.chunk_address(VoxelChunkCoord::new(IVec3::X)).unwrap();
    assert_eq!(
        world.modifications().for_chunk(left).collect::<Vec<_>>(),
        vec![edit]
    );
    assert_eq!(
        world.modifications().for_chunk(right).collect::<Vec<_>>(),
        vec![edit]
    );
}
}
