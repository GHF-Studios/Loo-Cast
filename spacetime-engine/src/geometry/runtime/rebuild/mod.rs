//! Authored asset -> compiled model -> disposable runtime-scene reconciliation.

use super::*;

pub(in crate::geometry) fn reconcile_authored_map_scenes(
    mut commands: Commands,
    mut events: MessageReader<AssetEvent<AuthoredMap>>,
    maps: Res<Assets<AuthoredMap>>,
    mut scenes: Query<(Entity, &mut AuthoredMapScene)>,
    generated: Query<(Entity, &AuthoredMapManifestationOf)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let changed: Vec<_> = events
        .read()
        .filter_map(|event| match event {
            AssetEvent::Added { id }
            | AssetEvent::Modified { id }
            | AssetEvent::LoadedWithDependencies { id } => Some(*id),
            AssetEvent::Removed { .. } | AssetEvent::Unused { .. } => None,
        })
        .collect();

    if !changed.is_empty() {
        for (_, mut scene) in &mut scenes {
            if changed.iter().any(|id| *id == scene.handle.id()) {
                scene.dirty = true;
            }
        }
    }

    for (source, mut scene) in &mut scenes {
        if !scene.dirty {
            continue;
        }

        let Some(map) = maps.get(&scene.handle) else {
            continue;
        };

        for (entity, generated) in &generated {
            if generated.source == source {
                commands.entity(entity).despawn();
            }
        }

        let material_handles = materialize_map_materials(map, &mut materials);
        for node in compile_authored_map(map) {
            spawn_compiled_node(&mut commands, source, node, &material_handles, &mut meshes);
        }

        debug!("reconciled authored map {:?}", map.name);
        scene.dirty = false;
    }
}

/// Rebuild creates fresh disposable manifestations from the authored asset.
/// Scene dirtiness clears only after all generated nodes have been queued.
fn materialize_map_materials(
    map: &AuthoredMap,
    materials: &mut Assets<StandardMaterial>,
) -> HashMap<String, Handle<StandardMaterial>> {
    map.materials
        .iter()
        .map(|definition| {
            let handle = materials.add(StandardMaterial {
                base_color: Color::srgb(definition.color.0, definition.color.1, definition.color.2),
                metallic: definition.metallic,
                perceptual_roughness: definition.roughness,
                double_sided: true,
                cull_mode: None,
                ..default()
            });
            (definition.id.clone(), handle)
        })
        .collect()
}

fn spawn_compiled_node(
    commands: &mut Commands,
    source: Entity,
    node: CompiledNode,
    material_handles: &HashMap<String, Handle<StandardMaterial>>,
    meshes: &mut Assets<Mesh>,
) {
    match node {
        CompiledNode::Geometry(geometry) => {
            spawn_geometry(commands, source, geometry, material_handles, meshes)
        }
        CompiledNode::Marker(marker) => {
            commands.spawn((
                Name::new(format!("Map Marker: {}", marker.id)),
                AuthoredMapManifestationOf { source },
                AuthoredMapMarker {
                    id: marker.id,
                    zone: marker.zone,
                    kind: marker.kind,
                    tags: marker.tags,
                },
                marker.transform,
            ));
        }
        CompiledNode::PointLight(light) => {
            commands.spawn((
                Name::new(format!("Map Light: {}", light.id)),
                AuthoredMapManifestationOf { source },
                AuthoredMapObject {
                    id: light.id,
                    zone: light.zone,
                    tags: vec!["light".into()],
                },
                PointLight {
                    color: light.color,
                    intensity: light.intensity,
                    range: light.range,
                    shadow_maps_enabled: light.shadows,
                    ..default()
                },
                Transform::from_translation(light.position),
            ));
        }
        CompiledNode::DirectionalLight(light) => {
            commands.spawn((
                Name::new(format!("Map Directional Light: {}", light.id)),
                AuthoredMapManifestationOf { source },
                AuthoredMapObject {
                    id: light.id,
                    zone: light.zone,
                    tags: vec!["light".into(), "directional".into()],
                },
                DirectionalLight {
                    color: light.color,
                    illuminance: light.illuminance,
                    shadow_maps_enabled: light.shadows,
                    ..default()
                },
                Transform::from_rotation(light.rotation),
            ));
        }
    }
}
