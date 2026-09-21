//! Per-materialization mesh construction and publication.

use avian3d::prelude::RigidBody;
use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

use crate::{
    config::EngineConfig,
    spatial::{UsfLocalScalePresentation, UsfScaleLayer, UsfSpatialFrame},
};

use super::{
    VoxelManifestation, VoxelManifestationPresentation, VoxelManifestationRegistry,
    collision::publish_collider_manifestation,
};
use super::super::{
    VoxelMaterialId, VoxelMaterializationChunkAddress, VoxelWorld,
    mesh::VoxelSurface,
    streaming::VoxelPresentationMaterial,
};

#[derive(Resource)]
pub(in crate::voxel) struct TranslucentVoxelMaterial(Handle<StandardMaterial>);

pub(in crate::voxel) fn initialize_translucent_voxel_material(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        alpha_mode: AlphaMode::Blend,
        perceptual_roughness: 0.18,
        metallic: 0.0,
        double_sided: true,
        ..default()
    });
    commands.insert_resource(TranslucentVoxelMaterial(material));
}

/// Rebuilds dirty one-to-one runtime manifestations within the configured frame
/// budget.
///
/// One manifestation root can own an opaque and a translucent presentation.
/// Collision remains an independent derived representation of the same surface.
pub(in crate::voxel) fn rebuild_dirty_manifestations(
    config: Res<EngineConfig>,
    mut commands: Commands,
    spatial_frame: Res<UsfSpatialFrame>,
    mut meshes: ResMut<Assets<Mesh>>,
    translucent_material: Res<TranslucentVoxelMaterial>,
    worlds: Query<(
        Entity,
        &VoxelWorld,
        &VoxelPresentationMaterial,
        &UsfScaleLayer,
    )>,
    mut manifestations: Query<&mut VoxelManifestation>,
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

        let mut opaque_mesh = build_opaque_mesh(&cache.surface, cache.debug_color);
        let mut root_entity = registry.entities.get(&key).copied();

        if let Some(entity) = root_entity {
            match manifestations.get_mut(entity) {
                Ok(mut manifestation) => {
                    match presentations.get(manifestation.presentation) {
                        Ok(Some(mesh3d)) => {
                            if let Some(replacement) = opaque_mesh.take() {
                                if let Some(mut existing) = meshes.get_mut(&mesh3d.0) {
                                    *existing = replacement;
                                } else {
                                    commands
                                        .entity(manifestation.presentation)
                                        .insert(Mesh3d(meshes.add(replacement)));
                                }
                            } else {
                                commands
                                    .entity(manifestation.presentation)
                                    .remove::<Mesh3d>();
                            }
                        }
                        Ok(None) => {
                            if let Some(replacement) = opaque_mesh.take() {
                                commands
                                    .entity(manifestation.presentation)
                                    .insert(Mesh3d(meshes.add(replacement)));
                            }
                        }
                        Err(_) => {
                            commands.entity(entity).despawn();
                            registry.entities.remove(&key);
                            root_entity = None;
                        }
                    }

                    if root_entity.is_some() {
                        sync_translucent_presentation(
                            &mut commands,
                            entity,
                            layer,
                            &cache.surface,
                            &translucent_material.0,
                            &mut meshes,
                            &presentations,
                            &mut manifestation,
                        );
                    }
                }
                Err(_) => {
                    commands.entity(entity).despawn();
                    registry.entities.remove(&key);
                    root_entity = None;
                }
            }
        }

        let root = if let Some(root) = root_entity {
            root
        } else {
            let Some(local_translation) =
                manifestation_runtime_translation(world, layer, &spatial_frame, key.address)
            else {
                registry.dirty.insert(key);
                continue;
            };

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
                    Name::new("Voxel Opaque Presentation"),
                    ChildOf(root),
                    VoxelManifestationPresentation,
                    UsfLocalScalePresentation::new(layer.scale()),
                    MeshMaterial3d(material.handle().clone()),
                    Transform::IDENTITY,
                    Visibility::Inherited,
                ))
                .id();
            if let Some(mesh) = opaque_mesh.take() {
                commands.entity(presentation).insert(Mesh3d(meshes.add(mesh)));
            }

            let translucent_presentation = spawn_translucent_presentation(
                &mut commands,
                root,
                layer,
                &cache.surface,
                &translucent_material.0,
                &mut meshes,
            );

            commands.entity(root).insert(VoxelManifestation {
                presentation,
                translucent_presentation,
            });
            registry.entities.insert(key, root);
            root
        };

        publish_collider_manifestation(&mut commands, root, false, None);
    }
}

