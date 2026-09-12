use std::{collections::HashSet, io};

use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
    reflect::TypePath,
};
use serde::Deserialize;

pub(crate) const MAX_GENERATED_OBJECTS: usize = 20_000;

pub type V3 = (f32, f32, f32);
pub type V2 = (f32, f32);

#[derive(Asset, TypePath, Debug, Deserialize)]
pub struct AuthoredMap {
    pub name: String,

    /// Conversion from one authored distance unit into one Bevy world unit.
    /// The playground campus uses Source inches: 0.0254 metres per unit.
    pub unit_scale: f32,

    #[serde(default)]
    pub zones: Vec<ZoneDef>,

    #[serde(default)]
    pub materials: Vec<MaterialDef>,

    #[serde(default)]
    pub objects: Vec<MapObject>,
}

impl AuthoredMap {
    pub fn validate(&self) -> Result<(), String> {
        if !self.unit_scale.is_finite() || self.unit_scale <= 0.0 {
            return Err("unit_scale must be finite and positive".into());
        }

        let mut zone_ids = HashSet::new();
        for zone in &self.zones {
            if zone.id.trim().is_empty() {
                return Err("zone id must not be empty".into());
            }
            if !zone_ids.insert(zone.id.clone()) {
                return Err(format!("duplicate zone id {:?}", zone.id));
            }
        }

        let mut material_ids = HashSet::new();
        for material in &self.materials {
            if material.id.trim().is_empty() {
                return Err("material id must not be empty".into());
            }
            if !material_ids.insert(material.id.clone()) {
                return Err(format!("duplicate material id {:?}", material.id));
            }
            validate_color(material.color, &format!("material {:?}", material.id))?;
            if !material.metallic.is_finite() || !(0.0..=1.0).contains(&material.metallic) {
                return Err(format!("material {:?} metallic must be in 0..=1", material.id));
            }
            if !material.roughness.is_finite() || !(0.0..=1.0).contains(&material.roughness) {
                return Err(format!("material {:?} roughness must be in 0..=1", material.id));
            }
        }

        let mut object_ids = HashSet::new();
        let mut generated_objects = 0usize;
        for object in &self.objects {
            let id = object.id();
            if id.trim().is_empty() {
                return Err("object id must not be empty".into());
            }
            if !object_ids.insert(id.to_string()) {
                return Err(format!("duplicate object id {id:?}"));
            }

            let zone = object.zone();
            if !zone.is_empty() && !zone_ids.contains(zone) {
                return Err(format!("object {id:?} references unknown zone {zone:?}"));
            }

            if let Some(material) = object.material() {
                if !material_ids.contains(material) {
                    return Err(format!(
                        "object {id:?} references unknown material {material:?}"
                    ));
                }
            }

            object.validate()?;

            generated_objects = generated_objects
                .checked_add(object.generated_count()?)
                .ok_or_else(|| "generated object count overflowed usize".to_string())?;
            if generated_objects > MAX_GENERATED_OBJECTS {
                return Err(format!(
                    "map generates {generated_objects} runtime objects; maximum is {MAX_GENERATED_OBJECTS}"
                ));
            }
        }

        Ok(())
    }
}

