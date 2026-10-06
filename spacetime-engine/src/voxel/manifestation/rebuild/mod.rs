//! Per-materialization mesh construction and publication.

use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

use crate::{
    config::EngineConfig,
    reconstructible::{ReconstructibleFrameBudget, ReconstructibleWorkClass},
    ecs::UsfPresentationProjectionOf,
    spatial::{
        UsfScaleFallbackPresentation, UsfScaleLayer, UsfScalePresentation, UsfRuntimeChartState,
    },
};

use super::{
    ManifestationKey, VoxelMaterializationPresentation, VoxelMaterializationRuntime,
    VoxelMaterializationRuntimeRegistry, VoxelPresentationFallbackRetireReady,
    VoxelRenderMaterial,
};
use super::super::{
    CelestialVoxelFrameBinding, VoxelMaterialId, VoxelMaterializationChunkAddress, VoxelWorld,
    mesh::VoxelSurface, VoxelPresentationMaterial,
    streaming::{VoxelStreaming, compare_work_ranks},
};

fn park_manifestation(
    commands: &mut Commands,
    registry: &mut VoxelMaterializationRuntimeRegistry,
    manifestations: &Query<(
        &VoxelMaterializationRuntime,
        &Transform,
        &Visibility,
    )>,
    entity: Entity,
) {
    let Ok((runtime, _, _)) = manifestations.get(entity) else {
        commands.entity(entity).despawn();
        return;
    };

    commands
        .entity(entity)
        .insert(((*runtime).parked(), Visibility::Hidden))
        .remove::<VoxelPresentationFallbackRetireReady>();
    if !registry.recycle(entity) {
        commands.entity(entity).despawn();
    }
}

fn update_presentation_mesh(
    commands: &mut Commands,
    entity: Entity,
    replacement: Option<Mesh>,
    meshes: &mut Assets<Mesh>,
    presentations: &Query<Option<&Mesh3d>, With<VoxelMaterializationPresentation>>,
) -> bool {
    let Ok(existing_mesh) = presentations.get(entity) else {
        return false;
    };

    match replacement {
        Some(mesh) => {
            match existing_mesh {
                Some(mesh3d) => {
                    if let Some(mut existing) = meshes.get_mut(&mesh3d.0) {
                        *existing = mesh;
                    } else {
                        commands.entity(entity).insert(Mesh3d(meshes.add(mesh)));
                    }
                }
                None => {
                    commands.entity(entity).insert(Mesh3d(meshes.add(mesh)));
                }
            }
            commands.entity(entity).insert(Visibility::Inherited);
        }
        None => {
            // Keep the mesh component/handle stable; hiding is non-structural.
            commands.entity(entity).insert(Visibility::Hidden);
        }
    }
    true
}

