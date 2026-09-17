//! Coarse volumetric distance-LOD over the same authoritative `VoxelWorld`.
//!
//! A coarse brick stores the same 12^3 sample topology as a fine materialization,
//! but adjacent samples are 10 native units apart. One brick therefore spans
//! 100^3 native units while costing the same sampling/Surface-Nets work as one
//! ordinary 10^3 fine brick. These caches are visual only: edits/physics remain
//! on the fine tier for now.

use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task, futures::check_ready},
};

use crate::spatial::{
    SpatialDemandScope, SpatialDemandSnapshot, UsfLocalScalePresentation, UsfPosition,
    UsfScaleLayer, UsfScaleLayerFrames,
};

use super::{
    MATERIALIZATION_CHUNK_SIZE, VoxelChunk, VoxelChunkOf, VoxelMaterializationDemand,
    VoxelQueryPosition, VoxelStreaming, VoxelWorld,
    mesh::{self, VoxelSurface},
    perf::{VoxelPerfStats, per_stage_in_flight_limit},
};

const COARSE_SAMPLE_SPACING: f32 = 10.0;
const COARSE_CHUNK_EXTENT: f32 = MATERIALIZATION_CHUNK_SIZE as f32 * COARSE_SAMPLE_SPACING;
const COARSE_PUBLISH_BUDGET_PER_FRAME: usize = 4;

#[derive(Component, Debug, Clone, Copy)]
pub(crate) struct VoxelCoarseChunk;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CoarseKey {
    origin: UsfPosition,
}

#[derive(Debug, Clone, Copy)]
struct CoarseDemand {
    key: CoarseKey,
    priority: i32,
    distance_squared: f32,
}

#[derive(Component, Default)]
pub(crate) struct VoxelCoarseState {
    resident: HashMap<CoarseKey, Entity>,
    material: Option<Handle<StandardMaterial>>,
}

struct CoarseOutput {
    surface: Option<VoxelSurface>,
    build_micros: u64,
}

#[derive(Component)]
pub(crate) struct VoxelCoarseTask {
    world: Entity,
    chunk_entity: Entity,
    key: CoarseKey,
    task: Task<CoarseOutput>,
}

pub(crate) fn ensure_coarse_states(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    worlds: Query<(Entity, &VoxelStreaming), (With<VoxelWorld>, Without<VoxelCoarseState>)>,
) {
    for (entity, streaming) in &worlds {
        let mut material = materials
            .get(streaming.material())
            .cloned()
            .unwrap_or_default();
        material.depth_bias = -64.0;

        commands.entity(entity).insert(VoxelCoarseState {
            resident: HashMap::new(),
            material: Some(materials.add(material)),
        });
    }
}

pub(crate) fn stream_coarse_chunks(
    mut commands: Commands,
    layer_frames: Res<UsfScaleLayerFrames>,
    demands: Res<SpatialDemandSnapshot>,
    voxel_sources: Query<(), With<VoxelMaterializationDemand>>,
    mut worlds: Query<(Entity, &VoxelWorld, &UsfScaleLayer, &mut VoxelCoarseState)>,
    in_flight: Query<(), With<VoxelCoarseTask>>,
) {
    let all_demands = demands
        .iter()
        .filter(|scope| voxel_sources.contains(scope.source()))
        .collect::<Vec<_>>();

    let mut available = (per_stage_in_flight_limit() / 2)
        .max(1)
        .saturating_sub(in_flight.iter().count());

    for (world_entity, world, layer, mut state) in &mut worlds {
        let layer_demands = all_demands
            .iter()
            .copied()
            .filter(|demand| demand.scale() == layer.scale())
            .collect::<Vec<_>>();

        let Ok(desired) = desired_coarse_chunks(world, &layer_demands) else {
            continue;
        };
        let desired_set = desired
            .iter()
            .map(|entry| entry.key)
            .collect::<HashSet<_>>();

        let stale = state
            .resident
            .iter()
            .filter_map(|(key, entity)| (!desired_set.contains(key)).then_some((*key, *entity)))
            .collect::<Vec<_>>();
        for (key, entity) in stale {
            state.resident.remove(&key);
            commands.entity(entity).despawn();
        }

        if available == 0 {
            continue;
        }

        for demanded in desired {
            if available == 0 {
                break;
            }
            if state.resident.contains_key(&demanded.key) {
                continue;
            }

            let world_origin = VoxelQueryPosition::new(*world.origin());
            let chunk_origin = VoxelQueryPosition::new(demanded.key.origin);
            let Ok(absolute_translation) = chunk_origin.relative_to(world_origin, 1_000_000.0)
            else {
                continue;
            };
            let local_translation = layer_frames.runtime_from_absolute(
                layer.scale(),
                bevy::math::DVec3::new(
                    absolute_translation.x as f64,
                    absolute_translation.y as f64,
                    absolute_translation.z as f64,
                ),
            );

            let chunk_entity = commands
                .spawn((
                    Name::new(format!(
                        "Voxel Coarse LOD Chunk {} [distance {:.1}]",
                        layer.scale(),
                        demanded.distance_squared.sqrt()
                    )),
                    VoxelCoarseChunk,
                    VoxelChunkOf::new(world_entity),
                    *layer,
                    Transform::from_translation(local_translation),
                    Visibility::Inherited,
                ))
                .id();

            state.resident.insert(demanded.key, chunk_entity);

            let base = world.base();
            let sampler = base.prepare_chunk_sampler(world_origin, chunk_origin);
            let task = AsyncComputeTaskPool::get().spawn(async move {
                let started = Instant::now();
                let chunk = VoxelChunk::generate(|sample_point| {
                    sampler.sample(sample_point * COARSE_SAMPLE_SPACING)
                });
                let surface = chunk
                    .has_surface_transition()
                    .then(|| mesh::extract_chunk_surface(&chunk));
                CoarseOutput {
                    surface,
                    build_micros: started.elapsed().as_micros() as u64,
                }
            });

            commands.spawn((
                Name::new(format!("Voxel Coarse LOD Build {}", layer.scale())),
                VoxelCoarseTask {
                    world: world_entity,
                    chunk_entity,
                    key: demanded.key,
                    task,
                },
            ));

            available -= 1;
        }
    }
}

