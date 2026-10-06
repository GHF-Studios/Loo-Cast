//! Validation rules owned by each authored geometry definition.

use super::super::*;
use super::constraints::*;

pub(super) trait ValidateDefinition {
    fn validate_definition(&self) -> Result<(), String>;
}

impl ValidateDefinition for BoxDef {
    fn validate_definition(&self) -> Result<(), String> {
        finite_v3(self.position, &self.id, "position")?;
        finite_v3(self.rotation_degrees, &self.id, "rotation_degrees")?;
        positive_v3(self.size, &self.id, "size")
    }
}

impl ValidateDefinition for RampDef {
    fn validate_definition(&self) -> Result<(), String> {
        finite_v3(self.start, &self.id, "start")?;
        finite(self.yaw_degrees, &self.id, "yaw_degrees")?;
        positive(self.width, &self.id, "width")?;
        positive(self.run, &self.id, "run")?;
        positive(self.thickness, &self.id, "thickness")?;
        finite(self.rise, &self.id, "rise")
    }
}

impl ValidateDefinition for CylinderDef {
    fn validate_definition(&self) -> Result<(), String> {
        finite_v3(self.position, &self.id, "position")?;
        finite_v3(self.rotation_degrees, &self.id, "rotation_degrees")?;
        positive(self.radius, &self.id, "radius")?;
        positive(self.height, &self.id, "height")
    }
}

impl ValidateDefinition for SphereDef {
    fn validate_definition(&self) -> Result<(), String> {
        finite_v3(self.position, &self.id, "position")?;
        positive(self.radius, &self.id, "radius")
    }
}

impl ValidateDefinition for CapsuleDef {
    fn validate_definition(&self) -> Result<(), String> {
        finite_v3(self.position, &self.id, "position")?;
        finite_v3(self.rotation_degrees, &self.id, "rotation_degrees")?;
        positive(self.radius, &self.id, "radius")?;
        positive(self.length, &self.id, "length")
    }
}

impl ValidateDefinition for ConvexPrismDef {
    fn validate_definition(&self) -> Result<(), String> {
        finite_v3(self.position, &self.id, "position")?;
        finite_v3(self.rotation_degrees, &self.id, "rotation_degrees")?;
        positive(self.depth, &self.id, "depth")?;
        if self.cross_section.len() < 3 {
            return Err(format!(
                "convex prism {:?} needs at least three cross-section points",
                self.id
            ));
        }
        for (x, y) in &self.cross_section {
            if !x.is_finite() || !y.is_finite() {
                return Err(format!(
                    "convex prism {:?} has a non-finite cross-section point",
                    self.id
                ));
            }
        }
        validate_convex_polygon(&self.cross_section, &self.id)
    }
}

impl ValidateDefinition for StaircaseDef {
    fn validate_definition(&self) -> Result<(), String> {
        finite_v3(self.origin, &self.id, "origin")?;
        finite(self.yaw_degrees, &self.id, "yaw_degrees")?;
        positive_usize(self.steps, &self.id, "steps")?;
        positive(self.width, &self.id, "width")?;
        positive(self.step_height, &self.id, "step_height")?;
        positive(self.step_depth, &self.id, "step_depth")
    }
}

impl ValidateDefinition for StepSweepDef {
    fn validate_definition(&self) -> Result<(), String> {
        finite_v3(self.origin, &self.id, "origin")?;
        finite(self.yaw_degrees, &self.id, "yaw_degrees")?;
        positive(self.start_height, &self.id, "start_height")?;
        positive(self.end_height, &self.id, "end_height")?;
        positive(self.increment, &self.id, "increment")?;
        positive(self.width, &self.id, "width")?;
        positive(self.depth, &self.id, "depth")?;
        non_negative(self.gap, &self.id, "gap")?;
        if self.end_height < self.start_height {
            return Err(format!(
                "step sweep {:?} end_height must be >= start_height",
                self.id
            ));
        }
        Ok(())
    }
}

impl ValidateDefinition for SlopeSweepDef {
    fn validate_definition(&self) -> Result<(), String> {
        finite_v3(self.origin, &self.id, "origin")?;
        finite(self.yaw_degrees, &self.id, "yaw_degrees")?;
        positive(self.run, &self.id, "run")?;
        positive(self.width, &self.id, "width")?;
        positive(self.thickness, &self.id, "thickness")?;
        positive(
            self.angle_increment_degrees,
            &self.id,
            "angle_increment_degrees",
        )?;
        non_negative(self.gap, &self.id, "gap")?;
        finite(self.start_angle_degrees, &self.id, "start_angle_degrees")?;
        finite(self.end_angle_degrees, &self.id, "end_angle_degrees")?;
        if self.start_angle_degrees.abs() >= 89.9 || self.end_angle_degrees.abs() >= 89.9 {
            return Err(format!(
                "slope sweep {:?} angles must remain between -89.9 and 89.9 degrees",
                self.id
            ));
        }
        if self.end_angle_degrees < self.start_angle_degrees {
            return Err(format!(
                "slope sweep {:?} end angle must be >= start angle",
                self.id
            ));
        }
        Ok(())
    }
}

impl ValidateDefinition for PillarGridDef {
    fn validate_definition(&self) -> Result<(), String> {
        finite_v3(self.origin, &self.id, "origin")?;
        finite(self.yaw_degrees, &self.id, "yaw_degrees")?;
        positive_usize(self.columns, &self.id, "columns")?;
        positive_usize(self.rows, &self.id, "rows")?;
        positive(self.spacing_x, &self.id, "spacing_x")?;
        positive(self.spacing_z, &self.id, "spacing_z")?;
        positive(self.pillar_width, &self.id, "pillar_width")?;
        positive(self.pillar_depth, &self.id, "pillar_depth")?;
        positive(self.pillar_height, &self.id, "pillar_height")
    }
}

impl ValidateDefinition for MovingBoxDef {
    fn validate_definition(&self) -> Result<(), String> {
        finite_v3(self.position, &self.id, "position")?;
        finite_v3(self.rotation_degrees, &self.id, "rotation_degrees")?;
        finite_v3(self.travel, &self.id, "travel")?;
        positive_v3(self.size, &self.id, "size")?;
        positive(self.period_seconds, &self.id, "period_seconds")?;
        finite(self.phase, &self.id, "phase")
    }
}

impl ValidateDefinition for MarkerDef {
    fn validate_definition(&self) -> Result<(), String> {
        finite_v3(self.position, &self.id, "position")?;
        finite_v3(self.rotation_degrees, &self.id, "rotation_degrees")
    }
}

impl ValidateDefinition for PointLightDef {
    fn validate_definition(&self) -> Result<(), String> {
        finite_v3(self.position, &self.id, "position")?;
        validate_color(self.color, &format!("point light {:?}", self.id))?;
        positive(self.intensity, &self.id, "intensity")?;
        positive(self.range, &self.id, "range")
    }
}

impl ValidateDefinition for DirectionalLightDef {
    fn validate_definition(&self) -> Result<(), String> {
        finite_v3(self.rotation_degrees, &self.id, "rotation_degrees")?;
        validate_color(self.color, &format!("directional light {:?}", self.id))?;
        positive(self.illuminance, &self.id, "illuminance")
    }
}
