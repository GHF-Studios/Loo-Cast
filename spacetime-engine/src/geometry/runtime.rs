use std::collections::HashMap;

use avian3d::prelude::{Collider, LinearVelocity, RigidBody};
use bevy::prelude::*;

use crate::physics::collision_topology::CollisionClipSource;

use super::{
    asset::AuthoredMap,
    compile::{CompiledGeometry, CompiledMotion, CompiledNode, CompiledShape, compile_map},
    mesh::{convex_prism_mesh, convex_prism_points},
};

#[derive(Component, Debug, Clone)]
pub struct AuthoredMapScene {
    handle: Handle<AuthoredMap>,
    dirty: bool,
}

impl AuthoredMapScene {
    pub fn new(handle: Handle<AuthoredMap>) -> Self {
        Self {
            handle,
            dirty: true,
        }
    }

    pub fn handle(&self) -> &Handle<AuthoredMap> {
        &self.handle
    }
}

/// Metadata preserved on each generated geometry entity for inspection/debug tooling.
#[derive(Component, Debug, Clone)]
pub struct AuthoredMapObject {
    pub id: String,
    pub zone: String,
    pub tags: Vec<String>,
}

/// A data-authored non-rendering anchor. Gameplay systems may opt into specific marker kinds
/// without the authored-map runtime itself knowing what they mean.
#[derive(Component, Debug, Clone)]
pub struct AuthoredMapMarker {
    pub id: String,
    pub zone: String,
    pub kind: String,
    pub tags: Vec<String>,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct GeneratedFromMap {
    source: Entity,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct AuthoredMotion {
    base: Transform,
    travel: Vec3,
    period_seconds: f32,
    phase: f32,
    elapsed_seconds: f32,
}

pub(crate) fn rebuild_authored_maps(
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

        info!("rebuilt authored map {:?}", map.name);
        scene.dirty = false;
    }
}

fn spawn_geometry(
    commands: &mut Commands,
    source: Entity,
    geometry: CompiledGeometry,
    material_handles: &HashMap<String, Handle<StandardMaterial>>,
    meshes: &mut Assets<Mesh>,
) {
    let Some(material) = material_handles.get(&geometry.material).cloned() else {
        error!(
            "authored geometry {:?} lost material {:?} after validation",
            geometry.id, geometry.material
        );
        return;
    };

    let (mesh, collider, clip_source) = match &geometry.shape {
        CompiledShape::Box { size } => (
            meshes.add(Cuboid::new(size.x, size.y, size.z)),
            Some(Collider::cuboid(size.x, size.y, size.z)),
            Some(CollisionClipSource::cuboid(*size)),
        ),
        CompiledShape::Cylinder { radius, height } => (
            meshes.add(Cylinder::new(*radius, *height)),
            Some(Collider::cylinder(*radius, *height)),
            None,
        ),
        CompiledShape::Sphere { radius } => (
            meshes.add(Sphere::new(*radius)),
            Some(Collider::sphere(*radius)),
            None,
        ),
        CompiledShape::Capsule { radius, length } => (
            meshes.add(Capsule3d::new(*radius, *length)),
            Some(Collider::capsule(*radius, *length)),
            None,
        ),
        CompiledShape::ConvexPrism {
            cross_section,
            depth,
        } => {
            let mesh = meshes.add(convex_prism_mesh(cross_section, *depth));
            let collider = Collider::convex_hull(convex_prism_points(cross_section, *depth));
            (mesh, collider, None)
        }
    };

    let motion = geometry
        .motion
        .map(|motion| authored_motion(geometry.transform, motion));

    let mut spawn_transform = geometry.transform;
    let mut initial_velocity = LinearVelocity::ZERO;
    if let Some(motion) = &motion {
        let (translation, velocity) = motion_state(motion);
        spawn_transform.translation = translation;
        initial_velocity.0 = velocity;
    }

    let mut entity = commands.spawn((
        Name::new(format!("Map Geometry: {}", geometry.id)),
        GeneratedFromMap { source },
        AuthoredMapObject {
            id: geometry.id,
            zone: geometry.zone,
            tags: geometry.tags,
        },
        Mesh3d(mesh),
        MeshMaterial3d(material),
        spawn_transform,
    ));

    if let Some(motion) = motion {
        if let Some(collider) = collider {
            entity.insert((RigidBody::Kinematic, collider, initial_velocity, motion));
        } else {
            warn!("moving authored convex geometry could not build a collider");
        }
    } else if geometry.solid {
        if let Some(collider) = collider {
            entity.insert((RigidBody::Static, collider));
            if let Some(source) = clip_source {
                entity.insert(source);
            }
        } else {
            warn!("authored convex geometry could not build a collider");
        }
    }
}

fn authored_motion(base: Transform, motion: CompiledMotion) -> AuthoredMotion {
    AuthoredMotion {
        base,
        travel: motion.travel,
        period_seconds: motion.period_seconds,
        phase: motion.phase,
        elapsed_seconds: 0.0,
    }
}

fn motion_state(motion: &AuthoredMotion) -> (Vec3, Vec3) {
    let phase = (motion.phase + motion.elapsed_seconds / motion.period_seconds).rem_euclid(1.0);
    let angle = std::f32::consts::TAU * phase;

    // Smooth endpoint-to-endpoint oscillation. Position starts at base when phase=0,
    // reaches base+travel at phase=0.5, then returns.
    let factor = 0.5 - 0.5 * angle.cos();
    let factor_velocity = 0.5 * std::f32::consts::TAU / motion.period_seconds * angle.sin();

    (
        motion.base.translation + motion.travel * factor,
        motion.travel * factor_velocity,
    )
}

pub(crate) fn animate_authored_movers(
    time: Res<Time<Fixed>>,
    mut movers: Query<(&mut Transform, &mut LinearVelocity, &mut AuthoredMotion)>,
) {
    for (mut transform, mut velocity, mut motion) in &mut movers {
        motion.elapsed_seconds += time.delta_secs();

        let (translation, linear_velocity) = motion_state(&motion);
        transform.translation = translation;
        transform.rotation = motion.base.rotation;
        velocity.0 = linear_velocity;
    }
}
