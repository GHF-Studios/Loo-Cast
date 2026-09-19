//! Per-materialization mesh construction and publication.

use avian3d::prelude::RigidBody;
use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

use crate::{
    config::EngineConfig,
    spatial::{UsfLocalScalePresentation, UsfScaleLayer, UsfScaleLayerFrames},
};

use super::{
    VoxelManifestation, VoxelManifestationPresentation, VoxelManifestationRegistry,
    collision::publish_collider_manifestation,
};
use super::super::{
    VoxelMaterializationChunkAddress, VoxelQueryPosition, VoxelWorld,
    mesh::VoxelSurface,
    streaming::VoxelPresentationMaterial,
};

/// Rebuilds dirty one-to-one runtime manifestations within the configured frame
/// budget.
///
/// A dirty surface rebuild affects only its own materialization. Collision data
/// is invalidated independently and recreated only when this manifestation is
/// inside the physics interaction region.
pub(in crate::voxel) fn rebuild_dirty_manifestations(
    config: Res<EngineConfig>,
    mut commands: Commands,
    layer_frames: Res<UsfScaleLayerFrames>,
    mut meshes: ResMut<Assets<Mesh>>,
    worlds: Query<(
        Entity,
        &VoxelWorld,
        &VoxelPresentationMaterial,
        &UsfScaleLayer,
    )>,
    manifestations: Query<&VoxelManifestation>,
    presentations: Query<Option<&Mesh3d>, With<VoxelManifestationPresentation>>,
    mut registry: ResMut<VoxelManifestationRegistry>,
) {
    for _ in 0..config.voxel.manifestation.rebuild_budget_per_frame {
        let Some(key) = registry.dirty.iter().next().copied() else {
            break;
        };
        registry.dirty.remove(&key);

        let Some(&expected_revision) = registry.revisions.get(&key) else {
            if let Some(entity) = registry.entities.remove(&key) {
                commands.entity(entity).despawn();
            }
            continue;
        };

        let Ok((_, world, material, layer)) = worlds.get(key.world) else {
            registry.revisions.remove(&key);
            if let Some(entity) = registry.entities.remove(&key) {
                commands.entity(entity).despawn();
            }
            continue;
        };

        let Some(cache) = world.materializations().surface(key.address) else {
            registry.dirty.insert(key);
            continue;
        };
        if cache.revision != expected_revision {
            registry.dirty.insert(key);
            continue;
        }

        let Some(mesh) = build_manifestation_mesh(&cache.surface, cache.debug_color) else {
            registry.dirty.insert(key);
            continue;
        };

        let mut mesh = Some(mesh);
        let mut root_entity = registry.entities.get(&key).copied();

        if let Some(entity) = root_entity
            && let Ok(manifestation) = manifestations.get(entity)
        {
            match presentations.get(manifestation.presentation) {
                Ok(Some(mesh3d)) => {
                    let replacement = mesh.take().expect("manifestation mesh is published once");
                    if let Some(mut existing) = meshes.get_mut(&mesh3d.0) {
                        *existing = replacement;
                    } else {
                        commands
                            .entity(manifestation.presentation)
                            .insert(Mesh3d(meshes.add(replacement)));
                    }
                }
                Ok(None) => {
                    let replacement = mesh.take().expect("manifestation mesh is published once");
                    commands
                        .entity(manifestation.presentation)
                        .insert(Mesh3d(meshes.add(replacement)));
                }
                Err(_) => {
                    commands.entity(entity).despawn();
                    registry.entities.remove(&key);
                    root_entity = None;
                }
            }
        } else {
            root_entity = None;
        }

        let root = if let Some(root) = root_entity {
            root
        } else {
            let Some(local_translation) =
                manifestation_runtime_translation(world, layer, &layer_frames, key.address)
            else {
                registry.dirty.insert(key);
                continue;
            };

            let mesh_handle = meshes.add(mesh.expect("manifestation mesh is still available"));
            let root = commands
                .spawn((
                    Name::new("Voxel Manifestation"),
                    *layer,
                    RigidBody::Static,
                    Transform::from_translation(local_translation),
                    Visibility::Inherited,
                ))
                .id();
            let presentation = commands
                .spawn((
                    Name::new("Voxel Manifestation Presentation"),
                    ChildOf(root),
                    VoxelManifestationPresentation,
                    UsfLocalScalePresentation::new(layer.scale()),
                    Mesh3d(mesh_handle),
                    MeshMaterial3d(material.handle().clone()),
                    Transform::IDENTITY,
                    Visibility::Inherited,
                ))
                .id();
            commands
                .entity(root)
                .insert(VoxelManifestation { presentation });
            registry.entities.insert(key, root);
            root
        };

        publish_collider_manifestation(&mut commands, root, false, None);
    }
}

fn manifestation_runtime_translation(
    world: &VoxelWorld,
    layer: &UsfScaleLayer,
    frames: &UsfScaleLayerFrames,
    address: VoxelMaterializationChunkAddress,
) -> Option<Vec3> {
    let world_origin = VoxelQueryPosition::new(*world.origin());
    let relative = address
        .query_origin()
        .relative_to(world_origin, 1_000_000.0)
        .ok()?;
    let absolute = bevy::math::DVec3::new(relative.x as f64, relative.y as f64, relative.z as f64);
    Some(frames.runtime_from_absolute(layer.scale(), absolute))
}

fn build_manifestation_mesh(surface: &VoxelSurface, debug_color: [f32; 4]) -> Option<Mesh> {
    if surface.positions.is_empty() || surface.indices.is_empty() {
        return None;
    }

    let colors = vec![debug_color; surface.positions.len()];
    Some(
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, surface.positions.clone())
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, surface.normals.clone())
        .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, surface.uvs.clone())
        .with_inserted_attribute(Mesh::ATTRIBUTE_TANGENT, surface.tangents.clone())
        .with_inserted_indices(Indices::U32(surface.indices.clone())),
    )
}
