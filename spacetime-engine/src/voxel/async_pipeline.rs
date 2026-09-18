//! Asynchronous reconstruction of disposable voxel render and physics caches.
//!
//! This deliberately lives beside the synchronous `mesh` module rather than
//! rewriting it. Dense voxel chunks remain authoritative working data; this
//! module snapshots a chunk revision, performs surface extraction + collider
//! construction on Bevy's async compute pool, then publishes the result only if
//! that revision is still current.

use std::time::Instant;

use avian3d::prelude::{Collider, CollisionMargin, RigidBody};
use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task, futures::check_ready},
};

use crate::spatial::{SPATIAL_SCALE_MAX, SpatialScale, UsfPosition, UsfScaleLayer, UsfViewFrame};

use super::{
    VoxelChunk, VoxelChunkPhysicsLod, VoxelChunkPresentation, VoxelMaterializationChunkAddress,
    mesh::{self, VoxelSurface},
    perf::{VoxelPerfStats, per_stage_in_flight_limit},
    physics,
};

const DERIVED_TASK_START_BUDGET_PER_FRAME: usize = 8;
const DERIVED_PUBLISH_BUDGET_PER_FRAME: usize = 8;
/// Physics activation is independent of visual resolution. This is only a
/// local interaction bubble over the exact same scale-native geometry.
const PHYSICS_INTERACTION_RADIUS_NATIVE: f32 = 32.0;

struct VoxelDerivedOutput {
    surface: Option<VoxelSurface>,
    collider: Option<Collider>,
    collider_requested: bool,
    debug_color: [f32; 4],
    build_micros: u64,
}

/// One in-flight render/physics reconstruction for a particular chunk revision.
#[derive(Component)]
pub(crate) struct VoxelDerivedTask {
    revision: u64,
    task: Task<VoxelDerivedOutput>,
}

/// Polls completed worker tasks and publishes only results that still correspond
/// to the authoritative chunk revision.
pub(crate) fn publish_completed_chunk_builds(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut chunks: Query<(
        Entity,
        &mut VoxelChunk,
        &mut VoxelDerivedTask,
        &VoxelChunkPresentation,
        &mut VoxelChunkPhysicsLod,
    )>,
    mut perf: ResMut<VoxelPerfStats>,
) {
    let mut published = 0;

    for (entity, mut chunk, mut build, presentation, mut physics_lod) in &mut chunks {
        if published >= DERIVED_PUBLISH_BUDGET_PER_FRAME {
            break;
        }

        let Some(output) = check_ready(&mut build.task) else {
            continue;
        };
        let built_revision = build.revision;

        // Never let a slow worker overwrite a newer edit. Removing the finished
        // task leaves the chunk dirty, so a fresh build can be queued next frame.
        if chunk.revision() != built_revision {
            commands.entity(entity).remove::<VoxelDerivedTask>();
            published += 1;
            continue;
        }

        let VoxelDerivedOutput {
            surface,
            collider,
            collider_requested,
            debug_color,
            build_micros,
        } = output;
        perf.record_derived(build_micros);

        if let Some(surface) = surface {
            let has_surface = !surface.positions.is_empty() && !surface.indices.is_empty();
            if has_surface {
                commands
                    .entity(presentation.0)
                    .insert(Mesh3d(meshes.add(surface_into_mesh(surface, debug_color))));
            } else {
                commands.entity(presentation.0).remove::<Mesh3d>();
            }
            chunk.mark_meshed();
        }

        let mut entity_commands = commands.entity(entity);
        if let Some(collider) = collider {
            entity_commands.insert((
                RigidBody::Static,
                collider,
                CollisionMargin(physics::VOXEL_COLLISION_MARGIN),
            ));
        } else {
            entity_commands.remove::<RigidBody>();
            entity_commands.remove::<Collider>();
            entity_commands.remove::<CollisionMargin>();
        }
        physics_lod.0 = collider_requested;

        entity_commands.remove::<VoxelDerivedTask>();
        published += 1;
    }
}

