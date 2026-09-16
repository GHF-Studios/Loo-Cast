//! Player-centered materialization of dense voxel chunk caches.
//!
//! Streaming is optional per [`VoxelWorld`]. The authoritative world remains
//! procedural base + sparse modifications; this module only decides which dense
//! chunk caches should currently exist around a viewer.

use std::collections::HashSet;

use bevy::{camera::visibility::NoFrustumCulling, prelude::*};

use super::{
    VoxelChunkCoord, VoxelChunkOf, VoxelRenderMesh, VoxelWorld,
};

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

pub(crate) fn stream_voxel_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    viewers: Query<&Transform>,
    mut worlds: Query<(Entity, &mut VoxelWorld, &VoxelStreaming)>,
) {
    for (world_entity, mut world, streaming) in &mut worlds {
        let Ok(viewer) = viewers.get(streaming.viewer) else {
            continue;
        };

        let center = VoxelChunkCoord::containing(viewer.translation);
        let desired = desired_chunk_coords(center, streaming.radius);
        let desired_set = desired.iter().copied().collect::<HashSet<_>>();

        // Dense chunks are disposable. Drop caches that left the active window;
        // authoritative edits remain in VoxelWorld and will be replayed if the
        // chunk is materialized again later.
        let stale = world
            .chunk_entries()
            .filter(|(coord, _)| !desired_set.contains(coord))
            .collect::<Vec<_>>();

        for (coord, entity) in stale {
            world.remove_chunk(coord);
            commands.entity(entity).despawn();
        }

        // Materialize nearest missing chunks first and cap work per frame. Mesh
        // extraction happens later in PostUpdate, so the same budget naturally
        // bounds fresh remesh work as the window expands.
        let mut loaded = 0;
        for coord in desired {
            if world.chunk_entity(coord).is_some() {
                continue;
            }

            let chunk = world.materialize_chunk(coord);
            let value = coord.0;
            let chunk_entity = commands
                .spawn((
                    Name::new(format!(
                        "Voxel Chunk ({}, {}, {})",
                        value.x, value.y, value.z
                    )),
                    VoxelChunkOf::new(world_entity, coord),
                    chunk,
                    VoxelRenderMesh::new(&mut meshes),
                    MeshMaterial3d(streaming.material.clone()),
                    NoFrustumCulling,
                    Transform::IDENTITY,
                ))
                .id();

            commands.entity(world_entity).add_child(chunk_entity);
            assert!(world.insert_chunk(coord, chunk_entity).is_none());

            loaded += 1;
            if loaded >= streaming.load_budget_per_frame {
                break;
            }
        }
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
}