pub(in crate::voxel) fn rebuild_dirty_manifestations(
    config: Res<EngineConfig>,
    mut commands: Commands,
    spatial_frame: Res<UsfRuntimeChartState>,
    mut meshes: ResMut<Assets<Mesh>>,
    worlds: Query<(
        Entity,
        &VoxelWorld,
        &VoxelPresentationMaterial,
        &UsfScaleLayer,
        Option<&UsfScaleFallbackPresentation>,
        Option<&VoxelStreaming>,
    )>,
    manifestations: Query<(
        &VoxelMaterializationRuntime,
        &Transform,
        &Visibility,
    )>,
    presentations: Query<Option<&Mesh3d>, With<VoxelMaterializationPresentation>>,
    mut registry: ResMut<VoxelMaterializationRuntimeRegistry>,
    mut frame_budget: ResMut<ReconstructibleFrameBudget>,
) {
    for _ in 0..config.voxel.manifestation.rebuild_budget_per_frame {
        if frame_budget
            .begin(ReconstructibleWorkClass::Publication)
            .is_none()
        {
            break;
        }
        //
        // `dirty` is a HashSet, so iteration order is not scheduling policy.
        // Preserve the same demand/contact/trajectory rank used by generation
        // and surface derivation all the way to the actual visible mesh.
        let rank_for = |key: ManifestationKey| {
            worlds
                .get(key.world)
                .ok()
                .and_then(|(_, _, _, _, _, streaming)| streaming)
                .and_then(|streaming| streaming.work_rank_for_key(key.key))
        };
        let Some(key) = registry
            .dirty
            .iter()
            .copied()
            .min_by(|a, b| {
                match (rank_for(*a), rank_for(*b)) {
                    (Some(a), Some(b)) => compare_work_ranks(a, b),
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (None, None) => std::cmp::Ordering::Equal,
                }
            })
        else {
            break;
        };
        registry.dirty.remove(&key);

        let Some(&expected_revision) = registry.revisions.get(&key) else {
            if let Some(entity) = registry.entities.remove(&key) {
                park_manifestation(&mut commands, &mut registry, &manifestations, entity);
            }
            continue;
        };

        let Ok((_, world, material, layer, fallback, _streaming)) =
            worlds.get(key.world)
        else {
            registry.revisions.remove(&key);
            if let Some(entity) = registry.entities.remove(&key) {
                park_manifestation(&mut commands, &mut registry, &manifestations, entity);
            }
            continue;
        };

        let Ok(address) = world.materialization_address(key.key) else {
            registry.revisions.remove(&key);
            if let Some(entity) = registry.entities.remove(&key) {
                park_manifestation(&mut commands, &mut registry, &manifestations, entity);
            }
            continue;
        };

        let Some((opaque_material, translucent_material)) = material.handles() else {
            registry.dirty.insert(key);
            continue;
        };

        if world.materializations().active_derived_revision(key.key)
            != Some(expected_revision)
        {
            registry.dirty.insert(key);
            continue;
        }

        let Some(cache) = world.materializations().active_surface(key.key) else {
            registry.revisions.remove(&key);
            if let Some(entity) = registry.entities.remove(&key) {
                park_manifestation(&mut commands, &mut registry, &manifestations, entity);
            }
            continue;
        };
        if cache.revision != expected_revision || !cache.surface.has_triangles() {
            registry.revisions.remove(&key);
            if let Some(entity) = registry.entities.remove(&key) {
                park_manifestation(&mut commands, &mut registry, &manifestations, entity);
            }
            continue;
        }

        let Some(local_translation) =
            materialization_runtime_translation(layer, &spatial_frame, address)
        else {
            registry.dirty.insert(key);
            continue;
        };

        registry.record_mesh_publication();
        let mut opaque_mesh = build_opaque_mesh(&cache.surface, cache.debug_color);

        // Existing active root: mutate in place.
        if let Some(&entity) = registry.entities.get(&key) {
            let Ok((runtime_ref, current_transform, current_visibility)) =
                manifestations.get(entity)
            else {
                commands.entity(entity).despawn();
                registry.entities.remove(&key);
                registry.dirty.insert(key);
                continue;
            };
            let mut runtime = (*runtime_ref).rebound(
                key.world,
                key.key,
                expected_revision,
            );

            registry.record_rebuild_existing();
            commands.entity(entity).insert((runtime, *layer));
            sync_manifestation_root_state(
                &mut commands,
                &mut registry,
                entity,
                current_transform,
                current_visibility,
                local_translation,
            );
            sync_fallback(&mut commands, entity, fallback);

            commands.entity(runtime.presentation).insert((
                UsfPresentationProjectionOf(key.world),
                UsfScalePresentation::new(*address.origin(), layer.scale()),
                MeshMaterial3d(opaque_material.clone()),
            ));
            if !update_presentation_mesh(
                &mut commands,
                runtime.presentation,
                opaque_mesh.take(),
                &mut meshes,
                &presentations,
            ) {
                commands.entity(entity).despawn();
                registry.entities.remove(&key);
                registry.dirty.insert(key);
                continue;
            }

            runtime.translucent_presentation = sync_translucent_presentation(
                &mut commands,
                entity,
                key.world,
                layer,
                address,
                Some(&cache.surface),
                translucent_material,
                &mut meshes,
                &presentations,
                runtime.translucent_presentation,
            );
            commands.entity(entity).insert(runtime);
            continue;
        }

        // Pool first: reuse the stable root + opaque child and mesh handle.
        if let Some(entity) = registry.take_pooled() {
            let Ok((runtime_ref, current_transform, current_visibility)) =
                manifestations.get(entity)
            else {
                commands.entity(entity).despawn();
                registry.dirty.insert(key);
                continue;
            };
            if presentations.get(runtime_ref.presentation).is_err() {
                commands.entity(entity).despawn();
                registry.dirty.insert(key);
                continue;
            }

            let mut runtime = (*runtime_ref).rebound(
                key.world,
                key.key,
                expected_revision,
            );
            commands.entity(entity).insert((runtime, *layer));
            sync_manifestation_root_state(
                &mut commands,
                &mut registry,
                entity,
                current_transform,
                current_visibility,
                local_translation,
            );
            sync_fallback(&mut commands, entity, fallback);

            commands.entity(runtime.presentation).insert((
                UsfPresentationProjectionOf(key.world),
                UsfScalePresentation::new(*address.origin(), layer.scale()),
                MeshMaterial3d(opaque_material.clone()),
            ));
            if !update_presentation_mesh(
                &mut commands,
                runtime.presentation,
                opaque_mesh.take(),
                &mut meshes,
                &presentations,
            ) {
                commands.entity(entity).despawn();
                registry.dirty.insert(key);
                continue;
            }

            runtime.translucent_presentation = sync_translucent_presentation(
                &mut commands,
                entity,
                key.world,
                layer,
                address,
                Some(&cache.surface),
                translucent_material,
                &mut meshes,
                &presentations,
                runtime.translucent_presentation,
            );
            commands.entity(entity).insert(runtime);
            registry.entities.insert(key, entity);
            continue;
        }

        // Pool empty: allocate one stable shell.
        registry.record_spawned();
        let root = commands
            .spawn((
                Name::new("Voxel Manifestation"),
                *layer,
                Transform::from_translation(local_translation),
                Visibility::Inherited,
            ))
            .id();
        sync_fallback(&mut commands, root, fallback);

        let opaque_visible = opaque_mesh.is_some();
        let initial_mesh =
            opaque_mesh.take().unwrap_or_else(empty_presentation_mesh);
        let presentation = commands
            .spawn((
                Name::new("Voxel Opaque Presentation"),
                ChildOf(root),
                VoxelMaterializationPresentation,
                UsfPresentationProjectionOf(key.world),
                UsfScalePresentation::new(*address.origin(), layer.scale()),
                Mesh3d(meshes.add(initial_mesh)),
                MeshMaterial3d(opaque_material.clone()),
                Transform::IDENTITY,
                if opaque_visible {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                },
            ))
            .id();

        let translucent_presentation = sync_translucent_presentation(
            &mut commands,
            root,
            key.world,
            layer,
            address,
            Some(&cache.surface),
            translucent_material,
            &mut meshes,
            &presentations,
            None,
        );

        commands.entity(root).insert(VoxelMaterializationRuntime {
            world: key.world,
            key: key.key,
            revision: expected_revision,
            presentation,
            translucent_presentation,
            active: true,
        });
        registry.entities.insert(key, root);
    }
}

