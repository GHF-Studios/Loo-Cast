//! Object identity and per-kind authored geometry constraints.

use super::super::MapObject;
use super::constraints::sweep_count;
use super::definitions::ValidateDefinition;

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

    pub(super) fn generated_count(&self) -> Result<usize, String> {
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

    pub(super) fn validate(&self) -> Result<(), String> {
        match self {
            Self::Box(value) => value.validate_definition(),
            Self::Ramp(value) => value.validate_definition(),
            Self::Cylinder(value) => value.validate_definition(),
            Self::Sphere(value) => value.validate_definition(),
            Self::Capsule(value) => value.validate_definition(),
            Self::ConvexPrism(value) => value.validate_definition(),
            Self::Staircase(value) => value.validate_definition(),
            Self::StepSweep(value) => value.validate_definition(),
            Self::SlopeSweep(value) => value.validate_definition(),
            Self::PillarGrid(value) => value.validate_definition(),
            Self::MovingBox(value) => value.validate_definition(),
            Self::Marker(value) => value.validate_definition(),
            Self::PointLight(value) => value.validate_definition(),
            Self::DirectionalLight(value) => value.validate_definition(),
        }
    }
}
