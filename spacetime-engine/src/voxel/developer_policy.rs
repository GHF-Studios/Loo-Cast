//! Developer-only reconstructible presentation policy.
//!
//! developer-celestial-height-policy-v1
//!
//! Canonical celestial terrain remains authoritative and unchanged. This module
//! adapts the semantic surface into an optional scripted PRESENTATION surface
//! for the Rhai Developer Lab proving ground.

use bevy::{math::DVec3, prelude::Vec3};

use crate::{
    devtools::DeveloperScalarPolicyRuntime,
    spatial::SpatialScale,
};

use super::CelestialVoxelField;

/// Keep the first live worldgen experiment bounded enough that the existing
/// local clipmap working set can still represent it. This is presentation
/// safety, not a semantic terrain limit.
const MAX_SCRIPTED_RELIEF_METRES: f64 = 100_000.0;
const MAX_SCRIPTED_RELIEF_RADIUS_FRACTION: f64 = 0.10;

fn relief_limit_metres(field: CelestialVoxelField) -> f64 {
    (field.radius_metres() * MAX_SCRIPTED_RELIEF_RADIUS_FRACTION)
        .min(MAX_SCRIPTED_RELIEF_METRES)
        .max(0.0)
}

/// Returns body-local presentation geometry derived from canonical semantic
/// terrain, optionally replacing its radial displacement through a committed
/// Developer Lab scalar policy.
///
/// Input/output semantics for the script are SI metres of radial displacement:
///
///     value = semantic_surface_radius - authored_body_radius
///
/// Runtime errors fall back to canonical semantic displacement for that sample.
pub(crate) fn presentation_surface_local_metres(
    field: CelestialVoxelField,
    direction: Vec3,
    scale: SpatialScale,
    policy: Option<&DeveloperScalarPolicyRuntime>,
) -> Option<DVec3> {
    let semantic = field.surface_local_metres(direction, scale).ok()?;
    if !semantic.is_finite() {
        return None;
    }

    let semantic_radius = semantic.length();
    if !semantic_radius.is_finite() || semantic_radius <= f64::EPSILON {
        return Some(semantic);
    }

    let Some(policy) = policy else {
        return Some(semantic);
    };

    let canonical_height = semantic_radius - field.radius_metres();
    let scripted_height = policy.evaluate(canonical_height).unwrap_or(canonical_height);

    let limit = relief_limit_metres(field);
    let lower = -limit.min(field.radius_metres() * 0.90);
    let upper = limit;
    let height = scripted_height.clamp(lower, upper);
    let radius = (field.radius_metres() + height).max(field.radius_metres() * 0.01);

    let radial_direction = semantic / semantic_radius;
    let transformed = radial_direction * radius;
    transformed.is_finite().then_some(transformed)
}

pub(crate) fn presentation_surface_radius_metres(
    field: CelestialVoxelField,
    direction: Vec3,
    scale: SpatialScale,
    policy: Option<&DeveloperScalarPolicyRuntime>,
) -> Option<f64> {
    presentation_surface_local_metres(field, direction, scale, policy)
        .map(|point| point.length())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::voxel::CelestialBodyProfile;

    #[test]
    fn no_policy_is_exactly_canonical_presentation_surface() {
        let field = CelestialVoxelField::new(
            6_371_000.0,
            SpatialScale::new(6).unwrap(),
            0x4541_5254,
            CelestialBodyProfile::Rocky,
        );
        let direction = Vec3::new(0.3, 0.8, -0.4).normalize();
        let scale = SpatialScale::new(4).unwrap();

        let canonical = field.surface_local_metres(direction, scale).unwrap();
        let presentation =
            presentation_surface_local_metres(field, direction, scale, None).unwrap();
        assert_eq!(presentation, canonical);
    }
}