fn sync_manifestation_root_state(
    commands: &mut Commands,
    registry: &mut VoxelMaterializationRuntimeRegistry,
    entity: Entity,
    current_transform: &Transform,
    current_visibility: &Visibility,
    local_translation: Vec3,
) {
    let target_transform = Transform::from_translation(local_translation);
    let transform_changed = current_transform.translation != target_transform.translation
        || current_transform.rotation != target_transform.rotation
        || current_transform.scale != target_transform.scale;

    registry.record_root_transform(transform_changed);
    if transform_changed {
        commands.entity(entity).insert(target_transform);
    }

    if *current_visibility != Visibility::Inherited {
        registry.record_visibility_write();
        commands.entity(entity).insert(Visibility::Inherited);
    }

    // A rebound/reused manifestation is active truth again. Any retirement
    // decision belonged to its previous disposable binding.
    commands
        .entity(entity)
        .remove::<VoxelPresentationFallbackRetireReady>();
}

fn sync_fallback(
    commands: &mut Commands,
    entity: Entity,
    fallback: Option<&UsfScaleFallbackPresentation>,
) {
    if let Some(fallback) = fallback {
        commands.entity(entity).insert(*fallback);
    } else {
        commands.entity(entity).remove::<UsfScaleFallbackPresentation>();
    }
}

