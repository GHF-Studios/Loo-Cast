//! Viewer-centered materialization of dense voxel chunk caches.
//!
//! Streaming is optional per [`VoxelWorld`]. The authoritative world remains
//! procedural base + sparse semantic modifications; this module only decides
//! which disposable dense base materializations should exist around a viewer.

use std::collections::HashSet;

use bevy::{
    camera::visibility::NoFrustumCulling,
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task, futures::check_ready},
};

use crate::spatial::UsfSpatialFrame;

use super::{
    MATERIALIZATION_CHUNK_SIZE, VoxelChunk, VoxelChunkOf, VoxelMaterializationChunkAddress,
    VoxelQueryPosition, VoxelWorld, empty_voxel_mesh, world::VoxelChunkRecipe,
};

/// Maximum number of finished generation tasks published into ECS in one frame.
const GENERATION_PUBLISH_BUDGET_PER_FRAME: usize = 8;

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

/// Background construction of one dense chunk from an immutable semantic
/// snapshot. `applied_edit_count` lets completion catch up with edits recorded
/// while the task was running instead of invalidating and restarting the work.
#[derive(Component)]
pub(crate) struct VoxelChunkGenerationTask {
    applied_edit_count: usize,
    task: Task<VoxelChunk>,
}

impl VoxelChunkGenerationTask {
    fn spawn(recipe: VoxelChunkRecipe) -> Self {
        let applied_edit_count = recipe.applied_edit_count();
        let task = AsyncComputeTaskPool::get().spawn(async move { recipe.materialize() });
        Self {
            applied_edit_count,
            task,
        }
    }
}

pub(crate) fn finish_chunk_generation(
    mut commands: Commands,
    worlds: Query<&VoxelWorld>,
    mut tasks: Query<(
        Entity,
        &VoxelChunkOf,
        &VoxelMaterializationChunkAddress,
        &mut VoxelChunkGenerationTask,
    )>,
) {
    let mut published = 0;

    for (entity, chunk_of, address, mut generation) in &mut tasks {
        if published >= GENERATION_PUBLISH_BUDGET_PER_FRAME {
            break;
        }

        let Some(mut chunk) = check_ready(&mut generation.task) else {
            continue;
        };

        let Ok(world) = worlds.get(chunk_of.world) else {
            commands.entity(entity).despawn();
            continue;
        };

        catch_up_generated_chunk(
            world,
            *address,
            generation.applied_edit_count,
            &mut chunk,
        );
        commands
            .entity(entity)
            .insert(chunk)
            .remove::<VoxelChunkGenerationTask>();
        published += 1;
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
) {
    for (entity, chunk_of) in &chunks {
        if worlds.get(chunk_of.world).is_err() {
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
            let generation = VoxelChunkGenerationTask::spawn(world.chunk_recipe(address));
            let chunk_entity = commands
                .spawn((
                    Name::new(format!(
                        "Voxel Materialization Chunk [{:+}, {:+}, {:+}]",
                        offset.x, offset.y, offset.z
                    )),
                    VoxelChunkOf::new(world_entity),
                    address,
                    generation,
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

            requested += 1;
            if requested >= streaming.load_budget_per_frame {
                break;
            }
        }
    }
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
