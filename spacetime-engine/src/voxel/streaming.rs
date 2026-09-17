//! Demand-driven materialization of dense voxel chunk caches.
//!
//! Streaming is optional per [`VoxelWorld`]. The authoritative world remains
//! procedural base + sparse semantic modifications; this module realizes the
//! union of generic canonical spatial demand that opted into voxel materialization.

use std::{
    collections::{HashMap, HashSet, VecDeque},
    time::Instant,
};

use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task, futures::check_ready},
};

use crate::spatial::{
    SpatialDemandScope, SpatialDemandSnapshot, UsfActiveScaleLayer, UsfLocalScalePresentation,
    UsfScaleLayer, UsfScaleLayerFrames,
};

use super::{
    MATERIALIZATION_CHUNK_SIZE, VoxelChunk, VoxelChunkOf, VoxelChunkPhysicsLod,
    VoxelChunkPresentation, VoxelMaterializationChunkAddress, VoxelQueryPosition, VoxelWorld,
    aggregate::{VoxelMaterializationAggregateExtent, VoxelMaterializationAggregateScope},
    perf::{VoxelPerfStats, per_stage_in_flight_limit},
    world::VoxelChunkRecipe,
};

/// Maximum number of finished base materializations published into ECS in one frame.
const GENERATION_PUBLISH_BUDGET_PER_FRAME: usize = 16;

/// Current voxel-field generation work scope. This is deliberately a scheduler
/// choice, not materialization identity; `1000³` alignment is supported by the
/// same aggregate-scope primitive without forcing this subsystem to use it.
const FIELD_GENERATION_AGGREGATE_EXTENT: VoxelMaterializationAggregateExtent =
    VoxelMaterializationAggregateExtent::HUNDRED;

/// Keep first-materialization latency reasonable while still proving that one
/// worker item can process several individually addressable base chunks. Larger
/// batches remain a future policy/performance choice.
const MAX_CHUNKS_PER_AGGREGATE_GENERATION_TASK: usize = 4;

/// Dense editable/physical detail radius. This is not render distance:
/// `coarse` keeps the entire requested visible extent represented.
const FINE_HALF_EXTENT_NATIVE: f32 = 10.0;

/// Opt-in voxel realization configuration for one [`VoxelWorld`].
///
/// Spatial extent no longer lives here: generic [`SpatialDemandScope`] values
/// decide *where* realization is requested. This component only owns
/// representation-specific policy/assets for satisfying that demand.
#[derive(Component, Debug, Clone)]
pub struct VoxelStreaming {
    load_budget_per_frame: usize,
    material: Handle<StandardMaterial>,
    demand_key: Vec<VoxelDemandPlanKey>,
    pending_desired: VecDeque<DemandedChunk>,
    cached_desired_set: HashSet<VoxelMaterializationChunkAddress>,
}

impl VoxelStreaming {
    pub fn new(load_budget_per_frame: usize, material: Handle<StandardMaterial>) -> Self {
        Self {
            load_budget_per_frame: load_budget_per_frame.max(1),
            material,
            demand_key: Vec::new(),
            pending_desired: VecDeque::new(),
            cached_desired_set: HashSet::new(),
        }
    }

    pub const fn load_budget_per_frame(&self) -> usize {
        self.load_budget_per_frame
    }

    pub(crate) fn material(&self) -> &Handle<StandardMaterial> {
        &self.material
    }
}

/// Marks a generic [`crate::spatial::SpatialDemandSource`] as requesting voxel
/// materialization. Other realization subsystems can define their own opt-in
/// markers without making the generic spatial-demand layer know about them.
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
    entity: Entity,
    address: VoxelMaterializationChunkAddress,
    recipe: VoxelChunkRecipe,
}

struct VoxelGeneratedChunk {
    entity: Entity,
    address: VoxelMaterializationChunkAddress,
    applied_edit_count: usize,
    generation_micros: u64,
    chunk: VoxelChunk,
}

struct PendingAggregateGeneration {
    scope: VoxelMaterializationAggregateScope,
    jobs: Vec<VoxelGenerationJob>,
}