fn sync_translucent_presentation(
    commands: &mut Commands,
    root: Entity,
    world: Entity,
    layer: &UsfScaleLayer,
    address: VoxelMaterializationChunkAddress,
    surface: Option<&VoxelSurface>,
    material: &Handle<VoxelRenderMaterial>,
    meshes: &mut Assets<Mesh>,
    presentations: &Query<Option<&Mesh3d>, With<VoxelMaterializationPresentation>>,
    existing_entity: Option<Entity>,
) -> Option<Entity> {
    let mesh = surface.and_then(build_translucent_mesh);

    if let Some(entity) = existing_entity
        && presentations.get(entity).is_ok()
    {
        commands.entity(entity).insert((
            UsfPresentationProjectionOf(world),
            UsfScalePresentation::new(*address.origin(), layer.scale()),
            MeshMaterial3d(material.clone()),
        ));
        let _ = update_presentation_mesh(
            commands,
            entity,
            mesh,
            meshes,
            presentations,
        );
        return Some(entity);
    }

    let mesh = mesh?;
    Some(
        commands
            .spawn((
                Name::new("Voxel Translucent Presentation"),
                ChildOf(root),
                VoxelMaterializationPresentation,
                UsfPresentationProjectionOf(world),
                UsfScalePresentation::new(*address.origin(), layer.scale()),
                Mesh3d(meshes.add(mesh)),
                MeshMaterial3d(material.clone()),
                Transform::IDENTITY,
                Visibility::Inherited,
            ))
            .id(),
    )
}

fn empty_presentation_mesh() -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, Vec::<[f32; 3]>::new())
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, Vec::<[f32; 3]>::new())
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, Vec::<[f32; 4]>::new())
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, Vec::<[f32; 2]>::new())
    .with_inserted_attribute(Mesh::ATTRIBUTE_TANGENT, Vec::<[f32; 4]>::new())
    .with_inserted_indices(Indices::U32(Vec::new()))
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
        Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
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
        Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, surface.positions.clone())
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, surface.normals.clone())
            .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, surface.uvs.clone())
            .with_inserted_attribute(Mesh::ATTRIBUTE_TANGENT, surface.tangents.clone())
            .with_inserted_indices(Indices::U32(indices)),
    )
}

fn materialization_runtime_translation(
    layer: &UsfScaleLayer,
    frame: &UsfRuntimeChartState,
    address: VoxelMaterializationChunkAddress,
) -> Option<Vec3> {
    address
        .query_origin()
        .usf()
        .relative_at_scale_bounded(frame.origin(), layer.scale(), 16_384.0)
        .ok()
}

fn write_materialization_runtime_translation(
    transform: &mut Transform,
    translation: Vec3,
) {
    if transform.translation != translation {
        transform.translation = translation;
    }
}

pub(in crate::voxel) fn sync_manifestation_runtime_transforms(
    mut commands: Commands,
    frame: Res<UsfRuntimeChartState>,
    worlds: Query<(&VoxelWorld, Option<Ref<CelestialVoxelFrameBinding>>)>,
    mut runtimes: Query<(&VoxelMaterializationRuntime, &UsfScaleLayer, &mut Transform)>,
) {
    let chart_changed=frame.is_changed();
    for (runtime,layer,mut transform) in &mut runtimes {
        if !runtime.active(){continue;}
        let Ok((world,binding))=worlds.get(runtime.world()) else {continue;};
        let pose_changed=binding.as_ref().is_some_and(|b|b.is_changed());
        if !chart_changed && !pose_changed {continue;}
        let Ok(address)=world.materialization_address(runtime.key()) else {continue;};
        let Some(translation)=materialization_runtime_translation(layer,&frame,address) else {continue;};
        write_materialization_runtime_translation(&mut transform,translation);
        commands.entity(runtime.presentation()).insert(UsfScalePresentation::new(*address.origin(),layer.scale()));
        if let Some(entity)=runtime.translucent_presentation(){commands.entity(entity).insert(UsfScalePresentation::new(*address.origin(),layer.scale()));}
    }
}
