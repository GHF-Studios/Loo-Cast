use bevy::prelude::*;

use super::asset::{
    AuthoredMap, BoxDef, CapsuleDef, ConvexPrismDef, CylinderDef, DirectionalLightDef, MapObject,
    MovingBoxDef, PillarGridDef, PointLightDef, RampDef, SlopeSweepDef, SphereDef, StaircaseDef,
    StepSweepDef, V2, V3,
};

#[derive(Debug, Clone)]
pub(crate) enum CompiledNode {
    Geometry(CompiledGeometry),
    Marker(CompiledMarker),
    PointLight(CompiledPointLight),
    DirectionalLight(CompiledDirectionalLight),
}

#[derive(Debug, Clone)]
pub(crate) struct CompiledGeometry {
    pub id: String,
    pub zone: String,
    pub tags: Vec<String>,
    pub transform: Transform,
    pub shape: CompiledShape,
    pub material: String,
    pub solid: bool,
    pub motion: Option<CompiledMotion>,
}

#[derive(Debug, Clone)]
pub(crate) enum CompiledShape {
    Box { size: Vec3 },
    Cylinder { radius: f32, height: f32 },
    Sphere { radius: f32 },
    Capsule { radius: f32, length: f32 },
    ConvexPrism { cross_section: Vec<Vec2>, depth: f32 },
}

#[derive(Debug, Clone)]
pub(crate) struct CompiledMotion {
    pub travel: Vec3,
    pub period_seconds: f32,
    pub phase: f32,
}

#[derive(Debug, Clone)]
pub(crate) struct CompiledMarker {
    pub id: String,
    pub zone: String,
    pub kind: String,
    pub tags: Vec<String>,
    pub transform: Transform,
}