/// One asynchronous processing item over several individually addressable base
/// materializations. The aggregate scope groups work only: every result remains
/// a separate `VoxelChunk` cache registered by its canonical base address.
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
                        entity: job.entity,
                        address: job.address,
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
    mut commands: Commands,
    worlds: Query<&VoxelWorld>,
    mut tasks: Query<(Entity, &mut VoxelAggregateGenerationTask)>,
    mut perf: ResMut<VoxelPerfStats>,
) {
    let mut published = 0;

    for (task_entity, mut generation) in &mut tasks {
        if published >= GENERATION_PUBLISH_BUDGET_PER_FRAME {
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

        let Ok(world) = worlds.get(generation.world) else {
            for output in generation.ready.drain(..) {
                commands.entity(output.entity).despawn();
            }
            commands.entity(task_entity).despawn();
            continue;
        };

        while published < GENERATION_PUBLISH_BUDGET_PER_FRAME {
            let Some(mut output) = generation.ready.pop_front() else {
                break;
            };

            // Demand can migrate or disappear while aggregate work is in flight. Never
            // resurrect a base chunk whose canonical reservation was retired.
            if world.chunk_entity(output.address) != Some(output.entity) {
                continue;
            }

            catch_up_generated_chunk(
                world,
                output.address,
                output.applied_edit_count,
                &mut output.chunk,
            );
            perf.record_generation(output.generation_micros);
            commands.entity(output.entity).insert(output.chunk);
            published += 1;
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

/// Materialization entities are roots so their Transform is directly projected
/// into the bounded runtime frame. This cleanup supplies lifecycle ownership
/// without reintroducing transform inheritance from a potentially far-away
/// `VoxelWorld` root.
pub(crate) fn retire_orphaned_chunks(
    mut commands: Commands,
    worlds: Query<(), With<VoxelWorld>>,
    chunks: Query<(Entity, &VoxelChunkOf)>,
    aggregate_tasks: Query<(Entity, &VoxelAggregateGenerationTask)>,
) {
    for (entity, chunk_of) in &chunks {
        if worlds.get(chunk_of.world).is_err() {
            commands.entity(entity).despawn();
        }
    }
    for (entity, task) in &aggregate_tasks {
        if worlds.get(task.world).is_err() {
            commands.entity(entity).despawn();
        }
    }
}

pub(crate) fn stream_voxel_chunks(
    mut commands: Commands,
    active: Res<UsfActiveScaleLayer>,
    layer_frames: Res<UsfScaleLayerFrames>,
    demand_snapshot: Res<SpatialDemandSnapshot>,
    voxel_demand_sources: Query<(), With<VoxelMaterializationDemand>>,
    mut worlds: Query<(Entity, &mut VoxelWorld, &mut VoxelStreaming, &UsfScaleLayer)>,
    generation_tasks: Query<(), With<VoxelAggregateGenerationTask>>,
    mut perf: ResMut<VoxelPerfStats>,
) {
    let all_voxel_demands = demand_snapshot
        .iter()
        .filter(|scope| voxel_demand_sources.contains(scope.source()))
        .collect::<Vec<_>>();

    let mut generation_slots =
        per_stage_in_flight_limit().saturating_sub(generation_tasks.iter().count());

    for (world_entity, mut world, mut streaming, layer) in &mut worlds {
        let voxel_demands = if layer.scale() == active.scale() {
            all_voxel_demands
                .iter()
                .copied()
                .filter(|demand| demand.scale() == layer.scale())
                .map(|demand| {
                    SpatialDemandScope::at_scale(
                        demand.source(),
                        demand.scale(),
                        demand.center(),
                        demand
                            .half_extent_native()
                            .min(Vec3::splat(FINE_HALF_EXTENT_NATIVE)),
                        demand.priority(),
                    )
                })
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };
        let changed = match refresh_demand_plan(&world, &voxel_demands, &mut streaming, &mut perf) {
            Ok(changed) => changed,
            Err(_) => {
                error!("voxel spatial demand could not be represented canonically");
                continue;
            }
        };

        if changed {
            let stale = world
                .chunk_entries()
                .filter(|(address, _)| !streaming.cached_desired_set.contains(address))
                .collect::<Vec<_>>();
            for (address, entity) in stale {
                world.remove_chunk(address);
                commands.entity(entity).despawn();
            }
        }

        let projection_bound = voxel_demands
            .iter()
            .filter_map(|demand| {
                demand
                    .center()
                    .relative_native_bounded(world.origin(), 1_000_000.0)
                    .ok()
                    .map(|local| {
                        local.abs().max_element()
                            + demand.half_extent_native().max_element()
                            + MATERIALIZATION_CHUNK_SIZE as f32 * 2.0
                            + 1.0
                    })
            })
            .fold(512.0_f32, f32::max);

        let load_budget = streaming.load_budget_per_frame;
        let material = streaming.material.clone();
        let mut requested = 0;
        let mut aggregate_batches = Vec::<PendingAggregateGeneration>::new();
        while requested < load_budget && generation_slots > 0 {
            let Some(demanded) = streaming.pending_desired.pop_front() else {
                break;
            };
            let address = demanded.address;
            if world.chunk_entity(address).is_some() {
                continue;
            }

            let world_origin = VoxelQueryPosition::new(*world.origin());
            let Ok(absolute_translation) = address
                .query_origin()
                .relative_to(world_origin, projection_bound)
            else {
                // Demand remains semantic even if this first fixed-scale runtime
                // projection cannot comfortably represent it. M8 generalizes
                // simultaneous manifestations/projections rather than turning
                // canonical demand into a giant runtime coordinate.
                continue;
            };
            let absolute_translation = bevy::math::DVec3::new(
                absolute_translation.x as f64,
                absolute_translation.y as f64,
                absolute_translation.z as f64,
            );
            let local_translation =
                layer_frames.runtime_from_absolute(layer.scale(), absolute_translation);

            let Ok(scope) = VoxelMaterializationAggregateScope::containing(
                &world,
                address,
                FIELD_GENERATION_AGGREGATE_EXTENT,
            ) else {
                error!(
                    ?address,
                    "voxel aggregate scope could not be derived canonically"
                );
                continue;
            };
            if !generation_batch_can_accept(&aggregate_batches, scope, generation_slots) {
                streaming.pending_desired.push_front(demanded);
                break;
            }

            let recipe = world.chunk_recipe(address);
            let chunk_entity = commands
                .spawn((
                    Name::new(format!(
                        "Voxel Materialization Chunk [priority {}, distance {:.1}]",
                        demanded.priority,
                        demanded.distance_squared.sqrt(),
                    )),
                    VoxelChunkOf::new(world_entity),
                    address,
                    *layer,
                    VoxelChunkPhysicsLod::default(),
                    Transform::from_translation(local_translation),
                    Visibility::Inherited,
                ))
                .id();

            let presentation_entity = commands
                .spawn((
                    Name::new(format!("Voxel Chunk {} Presentation", layer.scale())),
                    ChildOf(chunk_entity),
                    UsfLocalScalePresentation::new(layer.scale()),
                    MeshMaterial3d(material.clone()),
                    Transform::IDENTITY,
                    Visibility::Inherited,
                ))
                .id();
            commands
                .entity(chunk_entity)
                .insert(VoxelChunkPresentation(presentation_entity));

            assert!(world.insert_chunk(address, chunk_entity).is_none());
            push_generation_job(
                &mut aggregate_batches,
                scope,
                VoxelGenerationJob {
                    entity: chunk_entity,
                    address,
                    recipe,
                },
            );

            requested += 1;
        }

        for batch in aggregate_batches {
            let native_extent = batch.scope.extent().native_units_per_axis();
            commands.spawn((
                Name::new(format!(
                    "Voxel Aggregate Generation {native_extent}³ ({} chunks)",
                    batch.jobs.len()
                )),
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
        .filter(|chunk| world.chunk_entity(chunk.address).is_none())
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
        let center = VoxelQueryPosition::new(demand.center());
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
        let center = VoxelQueryPosition::new(demand.center());
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
) -> bool {
    batches.iter().any(|batch| {
        batch.scope == scope && batch.jobs.len() < MAX_CHUNKS_PER_AGGREGATE_GENERATION_TASK
    }) || batches.len() < max_batches
}

fn push_generation_job(
    batches: &mut Vec<PendingAggregateGeneration>,
    scope: VoxelMaterializationAggregateScope,
    job: VoxelGenerationJob,
) {
    if let Some(batch) = batches.iter_mut().find(|batch| {
        batch.scope == scope && batch.jobs.len() < MAX_CHUNKS_PER_AGGREGATE_GENERATION_TASK
    }) {
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
