//! Player-centered materialization of dense voxel chunk caches.
//!
//! Streaming is optional per [`VoxelWorld`]. The authoritative world remains
//! procedural base + sparse modifications; this module only decides which dense
//! chunk caches should currently exist around a viewer. Expensive field
//! generation runs on Bevy's async compute pool and is published later.

use std::collections::HashSet;

use bevy::{
    camera::visibility::NoFrustumCulling,
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task, futures::check_ready},
};

use super::{
    VoxelChunk, VoxelChunkCoord, VoxelChunkOf, VoxelWorld, empty_voxel_mesh,
    world::VoxelChunkRecipe,
};

/// Maximum number of finished generation tasks published into ECS in one frame.
/// Task execution itself is already parallel; this caps main-thread structural
/// work when many chunks happen to complete together.
const GENERATION_PUBLISH_BUDGET_PER_FRAME: usize = 8;

/// Opt-in policy for keeping chunks materialized around one viewer.
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

/// Background construction of one dense chunk from an immutable procedural/edit
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
    mut tasks: Query<(Entity, &VoxelChunkOf, &mut VoxelChunkGenerationTask)>,
) {
    let mut published = 0;

    for (entity, chunk_of, mut generation) in &mut tasks {
        if published >= GENERATION_PUBLISH_BUDGET_PER_FRAME {
            break;
        }

        let Some(mut chunk) = check_ready(&mut generation.task) else {
            continue;
        };

        let Ok(world) = worlds.get(chunk_of.world) else {
            // The owning world disappeared while background work was running.
            commands.entity(entity).despawn();
            continue;
        };

        catch_up_generated_chunk(
            world,
            chunk_of.coord,
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

pub(crate) fn stream_voxel_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    viewers: Query<&Transform>,
    mut worlds: Query<(Entity, &mut VoxelWorld, &VoxelStreaming, &Transform)>,
) {
    for (world_entity, mut world, streaming, world_transform) in &mut worlds {
        let Ok(viewer) = viewers.get(streaming.viewer) else {
            continue;
        };

        // Query/generation coordinates remain local to the VoxelWorld root during
        // M7.1 Pass A; canonical materialization identity is resolved separately.
        // Floating-origin rebases move both viewer and world root, so convert the
        // viewer back into that stable world-local chart before addressing bricks.
        let viewer_in_world = viewer.translation - world_transform.translation;
        let center = VoxelChunkCoord::containing(viewer_in_world);
        let desired = desired_chunk_coords(center, streaming.radius)
            .into_iter()
            .filter_map(|coord| match world.chunk_address(coord) {
                Ok(address) => Some((coord, address)),
                Err(error) => {
                    error!(?coord, ?error, "voxel materialization address overflow");
                    None
                }
            })
            .collect::<Vec<_>>();
        let desired_set = desired
            .iter()
            .map(|(_, address)| *address)
            .collect::<HashSet<_>>();

        // Dense chunks and in-flight chunk tasks are disposable. Drop either
        // when their canonical materialization address leaves the active window;
        // authoritative edits remain in the VoxelWorld and will be replayed if
        // that address returns later.
        let stale = world
            .chunk_entries()
            .filter(|(address, _)| !desired_set.contains(address))
            .collect::<Vec<_>>();

        for (address, entity) in stale {
            world.remove_chunk(address);
            commands.entity(entity).despawn();
        }

        // Reserve nearest missing coordinates first, but only start a bounded
        // number of expensive generation tasks each frame. The returned Task is
        // retained on the placeholder entity; dropping that entity cancels work
        // that streamed out before completion. The local coordinate remains only
        // the Pass-B compatibility input to generation/projection; reservation
        // identity is already canonical.
        let mut requested = 0;
        for (coord, address) in desired {
            if world.chunk_entity(address).is_some() {
                continue;
            }

            let generation = VoxelChunkGenerationTask::spawn(world.chunk_recipe(coord));
            let value = coord.0;
            let chunk_entity = commands
                .spawn((
                    Name::new(format!(
                        "Voxel Materialization Chunk ({}, {}, {})",
                        value.x, value.y, value.z
                    )),
                    VoxelChunkOf::new(world_entity, coord),
                    address,
                    generation,
                    Mesh3d(meshes.add(empty_voxel_mesh())),
                    MeshMaterial3d(streaming.material.clone()),
                    NoFrustumCulling,
                    Transform::from_translation(coord.origin().as_vec3()),
                ))
                .id();

            // Generation is asynchronous, so the old attribute-less placeholder
            // could otherwise survive into render extraction for several frames.
            commands.entity(chunk_entity).remove::<Mesh3d>();

            commands.entity(world_entity).add_child(chunk_entity);
            assert!(world.insert_chunk(address, chunk_entity).is_none());

            requested += 1;
            if requested >= streaming.load_budget_per_frame {
                break;
            }
        }
    }
}

/// Applies edits appended after a generation task took its immutable snapshot.
/// This preserves ordered edit semantics without throwing away completed work.
fn catch_up_generated_chunk(
    world: &VoxelWorld,
    coord: VoxelChunkCoord,
    applied_edit_count: usize,
    chunk: &mut VoxelChunk,
) {
    for edit in world
        .modifications()
        .for_chunk_since(coord, applied_edit_count)
    {
        chunk.apply_edit(edit);
    }
}

fn desired_chunk_coords(center: VoxelChunkCoord, radius: IVec3) -> Vec<VoxelChunkCoord> {
    let radius = radius.max(IVec3::ZERO);
    let mut result = Vec::with_capacity(
        ((radius.x * 2 + 1) * (radius.y * 2 + 1) * (radius.z * 2 + 1)) as usize,
    );

    for z in -radius.z..=radius.z {
        for y in -radius.y..=radius.y {
            for x in -radius.x..=radius.x {
                result.push(VoxelChunkCoord::new(center.0 + IVec3::new(x, y, z)));
            }
        }
    }

    result.sort_by_key(|coord| chunk_distance_squared(center, *coord));
    result
}

fn chunk_distance_squared(a: VoxelChunkCoord, b: VoxelChunkCoord) -> i64 {
    let delta = b.0 - a.0;
    let x = delta.x as i64;
    let y = delta.y as i64;
    let z = delta.z as i64;
    x * x + y * y + z * z
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::voxel::{VoxelBase, VoxelBrush, VoxelEdit, VoxelMaterialId};

    #[test]
    fn streaming_window_is_centered_unique_and_nearest_first() {
        let center = VoxelChunkCoord::new(IVec3::new(7, -3, 12));
        let coords = desired_chunk_coords(center, IVec3::new(2, 1, 2));

        assert_eq!(coords.len(), 75);
        assert_eq!(coords[0], center);
        assert_eq!(coords.iter().copied().collect::<HashSet<_>>().len(), 75);
        assert!(coords.iter().all(|coord| {
            let delta = coord.0 - center.0;
            delta.x.abs() <= 2 && delta.y.abs() <= 1 && delta.z.abs() <= 2
        }));
    }

    #[test]
    fn generation_completion_replays_edits_recorded_while_task_was_running() {
        let coord = VoxelChunkCoord::new(IVec3::ZERO);
        let center = Vec3::splat(8.0);
        let mut world = VoxelWorld::new(VoxelBase::Empty);
        let recipe = world.chunk_recipe(coord);
        let applied_edit_count = recipe.applied_edit_count();

        // A distant post-snapshot edit consumes a global edit index but should
        // never be considered while catching this chunk up.
        world.record_edit(VoxelEdit::Add {
            brush: VoxelBrush::sphere(Vec3::splat(1000.0), 2.0),
            material: VoxelMaterialId::ROCK,
        });
        world.record_edit(VoxelEdit::Add {
            brush: VoxelBrush::sphere(center, 2.0),
            material: VoxelMaterialId::ROCK,
        });

        // Pretend this value arrived from the background worker after the edit.
        let mut chunk = recipe.materialize();
        assert!(chunk.sample(center.as_ivec3()).unwrap().distance.is_empty());

        catch_up_generated_chunk(&world, coord, applied_edit_count, &mut chunk);
        assert!(chunk.sample(center.as_ivec3()).unwrap().distance.is_solid());
    }
}