#[derive(Debug, Clone)]
pub(crate) struct CompiledPointLight {
    pub id: String,
    pub zone: String,
    pub position: Vec3,
    pub color: Color,
    pub intensity: f32,
    pub range: f32,
    pub shadows: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct CompiledDirectionalLight {
    pub id: String,
    pub zone: String,
    pub rotation: Quat,
    pub color: Color,
    pub illuminance: f32,
    pub shadows: bool,
}

pub(crate) fn compile_map(map: &AuthoredMap) -> Vec<CompiledNode> {
    let mut output = Vec::new();

    for object in &map.objects {
        match object {
            MapObject::Box(value) => push_geometry(&mut output, compile_box(value, map.unit_scale)),
            MapObject::Ramp(value) => {
                push_geometry(&mut output, compile_ramp(value, map.unit_scale))
            }
            MapObject::Cylinder(value) => {
                push_geometry(&mut output, compile_cylinder(value, map.unit_scale))
            }
            MapObject::Sphere(value) => {
                push_geometry(&mut output, compile_sphere(value, map.unit_scale))
            }
            MapObject::Capsule(value) => {
                push_geometry(&mut output, compile_capsule(value, map.unit_scale))
            }
            MapObject::ConvexPrism(value) => {
                push_geometry(&mut output, compile_convex_prism(value, map.unit_scale))
            }
            MapObject::Staircase(value) => {
                compile_staircase(&mut output, value, map.unit_scale)
            }
            MapObject::StepSweep(value) => {
                compile_step_sweep(&mut output, value, map.unit_scale)
            }
            MapObject::SlopeSweep(value) => {
                compile_slope_sweep(&mut output, value, map.unit_scale)
            }
            MapObject::PillarGrid(value) => {
                compile_pillar_grid(&mut output, value, map.unit_scale)
            }
            MapObject::MovingBox(value) => {
                push_geometry(&mut output, compile_moving_box(value, map.unit_scale))
            }
            MapObject::Marker(value) => output.push(CompiledNode::Marker(CompiledMarker {
                id: value.id.clone(),
                zone: value.zone.clone(),
                kind: value.kind.clone(),
                tags: value.tags.clone(),
                transform: authored_transform(
                    value.position,
                    value.rotation_degrees,
                    map.unit_scale,
                ),
            })),
            MapObject::PointLight(value) => {
                output.push(CompiledNode::PointLight(compile_point_light(value, map.unit_scale)))
            }
            MapObject::DirectionalLight(value) => {
                output.push(CompiledNode::DirectionalLight(compile_directional_light(value)))
            }
        }

    }

    output
}

fn push_geometry(output: &mut Vec<CompiledNode>, geometry: CompiledGeometry) {
    output.push(CompiledNode::Geometry(geometry));
}

fn compile_box(value: &BoxDef, unit_scale: f32) -> CompiledGeometry {
    CompiledGeometry {
        id: value.id.clone(),
        zone: value.zone.clone(),
        tags: value.tags.clone(),
        transform: authored_transform(value.position, value.rotation_degrees, unit_scale),
        shape: CompiledShape::Box {
            size: v3(value.size) * unit_scale,
        },
        material: value.material.clone(),
        solid: value.solid,
        motion: None,
    }
}

fn compile_cylinder(value: &CylinderDef, unit_scale: f32) -> CompiledGeometry {
    CompiledGeometry {
        id: value.id.clone(),
        zone: value.zone.clone(),
        tags: value.tags.clone(),
        transform: authored_transform(value.position, value.rotation_degrees, unit_scale),
        shape: CompiledShape::Cylinder {
            radius: value.radius * unit_scale,
            height: value.height * unit_scale,
        },
        material: value.material.clone(),
        solid: value.solid,
        motion: None,
    }
}

fn compile_sphere(value: &SphereDef, unit_scale: f32) -> CompiledGeometry {
    CompiledGeometry {
        id: value.id.clone(),
        zone: value.zone.clone(),
        tags: value.tags.clone(),
        transform: Transform::from_translation(v3(value.position) * unit_scale),
        shape: CompiledShape::Sphere {
            radius: value.radius * unit_scale,
        },
        material: value.material.clone(),
        solid: value.solid,
        motion: None,
    }
}

fn compile_capsule(value: &CapsuleDef, unit_scale: f32) -> CompiledGeometry {
    CompiledGeometry {
        id: value.id.clone(),
        zone: value.zone.clone(),
        tags: value.tags.clone(),
        transform: authored_transform(value.position, value.rotation_degrees, unit_scale),
        shape: CompiledShape::Capsule {
            radius: value.radius * unit_scale,
            length: value.length * unit_scale,
        },
        material: value.material.clone(),
        solid: value.solid,
        motion: None,
    }
}

fn compile_convex_prism(value: &ConvexPrismDef, unit_scale: f32) -> CompiledGeometry {
    CompiledGeometry {
        id: value.id.clone(),
        zone: value.zone.clone(),
        tags: value.tags.clone(),
        transform: authored_transform(value.position, value.rotation_degrees, unit_scale),
        shape: CompiledShape::ConvexPrism {
            cross_section: value
                .cross_section
                .iter()
                .copied()
                .map(v2)
                .map(|point| point * unit_scale)
                .collect(),
            depth: value.depth * unit_scale,
        },
        material: value.material.clone(),
        solid: value.solid,
        motion: None,
    }
}

fn compile_ramp(value: &RampDef, unit_scale: f32) -> CompiledGeometry {
    let start = v3(value.start) * unit_scale;
    let width = value.width * unit_scale;
    let run = value.run * unit_scale;
    let rise = value.rise * unit_scale;
    let thickness = value.thickness * unit_scale;

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

fn compile_staircase(output: &mut Vec<CompiledNode>, value: &StaircaseDef, unit_scale: f32) {
    let origin = v3(value.origin) * unit_scale;
    let yaw = value.yaw_degrees.to_radians();
    let rotation = Quat::from_rotation_y(yaw);
    let forward = rotation * Vec3::Z;
    let width = value.width * unit_scale;
    let step_height = value.step_height * unit_scale;
    let step_depth = value.step_depth * unit_scale;

    for index in 0..value.steps {
        let height = step_height * (index + 1) as f32;
        let center = origin
            + forward * (step_depth * (index as f32 + 0.5))
            + Vec3::Y * (height * 0.5);

        push_geometry(
            output,
            CompiledGeometry {
                id: format!("{}/{index:03}", value.id),
                zone: value.zone.clone(),
                tags: value.tags.clone(),
                transform: Transform::from_translation(center).with_rotation(rotation),
                shape: CompiledShape::Box {
                    size: Vec3::new(width, height, step_depth),
                },
                material: value.material.clone(),
                solid: true,
                motion: None,
            },
        );
    }
}

fn compile_step_sweep(output: &mut Vec<CompiledNode>, value: &StepSweepDef, unit_scale: f32) {
    let origin = v3(value.origin) * unit_scale;
    let rotation = Quat::from_rotation_y(value.yaw_degrees.to_radians());
    let right = rotation * Vec3::X;
    let width = value.width * unit_scale;
    let depth = value.depth * unit_scale;
    let gap = value.gap * unit_scale;

    let mut height = value.start_height;
    let mut index = 0usize;
    while height <= value.end_height + value.increment * 0.25 {
        let world_height = height * unit_scale;
        let center = origin
            + right * ((width + gap) * index as f32)
            + Vec3::Y * (world_height * 0.5);

        push_geometry(
            output,
            CompiledGeometry {
                id: format!("{}/{index:03}_{height:.3}", value.id),
                zone: value.zone.clone(),
                tags: value.tags.clone(),
                transform: Transform::from_translation(center).with_rotation(rotation),
                shape: CompiledShape::Box {
                    size: Vec3::new(width, world_height, depth),
                },
                material: value.material.clone(),
                solid: true,
                motion: None,
            },
        );

        index += 1;
        height += value.increment;
    }
}

fn compile_slope_sweep(output: &mut Vec<CompiledNode>, value: &SlopeSweepDef, unit_scale: f32) {
    let base_origin = v3(value.origin);
    let mut angle = value.start_angle_degrees;
    let mut index = 0usize;

    while angle <= value.end_angle_degrees + value.angle_increment_degrees * 0.25 {
        let rise = value.run * angle.to_radians().tan();
        let lateral = index as f32 * (value.width + value.gap);
        let yaw_rotation = Quat::from_rotation_y(value.yaw_degrees.to_radians());
        let lateral_world = yaw_rotation * Vec3::X * lateral;
        let start = base_origin + lateral_world;

        let ramp = RampDef {
            id: format!("{}/{index:03}_{angle:.3}deg", value.id),
            zone: value.zone.clone(),
            start: tuple3(start),
            yaw_degrees: value.yaw_degrees,
            width: value.width,
            run: value.run,
            rise,
            thickness: value.thickness,
            material: value.material.clone(),
            solid: true,
            tags: value.tags.clone(),
        };

        push_geometry(output, compile_ramp(&ramp, unit_scale));

        index += 1;
        angle += value.angle_increment_degrees;
    }
}

fn compile_pillar_grid(output: &mut Vec<CompiledNode>, value: &PillarGridDef, unit_scale: f32) {
    let origin = v3(value.origin) * unit_scale;
    let rotation = Quat::from_rotation_y(value.yaw_degrees.to_radians());
    let right = rotation * Vec3::X;
    let forward = rotation * Vec3::Z;

    let width = value.pillar_width * unit_scale;
    let depth = value.pillar_depth * unit_scale;
    let height = value.pillar_height * unit_scale;
    let spacing_x = value.spacing_x * unit_scale;
    let spacing_z = value.spacing_z * unit_scale;

    for column in 0..value.columns {
        for row in 0..value.rows {
            let center = origin
                + right * (column as f32 * spacing_x)
                + forward * (row as f32 * spacing_z)
                + Vec3::Y * (height * 0.5);

            push_geometry(
                output,
                CompiledGeometry {
                    id: format!("{}/{column:02}_{row:02}", value.id),
                    zone: value.zone.clone(),
                    tags: value.tags.clone(),
                    transform: Transform::from_translation(center).with_rotation(rotation),
                    shape: CompiledShape::Box {
                        size: Vec3::new(width, height, depth),
                    },
                    material: value.material.clone(),
                    solid: true,
                    motion: None,
                },
            );
        }
    }
}

fn compile_moving_box(value: &MovingBoxDef, unit_scale: f32) -> CompiledGeometry {
    CompiledGeometry {
        id: value.id.clone(),
        zone: value.zone.clone(),
        tags: value.tags.clone(),
        transform: authored_transform(value.position, value.rotation_degrees, unit_scale),
        shape: CompiledShape::Box {
            size: v3(value.size) * unit_scale,
        },
        material: value.material.clone(),
        solid: true,
        motion: Some(CompiledMotion {
            travel: v3(value.travel) * unit_scale,
            period_seconds: value.period_seconds,
            phase: value.phase,
        }),
    }
}

fn compile_point_light(value: &PointLightDef, unit_scale: f32) -> CompiledPointLight {
    CompiledPointLight {
        id: value.id.clone(),
        zone: value.zone.clone(),
        position: v3(value.position) * unit_scale,
        color: Color::srgb(value.color.0, value.color.1, value.color.2),
        intensity: value.intensity,
        range: value.range * unit_scale,
        shadows: value.shadows,
    }
}

fn compile_directional_light(value: &DirectionalLightDef) -> CompiledDirectionalLight {
    CompiledDirectionalLight {
        id: value.id.clone(),
        zone: value.zone.clone(),
        rotation: authored_rotation(value.rotation_degrees),
        color: Color::srgb(value.color.0, value.color.1, value.color.2),
        illuminance: value.illuminance,
        shadows: value.shadows,
    }
}

fn authored_transform(position: V3, rotation_degrees: V3, unit_scale: f32) -> Transform {
    Transform::from_translation(v3(position) * unit_scale)
        .with_rotation(authored_rotation(rotation_degrees))
}

fn authored_rotation(rotation_degrees: V3) -> Quat {
    Quat::from_euler(
        EulerRot::YXZ,
        rotation_degrees.1.to_radians(),
        rotation_degrees.0.to_radians(),
        rotation_degrees.2.to_radians(),
    )
}

fn v3(value: V3) -> Vec3 {
    Vec3::new(value.0, value.1, value.2)
}

fn v2(value: V2) -> Vec2 {
    Vec2::new(value.0, value.1)
}

fn tuple3(value: Vec3) -> V3 {
    (value.x, value.y, value.z)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn staircase_generates_one_solid_column_per_tread() {
        let definition = StaircaseDef {
            id: "stairs".into(),
            zone: "test".into(),
            origin: (0.0, 0.0, 0.0),
            yaw_degrees: 0.0,
            steps: 3,
            width: 100.0,
            step_height: 10.0,
            step_depth: 20.0,
            material: "concrete".into(),
            tags: Vec::new(),
        };

        let mut output = Vec::new();
        compile_staircase(&mut output, &definition, 1.0);

        assert_eq!(output.len(), 3);
        let CompiledNode::Geometry(third) = &output[2] else {
            panic!("expected geometry");
        };
        let CompiledShape::Box { size } = &third.shape else {
            panic!("expected box");
        };
        assert_eq!(size.y, 30.0);
        assert_eq!(third.transform.translation, Vec3::new(0.0, 15.0, 50.0));
    }

    #[test]
    fn ramp_top_surface_starts_at_authored_start() {
        let definition = RampDef {
            id: "ramp".into(),
            zone: "test".into(),
            start: (0.0, 0.0, 0.0),
            yaw_degrees: 0.0,
            width: 100.0,
            run: 100.0,
            rise: 100.0,
            thickness: 10.0,
            material: "concrete".into(),
            solid: true,
            tags: Vec::new(),
        };

        let ramp = compile_ramp(&definition, 1.0);
        let local_up = ramp.transform.rotation * Vec3::Y;
        let CompiledShape::Box { size } = ramp.shape else {
            panic!("expected box");
        };
        let top_surface_center = ramp.transform.translation + local_up * size.y * 0.5;

        assert!((top_surface_center - Vec3::new(0.0, 50.0, 50.0)).length() < 0.0001);
    }
}
