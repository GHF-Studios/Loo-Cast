//! Asynchronous reconstruction of disposable voxel render and physics caches.
//!
//! This deliberately lives beside the synchronous `mesh` module rather than
//! rewriting it. Dense voxel chunks remain authoritative working data; this
//! module snapshots a chunk revision, performs surface extraction + collider
//! construction on Bevy's async compute pool, then publishes the result only if
//! that revision is still current.

use avian3d::prelude::{Collider, CollisionMargin, RigidBody};
use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task, futures::check_ready},
};

use crate::spatial::UsfViewFrame;

use super::{
    VoxelChunk, VoxelChunkPhysicsLod, VoxelChunkPresentation, VoxelMaterializationChunkAddress,
    mesh::{self, VoxelSurface},
    physics,
};

const DERIVED_TASK_START_BUDGET_PER_FRAME: usize = 16;
const DERIVED_PUBLISH_BUDGET_PER_FRAME: usize = 8;
const PHYSICS_LOD_RADIUS_NATIVE: f32 = 48.0;

struct VoxelDerivedOutput {
    surface: VoxelSurface,
    collider: Option<Collider>,
    collider_requested: bool,
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
        } = output;
        let has_surface = !surface.positions.is_empty() && !surface.indices.is_empty();

        if has_surface {
            commands
                .entity(presentation.0)
                .insert(Mesh3d(meshes.add(surface_into_mesh(surface))));
        } else {
            commands.entity(presentation.0).remove::<Mesh3d>();
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
        chunk.mark_meshed();
        published += 1;
    }
}

/// Starts a bounded number of expensive derived-cache builds. Existing render
/// and physics representations remain live while an edited replacement is being
/// computed in the background.
pub(crate) fn queue_dirty_chunk_builds(
    mut commands: Commands,
    view: Res<UsfViewFrame>,
    chunks: Query<
        (
            Entity,
            &VoxelChunk,
            &VoxelChunkPresentation,
            &VoxelMaterializationChunkAddress,
            &VoxelChunkPhysicsLod,
        ),
        Without<VoxelDerivedTask>,
    >,
) {
    let pool = AsyncComputeTaskPool::get();
    let mut started = 0;

    for (entity, chunk, presentation, address, physics_lod) in &chunks {
        if started >= DERIVED_TASK_START_BUDGET_PER_FRAME {
            break;
        }

        let wants_collider = address
            .origin()
            .relative_native_bounded(view.anchor(), PHYSICS_LOD_RADIUS_NATIVE + 24.0)
            .map(|delta| {
                let center = delta + Vec3::splat(super::MATERIALIZATION_CHUNK_SIZE as f32 * 0.5);
                center.length() <= PHYSICS_LOD_RADIUS_NATIVE
            })
            .unwrap_or(false);

        if !chunk.needs_remesh() && physics_lod.0 == wants_collider {
            continue;
        }

        // Never expose an attribute-less placeholder mesh while the first
        // asynchronous derived build is in flight. Existing valid meshes stay
        // visible during later revision rebuilds.
        if chunk.meshed_revision().is_none() {
            commands.entity(presentation.0).remove::<Mesh3d>();
        }

        let revision = chunk.revision();

        // M5 intentionally pays for a simple immutable snapshot instead of
        // introducing shared/COW voxel storage prematurely. This clones distance
        // and material arrays; a later profiling pass can shrink that to exactly
        // the fields each worker stage consumes.
        let snapshot = chunk.clone();
        let task = pool.spawn(async move {
            let surface = mesh::extract_chunk_surface(&snapshot);
            let collider = if wants_collider {
                physics::build_chunk_collider(&snapshot, &surface)
            } else {
                None
            };
            VoxelDerivedOutput {
                surface,
                collider,
                collider_requested: wants_collider,
            }
        });

        commands
            .entity(entity)
            .insert(VoxelDerivedTask { revision, task });
        started += 1;
    }
}

fn surface_into_mesh(surface: VoxelSurface) -> Mesh {
    let VoxelSurface {
        positions,
        normals,
        uvs,
        tangents,
        indices,
    } = surface;

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_attribute(Mesh::ATTRIBUTE_TANGENT, tangents)
    .with_inserted_indices(Indices::U32(indices))
}