/// Starts a bounded number of expensive derived-cache builds. Existing render
/// and physics representations remain live while an edited replacement is being
/// computed in the background.
pub(crate) fn queue_dirty_chunk_builds(
    mut commands: Commands,
    view: Res<UsfViewFrame>,
    mut chunks: Query<
        (
            Entity,
            &mut VoxelChunk,
            &VoxelChunkPresentation,
            &VoxelMaterializationChunkAddress,
            &UsfScaleLayer,
            &mut VoxelChunkPhysicsLod,
        ),
        Without<VoxelDerivedTask>,
    >,
    in_flight: Query<(), With<VoxelDerivedTask>>,
    mut perf: ResMut<VoxelPerfStats>,
) {
    let pool = AsyncComputeTaskPool::get();
    let mut started = 0;
    let available = per_stage_in_flight_limit().saturating_sub(in_flight.iter().count());
    if available == 0 {
        return;
    }

    for (entity, mut chunk, presentation, address, layer, mut physics_lod) in &mut chunks {
        if started >= DERIVED_TASK_START_BUDGET_PER_FRAME || started >= available {
            break;
        }

        let wants_collider = layer.scale() == view.dominant_scale()
            && address
                .origin()
                .relative_native_bounded(
                    view.anchor(),
                    PHYSICS_INTERACTION_RADIUS_NATIVE
                        + super::MATERIALIZATION_CHUNK_SIZE as f32 * 2.0,
                )
                .map(|minimum| {
                    let maximum = minimum + Vec3::splat(super::MATERIALIZATION_CHUNK_SIZE as f32);
                    let nearest = Vec3::new(
                        0.0_f32.clamp(minimum.x, maximum.x),
                        0.0_f32.clamp(minimum.y, maximum.y),
                        0.0_f32.clamp(minimum.z, maximum.z),
                    );
                    nearest.length_squared()
                        <= PHYSICS_INTERACTION_RADIUS_NATIVE * PHYSICS_INTERACTION_RADIUS_NATIVE
                })
                .unwrap_or(false);

        if !wants_collider && physics_lod.0 {
            let mut entity_commands = commands.entity(entity);
            entity_commands.remove::<RigidBody>();
            entity_commands.remove::<Collider>();
            entity_commands.remove::<CollisionMargin>();
            physics_lod.0 = false;
            if !chunk.needs_remesh() {
                continue;
            }
        }

        if !chunk.needs_remesh() && physics_lod.0 == wants_collider {
            continue;
        }

        if !chunk.has_surface_transition() {
            commands.entity(presentation.0).remove::<Mesh3d>();
            let mut entity_commands = commands.entity(entity);
            entity_commands.remove::<RigidBody>();
            entity_commands.remove::<Collider>();
            entity_commands.remove::<CollisionMargin>();
            chunk.mark_meshed();
            physics_lod.0 = wants_collider;
            perf.record_uniform_shortcut();
            continue;
        }

        // Never expose an attribute-less placeholder mesh while the first
        // asynchronous derived build is in flight. Existing valid meshes stay
        // visible during later revision rebuilds.
        if chunk.meshed_revision().is_none() {
            commands.entity(presentation.0).remove::<Mesh3d>();
        }

        let revision = chunk.revision();
        let mesh_requested = chunk.needs_remesh();
        let debug_color = debug_chunk_color(*address, layer.scale());

        // M5 intentionally pays for a simple immutable snapshot instead of
        // introducing shared/COW voxel storage prematurely. This clones distance
        // and material arrays; a later profiling pass can shrink that to exactly
        // the fields each worker stage consumes.
        let snapshot = chunk.clone();
        let task = pool.spawn(async move {
            let started_at = Instant::now();
            let surface = mesh::extract_chunk_surface(&snapshot);
            let collider = if wants_collider {
                physics::build_chunk_collider(&snapshot, &surface)
            } else {
                None
            };
            VoxelDerivedOutput {
                surface: mesh_requested.then_some(surface),
                collider,
                collider_requested: wants_collider,
                debug_color,
                build_micros: started_at.elapsed().as_micros() as u64,
            }
        });

        commands
            .entity(entity)
            .insert(VoxelDerivedTask { revision, task });
        started += 1;
    }
}

fn debug_chunk_color(address: VoxelMaterializationChunkAddress, scale: SpatialScale) -> [f32; 4] {
    let origin = *address.origin();
    let chart_zero = UsfPosition::zero(scale);
    let mut color = Vec3::splat(0.72);
    let mut initialized = false;

    for raw in (scale.exponent()..=SPATIAL_SCALE_MAX).rev() {
        let level = SpatialScale::new(raw).expect("validated debug color scale");
        let relative = origin
            .relative_at_scale_bounded(&chart_zero, level, 1_000_000.0)
            .unwrap_or(Vec3::ZERO);
        let cell = (relative / 10.0).floor().as_ivec3();
        let hash = debug_hash(cell, level);
        let candidate = Vec3::new(
            0.32 + ((hash & 0xFF) as f32 / 255.0) * 0.62,
            0.32 + (((hash >> 8) & 0xFF) as f32 / 255.0) * 0.62,
            0.32 + (((hash >> 16) & 0xFF) as f32 / 255.0) * 0.62,
        );

        if initialized {
            color = color.lerp(candidate, 0.20);
        } else {
            color = candidate;
            initialized = true;
        }
    }

    [color.x, color.y, color.z, 1.0]
}

fn debug_hash(cell: IVec3, scale: SpatialScale) -> u32 {
    let mut value = (scale.exponent() as i32 as u32).wrapping_mul(0x9E37_79B9);
    for component in [cell.x, cell.y, cell.z] {
        value ^= (component as u32).wrapping_mul(0x85EB_CA6B);
        value ^= value >> 16;
        value = value.wrapping_mul(0x7FEB_352D);
        value ^= value >> 15;
    }
    value
}

fn surface_into_mesh(surface: VoxelSurface, debug_color: [f32; 4]) -> Mesh {
    let VoxelSurface {
        positions,
        normals,
        uvs,
        tangents,
        indices,
    } = surface;
    let colors = vec![debug_color; positions.len()];

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_attribute(Mesh::ATTRIBUTE_TANGENT, tangents)
    .with_inserted_indices(Indices::U32(indices))
}