fn sync_translucent_presentation(
    commands: &mut Commands,
    root: Entity,
    layer: &UsfScaleLayer,
    surface: &VoxelSurface,
    material: &Handle<StandardMaterial>,
    meshes: &mut Assets<Mesh>,
    presentations: &Query<Option<&Mesh3d>, With<VoxelManifestationPresentation>>,
    manifestation: &mut VoxelManifestation,
) {
    let Some(mesh) = build_translucent_mesh(surface) else {
        if let Some(entity) = manifestation.translucent_presentation.take() {
            commands.entity(entity).despawn();
        }
        return;
    };

    if let Some(entity) = manifestation.translucent_presentation {
        match presentations.get(entity) {
            Ok(Some(mesh3d)) => {
                if let Some(mut existing) = meshes.get_mut(&mesh3d.0) {
                    *existing = mesh;
                } else {
                    commands.entity(entity).insert(Mesh3d(meshes.add(mesh)));
                }
                return;
            }
            Ok(None) => {
                commands.entity(entity).insert(Mesh3d(meshes.add(mesh)));
                return;
            }
            Err(_) => {
                manifestation.translucent_presentation = None;
            }
        }
    }

    manifestation.translucent_presentation = spawn_translucent_presentation(
        commands,
        root,
        layer,
        surface,
        material,
        meshes,
    );
}

fn spawn_translucent_presentation(
    commands: &mut Commands,
    root: Entity,
    layer: &UsfScaleLayer,
    surface: &VoxelSurface,
    material: &Handle<StandardMaterial>,
    meshes: &mut Assets<Mesh>,
) -> Option<Entity> {
    let mesh = build_translucent_mesh(surface)?;
    Some(
        commands
            .spawn((
                Name::new("Voxel Translucent Presentation"),
                ChildOf(root),
                VoxelManifestationPresentation,
                UsfLocalScalePresentation::new(layer.scale()),
                Mesh3d(meshes.add(mesh)),
                MeshMaterial3d(material.clone()),
                Transform::IDENTITY,
                Visibility::Inherited,
            ))
            .id(),
    )
}

fn material_vertex_color(material: VoxelMaterialId) -> [f32; 4] {
    match material {
        VoxelMaterialId::GLASS => [0.62, 0.86, 1.0, material.behavior().opacity],
        VoxelMaterialId::NEBULA => [0.48, 0.16, 0.78, material.behavior().opacity],
        _ => [1.0, 1.0, 1.0, material.behavior().opacity],
    }
}

fn build_translucent_mesh(surface: &VoxelSurface) -> Option<Mesh> {
    let indices = surface.translucent_indices();
    if surface.positions.is_empty() || indices.is_empty() {
        return None;
    }

    let colors = surface
        .vertex_materials
        .iter()
        .copied()
        .map(material_vertex_color)
        .collect::<Vec<_>>();

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
        .with_inserted_indices(Indices::U32(indices)),
    )
}

fn build_opaque_mesh(surface: &VoxelSurface, debug_color: [f32; 4]) -> Option<Mesh> {
    let indices = surface.opaque_indices();
    if surface.positions.is_empty() || indices.is_empty() {
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
        .with_inserted_indices(Indices::U32(indices)),
    )
}

fn manifestation_runtime_translation(
    _world: &VoxelWorld,
    layer: &UsfScaleLayer,
    frame: &UsfSpatialFrame,
    address: VoxelMaterializationChunkAddress,
) -> Option<Vec3> {
    // Both ends stay canonical until the final bounded scale-local projection.
    // No huge float absolute is ever constructed or subtracted.
    let local_origin = frame.origin().reexpressed_at(layer.scale()).ok()?;
    address
        .query_origin()
        .usf()
        .relative_native_bounded(&local_origin, 1_000_000.0)
        .ok()
}
