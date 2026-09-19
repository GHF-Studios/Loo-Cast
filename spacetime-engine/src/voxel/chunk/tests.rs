use bevy::prelude::Vec3;

use super::*;
use crate::voxel::{VoxelBrush, VoxelQueryPosition};

fn address() -> VoxelMaterializationChunkAddress {
    VoxelMaterializationChunkAddress::new(crate::spatial::UsfPosition::default())
}

fn query(local: Vec3) -> VoxelQueryPosition {
    address().query_origin().translated(local).unwrap()
}

#[test]
fn new_chunk_needs_initial_mesh() {
    let chunk = VoxelChunk::filled(VoxelSample::empty(100.0));
    assert_eq!(chunk.revision(), 0);
    assert_eq!(chunk.meshed_revision(), None);
    assert!(chunk.needs_remesh());
}

#[test]
fn meshed_revision_tracks_authoritative_revision() {
    let mut chunk = VoxelChunk::filled(VoxelSample::empty(100.0));
    chunk.mark_meshed();
    assert!(!chunk.needs_remesh());

    let result = chunk.apply_edit(
        address(),
        VoxelEdit::Add {
            brush: VoxelBrush::sphere(query(Vec3::splat(8.0)), 2.0),
            material: VoxelMaterialId::ROCK,
        },
    );

    assert!(result.changed());
    assert_eq!(chunk.revision(), 1);
    assert!(chunk.needs_remesh());
}

#[test]
fn add_then_remove_changes_the_authoritative_field() {
    let center = IVec3::splat(8);
    let mut chunk = VoxelChunk::filled(VoxelSample::empty(100.0));

    chunk.apply_edit(
        address(),
        VoxelEdit::Add {
            brush: VoxelBrush::sphere(query(center.as_vec3()), 2.0),
            material: VoxelMaterialId::ROCK,
        },
    );
    let added = chunk.sample(center).expect("center sample must exist");
    assert!(added.distance.is_solid());
    assert_eq!(added.material, VoxelMaterialId::ROCK);

    chunk.apply_edit(
        address(),
        VoxelEdit::Remove {
            brush: VoxelBrush::sphere(query(center.as_vec3()), 2.0),
        },
    );
    let removed = chunk.sample(center).expect("center sample must exist");
    assert!(removed.distance.is_empty());
    assert_eq!(removed.material, VoxelMaterialId::VOID);
}

#[test]
fn padded_neighbor_samples_are_addressable() {
    let chunk = VoxelChunk::generate(|point| {
        VoxelSample::empty(point.x + point.y * 100.0 + point.z * 10_000.0)
    });

    assert!(chunk.sample(IVec3::splat(-1)).is_some());
    assert!(
        chunk
            .sample(IVec3::splat(MATERIALIZATION_CHUNK_SIZE as i32))
            .is_some()
    );
    assert!(chunk.sample(IVec3::splat(-2)).is_none());
    assert!(
        chunk
            .sample(IVec3::splat(MATERIALIZATION_CHUNK_SIZE as i32 + 1))
            .is_none()
    );
}

#[test]
fn raycast_finds_a_generated_sphere() {
    // Keep the analytic surface inside this chunk's sampled/interpolatable
    // domain. With the decimal 10³ base chunk, a radius-3 sphere centered
    // at 8 would have its front surface at z=11, outside local storage.
    let center = Vec3::splat(5.0);
    let chunk = VoxelChunk::generate(|point| {
        let distance = point.distance(center) - 3.0;
        VoxelSample::new(
            distance,
            if distance < 0.0 {
                VoxelMaterialId::ROCK
            } else {
                VoxelMaterialId::VOID
            },
        )
    });

    let hit = chunk
        .raycast(Vec3::new(5.0, 5.0, 9.0), Vec3::NEG_Z, 20.0)
        .expect("ray should hit sphere");
    assert!((hit.position.z - 8.0).abs() < 0.1);
}