fn validate_color(color: V3, context: &str) -> Result<(), String> {
    let (r, g, b) = color;
    if [r, g, b]
        .into_iter()
        .any(|value| !value.is_finite() || !(0.0..=1.0).contains(&value))
    {
        return Err(format!("{context} color channels must be finite and in 0..=1"));
    }
    Ok(())
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

impl MapObject {
    pub fn id(&self) -> &str {
        match self {
            Self::Box(value) => &value.id,
            Self::Ramp(value) => &value.id,
            Self::Cylinder(value) => &value.id,
            Self::Sphere(value) => &value.id,
            Self::Capsule(value) => &value.id,
            Self::ConvexPrism(value) => &value.id,
            Self::Staircase(value) => &value.id,
            Self::StepSweep(value) => &value.id,
            Self::SlopeSweep(value) => &value.id,
            Self::PillarGrid(value) => &value.id,
            Self::MovingBox(value) => &value.id,
            Self::Marker(value) => &value.id,
            Self::PointLight(value) => &value.id,
            Self::DirectionalLight(value) => &value.id,
        }
    }


    pub fn zone(&self) -> &str {
        match self {
            Self::Box(value) => &value.zone,
            Self::Ramp(value) => &value.zone,
            Self::Cylinder(value) => &value.zone,
            Self::Sphere(value) => &value.zone,
            Self::Capsule(value) => &value.zone,
            Self::ConvexPrism(value) => &value.zone,
            Self::Staircase(value) => &value.zone,
            Self::StepSweep(value) => &value.zone,
            Self::SlopeSweep(value) => &value.zone,
            Self::PillarGrid(value) => &value.zone,
            Self::MovingBox(value) => &value.zone,
            Self::Marker(value) => &value.zone,
            Self::PointLight(value) => &value.zone,
            Self::DirectionalLight(value) => &value.zone,
        }
    }

    pub fn material(&self) -> Option<&str> {
        match self {
            Self::Box(value) => Some(&value.material),
            Self::Ramp(value) => Some(&value.material),
            Self::Cylinder(value) => Some(&value.material),
            Self::Sphere(value) => Some(&value.material),
            Self::Capsule(value) => Some(&value.material),
            Self::ConvexPrism(value) => Some(&value.material),
            Self::Staircase(value) => Some(&value.material),
            Self::StepSweep(value) => Some(&value.material),
            Self::SlopeSweep(value) => Some(&value.material),
            Self::PillarGrid(value) => Some(&value.material),
            Self::MovingBox(value) => Some(&value.material),
            Self::Marker(_) | Self::PointLight(_) | Self::DirectionalLight(_) => None,
        }
    }

    fn generated_count(&self) -> Result<usize, String> {
        match self {
            Self::Staircase(value) => Ok(value.steps),
            Self::StepSweep(value) => sweep_count(
                value.start_height,
                value.end_height,
                value.increment,
                &value.id,
            ),
            Self::SlopeSweep(value) => sweep_count(
                value.start_angle_degrees,
                value.end_angle_degrees,
                value.angle_increment_degrees,
                &value.id,
            ),
            Self::PillarGrid(value) => value
                .columns
                .checked_mul(value.rows)
                .ok_or_else(|| format!("pillar grid {:?} object count overflowed usize", value.id)),
            _ => Ok(1),
        }
    }

    fn validate(&self) -> Result<(), String> {
        match self {
            Self::Box(value) => {
                finite_v3(value.position, &value.id, "position")?;
                finite_v3(value.rotation_degrees, &value.id, "rotation_degrees")?;
                positive_v3(value.size, &value.id, "size")
            },
            Self::Ramp(value) => {
                finite_v3(value.start, &value.id, "start")?;
                finite(value.yaw_degrees, &value.id, "yaw_degrees")?;
                positive(value.width, &value.id, "width")?;
                positive(value.run, &value.id, "run")?;
                positive(value.thickness, &value.id, "thickness")?;
                finite(value.rise, &value.id, "rise")
            }
            Self::Cylinder(value) => {
                finite_v3(value.position, &value.id, "position")?;
                finite_v3(value.rotation_degrees, &value.id, "rotation_degrees")?;
                positive(value.radius, &value.id, "radius")?;
                positive(value.height, &value.id, "height")
            }
            Self::Sphere(value) => {
                finite_v3(value.position, &value.id, "position")?;
                positive(value.radius, &value.id, "radius")
            }
            Self::Capsule(value) => {
                finite_v3(value.position, &value.id, "position")?;
                finite_v3(value.rotation_degrees, &value.id, "rotation_degrees")?;
                positive(value.radius, &value.id, "radius")?;
                positive(value.length, &value.id, "length")
            }
            Self::ConvexPrism(value) => {
                finite_v3(value.position, &value.id, "position")?;
                finite_v3(value.rotation_degrees, &value.id, "rotation_degrees")?;
                positive(value.depth, &value.id, "depth")?;
                if value.cross_section.len() < 3 {
                    return Err(format!(
                        "convex prism {:?} needs at least three cross-section points",
                        value.id
                    ));
                }
                for (x, y) in &value.cross_section {
                    if !x.is_finite() || !y.is_finite() {
                        return Err(format!(
                            "convex prism {:?} has a non-finite cross-section point",
                            value.id
                        ));
                    }
                }
                validate_convex_polygon(&value.cross_section, &value.id)
            }
            Self::Staircase(value) => {
                finite_v3(value.origin, &value.id, "origin")?;
                finite(value.yaw_degrees, &value.id, "yaw_degrees")?;
                positive_usize(value.steps, &value.id, "steps")?;
                positive(value.width, &value.id, "width")?;
                positive(value.step_height, &value.id, "step_height")?;
                positive(value.step_depth, &value.id, "step_depth")
            }
            Self::StepSweep(value) => {
                finite_v3(value.origin, &value.id, "origin")?;
                finite(value.yaw_degrees, &value.id, "yaw_degrees")?;
                positive(value.start_height, &value.id, "start_height")?;
                positive(value.end_height, &value.id, "end_height")?;
                positive(value.increment, &value.id, "increment")?;
                positive(value.width, &value.id, "width")?;
                positive(value.depth, &value.id, "depth")?;
                non_negative(value.gap, &value.id, "gap")?;
                if value.end_height < value.start_height {
                    return Err(format!(
                        "step sweep {:?} end_height must be >= start_height",
                        value.id
                    ));
                }
                Ok(())
            }
            Self::SlopeSweep(value) => {
                finite_v3(value.origin, &value.id, "origin")?;
                finite(value.yaw_degrees, &value.id, "yaw_degrees")?;
                positive(value.run, &value.id, "run")?;
                positive(value.width, &value.id, "width")?;
                positive(value.thickness, &value.id, "thickness")?;
                positive(value.angle_increment_degrees, &value.id, "angle_increment_degrees")?;
                non_negative(value.gap, &value.id, "gap")?;
                finite(value.start_angle_degrees, &value.id, "start_angle_degrees")?;
                finite(value.end_angle_degrees, &value.id, "end_angle_degrees")?;
                if value.start_angle_degrees.abs() >= 89.9
                    || value.end_angle_degrees.abs() >= 89.9
                {
                    return Err(format!(
                        "slope sweep {:?} angles must remain between -89.9 and 89.9 degrees",
                        value.id
                    ));
                }
                if value.end_angle_degrees < value.start_angle_degrees {
                    return Err(format!(
                        "slope sweep {:?} end angle must be >= start angle",
                        value.id
                    ));
                }
                Ok(())
            }
            Self::PillarGrid(value) => {
                finite_v3(value.origin, &value.id, "origin")?;
                finite(value.yaw_degrees, &value.id, "yaw_degrees")?;
                positive_usize(value.columns, &value.id, "columns")?;
                positive_usize(value.rows, &value.id, "rows")?;
                positive(value.spacing_x, &value.id, "spacing_x")?;
                positive(value.spacing_z, &value.id, "spacing_z")?;
                positive(value.pillar_width, &value.id, "pillar_width")?;
                positive(value.pillar_depth, &value.id, "pillar_depth")?;
                positive(value.pillar_height, &value.id, "pillar_height")
            }
            Self::MovingBox(value) => {
                finite_v3(value.position, &value.id, "position")?;
                finite_v3(value.rotation_degrees, &value.id, "rotation_degrees")?;
                finite_v3(value.travel, &value.id, "travel")?;
                positive_v3(value.size, &value.id, "size")?;
                positive(value.period_seconds, &value.id, "period_seconds")?;
                finite(value.phase, &value.id, "phase")
            }
            Self::Marker(value) => {
                finite_v3(value.position, &value.id, "position")?;
                finite_v3(value.rotation_degrees, &value.id, "rotation_degrees")
            },
            Self::PointLight(value) => {
                finite_v3(value.position, &value.id, "position")?;
                validate_color(value.color, &format!("point light {:?}", value.id))?;
                positive(value.intensity, &value.id, "intensity")?;
                positive(value.range, &value.id, "range")
            }
            Self::DirectionalLight(value) => {
                finite_v3(value.rotation_degrees, &value.id, "rotation_degrees")?;
                validate_color(value.color, &format!("directional light {:?}", value.id))?;
                positive(value.illuminance, &value.id, "illuminance")
            }
        }
    }
}

fn validate_convex_polygon(points: &[V2], id: &str) -> Result<(), String> {
    let mut winding = 0.0f32;

    for index in 0..points.len() {
        let a = points[index];
        let b = points[(index + 1) % points.len()];
        let c = points[(index + 2) % points.len()];
        let ab = (b.0 - a.0, b.1 - a.1);
        let bc = (c.0 - b.0, c.1 - b.1);
        let cross = ab.0 * bc.1 - ab.1 * bc.0;

        if cross.abs() <= f32::EPSILON {
            continue;
        }

        if winding == 0.0 {
            winding = cross.signum();
        } else if cross.signum() != winding {
            return Err(format!(
                "convex prism {id:?} cross-section is not convex or not in winding order"
            ));
        }
    }

    if winding == 0.0 {
        return Err(format!("convex prism {id:?} cross-section is degenerate"));
    }

    Ok(())
}

fn sweep_count(start: f32, end: f32, increment: f32, id: &str) -> Result<usize, String> {
    if !start.is_finite() || !end.is_finite() || !increment.is_finite() || increment <= 0.0 {
        return Err(format!("sweep {id:?} has invalid range"));
    }
    if end < start {
        return Err(format!("sweep {id:?} end must be >= start"));
    }

    let count = ((end - start) / increment).floor() + 1.0;
    if !count.is_finite() || count < 1.0 || count > usize::MAX as f32 {
        return Err(format!("sweep {id:?} object count is invalid"));
    }
    Ok(count as usize)
}

fn finite_v3(value: V3, id: &str, field: &str) -> Result<(), String> {
    finite(value.0, id, field)?;
    finite(value.1, id, field)?;
    finite(value.2, id, field)
}

fn finite(value: f32, id: &str, field: &str) -> Result<(), String> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(format!("object {id:?} field {field} must be finite"))
    }
}

