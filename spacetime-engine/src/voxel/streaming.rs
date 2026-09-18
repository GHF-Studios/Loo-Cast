//! Demand-driven residency of voxel materialization caches.
//!
//! Streaming owns *which canonical addresses are active*. Dense voxel data lives
//! in [`VoxelWorld`]'s compact materialization store rather than in one ECS entity
//! per address. Generation jobs are transient ECS participants only.

use std::{
    collections::{HashMap, HashSet, VecDeque},
    time::Instant,
};

use bevy::{
    ecs::lifecycle::RemovedComponents,
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task, futures::check_ready},
};

use crate::{
    config::EngineConfig,
    spatial::{SpatialDemandScope, SpatialDemandSnapshot, UsfScaleLayer},
};

use super::{
    MATERIALIZATION_CHUNK_SIZE, VoxelChunk, VoxelMaterializationChunkAddress, VoxelWorld,
    aggregate::{VoxelMaterializationAggregateExtent, VoxelMaterializationAggregateScope},
    perf::{VoxelPerfStats, per_stage_in_flight_limit},
    world::VoxelChunkRecipe,
};

/// Demand-streaming policy for one [`VoxelWorld`].
///
/// Presentation material is intentionally separate: residency policy should not
/// own renderer state, and manually resident worlds can use the same realization
/// pipeline without pretending to be streamed.
#[derive(Component, Debug, Clone)]
pub struct VoxelStreaming {
    load_budget_per_frame: usize,
    demand_key: Vec<VoxelDemandPlanKey>,
    pending_desired: VecDeque<DemandedChunk>,
    cached_desired_set: HashSet<VoxelMaterializationChunkAddress>,
}

impl VoxelStreaming {
    pub fn new(load_budget_per_frame: usize) -> Self {
        Self {
            load_budget_per_frame: load_budget_per_frame.max(1),
            demand_key: Vec::new(),
            pending_desired: VecDeque::new(),
            cached_desired_set: HashSet::new(),
        }
    }

    pub const fn load_budget_per_frame(&self) -> usize {
        self.load_budget_per_frame
    }
}

/// Render material used by disposable voxel manifestations.
#[derive(Component, Debug, Clone)]
pub struct VoxelPresentationMaterial(Handle<StandardMaterial>);

impl VoxelPresentationMaterial {
    pub fn new(material: Handle<StandardMaterial>) -> Self {
        Self(material)
    }

    pub(crate) fn handle(&self) -> &Handle<StandardMaterial> {
        &self.0
    }
}

/// Marks a generic [`crate::spatial::SpatialDemandSource`] as requesting voxel
/// materialization.
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct VoxelMaterializationDemand;

