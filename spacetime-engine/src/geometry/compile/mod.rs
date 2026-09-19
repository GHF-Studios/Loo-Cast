use bevy::prelude::*;

use super::asset::{
    AuthoredMap, BoxDef, CapsuleDef, ConvexPrismDef, CylinderDef, DirectionalLightDef, MapObject,
    MovingBoxDef, PillarGridDef, PointLightDef, RampDef, SlopeSweepDef, SphereDef, StaircaseDef,
    StepSweepDef, V2, V3,
};

#[derive(Debug, Clone)]
pub(super) enum CompiledNode {
    Geometry(CompiledGeometry),
    Marker(CompiledMarker),
    PointLight(CompiledPointLight),
    DirectionalLight(CompiledDirectionalLight),
}

#[derive(Debug, Clone)]
pub(super) struct CompiledGeometry {
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
pub(super) enum CompiledShape {
    Box {
        size: Vec3,
    },
    Cylinder {
        radius: f32,
        height: f32,
    },
    Sphere {
        radius: f32,
    },
    Capsule {
        radius: f32,
        length: f32,
    },
    ConvexPrism {
        cross_section: Vec<Vec2>,
        depth: f32,
    },
}

#[derive(Debug, Clone)]
pub(super) struct CompiledMotion {
    pub travel: Vec3,
    pub period_seconds: f32,
    pub phase: f32,
}

#[derive(Debug, Clone)]
pub(super) struct CompiledMarker {
    pub id: String,
    pub zone: String,
    pub kind: String,
    pub tags: Vec<String>,
    pub transform: Transform,
}

#[derive(Debug, Clone)]
pub(super) struct CompiledPointLight {
    pub id: String,
    pub zone: String,
    pub position: Vec3,
    pub color: Color,
    pub intensity: f32,
    pub range: f32,
    pub shadows: bool,
}

#[derive(Debug, Clone)]
pub(super) struct CompiledDirectionalLight {
    pub id: String,
    pub zone: String,
    pub rotation: Quat,
    pub color: Color,
    pub illuminance: f32,
    pub shadows: bool,
}

mod generators;
mod lighting;
mod primitives;

use generators::{
    compile_pillar_grid, compile_slope_sweep, compile_staircase, compile_step_sweep,
};
use lighting::{compile_directional_light, compile_point_light};
use primitives::{
    compile_box, compile_capsule, compile_convex_prism, compile_cylinder,
    compile_moving_box, compile_ramp, compile_sphere,
};

pub(super) fn compile_map(map: &AuthoredMap) -> Vec<CompiledNode> {
    let mut output = Vec::new();

    for object in &map.objects {
        match object {
            MapObject::Box(value) => push_geometry(&mut output, compile_box(value)),
            MapObject::Ramp(value) => push_geometry(&mut output, compile_ramp(value)),
            MapObject::Cylinder(value) => push_geometry(&mut output, compile_cylinder(value)),
            MapObject::Sphere(value) => push_geometry(&mut output, compile_sphere(value)),
            MapObject::Capsule(value) => push_geometry(&mut output, compile_capsule(value)),
            MapObject::ConvexPrism(value) => {
                push_geometry(&mut output, compile_convex_prism(value))
            }
            MapObject::Staircase(value) => compile_staircase(&mut output, value),
            MapObject::StepSweep(value) => compile_step_sweep(&mut output, value),
            MapObject::SlopeSweep(value) => compile_slope_sweep(&mut output, value),
            MapObject::PillarGrid(value) => compile_pillar_grid(&mut output, value),
            MapObject::MovingBox(value) => push_geometry(&mut output, compile_moving_box(value)),
            MapObject::Marker(value) => output.push(CompiledNode::Marker(CompiledMarker {
                id: value.id.clone(),
                zone: value.zone.clone(),
                kind: value.kind.clone(),
                tags: value.tags.clone(),
                transform: authored_transform(value.position, value.rotation_degrees),
            })),
            MapObject::PointLight(value) => {
                output.push(CompiledNode::PointLight(compile_point_light(value)))
            }
            MapObject::DirectionalLight(value) => output.push(CompiledNode::DirectionalLight(
                compile_directional_light(value),
            )),
        }
    }

    output
}

fn push_geometry(output: &mut Vec<CompiledNode>, geometry: CompiledGeometry) {
    output.push(CompiledNode::Geometry(geometry));
}

fn authored_transform(position: V3, rotation_degrees: V3) -> Transform {
    Transform::from_translation(v3(position)).with_rotation(authored_rotation(rotation_degrees))
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
mod tests;
