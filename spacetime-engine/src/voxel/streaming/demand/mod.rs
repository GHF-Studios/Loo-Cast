//! Spatial-demand interpretation and voxel residency reconciliation.

use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    config::EngineConfig,
    spatial::{SpatialDemandScope, SpatialDemandSnapshot, UsfScaleLayer},
};

use super::{VoxelMaterializationDemand, VoxelStreaming};
use super::super::{
    MATERIALIZATION_CHUNK_SIZE, VoxelMaterializationChunkAddress, VoxelWorld,
    perf::VoxelPerfStats,
};

#[derive(Debug, Clone, Copy)]
pub(super) struct DemandedChunk {
    pub(super) address: VoxelMaterializationChunkAddress,
    pub(super) priority: i32,
    pub(super) distance_squared: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct VoxelDemandPlanKey {
    source: Entity,
    center_address: VoxelMaterializationChunkAddress,
    minimum: IVec3,
    maximum: IVec3,
    priority: i32,
}

/// Reconciles active voxel materialization residency with the latest spatial
/// demand snapshot.
///
/// This stage owns demand interpretation and hot/warm residency transitions. It
/// does not spawn asynchronous generation work.
pub(crate) fn refresh_voxel_residency(
    config: Res<EngineConfig>,
    demand_snapshot: Res<SpatialDemandSnapshot>,
    voxel_demand_sources: Query<(), With<VoxelMaterializationDemand>>,
    mut worlds: Query<(&mut VoxelWorld, &mut VoxelStreaming, &UsfScaleLayer)>,
    mut perf: ResMut<VoxelPerfStats>,
    mut all_voxel_demands: Local<Vec<SpatialDemandScope>>,
    mut voxel_demands: Local<Vec<SpatialDemandScope>>,
) {
    all_voxel_demands.clear();
    all_voxel_demands.extend(
        demand_snapshot
            .iter()
            .filter(|scope| voxel_demand_sources.contains(scope.source())),
    );

    let warm_limit = config.voxel.streaming.warm_inactive_materialization_limit;

    for (mut world, mut streaming, layer) in &mut worlds {
        voxel_demands.clear();
        voxel_demands.extend(
            all_voxel_demands
                .iter()
                .copied()
                .filter(|demand| demand.scale() == layer.scale()),
        );

        let changed = match refresh_demand_plan(&world, &voxel_demands, &mut streaming, &mut perf) {
            Ok(changed) => changed,
            Err(_) => {
                error!("voxel spatial demand could not be represented canonically");
                continue;
            }
        };

        if changed {
            reconcile_materialization_residency(&mut world, &mut streaming, warm_limit);
        }
    }
}

fn reconcile_materialization_residency(
    world: &mut VoxelWorld,
    streaming: &mut VoxelStreaming,
    warm_inactive_materialization_limit: usize,
) {
    let stale = world
        .materializations()
        .active_addresses()
        .filter(|address| !streaming.cached_desired_set.contains(address))
        .collect::<Vec<_>>();
    for address in stale {
        world.materializations_mut().deactivate(address);
    }

    // Warm dense entries become active immediately and avoid regeneration.
    let desired = streaming
        .cached_desired_set
        .iter()
        .copied()
        .collect::<Vec<_>>();
    for address in desired {
        world.materializations_mut().reactivate(address);
    }

    world
        .materializations_mut()
        .trim_inactive(warm_inactive_materialization_limit);

    streaming
        .pending_desired
        .retain(|demanded| !world.materializations().is_active(demanded.address));
}

fn refresh_demand_plan(
    world: &VoxelWorld,
    demands: &[SpatialDemandScope],
    streaming: &mut VoxelStreaming,
    perf: &mut VoxelPerfStats,
) -> Result<bool, crate::spatial::UsfPositionError> {
    let key = demand_plan_key(world, demands)?;
    if key == streaming.demand_key {
        return Ok(false);
    }

    let desired = demanded_chunk_addresses(world, demands)?;
    streaming.cached_desired_set.clear();
    streaming
        .cached_desired_set
        .extend(desired.iter().map(|chunk| chunk.address));
    streaming.pending_desired = desired
        .iter()
        .copied()
        .filter(|chunk| !world.materializations().is_active(chunk.address))
        .collect();
    streaming.demand_key = key;
    perf.record_demand_rebuild();
    Ok(true)
}

fn demand_plan_key(
    world: &VoxelWorld,
    demands: &[SpatialDemandScope],
) -> Result<Vec<VoxelDemandPlanKey>, crate::spatial::UsfPositionError> {
    let mut result = Vec::with_capacity(demands.len());
    let size = MATERIALIZATION_CHUNK_SIZE as f32;
    for demand in demands {
        let center = super::VoxelQueryPosition::new(demand.center());
        let center_address = world.materialization_address_containing(center)?;
        let local = center.relative_to(center_address.query_origin(), size + 0.01)?;
        let half = demand.half_extent_native();
        result.push(VoxelDemandPlanKey {
            source: demand.source(),
            center_address,
            minimum: checked_ivec3(((local - half) / size).floor())?,
            maximum: checked_ivec3(((local + half) / size).floor())?,
            priority: demand.priority(),
        });
    }
    Ok(result)
}

pub(super) fn demanded_chunk_addresses(
    world: &VoxelWorld,
    demands: &[SpatialDemandScope],
) -> Result<Vec<DemandedChunk>, crate::spatial::UsfPositionError> {
    let mut merged = HashMap::<VoxelMaterializationChunkAddress, DemandedChunk>::new();

    for demand in demands {
        let center = super::VoxelQueryPosition::new(demand.center());
        let center_address = world.materialization_address_containing(center)?;
        let size = MATERIALIZATION_CHUNK_SIZE as f32;
        let local_center = center.relative_to(center_address.query_origin(), size + 0.01)?;
        let half = demand.half_extent_native();
        let minimum = checked_ivec3(((local_center - half) / size).floor())?;
        let maximum = checked_ivec3(((local_center + half) / size).floor())?;

        for z in minimum.z..=maximum.z {
            for y in minimum.y..=maximum.y {
                for x in minimum.x..=maximum.x {
                    let offset = IVec3::new(x, y, z);
                    let address = center_address.translated_chunks(offset)?;
                    let chunk_center = offset.as_vec3() * size + Vec3::splat(size * 0.5);
                    let distance_squared = (chunk_center - local_center).length_squared();
                    let candidate = DemandedChunk {
                        address,
                        priority: demand.priority(),
                        distance_squared,
                    };

                    merged
                        .entry(address)
                        .and_modify(|current| {
                            if candidate.priority > current.priority
                                || (candidate.priority == current.priority
                                    && candidate.distance_squared < current.distance_squared)
                            {
                                *current = candidate;
                            }
                        })
                        .or_insert(candidate);
                }
            }
        }
    }

    let mut desired = merged.into_values().collect::<Vec<_>>();
    desired.sort_by(|a, b| {
        b.priority
            .cmp(&a.priority)
            .then_with(|| a.distance_squared.total_cmp(&b.distance_squared))
    });
    Ok(desired)
}

fn checked_ivec3(value: Vec3) -> Result<IVec3, crate::spatial::UsfPositionError> {
    fn component(value: f32) -> Result<i32, crate::spatial::UsfPositionError> {
        let value64 = f64::from(value);
        if !value.is_finite() || value64 < i32::MIN as f64 || value64 > i32::MAX as f64 {
            Err(crate::spatial::UsfPositionError::TranslationTooLarge)
        } else {
            Ok(value as i32)
        }
    }

    Ok(IVec3::new(
        component(value.x)?,
        component(value.y)?,
        component(value.z)?,
    ))
}
