use super::*;
use super::demand::demanded_chunk_addresses;
use super::generation::catch_up_generated_chunk;
use crate::spatial::SpatialDemandScope;
use bevy::prelude::*;
use std::collections::HashSet;
use crate::voxel::{VoxelBase, VoxelBrush, VoxelEdit, VoxelMaterialId, VoxelQueryPosition, VoxelWorld};

fn query(local: Vec3) -> VoxelQueryPosition {
    VoxelQueryPosition::from_scale0_local(local).unwrap()
}

#[test]
fn overlapping_spatial_demands_merge_without_duplicate_materialization_identity() {
    let world = VoxelWorld::new(VoxelBase::Empty);
    let mut ecs = World::new();
    let first = SpatialDemandScope::new(
        ecs.spawn_empty().id(),
        query(Vec3::ZERO).usf(),
        Vec3::splat(12.0),
        1,
    );
    let second = SpatialDemandScope::new(
        ecs.spawn_empty().id(),
        query(Vec3::new(5.0, 0.0, 0.0)).usf(),
        Vec3::splat(12.0),
        5,
    );
    let desired = demanded_chunk_addresses(&world, &[first, second]).unwrap();
    let unique = desired
        .iter()
        .map(|chunk| chunk.address)
        .collect::<HashSet<_>>();

    assert_eq!(unique.len(), desired.len());
    assert!(
        desired
            .windows(2)
            .all(|pair| pair[0].priority >= pair[1].priority)
    );
    assert!(desired.iter().any(|chunk| chunk.priority == 5));
}

#[test]
fn removing_one_source_preserves_other_sources_requests() {
    let world = VoxelWorld::new(VoxelBase::Empty);
    let mut ecs = World::new();
    let player = SpatialDemandScope::new(
        ecs.spawn_empty().id(),
        query(Vec3::ZERO).usf(),
        Vec3::splat(12.0),
        100,
    );
    let cube = SpatialDemandScope::new(
        ecs.spawn_empty().id(),
        query(Vec3::new(40.0, 0.0, 0.0)).usf(),
        Vec3::splat(12.0),
        50,
    );

    let both = demanded_chunk_addresses(&world, &[player, cube])
        .unwrap()
        .into_iter()
        .map(|chunk| chunk.address)
        .collect::<HashSet<_>>();
    let cube_only = demanded_chunk_addresses(&world, &[cube])
        .unwrap()
        .into_iter()
        .map(|chunk| chunk.address)
        .collect::<HashSet<_>>();

    assert!(!cube_only.is_empty());
    assert!(cube_only.is_subset(&both));
    assert!(both.difference(&cube_only).next().is_some());
}

#[test]
fn no_spatial_demand_requests_no_materializations() {
    let world = VoxelWorld::new(VoxelBase::Empty);
    assert!(demanded_chunk_addresses(&world, &[]).unwrap().is_empty());
}

#[test]
fn moving_spatial_demand_migrates_the_requested_materialization_set() {
    let world = VoxelWorld::new(VoxelBase::Empty);
    let mut ecs = World::new();
    let source = ecs.spawn_empty().id();
    let half_extent = Vec3::splat(12.0);
    let before = SpatialDemandScope::new(source, query(Vec3::ZERO).usf(), half_extent, 1);
    let after = SpatialDemandScope::new(
        source,
        query(Vec3::new(40.0, 0.0, 0.0)).usf(),
        half_extent,
        1,
    );

    let before = demanded_chunk_addresses(&world, &[before])
        .unwrap()
        .into_iter()
        .map(|chunk| chunk.address)
        .collect::<HashSet<_>>();
    let after = demanded_chunk_addresses(&world, &[after])
        .unwrap()
        .into_iter()
        .map(|chunk| chunk.address)
        .collect::<HashSet<_>>();

    assert_ne!(before, after);
    assert!(before.difference(&after).next().is_some());
    assert!(after.difference(&before).next().is_some());
}

#[test]
fn demand_crosses_canonical_digit_carry_without_flat_coordinates() {
    let origin =
        crate::spatial::UsfPosition::from_scale0_local(Vec3::new(499.0, 0.0, 0.0)).unwrap();
    let world = VoxelWorld::new_at(VoxelBase::Empty, origin);
    let mut ecs = World::new();
    let center = origin.translated_native(Vec3::new(8.0, 0.0, 0.0)).unwrap();
    let demand =
        SpatialDemandScope::new(ecs.spawn_empty().id(), center, Vec3::new(20.0, 5.0, 5.0), 1);

    let desired = demanded_chunk_addresses(&world, &[demand]).unwrap();
    let unique = desired
        .iter()
        .map(|chunk| chunk.address)
        .collect::<HashSet<_>>();

    assert_eq!(unique.len(), desired.len());
    assert!(desired.len() > 1);
    assert!(desired.iter().any(|chunk| {
        chunk
            .address
            .query_origin()
            .relative_to(VoxelQueryPosition::new(origin), 64.0)
            .map_or(false, |delta| delta.x < 0.0)
    }));
    assert!(desired.iter().any(|chunk| {
        chunk
            .address
            .query_origin()
            .relative_to(VoxelQueryPosition::new(origin), 64.0)
            .map_or(false, |delta| delta.x > 0.0)
    }));
}

#[test]
fn semantic_edit_survives_dense_cache_rematerialization() {
    let mut world = VoxelWorld::new(VoxelBase::Empty);
    let center = query(Vec3::splat(5.0));
    world
        .record_edit(VoxelEdit::Add {
            brush: VoxelBrush::sphere(center, 2.0),
            material: VoxelMaterialId::ROCK,
        })
        .unwrap();
    let address = world.materialization_address_containing(center).unwrap();

    let first = world.materialize_chunk(address);
    let second = world.materialize_chunk(address);

    assert!(first.sample(IVec3::splat(5)).unwrap().distance.is_solid());
    assert!(second.sample(IVec3::splat(5)).unwrap().distance.is_solid());
}

#[test]
fn generation_completion_replays_edits_recorded_while_task_was_running() {
    let mut world = VoxelWorld::new(VoxelBase::Empty);
    let address = world
        .materialization_address_containing(query(Vec3::splat(8.0)))
        .unwrap();
    let center = query(Vec3::splat(8.0));
    let recipe = world.chunk_recipe(address);
    let applied_edit_count = recipe.applied_edit_count();

    world
        .record_edit(VoxelEdit::Add {
            brush: VoxelBrush::sphere(query(Vec3::splat(1000.0)), 2.0),
            material: VoxelMaterialId::ROCK,
        })
        .unwrap();
    world
        .record_edit(VoxelEdit::Add {
            brush: VoxelBrush::sphere(center, 2.0),
            material: VoxelMaterialId::ROCK,
        })
        .unwrap();

    let mut chunk = recipe.materialize();
    assert!(chunk.sample(IVec3::splat(8)).unwrap().distance.is_empty());

    catch_up_generated_chunk(&world, address, applied_edit_count, &mut chunk);
    assert!(chunk.sample(IVec3::splat(8)).unwrap().distance.is_solid());
}