#[derive(Debug, Clone, Copy)]
struct DemandedChunk {
    address: VoxelMaterializationChunkAddress,
    priority: i32,
    distance_squared: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct VoxelDemandPlanKey {
    source: Entity,
    center_address: VoxelMaterializationChunkAddress,
    minimum: IVec3,
    maximum: IVec3,
    priority: i32,
}

struct VoxelGenerationJob {
    address: VoxelMaterializationChunkAddress,
    token: u64,
    recipe: VoxelChunkRecipe,
}

struct VoxelGeneratedChunk {
    address: VoxelMaterializationChunkAddress,
    token: u64,
    applied_edit_count: usize,
    generation_micros: u64,
    chunk: VoxelChunk,
}

struct PendingAggregateGeneration {
    scope: VoxelMaterializationAggregateScope,
    jobs: Vec<VoxelGenerationJob>,
}

/// One asynchronous work item over several independently addressable atoms.
#[derive(Component)]
pub(crate) struct VoxelAggregateGenerationTask {
    world: Entity,
    scope: VoxelMaterializationAggregateScope,
    task: Option<Task<Vec<VoxelGeneratedChunk>>>,
    ready: VecDeque<VoxelGeneratedChunk>,
}

impl VoxelAggregateGenerationTask {
    fn spawn(
        world: Entity,
        scope: VoxelMaterializationAggregateScope,
        jobs: Vec<VoxelGenerationJob>,
    ) -> Self {
        debug_assert!(!jobs.is_empty());
        let task = AsyncComputeTaskPool::get().spawn(async move {
            jobs.into_iter()
                .map(|job| {
                    let applied_edit_count = job.recipe.applied_edit_count();
                    let started = Instant::now();
                    let chunk = job.recipe.materialize();
                    VoxelGeneratedChunk {
                        address: job.address,
                        token: job.token,
                        applied_edit_count,
                        generation_micros: started.elapsed().as_micros() as u64,
                        chunk,
                    }
                })
                .collect()
        });
        Self {
            world,
            scope,
            task: Some(task),
            ready: VecDeque::new(),
        }
    }
}

pub(crate) fn finish_chunk_generation(
    config: Res<EngineConfig>,
    mut commands: Commands,
    mut worlds: Query<&mut VoxelWorld>,
    mut tasks: Query<(Entity, &mut VoxelAggregateGenerationTask)>,
    mut perf: ResMut<VoxelPerfStats>,
) {
    let publish_budget = config.voxel.streaming.generation_publish_budget_per_frame;
    let mut published = 0;

    for (task_entity, mut generation) in &mut tasks {
        if published >= publish_budget {
            break;
        }

        if generation.ready.is_empty() {
            let completed = generation.task.as_mut().and_then(|task| check_ready(task));
            let Some(completed) = completed else {
                continue;
            };
            generation.task = None;
            generation.ready = completed.into();
        }

        let Ok(mut world) = worlds.get_mut(generation.world) else {
            generation.ready.clear();
            commands.entity(task_entity).despawn();
            continue;
        };

        while published < publish_budget {
            let Some(mut output) = generation.ready.pop_front() else {
                break;
            };

            catch_up_generated_chunk(
                &world,
                output.address,
                output.applied_edit_count,
                &mut output.chunk,
            );

            if world.materializations_mut().publish_generated(
                output.address,
                output.token,
                output.chunk,
            ) {
                perf.record_generation(output.generation_micros);
                published += 1;
            }
        }

        if generation.task.is_none() && generation.ready.is_empty() {
            trace!(
                scope = ?generation.scope,
                "voxel aggregate generation work item completed"
            );
            commands.entity(task_entity).despawn();
        }
    }
}

/// Generation jobs are the only per-materialization ECS objects left in this
/// stage. If their semantic world disappears, retire them immediately.
pub(crate) fn retire_orphaned_tasks(
    mut commands: Commands,
    mut removed_worlds: RemovedComponents<VoxelWorld>,
    aggregate_tasks: Query<(Entity, &VoxelAggregateGenerationTask)>,
    mut removed: Local<Vec<Entity>>,
) {
    removed.clear();
    removed.extend(removed_worlds.read());
    if removed.is_empty() {
        return;
    }

    for (entity, task) in &aggregate_tasks {
        if removed.contains(&task.world) {
            commands.entity(entity).despawn();
        }
    }
}

pub(crate) fn stream_voxel_chunks(
    config: Res<EngineConfig>,
    mut commands: Commands,
    demand_snapshot: Res<SpatialDemandSnapshot>,
    voxel_demand_sources: Query<(), With<VoxelMaterializationDemand>>,
    mut worlds: Query<(Entity, &mut VoxelWorld, &mut VoxelStreaming, &UsfScaleLayer)>,
    generation_tasks: Query<(), With<VoxelAggregateGenerationTask>>,
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

    let mut generation_slots =
        per_stage_in_flight_limit().saturating_sub(generation_tasks.iter().count());
    let streaming_config = config.voxel.streaming;
    let generation_extent = VoxelMaterializationAggregateExtent::from_base_chunks_per_axis(
        streaming_config.generation_group_base_chunks_per_axis,
    )
    .expect("validated engine config must produce a generation grouping extent");

    for (world_entity, mut world, mut streaming, layer) in &mut worlds {
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
            let stale = world
                .materializations()
                .active_addresses()
                .filter(|address| !streaming.cached_desired_set.contains(address))
                .collect::<Vec<_>>();
            for address in stale {
                world.materializations_mut().deactivate(address);
            }

            // Warm dense entries become active immediately and avoid generation.
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
                .trim_inactive(streaming_config.warm_inactive_materialization_limit);

            streaming
                .pending_desired
                .retain(|demanded| !world.materializations().is_active(demanded.address));
        }

        let load_budget = streaming.load_budget_per_frame;
        let mut requested = 0;
        let mut aggregate_batches = Vec::<PendingAggregateGeneration>::new();

        while requested < load_budget && generation_slots > 0 {
            let Some(demanded) = streaming.pending_desired.pop_front() else {
                break;
            };
            let address = demanded.address;

            if world.materializations().is_active(address) {
                continue;
            }

            let Ok(scope) =
                VoxelMaterializationAggregateScope::containing(&world, address, generation_extent)
            else {
                error!(
                    ?address,
                    "voxel aggregate scope could not be derived canonically"
                );
                continue;
            };

            if !generation_batch_can_accept(
                &aggregate_batches,
                scope,
                generation_slots,
                streaming_config.max_chunks_per_generation_task,
            ) {
                streaming.pending_desired.push_front(demanded);
                break;
            }

            let Some(token) = world.materializations_mut().reserve_generation(address) else {
                continue;
            };
            let recipe = world.chunk_recipe(address);
            push_generation_job(
                &mut aggregate_batches,
                scope,
                streaming_config.max_chunks_per_generation_task,
                VoxelGenerationJob {
                    address,
                    token,
                    recipe,
                },
            );
            requested += 1;
        }

        for batch in aggregate_batches {
            commands.spawn((
                Name::new("Voxel Aggregate Generation"),
                VoxelAggregateGenerationTask::spawn(world_entity, batch.scope, batch.jobs),
            ));
            generation_slots = generation_slots.saturating_sub(1);
        }
    }
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

fn demanded_chunk_addresses(
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

fn generation_batch_can_accept(
    batches: &[PendingAggregateGeneration],
    scope: VoxelMaterializationAggregateScope,
    max_batches: usize,
    max_chunks_per_batch: usize,
) -> bool {
    batches
        .iter()
        .any(|batch| batch.scope == scope && batch.jobs.len() < max_chunks_per_batch)
        || batches.len() < max_batches
}

fn push_generation_job(
    batches: &mut Vec<PendingAggregateGeneration>,
    scope: VoxelMaterializationAggregateScope,
    max_chunks_per_batch: usize,
    job: VoxelGenerationJob,
) {
    if let Some(batch) = batches
        .iter_mut()
        .find(|batch| batch.scope == scope && batch.jobs.len() < max_chunks_per_batch)
    {
        batch.jobs.push(job);
        return;
    }

    batches.push(PendingAggregateGeneration {
        scope,
        jobs: vec![job],
    });
}

/// Applies edits appended after a generation task took its immutable snapshot.
fn catch_up_generated_chunk(
    world: &VoxelWorld,
    address: VoxelMaterializationChunkAddress,
    applied_edit_count: usize,
    chunk: &mut VoxelChunk,
) {
    for edit in world
        .modifications()
        .for_chunk_since(address, applied_edit_count)
    {
        chunk.apply_edit(address, edit);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::voxel::{VoxelBase, VoxelBrush, VoxelEdit, VoxelMaterialId, VoxelQueryPosition};

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
}
