//! Hot-reload reconciliation for authored map scenes.

use super::*;

pub(in crate::geometry) fn rebuild_authored_maps(
    mut commands: Commands,
    mut events: MessageReader<AssetEvent<AuthoredMap>>,
    maps: Res<Assets<AuthoredMap>>,
    mut scenes: Query<(Entity, &mut AuthoredMapScene)>,
    generated: Query<(Entity, &GeneratedFromMap)>,
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

        let material_handles: HashMap<_, _> = map
            .materials
            .iter()
            .map(|definition| {
                let handle = materials.add(StandardMaterial {
                    base_color: Color::srgb(
                        definition.color.0,
                        definition.color.1,
                        definition.color.2,
                    ),
                    metallic: definition.metallic,
                    perceptual_roughness: definition.roughness,
                    double_sided: true,
                    cull_mode: None,
                    ..default()
                });
                (definition.id.clone(), handle)
            })
            .collect();

        for node in compile_map(map) {
            match node {
                CompiledNode::Geometry(geometry) => spawn_geometry(
                    &mut commands,
                    source,
                    geometry,
                    &material_handles,
                    &mut meshes,
                ),
                CompiledNode::Marker(marker) => {
                    commands.spawn((
                        Name::new(format!("Map Marker: {}", marker.id)),
                        GeneratedFromMap { source },
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
                        GeneratedFromMap { source },
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
                        GeneratedFromMap { source },
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

        debug!("rebuilt authored map {:?}", map.name);
        scene.dirty = false;
    }
}
