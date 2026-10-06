//! Compiled geometry intermediate representation consumed by runtime spawning.

use bevy::prelude::*;

#[derive(Debug, Clone)]
pub(in crate::geometry) enum CompiledNode {
    Geometry(CompiledGeometry),
    Marker(CompiledMarker),
    PointLight(CompiledPointLight),
    DirectionalLight(CompiledDirectionalLight),
}

#[derive(Debug, Clone)]
pub(in crate::geometry) struct CompiledGeometry {
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
pub(in crate::geometry) enum CompiledShape {
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
pub(in crate::geometry) struct CompiledMotion {
    pub travel: Vec3,
    pub period_seconds: f32,
    pub phase: f32,
}

#[derive(Debug, Clone)]
pub(in crate::geometry) struct CompiledMarker {
    pub id: String,
    pub zone: String,
    pub kind: String,
    pub tags: Vec<String>,
    pub transform: Transform,
}

#[derive(Debug, Clone)]
pub(in crate::geometry) struct CompiledPointLight {
    pub id: String,
    pub zone: String,
    pub position: Vec3,
    pub color: Color,
    pub intensity: f32,
    pub range: f32,
    pub shadows: bool,
}

#[derive(Debug, Clone)]
pub(in crate::geometry) struct CompiledDirectionalLight {
    pub id: String,
    pub zone: String,
    pub rotation: Quat,
    pub color: Color,
    pub illuminance: f32,
    pub shadows: bool,
}
