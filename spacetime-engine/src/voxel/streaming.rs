//! Viewer-centered materialization of dense voxel chunk caches.
//!
//! Streaming is optional per [`VoxelWorld`]. The authoritative world remains
//! procedural base + sparse semantic modifications; this module only decides
//! which disposable dense base materializations should exist around a viewer.

use std::collections::{HashSet, VecDeque};

use bevy::{
    camera::visibility::NoFrustumCulling,
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task, futures::check_ready},
};

use crate::spatial::UsfSpatialFrame;

use super::{
    MATERIALIZATION_CHUNK_SIZE, VoxelChunk, VoxelChunkOf, VoxelMaterializationChunkAddress,
    VoxelQueryPosition, VoxelWorld,
    aggregate::{VoxelMaterializationAggregateExtent, VoxelMaterializationAggregateScope},
    empty_voxel_mesh, world::VoxelChunkRecipe,
};

/// Maximum number of finished base materializations published into ECS in one frame.
const GENERATION_PUBLISH_BUDGET_PER_FRAME: usize = 8;

/// Current voxel-field generation work scope. This is deliberately a scheduler
/// choice, not materialization identity; `1000³` alignment is supported by the
/// same aggregate-scope primitive without forcing this subsystem to use it.
const FIELD_GENERATION_AGGREGATE_EXTENT: VoxelMaterializationAggregateExtent =
    VoxelMaterializationAggregateExtent::HUNDRED;

/// Keep first-materialization latency reasonable while still proving that one
/// worker item can process several individually addressable base chunks. Larger
/// batches remain a future policy/performance choice.
const MAX_CHUNKS_PER_AGGREGATE_GENERATION_TASK: usize = 4;

/// Opt-in policy for keeping base materializations around one viewer.
///
/// `radius` is a bounded count of nearby `10³` base chunks, not a global chunk
/// coordinate and not spatial-demand authority. M7.2 will replace the hard-coded
/// viewer policy with merged canonical demand sources.
#[derive(Component, Debug, Clone)]
pub struct VoxelStreaming {
    viewer: Entity,
    radius: IVec3,
    load_budget_per_frame: usize,
    material: Handle<StandardMaterial>,
}

impl VoxelStreaming {
    pub fn new(
        viewer: Entity,
        radius: IVec3,
        load_budget_per_frame: usize,
        material: Handle<StandardMaterial>,
    ) -> Self {
        Self {
            viewer,
            radius: radius.max(IVec3::ZERO),
            load_budget_per_frame: load_budget_per_frame.max(1),
            material,
        }
    }

    pub const fn viewer(&self) -> Entity {
        self.viewer
    }

    pub const fn radius(&self) -> IVec3 {
        self.radius
    }

