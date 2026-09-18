//! Deserializable schema for human-authored geometry maps.

use bevy::{prelude::*, reflect::TypePath};
use serde::Deserialize;

use super::{V2, V3};

/// Human-authored geometry map. All distance-valued fields are metres.
#[derive(Asset, TypePath, Debug, Deserialize)]
pub struct AuthoredMap {
    pub name: String,

    #[serde(default)]
    pub zones: Vec<ZoneDef>,

    #[serde(default)]
    pub materials: Vec<MaterialDef>,

    #[serde(default)]
    pub objects: Vec<MapObject>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ZoneDef {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MaterialDef {
    pub id: String,
    pub color: V3,

    #[serde(default)]
    pub metallic: f32,

    #[serde(default = "default_roughness")]
    pub roughness: f32,
}

fn default_roughness() -> f32 {
    0.85
}

#[derive(Debug, Clone, Deserialize)]
pub enum MapObject {
    Box(BoxDef),
    Ramp(RampDef),
    Cylinder(CylinderDef),
    Sphere(SphereDef),
    Capsule(CapsuleDef),
    ConvexPrism(ConvexPrismDef),
    Staircase(StaircaseDef),
    StepSweep(StepSweepDef),
    SlopeSweep(SlopeSweepDef),
    PillarGrid(PillarGridDef),
    MovingBox(MovingBoxDef),
    Marker(MarkerDef),
    PointLight(PointLightDef),
    DirectionalLight(DirectionalLightDef),
}

#[derive(Debug, Clone, Deserialize)]
pub struct BoxDef {
    pub id: String,
    #[serde(default)]
    pub zone: String,
    pub position: V3,
    #[serde(default)]
    pub rotation_degrees: V3,
    pub size: V3,
    pub material: String,
    #[serde(default = "default_true")]
    pub solid: bool,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RampDef {
    pub id: String,
    #[serde(default)]
    pub zone: String,
    /// Center of the lower edge of the *walkable top surface*.
    pub start: V3,
    #[serde(default)]
    pub yaw_degrees: f32,
    pub width: f32,
    pub run: f32,
    pub rise: f32,
    #[serde(default = "default_slab_thickness")]
    pub thickness: f32,
    pub material: String,
    #[serde(default = "default_true")]
    pub solid: bool,
    #[serde(default)]
    pub tags: Vec<String>,
}

fn default_slab_thickness() -> f32 {
    0.1016
}

#[derive(Debug, Clone, Deserialize)]
pub struct CylinderDef {
    pub id: String,
    #[serde(default)]
    pub zone: String,
    pub position: V3,
    #[serde(default)]
    pub rotation_degrees: V3,
    pub radius: f32,
    pub height: f32,
    pub material: String,
    #[serde(default = "default_true")]
    pub solid: bool,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SphereDef {
    pub id: String,
    #[serde(default)]
    pub zone: String,
    pub position: V3,
    pub radius: f32,
    pub material: String,
    #[serde(default = "default_true")]
    pub solid: bool,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CapsuleDef {
    pub id: String,
    #[serde(default)]
    pub zone: String,
    pub position: V3,
    #[serde(default)]
    pub rotation_degrees: V3,
    pub radius: f32,
    /// Straight segment length between the hemispherical ends.
    pub length: f32,
    pub material: String,
    #[serde(default = "default_true")]
    pub solid: bool,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConvexPrismDef {
    pub id: String,
    #[serde(default)]
    pub zone: String,
    pub position: V3,
    #[serde(default)]
    pub rotation_degrees: V3,
    /// Convex XY polygon, in winding order, extruded symmetrically along local Z.
    pub cross_section: Vec<V2>,
    pub depth: f32,
    pub material: String,
    #[serde(default = "default_true")]
    pub solid: bool,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StaircaseDef {
    pub id: String,
    #[serde(default)]
    pub zone: String,
    /// Center of the first tread's front edge at ground height.
    pub origin: V3,
    #[serde(default)]
    pub yaw_degrees: f32,
    pub steps: usize,
    pub width: f32,
    pub step_height: f32,
    pub step_depth: f32,
    pub material: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StepSweepDef {
    pub id: String,
    #[serde(default)]
    pub zone: String,
    /// Center of the first block on the floor. Blocks proceed along local +X.
    pub origin: V3,
    #[serde(default)]
    pub yaw_degrees: f32,
    pub start_height: f32,
    pub end_height: f32,
    pub increment: f32,
    pub width: f32,
    pub depth: f32,
    #[serde(default)]
    pub gap: f32,
    pub material: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SlopeSweepDef {
    pub id: String,
    #[serde(default)]
    pub zone: String,
    /// Lower surface start of the first ramp. Ramps proceed along local +X.
    pub origin: V3,
    #[serde(default)]
    pub yaw_degrees: f32,
    pub start_angle_degrees: f32,
    pub end_angle_degrees: f32,
    pub angle_increment_degrees: f32,
    pub run: f32,
    pub width: f32,
    #[serde(default)]
    pub gap: f32,
    #[serde(default = "default_slab_thickness")]
    pub thickness: f32,
    pub material: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PillarGridDef {
    pub id: String,
    #[serde(default)]
    pub zone: String,
    pub origin: V3,
    #[serde(default)]
    pub yaw_degrees: f32,
    pub columns: usize,
    pub rows: usize,
    pub spacing_x: f32,
    pub spacing_z: f32,
    pub pillar_width: f32,
    pub pillar_depth: f32,
    pub pillar_height: f32,
    pub material: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MovingBoxDef {
    pub id: String,
    #[serde(default)]
    pub zone: String,
    pub position: V3,
    #[serde(default)]
    pub rotation_degrees: V3,
    pub size: V3,
    /// World-space travel from the base position to the far endpoint.
    pub travel: V3,
    pub period_seconds: f32,
    #[serde(default)]
    pub phase: f32,
    pub material: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MarkerDef {
    pub id: String,
    #[serde(default)]
    pub zone: String,
    pub kind: String,
    pub position: V3,
    #[serde(default)]
    pub rotation_degrees: V3,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PointLightDef {
    pub id: String,
    #[serde(default)]
    pub zone: String,
    pub position: V3,
    pub color: V3,
    pub intensity: f32,
    pub range: f32,
    #[serde(default = "default_true")]
    pub shadows: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DirectionalLightDef {
    pub id: String,
    #[serde(default)]
    pub zone: String,
    #[serde(default)]
    pub rotation_degrees: V3,
    pub color: V3,
    pub illuminance: f32,
    #[serde(default = "default_true")]
    pub shadows: bool,
}

fn default_true() -> bool {
    true
}