pub(crate) fn publish_coarse_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut tasks: Query<(Entity, &mut VoxelCoarseTask)>,
    worlds: Query<(&VoxelCoarseState, &UsfScaleLayer), With<VoxelWorld>>,
    mut perf: ResMut<VoxelPerfStats>,
) {
    let mut published = 0;

    for (task_entity, mut build) in &mut tasks {
        if published >= COARSE_PUBLISH_BUDGET_PER_FRAME {
            break;
        }

        let Some(output) = check_ready(&mut build.task) else {
            continue;
        };

        let Ok((state, layer)) = worlds.get(build.world) else {
            commands.entity(build.chunk_entity).despawn();
            commands.entity(task_entity).despawn();
            continue;
        };

        if state.resident.get(&build.key).copied() != Some(build.chunk_entity) {
            commands.entity(build.chunk_entity).despawn();
            commands.entity(task_entity).despawn();
            continue;
        }

        perf.record_coarse(output.build_micros);

        if let (Some(surface), Some(material)) = (output.surface, state.material.as_ref()) {
            if !surface.positions.is_empty() && !surface.indices.is_empty() {
                commands.spawn((
                    Name::new(format!("Voxel Coarse {} Presentation", layer.scale())),
                    ChildOf(build.chunk_entity),
                    UsfLocalScalePresentation::with_authored_scale(
                        layer.scale(),
                        Vec3::splat(COARSE_SAMPLE_SPACING),
                    ),
                    Mesh3d(meshes.add(surface.into_mesh())),
                    MeshMaterial3d(material.clone()),
                    Transform::IDENTITY,
                    Visibility::Inherited,
                ));
            }
        }

        commands.entity(task_entity).despawn();
        published += 1;
    }
}

fn desired_coarse_chunks(
    world: &VoxelWorld,
    demands: &[SpatialDemandScope],
) -> Result<Vec<CoarseDemand>, crate::spatial::UsfPositionError> {
    let mut merged = HashMap::<CoarseKey, CoarseDemand>::new();

    for demand in demands {
        let center = VoxelQueryPosition::new(demand.center());
        let containing = coarse_containing(world, center)?;
        let containing_query = VoxelQueryPosition::new(containing.origin);
        let local_center = center.relative_to(containing_query, COARSE_CHUNK_EXTENT + 0.01)?;
        let half = demand.half_extent_native();

        let minimum = ((local_center - half) / COARSE_CHUNK_EXTENT)
            .floor()
            .as_ivec3();
        let maximum = ((local_center + half) / COARSE_CHUNK_EXTENT)
            .floor()
            .as_ivec3();

        for z in minimum.z..=maximum.z {
            for y in minimum.y..=maximum.y {
                for x in minimum.x..=maximum.x {
                    let offset = IVec3::new(x, y, z);
                    let chunk_center = offset.as_vec3() * COARSE_CHUNK_EXTENT
                        + Vec3::splat(COARSE_CHUNK_EXTENT * 0.5);
                    let distance_squared = (chunk_center - local_center).length_squared();

                    let reach = half.max_element() + COARSE_CHUNK_EXTENT * 0.866_025_4;
                    if distance_squared > reach * reach {
                        continue;
                    }

                    let origin = containing.origin.translated_whole_native([
                        i64::from(x) * COARSE_CHUNK_EXTENT as i64,
                        i64::from(y) * COARSE_CHUNK_EXTENT as i64,
                        i64::from(z) * COARSE_CHUNK_EXTENT as i64,
                    ])?;
                    let key = CoarseKey { origin };
                    let candidate = CoarseDemand {
                        key,
                        priority: demand.priority(),
                        distance_squared,
                    };

                    merged
                        .entry(key)
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

    let mut result = merged.into_values().collect::<Vec<_>>();
    result.sort_by(|a, b| {
        b.priority
            .cmp(&a.priority)
            .then_with(|| a.distance_squared.total_cmp(&b.distance_squared))
    });
    Ok(result)
}

fn coarse_containing(
    world: &VoxelWorld,
    point: VoxelQueryPosition,
) -> Result<CoarseKey, crate::spatial::UsfPositionError> {
    if point.usf().leaf_scale() != world.origin().leaf_scale() {
        return Err(crate::spatial::UsfPositionError::IncompatibleLeafScale);
    }

    let delta = point.usf().offset() - world.origin().offset();
    let remainder = Vec3::new(
        delta.x.rem_euclid(COARSE_CHUNK_EXTENT),
        delta.y.rem_euclid(COARSE_CHUNK_EXTENT),
        delta.z.rem_euclid(COARSE_CHUNK_EXTENT),
    );
    let origin = point.translated(-remainder)?.usf();
    Ok(CoarseKey { origin })
}
