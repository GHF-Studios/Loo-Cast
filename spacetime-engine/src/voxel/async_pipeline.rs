//! Asynchronous reconstruction of disposable voxel render and physics caches.
//!
//! Dense voxel chunks remain authoritative working data. Geometry dirtiness and
//! collider presence are separate derived-cache concerns: collider-only changes
//! may re-extract a surface for Avian, but they must never churn the render mesh.

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VoxelDerivedPurpose {
    Geometry,
    ColliderOnly,
}

struct VoxelDerivedOutput {
    render_surface: Option<VoxelSurface>,
    collider: Option<Collider>,
    collider_requested: bool,
    geometry_rebuilt: bool,
    debug_color: [f32; 4],
    build_micros: u64,
}

/// One in-flight derived-cache reconstruction for a particular chunk revision.
#[derive(Component)]
pub(crate) struct VoxelDerivedTask {
    revision: u64,
    pub(crate) purpose: VoxelDerivedPurpose,
    task: Task<VoxelDerivedOutput>,
}

#[derive(Debug, Clone, Copy)]
struct BuildCandidate {
    entity: Entity,
    wants_collider: bool,
    geometry_dirty: bool,
    distance_squared: f32,
    priority: u8,
}

/// Returns the squared distance from the observer to the nearest point on this
/// chunk AABB when the chunk belongs in the active physics bubble.
///
/// `None` means collider presence is intentionally not requested.
pub(crate) fn collider_proximity_squared(
    view: &UsfViewFrame,
    address: &VoxelMaterializationChunkAddress,
    layer: &UsfScaleLayer,
) -> Option<f32> {
    if layer.scale() != view.dominant_scale() {
        return None;
    }

    let minimum = address
        .origin()
        .relative_native_bounded(
            view.anchor(),
            PHYSICS_INTERACTION_RADIUS_NATIVE + super::MATERIALIZATION_CHUNK_SIZE as f32 * 2.0,
        )
        .ok()?;
    let maximum = minimum + Vec3::splat(super::MATERIALIZATION_CHUNK_SIZE as f32);
    let nearest = Vec3::new(
        0.0_f32.clamp(minimum.x, maximum.x),
        0.0_f32.clamp(minimum.y, maximum.y),
        0.0_f32.clamp(minimum.z, maximum.z),
    );
    let distance_squared = nearest.length_squared();
    (distance_squared <= PHYSICS_INTERACTION_RADIUS_NATIVE * PHYSICS_INTERACTION_RADIUS_NATIVE)
        .then_some(distance_squared)
}

/// Polls completed worker tasks and publishes only results that still correspond
/// to the authoritative chunk revision.
pub(crate) fn publish_completed_chunk_builds(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    presentation_meshes: Query<&Mesh3d>,
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
            render_surface,
            collider,
            collider_requested,
            geometry_rebuilt,
            debug_color,
            build_micros,
        } = output;
        perf.record_derived(build_micros);

        // Collider-only eligibility changes must preserve render mesh identity.
        // For genuine geometry rebuilds, mutate the existing Mesh asset in place
        // whenever possible instead of allocating a fresh handle every time.
        if let Some(surface) = render_surface {
            let has_surface = !surface.positions.is_empty() && !surface.indices.is_empty();
            if has_surface {
                let replacement = surface_into_mesh(surface, debug_color);
                if let Ok(mesh) = presentation_meshes.get(presentation.0)
                    && let Some(mut existing) = meshes.get_mut(&mesh.0)
                {
                    *existing = replacement;
                } else {
                    commands
                        .entity(presentation.0)
                        .insert(Mesh3d(meshes.add(replacement)));
                }
            } else {
                commands.entity(presentation.0).remove::<Mesh3d>();
            }
        }

        let collider_ready = collider.is_some();
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
        // Preserve enough state to distinguish pending work, a completed build
        // that produced no collider, and a collider component that disappeared
        // after a successful build.
        physics_lod.requested = collider_requested;
        physics_lod.built_revision = collider_requested.then_some(built_revision);
        physics_lod.collider_ready = collider_requested && collider_ready;

        entity_commands.remove::<VoxelDerivedTask>();
        if geometry_rebuilt {
            chunk.mark_meshed();
        }
        published += 1;
    }
}