    pub const fn load_budget_per_frame(&self) -> usize {
        self.load_budget_per_frame
    }
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
            jobs
                .into_iter()
                .map(|job| {
                    let applied_edit_count = job.recipe.applied_edit_count();
                    VoxelGeneratedChunk {
                        entity: job.entity,
                        address: job.address,
                        applied_edit_count,
                        chunk: job.recipe.materialize(),
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

            // The viewer can move while aggregate work is in flight. Never
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
    mut meshes: ResMut<Assets<Mesh>>,
    frame: Res<UsfSpatialFrame>,
    viewers: Query<&GlobalTransform>,
    mut worlds: Query<(Entity, &mut VoxelWorld, &VoxelStreaming)>,
) {
    let frame_origin = VoxelQueryPosition::new(*frame.origin());

    for (world_entity, mut world, streaming) in &mut worlds {
        let Ok(viewer) = viewers.get(streaming.viewer) else {
            continue;
        };
        let viewer_local = viewer.translation();
        let Ok(viewer_semantic) = frame_origin.translated(viewer_local) else {
            error!(?viewer_local, "voxel viewer semantic projection failed");
            continue;
        };
        let Ok(center) = world.materialization_address_containing(viewer_semantic) else {
            error!("voxel viewer uses an incompatible semantic leaf scale");
            continue;
        };
        let Ok(desired) = desired_chunk_addresses(center, streaming.radius) else {
            error!("voxel streaming window could not be represented canonically");
            continue;
        };
        let desired_set = desired
            .iter()
            .map(|(_, address)| *address)
            .collect::<HashSet<_>>();

        let stale = world
            .chunk_entries()
            .filter(|(address, _)| !desired_set.contains(address))
            .collect::<Vec<_>>();

        for (address, entity) in stale {
            world.remove_chunk(address);
            commands.entity(entity).despawn();
        }

        let radius_max = streaming.radius.max(IVec3::ZERO).max_element() as f32;
        let projection_bound = viewer_local.abs().max_element()
            + (radius_max + 2.0) * MATERIALIZATION_CHUNK_SIZE as f32
            + 1.0;

        let mut requested = 0;
        let mut aggregate_batches = Vec::<PendingAggregateGeneration>::new();
        for (offset, address) in desired {
            if world.chunk_entity(address).is_some() {
                continue;
            }

            let Ok(local_translation) = address
                .query_origin()
                .relative_to(frame_origin, projection_bound)
            else {
                error!(?address, "nearby voxel materialization could not project into local frame");
                continue;
            };
            let Ok(scope) = VoxelMaterializationAggregateScope::containing(
                &world,
                address,
                FIELD_GENERATION_AGGREGATE_EXTENT,
            ) else {
                error!(?address, "voxel aggregate scope could not be derived canonically");
                continue;
            };
            let recipe = world.chunk_recipe(address);
            let chunk_entity = commands
                .spawn((
                    Name::new(format!(
                        "Voxel Materialization Chunk [{:+}, {:+}, {:+}]",
                        offset.x, offset.y, offset.z
                    )),
                    VoxelChunkOf::new(world_entity),
                    address,
                    Mesh3d(meshes.add(empty_voxel_mesh())),
                    MeshMaterial3d(streaming.material.clone()),
                    NoFrustumCulling,
                    Transform::from_translation(local_translation),
                ))
                .id();

            // Generation is asynchronous, so the old attribute-less placeholder
            // could otherwise survive into render extraction for several frames.
            commands.entity(chunk_entity).remove::<Mesh3d>();
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
            if requested >= streaming.load_budget_per_frame {
                break;
            }
        }

        for batch in aggregate_batches {
            let native_extent = batch.scope.extent().native_units_per_axis();
            commands.spawn((
                Name::new(format!(
                    "Voxel Aggregate Generation {native_extent}³ ({} chunks)",
                    batch.jobs.len()
                )),
                VoxelAggregateGenerationTask::spawn(
                    world_entity,
                    batch.scope,
                    batch.jobs,
                ),
            ));
        }
    }
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

fn desired_chunk_addresses(
    center: VoxelMaterializationChunkAddress,
    radius: IVec3,
) -> Result<Vec<(IVec3, VoxelMaterializationChunkAddress)>, crate::spatial::UsfPositionError> {
    let radius = radius.max(IVec3::ZERO);
    // The radius is policy input rather than semantic identity. Avoid doing its
    // capacity arithmetic in i32 so even malformed/extreme policy values cannot
    // overflow before the bounded-neighborhood iteration itself is considered.
    let mut offsets = Vec::new();

    for z in -radius.z..=radius.z {
        for y in -radius.y..=radius.y {
            for x in -radius.x..=radius.x {
                offsets.push(IVec3::new(x, y, z));
            }
        }
    }
    offsets.sort_by_key(|offset| chunk_distance_squared(*offset));

    offsets
        .into_iter()
        .map(|offset| center.translated_chunks(offset).map(|address| (offset, address)))
        .collect()
}

fn chunk_distance_squared(delta: IVec3) -> i128 {
    let x = i128::from(delta.x);
    let y = i128::from(delta.y);
    let z = i128::from(delta.z);
    x * x + y * y + z * z
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::voxel::{
        VoxelBase, VoxelBrush, VoxelEdit, VoxelMaterialId, VoxelQueryPosition,
    };

    fn query(local: Vec3) -> VoxelQueryPosition {
        VoxelQueryPosition::from_scale0_local(local).unwrap()
    }

    #[test]
    fn streaming_window_is_centered_unique_and_nearest_first() {
        let center = VoxelMaterializationChunkAddress::new(crate::spatial::UsfPosition::default());
        let addresses = desired_chunk_addresses(center, IVec3::new(2, 1, 2)).unwrap();

        assert_eq!(addresses.len(), 75);
        assert_eq!(addresses[0], (IVec3::ZERO, center));
        assert_eq!(
            addresses
                .iter()
                .map(|(_, address)| *address)
                .collect::<HashSet<_>>()
                .len(),
            75
        );
        assert!(addresses.iter().all(|(offset, _)| {
            offset.x.abs() <= 2 && offset.y.abs() <= 1 && offset.z.abs() <= 2
        }));
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