fn positive(value: f32, id: &str, field: &str) -> Result<(), String> {
    finite(value, id, field)?;
    if value > 0.0 {
        Ok(())
    } else {
        Err(format!("object {id:?} field {field} must be positive"))
    }
}

fn non_negative(value: f32, id: &str, field: &str) -> Result<(), String> {
    finite(value, id, field)?;
    if value >= 0.0 {
        Ok(())
    } else {
        Err(format!("object {id:?} field {field} must be non-negative"))
    }
}

fn positive_usize(value: usize, id: &str, field: &str) -> Result<(), String> {
    if value > 0 {
        Ok(())
    } else {
        Err(format!("object {id:?} field {field} must be positive"))
    }
}

fn positive_v3(value: V3, id: &str, field: &str) -> Result<(), String> {
    positive(value.0, id, field)?;
    positive(value.1, id, field)?;
    positive(value.2, id, field)
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
    4.0
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

#[derive(Default, TypePath)]
pub struct AuthoredMapLoader;

impl AssetLoader for AuthoredMapLoader {
    type Asset = AuthoredMap;
    type Settings = ();
    type Error = io::Error;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let map: AuthoredMap = ron::de::from_bytes(&bytes).map_err(|error| {
            io::Error::new(io::ErrorKind::InvalidData, error.to_string())
        })?;

        map.validate().map_err(|error| {
            io::Error::new(io::ErrorKind::InvalidData, error)
        })?;

        Ok(map)
    }

    fn extensions(&self) -> &[&str] {
        &["spacemap"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn physics_campus_asset_parses_and_validates() {
        let bytes = include_bytes!("../../assets/maps/physics_campus.spacemap");
        let map: AuthoredMap = ron::de::from_bytes(bytes).expect("campus RON should parse");

        map.validate().expect("campus should satisfy authored-map invariants");
        assert!(map.objects.len() >= 150);
        assert!(map.zones.len() >= 10);
    }
}
