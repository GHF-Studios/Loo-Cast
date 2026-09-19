//! Compilation of directly-authored primitive geometry.

use super::*;

pub(super) fn compile_box(value: &BoxDef) -> CompiledGeometry {
    CompiledGeometry {
        id: value.id.clone(),
        zone: value.zone.clone(),
        tags: value.tags.clone(),
        transform: authored_transform(value.position, value.rotation_degrees),
        shape: CompiledShape::Box {
            size: v3(value.size),
        },
        material: value.material.clone(),
        solid: value.solid,
        motion: None,
    }
}

pub(super) fn compile_cylinder(value: &CylinderDef) -> CompiledGeometry {
    CompiledGeometry {
        id: value.id.clone(),
        zone: value.zone.clone(),
        tags: value.tags.clone(),
        transform: authored_transform(value.position, value.rotation_degrees),
        shape: CompiledShape::Cylinder {
            radius: value.radius,
            height: value.height,
        },
        material: value.material.clone(),
        solid: value.solid,
        motion: None,
    }
}

pub(super) fn compile_sphere(value: &SphereDef) -> CompiledGeometry {
    CompiledGeometry {
        id: value.id.clone(),
        zone: value.zone.clone(),
        tags: value.tags.clone(),
        transform: Transform::from_translation(v3(value.position)),
        shape: CompiledShape::Sphere {
            radius: value.radius,
        },
        material: value.material.clone(),
        solid: value.solid,
        motion: None,
    }
}

pub(super) fn compile_capsule(value: &CapsuleDef) -> CompiledGeometry {
    CompiledGeometry {
        id: value.id.clone(),
        zone: value.zone.clone(),
        tags: value.tags.clone(),
        transform: authored_transform(value.position, value.rotation_degrees),
        shape: CompiledShape::Capsule {
            radius: value.radius,
            length: value.length,
        },
        material: value.material.clone(),
        solid: value.solid,
        motion: None,
    }
}

pub(super) fn compile_convex_prism(value: &ConvexPrismDef) -> CompiledGeometry {
    CompiledGeometry {
        id: value.id.clone(),
        zone: value.zone.clone(),
        tags: value.tags.clone(),
        transform: authored_transform(value.position, value.rotation_degrees),
        shape: CompiledShape::ConvexPrism {
            cross_section: value.cross_section.iter().copied().map(v2).collect(),
            depth: value.depth,
        },
        material: value.material.clone(),
        solid: value.solid,
        motion: None,
    }
}

pub(super) fn compile_ramp(value: &RampDef) -> CompiledGeometry {
    let start = v3(value.start);
    let width = value.width;
    let run = value.run;
    let rise = value.rise;
    let thickness = value.thickness;

    let yaw = value.yaw_degrees.to_radians();
    let slope = rise.atan2(run);
    let rotation = Quat::from_rotation_y(yaw) * Quat::from_rotation_x(-slope);

    let forward = Quat::from_rotation_y(yaw) * Vec3::Z;
    let surface_midpoint = start + forward * (run * 0.5) + Vec3::Y * (rise * 0.5);
    let local_up = rotation * Vec3::Y;
    let center = surface_midpoint - local_up * (thickness * 0.5);

    CompiledGeometry {
        id: value.id.clone(),
        zone: value.zone.clone(),
        tags: value.tags.clone(),
        transform: Transform::from_translation(center).with_rotation(rotation),
        shape: CompiledShape::Box {
            size: Vec3::new(width, thickness, run.hypot(rise)),
        },
        material: value.material.clone(),
        solid: value.solid,
        motion: None,
    }
}

pub(super) fn compile_moving_box(value: &MovingBoxDef) -> CompiledGeometry {
    CompiledGeometry {
        id: value.id.clone(),
        zone: value.zone.clone(),
        tags: value.tags.clone(),
        transform: authored_transform(value.position, value.rotation_degrees),
        shape: CompiledShape::Box {
            size: v3(value.size),
        },
        material: value.material.clone(),
        solid: true,
        motion: Some(CompiledMotion {
            travel: v3(value.travel),
            period_seconds: value.period_seconds,
            phase: value.phase,
        }),
    }
}