/// Starts a bounded number of expensive derived-cache builds.
///
/// Selection is explicit rather than ECS-iteration-order dependent:
/// 1. chunks that just became collision-critical,
/// 2. dirty geometry inside the physics bubble,
/// 3. ordinary dirty render geometry.
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
            Option<&Collider>,
        ),
        Without<VoxelDerivedTask>,
    >,
    in_flight: Query<(), With<VoxelDerivedTask>>,
    mut perf: ResMut<VoxelPerfStats>,
) {
    let pool = AsyncComputeTaskPool::get();
    let mut candidates = Vec::<BuildCandidate>::new();

    // First perform cheap synchronous state cleanup and collect all expensive
    // candidates. This still runs even when worker capacity is saturated, so
    // leaving the physics bubble removes stale colliders promptly.
    for (entity, mut chunk, presentation, address, layer, mut physics_lod, collider) in &mut chunks
    {
        let proximity = collider_proximity_squared(&view, address, layer);
        let wants_collider = proximity.is_some();

        if !wants_collider {
            if physics_lod.requested || collider.is_some() {
                let mut entity_commands = commands.entity(entity);
                entity_commands.remove::<RigidBody>();
                entity_commands.remove::<Collider>();
                entity_commands.remove::<CollisionMargin>();
            }
            *physics_lod = VoxelChunkPhysicsLod::default();
        }

        let geometry_dirty = chunk.needs_remesh();
        let collider_dirty = wants_collider
            && (!physics_lod.requested
                || physics_lod.built_revision != Some(chunk.revision())
                || (physics_lod.collider_ready && collider.is_none()));
        if !geometry_dirty && !collider_dirty {
            continue;
        }

        if !chunk.has_surface_transition() {
            if geometry_dirty {
                commands.entity(presentation.0).remove::<Mesh3d>();
                chunk.mark_meshed();
            }
            let mut entity_commands = commands.entity(entity);
            entity_commands.remove::<RigidBody>();
            entity_commands.remove::<Collider>();
            entity_commands.remove::<CollisionMargin>();
            physics_lod.requested = wants_collider;
            physics_lod.built_revision = wants_collider.then_some(chunk.revision());
            physics_lod.collider_ready = false;
            perf.record_uniform_shortcut();
            continue;
        }

        let priority = if collider_dirty {
            0
        } else if wants_collider {
            1
        } else {
            2
        };
        candidates.push(BuildCandidate {
            entity,
            wants_collider,
            geometry_dirty,
            distance_squared: proximity.unwrap_or(f32::INFINITY),
            priority,
        });
    }

    let available = per_stage_in_flight_limit().saturating_sub(in_flight.iter().count());
    if available == 0 || candidates.is_empty() {
        return;
    }

    candidates.sort_by(|a, b| {
        a.priority
            .cmp(&b.priority)
            .then_with(|| a.distance_squared.total_cmp(&b.distance_squared))
    });

    let mut started = 0;
    for candidate in candidates {
        if started >= DERIVED_TASK_START_BUDGET_PER_FRAME || started >= available {
            break;
        }

        let Ok((entity, chunk, presentation, address, layer, _physics_lod, _collider)) =
            chunks.get_mut(candidate.entity)
        else {
            continue;
        };

        // Never expose an attribute-less placeholder mesh while the first
        // asynchronous geometry build is in flight. Existing valid meshes stay
        // visible during later revision rebuilds and all collider-only rebuilds.
        if candidate.geometry_dirty && chunk.meshed_revision().is_none() {
            commands.entity(presentation.0).remove::<Mesh3d>();
        }

        let revision = chunk.revision();
        let debug_color = debug_chunk_color(*address, layer.scale());
        let snapshot = chunk.clone();
        let wants_collider = candidate.wants_collider;
        let geometry_rebuilt = candidate.geometry_dirty;
        let purpose = if geometry_rebuilt {
            VoxelDerivedPurpose::Geometry
        } else {
            VoxelDerivedPurpose::ColliderOnly
        };
        let task = pool.spawn(async move {
            let started_at = Instant::now();
            let surface = mesh::extract_chunk_surface(&snapshot);
            let collider = if wants_collider {
                physics::build_chunk_collider(&snapshot, &surface)
            } else {
                None
            };
            let render_surface = geometry_rebuilt.then_some(surface);
            VoxelDerivedOutput {
                render_surface,
                collider,
                collider_requested: wants_collider,
                geometry_rebuilt,
                debug_color,
                build_micros: started_at.elapsed().as_micros() as u64,
            }
        });

        commands.entity(entity).insert(VoxelDerivedTask {
            revision,
            purpose,
            task,
        });
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
