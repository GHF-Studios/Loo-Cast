//! Runtime mesh/collider/entity realization of compiled authored geometry.

use super::*;

pub(super) fn spawn_geometry(
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
